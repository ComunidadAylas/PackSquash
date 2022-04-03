//! Contains code to optimize PNG files.

use std::{
	borrow::Cow,
	lazy::SyncLazy,
	num::{NonZeroU8, TryFromIntError},
	time::Duration
};

use bytes::BytesMut;
use imagequant::{liq_error, Attributes};
use indexmap::IndexSet;
use oxipng::{AlphaOptim, Deflaters, Headers, Options, PngError};
use png::{BitDepth, ColorType, Compression, EncodingError, FilterType};
use rgb::FromSlice;
use spng::{ContextFlags, DecodeFlags, Format, Limits, OutputInfo};
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use crate::{
	config::{ColorQuantizationTarget, PngFileOptions},
	zopfli_iterations_time_model::ZopfliIterationsTimeModel
};

use super::{AsyncReadAndSizeHint, PackFile, PackFileAssetType, PackFileConstructor};

#[cfg(test)]
mod tests;

/// Represents a resource pack PNG image file, which is used for in-game textures.
///
/// The optimization process may be customized via [PngFileOptions].
pub struct PngFile<T: AsyncRead + Send + Unpin + 'static> {
	read: T,
	file_length_hint: usize,
	asset_type: PackFileAssetType,
	optimization_settings: PngFileOptions
}

/// Optimizer decoder that transforms PNG files to an optimized representation.
pub struct OptimizerDecoder {
	asset_type: PackFileAssetType,
	optimization_settings: PngFileOptions,
	reached_eof: bool
}

