use std::{
	borrow::Cow,
	convert::TryInto,
	num::{NonZeroU8, TryFromIntError},
	time::Duration
};

use bytes::{BufMut, BytesMut};
use imagequant::{liq_error, Attributes, Image};
use indexmap::IndexSet;
use oxipng::{AlphaOptim, Deflaters, Headers, Options, PngError};
use png::{BitDepth, ColorType, Compression, EncodingError, FilterType};
use rgb::FromSlice;
use spng::{ContextFlags, DecodeFlags, Format, Limits};
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use crate::{config::PngFileOptions, zopfli_iterations_time_model::ZopfliIterationsTimeModel};

use super::{
	util::to_ascii_lowercase_extension, OptimizedBytes, PackFile, PackFileConstructor,
	PackFileConstructorArgs
};

#[cfg(test)]
mod tests;

/// Represents a resource pack PNG image file, which is used for in-game textures.
///
/// The optimization process may be customized via [PngFileOptions].
pub struct PngFile<T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length_hint: usize,
	optimization_settings: PngFileOptions
}

/// Optimizer decoder that transforms PNG files to an optimized representation.
pub struct OptimizerDecoder {
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
	type Item = (Cow<'static, str>, OptimizedBytes<Vec<u8>>);
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

		let zopfli_iterations_time_model = ZopfliIterationsTimeModel::new(
			self.optimization_settings.image_data_compression_iterations,
			7.0 / 8.0
		);
		let color_quantization_target = self.optimization_settings.color_quantization_target;
		let maximum_colors = color_quantization_target.max_colors();
		let quality_description;

		// Read the PNG IHDR and other information chunks, to validate it
		let (image_info, mut png_reader) = spng::Decoder::new(&**src)
			.with_limits(Limits {
				max_width: self.optimization_settings.maximum_width_and_height as u32,
				max_height: self.optimization_settings.maximum_width_and_height as u32
			})
			.with_decode_flags(DecodeFlags::GAMMA | DecodeFlags::TRANSPARENCY)
			.with_context_flags(ContextFlags::IGNORE_ADLER32)
			.with_output_format(Format::Rgba8)
			.read_info()?;

		// The size of the raw pixel data for this image. This is used to estimate
		// how much data is going to be compressed during PNG file optimizations
		let raw_pixel_data_size = image_info
			.color_type
			.samples()
			.saturating_mul(image_info.bit_depth as usize)
			.saturating_mul(image_info.width as usize)
			.saturating_mul(image_info.height as usize);

		let oxipng_input_buf;

		// Perform quantization if desired and if useful
		// (i.e. there are more pixels than possible colors)
		if color_quantization_target.should_quantize()
			&& image_info.width * image_info.height > maximum_colors
		{
			let mut quantization_result;

			// Compute the quantized palette in a subscope to free
			// temporary buffers as soon as possible
			let (palette, pixel_palette_indexes) = {
				// Decode pixel data to a RGBA8 buffer
				let mut original_pixels = vec![0; image_info.buffer_size];
				png_reader.next_frame(&mut original_pixels)?;

				// Dispose of the PNG reader, so it no longer borrows the src buffer
				drop(png_reader);

				// Get the quantization attributes to use
				let mut quantization_attributes = Attributes::new();
				quantization_attributes.set_max_colors(maximum_colors.try_into()?);
				quantization_attributes.set_speed(2);
				quantization_attributes.set_quality(0, 100);

				// Quantize the image
				let mut iq_image = Image::new(
					&quantization_attributes,
					original_pixels.as_rgba(),
					image_info.width.try_into()?,
					image_info.height.try_into()?,
					0.0 // sRGB
				)?;

				quantization_result = quantization_attributes.quantize(&iq_image)?;
				quantization_result.set_dithering_level(1.0);

				let mut buf = Box::new_uninit_slice(image_info.buffer_size / 4);
				quantization_result.remap_into(&mut iq_image, &mut *buf)?;

				(
					quantization_result.palette_ref(),
					// SAFETY: if remap_into returns successfully, it is guaranteed that
					// the buffer was fully initialized
					#[allow(unsafe_code)]
					unsafe {
						buf.assume_init()
					}
				)
			};

			// Set up a fast encoder for a temporary PNG that will hold the result
			let mut png_writer = {
				let mut buf = src.split_off(0);
				buf.clear();
				buf.writer()
			};
			let mut png_encoder =
				png::Encoder::new(&mut png_writer, image_info.width, image_info.height);
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
			// fast. See below why being space efficient here doesn't matter
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

			oxipng_input_buf = png_writer.into_inner();

			// Use imagequant's estimation of how much quality we kept
			quality_description =
				Cow::Owned(format!("{}%", quantization_result.quantization_quality()));
		} else {
			// When not performing quantization, pixel colors will never be changed
			drop(png_reader);
			oxipng_input_buf = src.split_off(0); // This sets src capacity to 0
			quality_description = Cow::Borrowed("lossless");
		}

		// At this point we have the PNG data we want to losslessly optimize
		// in oxipng_input_buf. Do so with OxiPNG
		let oxipng_options = Options {
			alphas: if self.optimization_settings.skip_alpha_optimizations {
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
			bit_depth_reduction: true,
			color_type_reduction: true,
			// Compute an appropriate number of Zopfli compression iterations using our
			// model. If the number of iterations drops to zero, switch to the much faster,
			// but not so space-efficient, cloudflare-zlib deflater
			deflate: match zopfli_iterations_time_model.iterations_for_data_size(
				raw_pixel_data_size.try_into().unwrap_or(u32::MAX),
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
				let mut optimization_filters = IndexSet::with_capacity(6);
				for i in 0..=5 {
					optimization_filters.insert(i);
				}

				optimization_filters
			},
			fix_errors: true, // Ignore CRC for speed. We assume a reliable data source
			force: true,
			idat_recoding: true,
			interlace: Some(0), // No interlacing (smaller file size)
			palette_reduction: true,
			grayscale_reduction: !self.optimization_settings.do_not_reduce_to_grayscale,
			preserve_attrs: false,
			pretend: false,
			strip: Headers::All,
			timeout: Some(Duration::from_secs(600)), // Bail out if the optimization takes too long
			use_heuristics: false
		};

		let optimized_png = oxipng::optimize_from_memory(&oxipng_input_buf, &oxipng_options)?;

		Ok(Some((
			Cow::Owned(format!("Optimized with {} quality", quality_description)),
			OptimizedBytes(optimized_png)
		)))
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFile for PngFile<T> {
	type ByteChunkType = Vec<u8>;
	type OptimizationError = OptimizationError;
	type OptimizedBytesChunksStream = FramedRead<T, OptimizerDecoder>;

	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				optimization_settings: self.optimization_settings,
				reached_eof: false
			},
			self.file_length_hint
		)
	}

	fn canonical_extension(&self) -> &str {
		"png"
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFileConstructor<T> for PngFile<T> {
	type OptimizationSettings = PngFileOptions;

	fn new<F: FnMut() -> Option<(T, u64)>>(
		mut file_read_producer: F,
		args: PackFileConstructorArgs<'_, PngFileOptions>
	) -> Option<Self> {
		let file_path = &*args.path;
		let extension = &*to_ascii_lowercase_extension(file_path.as_ref());

		let skip = !matches!(extension, "png")
			|| (args.optimization_settings.skip_pack_icon
				&& file_path
					.parent()
					.filter(|parent_path| !parent_path.as_os_str().is_empty())
					.is_none() && file_path.file_name().unwrap().to_str().unwrap() == "pack.png");

		if !skip {
			file_read_producer().map(|(read, file_length_hint)| Self {
				read,
				// The file is too big to fit in memory if this conversion fails anyway
				file_length_hint: file_length_hint.try_into().unwrap_or(usize::MAX),
				optimization_settings: args.optimization_settings
			})
		} else {
			None
		}
	}
}
