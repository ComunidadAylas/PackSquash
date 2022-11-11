//! Contains code to optimize PNG files.

use std::num::NonZeroU16;
use std::{
	borrow::Cow,
	cmp,
	num::{NonZeroU8, TryFromIntError},
	time::Duration
};

use bytes::BytesMut;
use imagequant::{liq_error, Attributes};
use indexmap::IndexSet;
use itertools::Itertools;
use oxipng::{AlphaOptim, Deflaters, Headers, Options, PngError};
use png::EncodingError;
use rgb::{ComponentBytes, FromSlice, RGBA8};
use spng::{ContextFlags, DecodeFlags, Format};
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use super::util::TryGetOrInsertOptionExt;
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
	#[error("Invalid PNG: {0}")]
	StripValidateError(&'static str),
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

		// Using a match expression is easier to understand and expand in the future
		#[allow(clippy::match_like_matches_macro)]
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
			// These textures may be used to pass data to shaders: their RGB values may
			// be arbitrarily used for computation. Leave them untouched
			PackFileAssetType::AuxiliaryShaderTargetTexture => false,
			_ => !self.optimization_settings.skip_alpha_optimizations
		};
		let is_auxiliary_shader_target_texture = matches!(
			self.asset_type,
			PackFileAssetType::AuxiliaryShaderTargetTexture
		);
		let color_quantization_target = self.optimization_settings.color_quantization_target;

		// First pass: strip non-critical PNG chunks we won't use. At worst this does nothing
		// to the input PNG, and at best it reduces its size, reducing memory requirements for
		// the next passes. It's relatively cheap to do this, although not free
		let first_pass_png = strip_unnecessary_chunks(
			src.split_off(0),
			self.optimization_settings.maximum_width_and_height
		)?;

		// Second pass: downsize most textures that consist of a single color to the minimum
		// size that does not cause side effects in Minecraft. If that can't be done, then
		// perform quantization if desired and useful (i.e., there are more pixels than
		// possible colors, and no option stops us from doing so). It does not make sense to
		// do both downsizing and quantization: there are no colors to quantize when downsizing
		// is successful, and quantizing is only useful when the image has many colors
		let second_pass_quality;
		let second_pass_pixel_data_size;
		let must_use_second_pass_result;
		let second_pass_result = {
			// Now it's a good time to read the PNG IHDR and other information chunks to validate
			// it. We should do this check no matter what the next passes are
			let (png_info, mut png_reader) = spng::Decoder::new(&*first_pass_png)
				.with_decode_flags(DecodeFlags::GAMMA | DecodeFlags::TRANSPARENCY)
				.with_context_flags(ContextFlags::IGNORE_ADLER32)
				.with_output_format(Format::Rgba8)
				.read_info()?;

			// Hold and lazily read PNG pixel data
			let mut png_pixel_data = None;
			let mut read_pixel_data = || -> Result<_, OptimizationError> {
				let mut pixels = vec![0; png_info.buffer_size];
				png_reader.next_frame(&mut pixels)?;

				Ok((
					pixels,
					NonZeroU16::new(png_info.width as u16).unwrap(),
					NonZeroU16::new(png_info.height as u16).unwrap()
				))
			};

			match if self.optimization_settings.downsize_if_single_color {
				let pixel_data = png_pixel_data.try_get_or_insert_with(&mut read_pixel_data)?;

				downsize_single_color(
					pixel_data.0.as_rgba(),
					pixel_data.1,
					pixel_data.2,
					can_change_color_type,
					is_auxiliary_shader_target_texture
				)?
				.map(|downsized_png| (downsized_png, None))
			} else {
				None
			} {
				Some(downsized_png) => {
					// Downsizing was possible and successful

					// Pessimistic estimation. The exact size does not matter in this case
					second_pass_pixel_data_size = 256;
					must_use_second_pass_result = false;

					Some(downsized_png)
				}
				None if color_quantization_target.should_quantize()
					&& png_info.width * png_info.height > color_quantization_target.max_colors()
					&& can_change_color_type
					&& can_change_transparent_pixel_colors =>
				{
					// The image could not be downsized, but it can be quantized

					// At most one byte per pixel when using color palettes
					second_pass_pixel_data_size = png_info.width.saturating_mul(png_info.height);
					must_use_second_pass_result =
						color_quantization_target.is_quantization_required();

					let pixel_data = png_pixel_data.try_get_or_insert_with(&mut read_pixel_data)?;
					Some(color_quantize(
						pixel_data.0.as_rgba(),
						pixel_data.1,
						pixel_data.2,
						color_quantization_target,
						self.optimization_settings
							.color_quantization_dithering_level
							.into()
					)?)
					.map(|(quantized_png, quality_value)| (quantized_png, Some(quality_value)))
				}
				None => {
					// No downsizing or quantization to do

					second_pass_pixel_data_size = (png_info.color_type.samples() as u32
						* ((png_info.bit_depth as u32 + 15) / 8 - 1))
						.saturating_mul(png_info.width)
						.saturating_mul(png_info.height);
					must_use_second_pass_result = false;

					None
				}
			}
		};

		// Third pass: complete lossless optimization of the second pass PNG, if quantization
		// or downsizing was done, or else the first pass PNG
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
		// pass, quantization or downsizing. The first pass never yields a bigger PNG than the
		// input PNG, but the third pass might do so in some edge cases, no matter if it optimized
		// the result of the first or second pass:
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
		// - Downsizing yields smaller files except in some extreme edge cases due to the
		//   heuristics used by the optimized in the third pass being inappropriate. This may be
		//   the case in grayscale images, for example.
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
		let (optimized_png, optimization_strategy_message) =
			if !must_use_second_pass_result && first_pass_png.len() < third_pass_png.len() {
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
					if let Some(Some(quantization_pass_quality)) = second_pass_quality {
						Cow::Owned(format!(
							"Optimized with {}% quality color quantization",
							quantization_pass_quality
						))
					} else if let Some(None) = second_pass_quality {
						Cow::Borrowed("Downsized and optimized")
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
///
/// While doing so, it also validates that neither image dimension exceeds the passed
/// threshold.
fn strip_unnecessary_chunks(
	input_png: BytesMut,
	maximum_dimension: u16
) -> Result<Vec<u8>, OptimizationError> {
	let mut stripped_png = Vec::with_capacity(input_png.len());

	// Helper macro to avoid non-panicking bounds checking verbosity
	static TOO_SMALL_ERROR_MESSAGE: &str =
		"The file is smaller than expected. Is it invalid or corrupt?";
	macro_rules! get_or_err {
		($range:expr) => {
			input_png
				.get($range)
				.ok_or(OptimizationError::StripValidateError(
					TOO_SMALL_ERROR_MESSAGE
				))?
		};
	}

	// The format of PNG files is dead simple: just a signature in the beginning
	// followed by as many chunks as desired. OxiPNG supports stripping chunks,
	// but it's amazing that implementing this ourselves takes barely the same
	// LOC than calling OxiPNG with the suitable options, and that there is no
	// nice library at crates.io to do this simple optimization. OxiPNG is also
	// pretty slow because it insists on decoding the image anyway. So do it
	// ourselves and strip every chunk that we know won't be of any use.
	// Normative reference: https://www.w3.org/TR/PNG/

	// Check and copy the signature
	let signature = get_or_err!(..8);
	if signature != [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
		return Err(OptimizationError::StripValidateError(
			"PNG signature not found. Is it really a PNG file?"
		));
	}
	stripped_png.extend_from_slice(signature);

	// Now copy the known-necessary chunks only. We barely do any validation on them
	// apart from checking whether the image is too big and what's necessary to do
	// the copy. spng and OxiPNG will take care of further validation
	let mut i = 8;
	while i < input_png.len() {
		let data_length_and_chunk_type = get_or_err!(i..i + 8);
		let data_length =
			u32::from_be_bytes(data_length_and_chunk_type[..4].try_into().unwrap()) as usize;
		let chunk_type = &data_length_and_chunk_type[4..];

		if matches!(
			chunk_type,
			// gAMA may be used by spng later on. PLTE is necessary for palette color images,
			// which may have its transparency stored in a tRNS chunk. IHDR, IDAT and IEND
			// are critical and must appear
			b"IHDR" | b"IDAT" | b"IEND" | b"PLTE" | b"tRNS" | b"gAMA"
		) {
			let chunk_data = get_or_err!(i + 8..i + 8 + data_length);
			let chunk_crc = get_or_err!(i + 8 + data_length..i + 8 + data_length + 4);

			if chunk_type == b"IHDR" {
				if data_length != 13 {
					return Err(OptimizationError::StripValidateError(
						"Unexpected size for IHDR chunk"
					));
				}

				let width = u32::from_be_bytes(chunk_data[..4].try_into().unwrap());
				let height = u32::from_be_bytes(chunk_data[4..8].try_into().unwrap());

				if width > maximum_dimension as u32 || height > maximum_dimension as u32 {
					return Err(OptimizationError::StripValidateError(
						"The texture width or height exceeds the configured maximum size. \
						More information: <https://packsquash.page.link/Too-big-PNG-help>"
					));
				}
			}

			stripped_png.extend_from_slice(data_length_and_chunk_type);
			stripped_png.extend_from_slice(chunk_data);
			stripped_png.extend_from_slice(chunk_crc);
		}

		i += 8 + data_length + 4;

		// No chunks can follow IEND in valid files, so stop once it's found
		if chunk_type == b"IEND" {
			break;
		}
	}

	// If we didn't consume all the input PNG bytes and didn't error out it's because
	// IEND was reached, but there are trailing bytes at the end of the file. This
	// explicitly violates the PNG standard. Reject such PNG files to future-proof
	// ourselves and the users against decoders that are not lenient (they are hard to
	// find, but exist)
	if i < input_png.len() {
		return Err(OptimizationError::StripValidateError(
			"Non-conforming trailing bytes at the end of the file. \
			Please re-export with a known-good PNG encoder"
		));
	}

	Ok(stripped_png)
}

/// Downsizes an image to the most space-efficient dimensions if it is single-color and such
/// resizing is not expected to impact how the pack looks. This may significantly decrease
/// file sizes and improve client stitching performance and memory usage in extreme cases.
/// However, there are other extreme cases where this might cause some breakage.
fn downsize_single_color<'pixels, P: Into<Cow<'pixels, [RGBA8]>>>(
	pixels: P,
	width: NonZeroU16,
	height: NonZeroU16,
	can_change_color_type: bool,
	is_auxiliary_shader_target_texture: bool
) -> Result<Option<Vec<u8>>, OptimizationError> {
	let pixels = pixels.into();
	let dimension = cmp::min(width, height);
	let minimum_mipmap_level_keeping_dimension =
		NonZeroU16::new(cmp::min(1u32 << dimension.trailing_zeros(), 16) as u16).unwrap();

	// We need to be careful with undesired side effects of downsizing textures:
	// - If they belong to a procedurally-generated atlas, downsizing them too much limits the
	//   maximum mipmap level for other textures in the atlas too, potentially degrading visual
	//   quality.
	// - In general, it is unsafe to downsize textures used by shaders. They may use functions
	//   such as texelFetch to access raw texel coordinates, whose behavior is undefined outside
	//   valid texture coordinates. Luckily, vanilla shaders can't rely on other kinds of textures
	//   they can read having fixed sizes.
	// - Downsizing animated textures may turn them too small for Minecraft to extract the required
	//   frames from them. This should be handled, but right now it isn't because it requires
	//   accessing other pack files from here. In practice, however, it makes little sense to
	//   animate a single-color texture. FIXME
	// - Single-color custom font textures would break, although one would question the usefulness
	//   of a font that cannot display anything, so this shouldn't matter too much in practice. FIXME
	if minimum_mipmap_level_keeping_dimension < dimension
		&& can_change_color_type
		&& !is_auxiliary_shader_target_texture
		&& pixels.iter().all_equal()
	{
		// Truecolor format may be smaller than palette format due to less header chunk overhead.
		// Try both and choose the smallest; encoding at most 16x16 PNGs is very fast
		let palette_png = encode_intermediate_palette_png(
			&[pixels[0]],
			&vec![
				0;
				minimum_mipmap_level_keeping_dimension.get() as usize
					* minimum_mipmap_level_keeping_dimension.get() as usize
			],
			minimum_mipmap_level_keeping_dimension,
			minimum_mipmap_level_keeping_dimension
		)?;
		let truecolor_png = encode_intermediate_truecolor_png(
			&vec![
				pixels[0];
				minimum_mipmap_level_keeping_dimension.get() as usize
					* minimum_mipmap_level_keeping_dimension.get() as usize
			],
			minimum_mipmap_level_keeping_dimension,
			minimum_mipmap_level_keeping_dimension
		)?;

		Ok(Some(if palette_png.len() < truecolor_png.len() {
			palette_png
		} else {
			truecolor_png
		}))
	} else {
		Ok(None)
	}
}

/// Performs color quantization on the image yielded by the provided PNG reader,
/// according to the specified parameters, and returns the resulting color-quantized
/// PNG, barely compressing it to save time, as it is expected for this PNG to be
/// fed to another optimizer. In some edge cases, even in combination with another
/// optimizer, this may be a size-increasing operation, depending on the dithering
/// pattern compressibility and how optimal the input image already was.
fn color_quantize<'pixels, P: Into<Cow<'pixels, [RGBA8]>>>(
	pixels: P,
	width: NonZeroU16,
	height: NonZeroU16,
	quantization_target: ColorQuantizationTarget,
	dithering_level: f32
) -> Result<(Vec<u8>, u8), OptimizationError> {
	// Compute the quantized palette in a subscope to free temporary buffers as
	// soon as possible
	let mut quantization_result;
	let (palette, pixel_palette_indexes) = {
		// Set the quantization attributes
		let mut quantization_attributes = Attributes::new();
		quantization_attributes.set_max_colors(quantization_target.max_colors())?;
		quantization_attributes.set_speed(2)?;
		quantization_attributes.set_quality(0, 100)?;

		// Wrap the pixel data in a iq image
		let pixels = pixels.into();
		let mut iq_image = quantization_attributes.new_image_borrowed(
			&pixels,
			width.get() as usize,
			height.get() as usize,
			0.0 // sRGB
		)?;

		// Generate quantized palette
		quantization_result = quantization_attributes.quantize(&mut iq_image)?;
		quantization_result.set_dithering_level(dithering_level)?;

		quantization_result.remapped(&mut iq_image)?
	};

	Ok((
		encode_intermediate_palette_png(&palette, &pixel_palette_indexes, width, height)?,
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
	let zopfli_iterations_model = ZopfliIterationsTimeModel::new(zopfli_compression_iterations, 2.0);

	let optimization_options = Options {
		alphas: if !can_change_transparent_pixel_colors {
			IndexSet::new()
		} else {
			let mut alpha_optimizations = IndexSet::with_capacity(6);
			alpha_optimizations.insert(AlphaOptim::Black);
			alpha_optimizations.insert(AlphaOptim::Left);
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
		deflate: match zopfli_iterations_model.iterations_for_data_size(
			// OxiPNG does one Zopfli compression attempt per PNG filter configured below,
			// and returns the combination that yields the best result, so it's desired to
			// consider that the data size was multiplied by the number of filters to bound
			// execution time properly
			pixel_data_size.saturating_mul(3),
			0,
			15
		) {
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
			const PIXEL_DATA_FILTERS: [u8; 3] = [
				0, // No filter (surprisingly good)
				4, // Paeth
				5  // Per-scanline filter chosen with MAD heuristic
			];

			let mut pixel_data_filters = IndexSet::with_capacity(PIXEL_DATA_FILTERS.len());
			for filter in PIXEL_DATA_FILTERS {
				pixel_data_filters.insert(filter);
			}

			pixel_data_filters
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
		use_heuristics: false,
		check: false
	};

	oxipng::optimize_from_memory(&input_png, &optimization_options)
}

/// Generates a PNG in truecolor format with the specified pixels, width and height.
fn encode_intermediate_truecolor_png(
	pixels: &[RGBA8],
	width: NonZeroU16,
	height: NonZeroU16
) -> Result<Vec<u8>, OptimizationError> {
	// FIXME refactor this when we have a global memory budget
	let mut png_buf = Vec::with_capacity(
		// IDAT payload
		width.get() as usize * height.get() as usize * 4 +
		// Length, chunk type and chunk CRC for each chunk (IHDR, IDAT, IEND)
		(4 + 4 + 4) * 3 +
		// PNG signature
		8
	);
	let mut png_encoder = png::Encoder::new(&mut png_buf, width.get() as u32, height.get() as u32);
	png_encoder.set_color(png::ColorType::Rgba);
	png_encoder.set_depth(png::BitDepth::Eight);
	png_encoder.set_compression(png::Compression::Fast);
	png_encoder.set_filter(png::FilterType::NoFilter);

	// Now write the image data
	png_encoder
		.write_header()?
		.write_image_data(pixels.as_bytes())?;

	Ok(png_buf)
}

/// Generates a PNG in palette color format with the specified palette, per-pixel palette
/// indexes, width and height.
fn encode_intermediate_palette_png(
	palette: &[RGBA8],
	pixel_palette_indexes: &[u8],
	width: NonZeroU16,
	height: NonZeroU16
) -> Result<Vec<u8>, OptimizationError> {
	// FIXME refactor this when we have a global memory budget
	let mut png_buf = Vec::with_capacity(
		// IDAT payload
		width.get() as usize * height.get() as usize +
		// tRNS and PLTE chunks payload
		4 * palette.len() +
		// Length, chunk type and chunk CRC for each chunk (IHDR, IDAT, tRNS, PLTE, IEND)
		(4 + 4 + 4) * 5 +
		// PNG signature
		8
	);
	let mut png_encoder = png::Encoder::new(&mut png_buf, width.get() as u32, height.get() as u32);
	png_encoder.set_color(png::ColorType::Indexed);
	png_encoder.set_depth(png::BitDepth::Eight);
	png_encoder.set_compression(png::Compression::Fast);
	png_encoder.set_filter(png::FilterType::NoFilter);

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
	// 8 bits, the palette indexes can be used as-is. This is not
	// space efficient for lower bit depths, but OxiPNG should take
	// care of reducing this
	png_encoder
		.write_header()?
		.write_image_data(pixel_palette_indexes)?;

	Ok(png_buf)
}
