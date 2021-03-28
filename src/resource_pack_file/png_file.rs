use std::{borrow::Cow, convert::TryInto, lazy::SyncLazy, num::TryFromIntError};

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

use super::{OptimizedBytes, ResourcePackFile};

#[cfg(test)]
mod tests;

/// Represents a resource pack PNG image file, which is used to generate textures.
///
/// The optimization process may be customized via [OptimizationSettings].
struct PngFile<'a, T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length: usize,
	extension: &'a str,
	optimization_settings: OptimizationSettings
}

/// Parameters that influence how a [PngFile] is optimized.
///
/// Note that, in any case, any PNG chunks (e.g. metadata) that are
/// not used by Minecraft to display the image will not be copied,
/// if present in the source file.
struct OptimizationSettings {
	/// Controls how the colors of the image will be (or not be) quantized.
	color_quantization_mode: ColorQuantizationMode,
	/// The maximum width and height of the images that will be accepted.
	/// This parameter sets a high bound of memory usage by PackSquash
	/// and helps authoring resource packs with reasonable texture sizes.
	// TODO: put this text in the docs
	// Internally, Minecraft builds a texture atlas with every texture it needs to draw,
	// including those of resource packs, for efficiency reasons.
	// Assuming a maximum texture size of 65536x65536 (which as of 2021 is an upper bound
	// for consumer GPUs), 256 individual textures of 4096x4096 would fit in such an atlas,
	// which is a pretty low number and not enough to hold all vanilla textures. Resource
	// packs with few textures may get away with textures that are even bigger than this,
	// maybe only on "beefy" GPUs, but it does not make much sense to mix and match small
	// textures with such big textures. I've not seen resource packs with textures bigger
	// than 2048x2048, and even that is quite a stretch. Therefore, establish an upper limit
	// of 4096x4096 to set an upper bound to memory usage per decoded RGBA8 image of ~64 MiB,
	// and reject textures that are impractically big, introducing a kind of "validation" that
	// may help limit how much resource pack creators can shoot themselves on the foot
	maximum_size: u32
}

impl Default for OptimizationSettings {
	fn default() -> Self {
		Self {
			color_quantization_mode: Default::default(),
			maximum_size: 4096
		}
	}
}

/// Possible strategies to quantize the colors of a [PngFile].
#[derive(Copy, Clone)]
enum ColorQuantizationMode {
	/// No quantization will be done, only lossless optimizations.
	None,
	/// The image will be quantized to at most 2 colors.
	OneBitDepth,
	/// The image will be quantized to at most 4 colors.
	TwoBitDepth,
	/// The image will be quantized to at most 16 colors.
	FourBitDepth,
	/// The image will be quantized to at most 256 colors.
	EightBitDepth
}

impl Default for ColorQuantizationMode {
	fn default() -> Self {
		ColorQuantizationMode::EightBitDepth
	}
}

impl ColorQuantizationMode {
	const fn depth(&self) -> u8 {
		match self {
			Self::None => 0,
			Self::OneBitDepth => 1,
			Self::TwoBitDepth => 2,
			Self::FourBitDepth => 4,
			Self::EightBitDepth => 8
		}
	}

	const fn max_colors(&self) -> u32 {
		u32::checked_pow(2, self.depth() as u32).unwrap()
	}

	const fn should_quantize(&self) -> bool {
		self.depth() > 0
	}
}

/// Optimizer decoder that transforms PNG files to an optimized representation.
struct OptimizerDecoder {
	file_length: usize,
	optimization_settings: OptimizationSettings
}

/// Represents an error that may happen while optimizing PNG files.
#[derive(Error, Debug)]
enum OptimizationError {
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

/// OxiPNG settings for lossless optimization of PNGs, using Zopfli DEFLATE.
static OXIPNG_OPTIONS: SyncLazy<Options> = SyncLazy::new(|| Options {
	alphas: {
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
	deflate: Deflaters::Zopfli,
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
	preserve_attrs: false,
	pretend: false,
	strip: Headers::All,
	timeout: None,
	use_heuristics: false
});

impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, OptimizedBytes<Vec<u8>>);
	type Error = OptimizationError;

	fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		// FIXME: actual framing?
		// (i.e. do not hold the entire file in memory before decoding, so that frame != file)

		// Check if we have the entire file (i.e. frame)
		if src.len() < self.file_length {
			return Ok(None);
		}

		let color_quantization_mode = self.optimization_settings.color_quantization_mode;
		let maximum_colors = color_quantization_mode.max_colors();
		let quality_description;

		// Read the PNG IHDR and other information chunks, to validate it
		let (image_info, mut png_reader) = spng::Decoder::new(&**src)
			.with_limits(Limits {
				max_width: self.optimization_settings.maximum_size,
				max_height: self.optimization_settings.maximum_size
			})
			.with_decode_flags(DecodeFlags::GAMMA | DecodeFlags::TRANSPARENCY)
			.with_context_flags(ContextFlags::IGNORE_ADLER32)
			.with_output_format(Format::Rgba8)
			.read_info()?;

		let oxipng_input_buf;

		// Perform quantization if desired and if useful
		// (i.e. there are more pixels than possible colors)
		if color_quantization_mode.should_quantize()
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
				quantization_attributes.set_max_colors(maximum_colors.try_into().unwrap());
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
		let optimized_png = oxipng::optimize_from_memory(&oxipng_input_buf, &OXIPNG_OPTIONS)?;

		Ok(Some((
			Cow::Owned(format!("Optimized with {} quality", quality_description)),
			OptimizedBytes(optimized_png)
		)))
	}
}

impl<T: AsyncRead + Unpin + 'static>
	ResourcePackFile<Vec<u8>, OptimizationError, FramedRead<T, OptimizerDecoder>> for PngFile<'_, T>
{
	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				file_length: self.file_length,
				optimization_settings: self.optimization_settings
			},
			self.file_length
		)
	}

	fn canonical_extension(&self) -> &str {
		self.extension
	}

	fn is_compressed(&self) -> bool {
		true
	}
}