/// Represents an error that may happen while optimizing PNG files.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OptimizationError {
	#[error("PNG decode error: {0}")]
	PngDecoding(#[from] spng::Error),
	#[error("PNG encode error: {0}")]
	PngEncoding(#[from] EncodingError),
	#[error("OxiPNG optimization error: {0}")]
	OxipngOptimization(#[from] PngError),
	#[error("libimagequant optimization error: {0}")]
	LibimagequantOptimization(#[from] liq_error),
	#[error("Integer overflow error: {0}")]
	Overflow(#[from] TryFromIntError),
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error)
}

// FIXME: actual framing?
// (i.e. do not hold the entire file in memory before decoding, so that frame != file)
impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, Vec<u8>);
	type Error = OptimizationError;

	fn decode(&mut self, _: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		Ok(None)
	}

	fn decode_eof(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		// This method will be called when EOF is reached until it returns None. Because we
		// will only ever output a single item in the stream, always return None if we have
		// executed once already
		if self.reached_eof {
			return Ok(None);
		}
		self.reached_eof = true;

		let can_change_color_type = match self.asset_type {
			PackFileAssetType::BannerLayer
				if self
					.optimization_settings
					.working_around_color_type_change_quirk =>
			{
				false
			}
			_ => true
		};
		let can_change_transparent_pixel_colors = match self.asset_type {
			PackFileAssetType::EyeLayer
				if self
					.optimization_settings
					.working_around_transparent_pixel_colors_change_quirk =>
			{
				false
			}
			_ => !self.optimization_settings.skip_alpha_optimizations
		};
		let color_quantization_target = self.optimization_settings.color_quantization_target;

		// First pass: strip non-critical PNG chunks we won't use. At worst this does nothing
		// to the input PNG, and at best it reduces its size, reducing memory requirements for
		// the next passes. It's relatively cheap to do this, although not free
		let first_pass_png = strip_unnecessary_chunks(src.split_off(0))?;

		// Second pass: perform quantization if desired and if useful (i.e., there are more pixels
		// than possible colors, and no option stops us from doing so)
		let second_pass_quality;
		let second_pass_pixel_data_size;
		let second_pass_result = {
			// Now it's a good time to read the PNG IHDR and other information chunks to validate
			// it. We should do this check no matter if the image is quantized or not
			let (png_info, png_reader) = spng::Decoder::new(&*first_pass_png)
				.with_limits(Limits {
					max_width: self.optimization_settings.maximum_width_and_height as u32,
					max_height: self.optimization_settings.maximum_width_and_height as u32
				})
				.with_decode_flags(DecodeFlags::GAMMA | DecodeFlags::TRANSPARENCY)
				.with_context_flags(ContextFlags::IGNORE_ADLER32)
				.with_output_format(Format::Rgba8)
				.read_info()?;

			if color_quantization_target.should_quantize()
				&& png_info.width * png_info.height > color_quantization_target.max_colors()
				&& can_change_color_type
				&& can_change_transparent_pixel_colors
			{
				// At most one byte per pixel when using color palettes
				second_pass_pixel_data_size = png_info.width.saturating_mul(png_info.height);

				Some(color_quantize(
					png_reader,
					png_info,
					color_quantization_target,
					self.optimization_settings
						.color_quantization_dithering_level
						.into()
				)?)
			} else {
				second_pass_pixel_data_size = (png_info.color_type.samples() as u32
					* ((png_info.bit_depth as u32 + 15) / 8 - 1))
					.saturating_mul(png_info.width)
					.saturating_mul(png_info.height);

				None
			}
		};

		// Third pass: complete lossless optimization of the second pass PNG, if quantization
		// was done, or else the first pass PNG
		let third_pass_png = visually_lossless_optimize(
			if let Some((second_pass_png, second_pass_quality_value)) = second_pass_result {
				second_pass_quality = Some(second_pass_quality_value);
				Cow::Owned(second_pass_png)
			} else {
				second_pass_quality = None;
				Cow::Borrowed(&first_pass_png)
			},
			second_pass_pixel_data_size,
			self.optimization_settings.image_data_compression_iterations,
			can_change_color_type,
			can_change_transparent_pixel_colors,
			!self
				.optimization_settings
				.working_around_grayscale_reduction_quirk
		)?;

		// Now decide the result of what pass to keep. The third pass is either an optimized
		// representation of the first pass, or an optimized representation of the second
		// pass, quantization. The first pass never yields a bigger PNG than the input PNG,
		// but the third pass might do so in some edge cases, no matter if it optimized the
		// first or second pass:
		//
		// - The input PNG may be already encoded in such a way that the heuristics used by
		//   our optimizers do not hold: it is extremely compressible by a compressor we don't
		//   use, for some reason doing some color format change yielded a worse size overall
		//   due to poorer compressibility, etc. This is relevant when any of both passes are
		//   used as input for the third pass.
		// - Color quantization may introduce noisy areas that are harder to compress, depending
		//   on the dithering intensity and the image colors. This bloats space quite a bit in
		//   PNG files with big areas of flat colors that turned into big areas of noisy
		//   dithering. This is relevant when the second pass is used as input for the third
		//   pass.
		//
		// We can't do much about the first point, but the second point may be handled by
		// retrying the third pass with the first pass as input, or varying the input parameters
		// for the second pass (especially dithering). However, doing so is very slow, because
		// the third pass is what takes the most execution time, and "drafting" its results by
		// using faster compression algorithms may lead to non-optimal decisions when Zopfli
		// compression is used for the PNG IDAT chunk.
		//
		// As a practical compromise, just return the result of the first pass if the third pass
		// turned out to yield a bigger PNG file, if quantization is not explicitly forced by
		// the user. While this has the desired properties of never increasing the input PNG file
		// size when quantization is not forced and executing each pass a single time, explicit
		// user configuration of the quantization parameters may be needed to achieve the most
		// optimal PNG we are capable of. Luckily, the points above are fairly rare
		let (optimized_png, optimization_strategy_message) = if !color_quantization_target
			.is_quantization_required()
			&& first_pass_png.len() < third_pass_png.len()
		{
			(
				first_pass_png,
				Cow::Borrowed(
					"Barely optimized. \
					If not optimized externally, try tweaking options for extra savings"
				)
			)
		} else {
			(
				third_pass_png,
				if let Some(quantization_pass_quality) = second_pass_quality {
					Cow::Owned(format!(
						"Optimized with {}% quality color quantization",
						quantization_pass_quality
					))
				} else if can_change_transparent_pixel_colors {
					Cow::Borrowed("Optimized with no visible color loss")
				} else {
					Cow::Borrowed("Optimized")
				}
			)
		};

		Ok(Some((optimization_strategy_message, optimized_png)))
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFile for PngFile<T> {
	type ByteChunkType = Vec<u8>;
	type OptimizationError = OptimizationError;
	type OptimizedByteChunksStream = FramedRead<T, OptimizerDecoder>;

	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				asset_type: self.asset_type,
				optimization_settings: self.optimization_settings,
				reached_eof: false
			},
			self.file_length_hint
		)
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFileConstructor<T> for PngFile<T> {
	type OptimizationSettings = PngFileOptions;

	fn new(
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<T>>,
		asset_type: PackFileAssetType,
		optimization_settings: Self::OptimizationSettings
	) -> Option<Self> {
		file_read_producer().map(|(read, file_length_hint)| Self {
			read,
			// The file is too big to fit in memory if this conversion fails anyway
			file_length_hint: file_length_hint.try_into().unwrap_or(usize::MAX),
			asset_type,
			optimization_settings
		})
	}
}

/// Performs a single, fast optimization to an input PNG image: removing non-critical
/// chunks that will not be parsed by any expected downstream decoder. This can never
/// increase the input PNG image size, only decrease or maintain it.
fn strip_unnecessary_chunks(input_png: BytesMut) -> Result<Vec<u8>, PngError> {
	/// OxiPNG configuration for doing the aforementioned optimization.
	static ONLY_STRIP_CHUNKS_OPTIONS: SyncLazy<Options> = SyncLazy::new(|| {
		Options {
			backup: false,
			fix_errors: true, // Ignore CRC for speed. We assume a reliable data source
			pretend: false,
			force: true,
			preserve_attrs: false,
			// The following options configure OxiPNG to not touch the image data (IDAT chunk)
			// in any way, where the most optimizations happen
			filter: IndexSet::new(),
			interlace: None,
			alphas: IndexSet::new(),
			bit_depth_reduction: false,
			color_type_reduction: false,
			palette_reduction: false,
			grayscale_reduction: false,
			idat_recoding: false,
			// Strip headers that neither we or Minecraft will ever use. gAMA and tRNS may
			// be used by our spng decoder
			strip: Headers::Keep(IndexSet::from_iter(
				[String::from("gAMA"), String::from("tRNS")].into_iter()
			)),
			// Ignored due to idat_recoding == false
			deflate: Deflaters::Zopfli {
				iterations: NonZeroU8::new(1).unwrap()
			},
			use_heuristics: false,
			timeout: None
		}
	});

	oxipng::optimize_from_memory(&input_png, &ONLY_STRIP_CHUNKS_OPTIONS)
}

/// Performs color quantization on the image yielded by the provided PNG reader,
/// according to the specified parameters, and returns the resulting color-quantized
/// PNG, barely compressing it to save time, as it is expected for this PNG to be
/// fed to another optimizer. In some edge cases, even in combination with another
/// optimizer, this may be a size-increasing operation, depending on the dithering
/// pattern compressibility and how optimal the input image already was.
fn color_quantize<R>(
	mut png_reader: spng::Reader<R>,
	png_info: OutputInfo,
	quantization_target: ColorQuantizationTarget,
	dithering_level: f32
) -> Result<(Vec<u8>, u8), OptimizationError> {
	let png_width = png_info.width.try_into()?;
	let png_height = png_info.height.try_into()?;

	// Compute the quantized palette in a subscope to free temporary buffers as
	// soon as possible
	let mut quantization_result;
	let (palette, pixel_palette_indexes) = {
		// Decode pixel data to a RGBA8 buffer
		let mut original_pixels = vec![0; png_info.buffer_size];
		png_reader.next_frame(&mut original_pixels)?;

		// Set the quantization attributes
		let mut quantization_attributes = Attributes::new();
		quantization_attributes.set_max_colors(quantization_target.max_colors())?;
		quantization_attributes.set_speed(2)?;
		quantization_attributes.set_quality(0, 100)?;

		// Wrap the pixel data in a iq image
		let mut iq_image = quantization_attributes.new_image_borrowed(
			original_pixels.as_rgba(),
			png_width,
			png_height,
			0.0 // sRGB
		)?;

		// Generate quantized palette
		quantization_result = quantization_attributes.quantize(&mut iq_image)?;
		quantization_result.set_dithering_level(dithering_level)?;

		quantization_result.remapped(&mut iq_image)?
	};

	// Set up a fast encoder for a temporary PNG that will hold the result
	let mut quantized_png_buf = Vec::with_capacity(
		// IDAT payload
		png_width * png_height +
		// tRNS and PLTE chunks payload
		4 * palette.len() +
		// Length, chunk type and chunk CRC for each chunk (IHDR, IDAT, tRNS, PLTE, IEND)
		(4 + 4 + 4) * 5 +
		// PNG signature
		8
	);
	let mut png_encoder = png::Encoder::new(&mut quantized_png_buf, png_info.width, png_info.height);
	png_encoder.set_color(ColorType::Indexed);
	png_encoder.set_depth(BitDepth::Eight);
	png_encoder.set_compression(Compression::Fast);
	png_encoder.set_filter(FilterType::NoFilter);

	// Now we need to generate the PLTE and tRNS headers. According to
	// http://www.libpng.org/pub/png/spec/1.2/PNG-Chunks.html:
	// - PLTE:
	// Palette color (entry) 0: R (1 byte), G (1 byte), B (1 byte)
	// Palette color (entry) 1: R (1 byte), G (1 byte), B (1 byte)
	// ... (up to 256 entries, minimum 1, no more than 2^depth)
	// - tRNS:
	// Alpha for palette color 0, 1 byte
	// Alpha for palette color 1, 1 byte
	// ... (up to 256 entries, less or equal than PLTE entries. If less, 255 alpha is assumed)
	// The algorithm here is naive and not space efficient, but it's
	// fast. OxiPNG should take care of optimizing this
	let mut plte_chunk = Vec::with_capacity(3 * palette.len());
	let mut trns_chunk = Vec::with_capacity(palette.len());
	for entry in palette {
		plte_chunk.push(entry.r);
		plte_chunk.push(entry.g);
		plte_chunk.push(entry.b);
		trns_chunk.push(entry.a);
	}
	png_encoder.set_palette(plte_chunk);
	png_encoder.set_trns(trns_chunk);

	// Now write the image data. Because we set the pixel depth to
	// 8 bits, the palette indexes returned by imagequant can be
	// used directly. This is not space efficient for lower bit depths,
	// but OxiPNG should take care of reducing this
	png_encoder
		.write_header()?
		.write_image_data(&pixel_palette_indexes)?;

	Ok((
		quantized_png_buf,
		quantization_result.quantization_quality().unwrap()
	))
}

/// Visually losslessly optimizes the specified input PNG: any visible (i.e., non
/// completely transparent) pixel will be decoded to exactly the same color and
/// transparency values. This may be a pessimization in extreme cases where the
/// input PNG was compressed better than we can came up with, or if the optimizer
/// assumptions turn out to be wrong (for example, palette format is not always
/// more efficient than RGBA format).
fn visually_lossless_optimize(
	input_png: Cow<'_, [u8]>,
	pixel_data_size: u32,
	zopfli_compression_iterations: u8,
	can_change_color_type: bool,
	can_change_transparent_pixel_colors: bool,
	can_convert_to_grayscale: bool
) -> Result<Vec<u8>, PngError> {
	let zopfli_iterations_model =
		ZopfliIterationsTimeModel::new(zopfli_compression_iterations, 7.0 / 8.0);

	let optimization_options = Options {
		alphas: if !can_change_transparent_pixel_colors {
			IndexSet::new()
		} else {
			let mut alpha_optimizations = IndexSet::with_capacity(6);
			alpha_optimizations.insert(AlphaOptim::Black);
			alpha_optimizations.insert(AlphaOptim::Down);
			alpha_optimizations.insert(AlphaOptim::Left);
			alpha_optimizations.insert(AlphaOptim::Right);
			alpha_optimizations.insert(AlphaOptim::Up);
			alpha_optimizations.insert(AlphaOptim::White);

			alpha_optimizations
		},
		backup: false,
		bit_depth_reduction: can_change_color_type,
		color_type_reduction: can_change_color_type,
		// Compute an appropriate number of Zopfli compression iterations using our
		// model. If the number of iterations drops to zero, switch to the much faster,
		// but not so space-efficient, cloudflare-zlib deflater
		deflate: match zopfli_iterations_model
			.iterations_for_data_size(
				// OxiPNG does one Zopfli compression attempt per PNG filter configured below,
				// and returns the combination that yields the best result, so it's desired to
				// consider that the data size was multiplied by the number of filters to bound
				// execution time properly
				pixel_data_size.saturating_mul(3),
				0,
				23
			)
			.saturating_sub(8)
		{
			0 => Deflaters::Zlib {
				compression: {
					// Use the maximum compression level for the best compression.
					// This is still acceptably fast for bigger images of realistic
					// sizes
					let mut levels = IndexSet::with_capacity(1);
					levels.insert(9);

					levels
				},
				strategies: {
					// Try every Zlib strategy to get the best compression possible
					let mut strategies = IndexSet::with_capacity(4);
					for i in 0..=3 {
						strategies.insert(i);
					}

					strategies
				},
				window: 15
			},
			zopfli_iterations => Deflaters::Zopfli {
				iterations: NonZeroU8::new(zopfli_iterations).unwrap()
			}
		},
		filter: {
			const OPTIMIZATION_FILTERS: [u8; 3] = [
				0, // No filter (surprisingly good)
				4, // Paeth
				5  // Per-scanline filter chosen with MAD heuristic
			];

			let mut optimization_filters = IndexSet::with_capacity(OPTIMIZATION_FILTERS.len());
			for filter in OPTIMIZATION_FILTERS {
				optimization_filters.insert(filter);
			}

			optimization_filters
		},
		fix_errors: true, // Ignore CRC for speed. We assume a reliable data source
		force: false,     // Give up with the second pass result if we can't reduce size further
		idat_recoding: true,
		interlace: Some(0), // No interlacing (smaller file size)
		palette_reduction: true,
		grayscale_reduction: can_convert_to_grayscale && can_change_color_type,
		preserve_attrs: false,
		pretend: false,
		strip: Headers::All,
		timeout: Some(Duration::from_secs(600)), // Bail out if the optimization takes too long
		use_heuristics: false
	};

	oxipng::optimize_from_memory(&input_png, &optimization_options)
}
