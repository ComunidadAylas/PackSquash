//! Contains routines and data types for processing single images.

use crate::config::ColorQuantizationTarget;
use crate::zopfli_iterations_time_model::ZopfliIterationsTimeModel;
use bytes::BytesMut;
use imagequant::{liq_error, Attributes};
use indexmap::IndexMap;
use itertools::Itertools;
use oxipng::internal_tests::{BitDepth, ColorType, IhdrData, PngImage};
use oxipng::{Deflaters, Headers, Interlacing, Options, RowFilter};
use rgb::{AsPixels, RGBA8};
use spng::{ContextFlags, DecodeFlags, Format};
use std::io::Read;
use std::num::{NonZeroU16, NonZeroU8};
use std::time::Duration;
use std::{cmp, iter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageProcessingError {
	#[error("Invalid PNG: {0}")]
	StripValidateError(&'static str),
	#[error("PNG decode error: {0}")]
	PngDecoding(#[from] spng::Error),
	#[error("OxiPNG optimization error: {0}")]
	OxipngError(#[from] oxipng::PngError),
	#[error("libimagequant optimization error: {0}")]
	LiqError(#[from] liq_error)
}

/// Performs a first fast optimization to an input PNG image: remove non-critical chunks
/// that will not be parsed by the expected downstream decoders. This can never increase
/// the input PNG image size, only decrease or maintain it.
///
/// It also validates that neither image dimension exceeds the specified threshold.
pub fn strip_unnecessary_chunks(
	input_png: BytesMut,
	maximum_dimension: NonZeroU16
) -> Result<Vec<u8>, ImageProcessingError> {
	let mut stripped_png = Vec::with_capacity(input_png.len());

	// Helper macro to avoid non-panicking bounds checking verbosity
	macro_rules! get_or_err {
		($range:expr) => {
			input_png
				.get($range)
				.ok_or(ImageProcessingError::StripValidateError(
					"The file is smaller than expected. Is it invalid or corrupt?"
				))?
		};
	}

	// The format of PNG files is dead simple: just a signature in the beginning
	// followed by as many chunks as desired. OxiPNG supports stripping chunks,
	// but it's amazing that implementing this ourselves takes barely the same
	// LOC than calling OxiPNG with the suitable options, and that there is no
	// nice library at crates.io to do this simple optimization. OxiPNG is also
	// not really meant to do this single thing as fast as possible. So do it
	// ourselves and strip every chunk that we know won't be of any use.
	// Normative reference: https://www.w3.org/TR/PNG/

	// Check and copy the signature
	let signature = get_or_err!(..8);
	if signature != [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A] {
		return Err(ImageProcessingError::StripValidateError(
			"The expected PNG signature was not found. Textures must be encoded in PNG format"
		));
	}
	stripped_png.extend_from_slice(signature);

	// Now copy the known-necessary chunks only. We barely do any validation on them
	// apart from checking whether the image is too big and what's necessary to do
	// the copy. spng will take care of further validation
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
					return Err(ImageProcessingError::StripValidateError(
						"Unexpected size for IHDR chunk"
					));
				}

				let width = u32::from_be_bytes(chunk_data[..4].try_into().unwrap());
				let height = u32::from_be_bytes(chunk_data[4..8].try_into().unwrap());

				if width > maximum_dimension.get() as u32 || height > maximum_dimension.get() as u32 {
					return Err(ImageProcessingError::StripValidateError(
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
	// explicitly violates the PNG standard. For now, reject such PNG files to
	// future-proof ourselves and users against decoders that are not lenient (they
	// are hard to find, but exist)
	// TODO for v0.4.0, turn this error to a warning instead
	if i < input_png.len() {
		return Err(ImageProcessingError::StripValidateError(
			"Non-conforming trailing bytes at the end of the file. \
			Please re-export with a known-good PNG encoder"
		));
	}

	Ok(stripped_png)
}

/// An in-memory rectangular array of pixels in 8-bit RGBA format,
/// stored as a raw byte buffer.
///
/// A pixel array is agnostic to the ordering of its pixels in memory,
/// but for interoperability and clarity reasons it's recommended to
/// order them from top-left to bottom-right. This is the ordering
/// assumed through the code in this module and by the PNG format.
pub struct PixelArray {
	width: NonZeroU16,
	height: NonZeroU16,
	buf: Vec<u8>
}

impl PixelArray {
	/// Returns the width of this pixel array.
	fn width(&self) -> NonZeroU16 {
		self.width
	}

	/// Returns the height of this pixel array.
	fn height(&self) -> NonZeroU16 {
		self.height
	}

	/// Returns the slice of pixels contained in this array.
	///
	/// This conversion is guaranteed to be zero-cost.
	fn as_slice(&self) -> &[RGBA8] {
		self.buf.as_pixels()
	}

	/// Consumes this RGBA8 pixel array and returns a byte buffer with its pixels.
	fn into_byte_buf(self) -> Vec<u8> {
		self.buf
	}
}

/// An in-memory PNG image that is being processed, that may be optimized in several ways.
///
/// A processed image can be in several states, depending on whether it is pending to
/// decode from an input PNG or whether optimization operations were applied to it.
/// A processed image will mostly transparently transition between these states, a
/// fact that further operations take advantage of to avoid doing repeated work (e.g.,
/// decoding the same PNG several times) or do it more efficiently.
pub enum ProcessedImage<R: Read> {
	/// A PNG image that was parsed by `spng`, but whose pixels have not been decoded yet.
	ParsedPng {
		png_info: spng::OutputInfo,
		png_reader: spng::Reader<R>
	},
	/// An in-memory image whose pixels have been decoded to RGBA8 format.
	RGBA8 { pixels: PixelArray },
	/// An in-memory image in palette format, generated after applying color quantization
	/// to other processed image types.
	Indexed {
		width: NonZeroU16,
		height: NonZeroU16,
		palette: Vec<RGBA8>,
		pixel_palette_indexes: Vec<u8>,
		quantization_quality: u8
	}
}

impl<R: Read> ProcessedImage<R> {
	/// Creates a new [`ProcessedImage`] by reading it from the PNG at the specified reader.
	///
	/// The PNG file will be validated, but not decoded. This method assumes that the width
	/// and height of the image fit in a 16-bit unsigned integer.
	pub fn read(reader: R) -> Result<Self, ImageProcessingError> {
		spng::Decoder::new(reader)
			.with_decode_flags(DecodeFlags::GAMMA | DecodeFlags::TRANSPARENCY) // Apply gamma correction to normalize to sRGB
			.with_context_flags(ContextFlags::IGNORE_ADLER32)
			.with_output_format(Format::Rgba8) // The RGBA8 format is necessary for PixelArray
			.read_info()
			.map_or_else(
				|err| Err(err.into()),
				|(png_info, png_reader)| {
					Ok(Self::ParsedPng {
						png_info,
						png_reader
					})
				}
			)
	}

	/// Returns the width of this image, in pixels.
	pub fn width(&self) -> NonZeroU16 {
		match self {
			Self::ParsedPng { png_info, .. } => NonZeroU16::new(png_info.width as u16).unwrap(),
			Self::RGBA8 { pixels } => pixels.width(),
			Self::Indexed { width, .. } => *width
		}
	}

	/// Returns the height of this image, in pixels.
	pub fn height(&self) -> NonZeroU16 {
		match self {
			Self::ParsedPng { png_info, .. } => NonZeroU16::new(png_info.height as u16).unwrap(),
			Self::RGBA8 { pixels } => pixels.height(),
			Self::Indexed { height, .. } => *height
		}
	}

	/// Returns the array of pixels of this image. This will trigger its decodification
	/// if necessary, hence why the `&mut self` receiver.
	///
	/// `Ok(None)` is returned if this image was quantized to a color palette, as in that
	/// case the image lacks a backing array of colors per pixel.
	fn as_pixel_array(&mut self) -> Result<Option<&mut PixelArray>, ImageProcessingError> {
		match self {
			Self::ParsedPng {
				png_info,
				png_reader
			} => {
				let mut buf = vec![0; png_info.buffer_size];
				png_reader.next_frame(&mut buf)?;

				*self = PixelArray {
					width: NonZeroU16::new(png_info.width as u16).unwrap(),
					height: NonZeroU16::new(png_info.height as u16).unwrap(),
					buf
				}
				.into();

				self.as_pixel_array()
			}
			Self::RGBA8 { pixels } => Ok(Some(pixels)),
			Self::Indexed { .. } => Ok(None)
		}
	}

	/// Like [`as_pixel_array`](Self::as_pixel_array), but consumes `self` to move
	/// the pixel array back to the caller.
	fn into_pixel_array(mut self) -> Result<Option<PixelArray>, ImageProcessingError> {
		self.as_pixel_array()?;

		Ok(match self {
			Self::RGBA8 { pixels, .. } => Some(pixels),
			Self::Indexed { .. } => None,
			// as_pixel_array ensures that we transitioned out of this state
			Self::ParsedPng { .. } => unreachable!()
		})
	}

	/// Returns an estimation of the visual image quality after performing color
	/// quantization, in the `0-100` range.
	///
	/// `None` is returned if this image was not color quantized.
	pub fn quantization_quality(&self) -> Option<u8> {
		if let Self::Indexed {
			quantization_quality,
			..
		} = self
		{
			Some(*quantization_quality)
		} else {
			None
		}
	}

	/// Performs color quantization on this image according to the specified parameters,
	/// and returns a new image with the result. In some edge cases, even in combination
	/// with another optimizer, this may be a size-increasing operation, depending on the
	/// dithering pattern compressibility and how optimal the input image already was.
	///
	/// Returns `Ok(None)` if the image was already quantized.
	pub fn quantize_color(
		&mut self,
		quantization_target: ColorQuantizationTarget,
		dithering_level: f32
	) -> Result<Option<Self>, ImageProcessingError> {
		let width = self.width();
		let height = self.height();

		// Set the quantization attributes
		let mut quantization_attributes = Attributes::new();
		quantization_attributes.set_max_colors(quantization_target.max_colors())?;
		quantization_attributes.set_speed(2)?;
		quantization_attributes.set_quality(0, 100)?;

		let bitmap = if let Some(pixel_array) = self.as_pixel_array()? {
			pixel_array.as_slice()
		} else {
			return Ok(None);
		};

		// Wrap the pixel data in an ImageQuant image
		let mut iq_image = quantization_attributes.new_image_borrowed(
			bitmap,
			width.get() as usize,
			height.get() as usize,
			0.0 // sRGB: spng applies gamma correction
		)?;

		// Configure the quantization operation for the image from its attributes
		let mut quantization_result = quantization_attributes.quantize(&mut iq_image)?;
		quantization_result.set_dithering_level(dithering_level)?;

		// Quantize the image
		let (palette, pixel_palette_indexes) = quantization_result.remapped(&mut iq_image)?;

		Ok(Some(ProcessedImage::Indexed {
			width,
			height,
			palette,
			pixel_palette_indexes,
			quantization_quality: quantization_result.quantization_quality().unwrap()
		}))
	}

	/// Downsizes this image to the most space-efficient dimensions if it is single-color and such
	/// resizing is not expected to impact how the pack looks, and returns the resulting image. This
	/// may significantly decrease file sizes and improve client stitching performance and memory
	/// usage in extreme cases. However, there are other extreme cases where this might cause some
	/// breakage.
	///
	/// Returns `Ok(None)` if the image can't be safely downsized, or if it was color quantized.
	pub fn downsize_single_color(
		&mut self,
		can_change_color_type: bool,
		is_auxiliary_shader_target_texture: bool
	) -> Result<Option<Self>, ImageProcessingError> {
		let dimension = cmp::min(self.width(), self.height());
		let minimum_mipmap_level_keeping_dimension =
			NonZeroU16::new(cmp::min(1u32 << dimension.trailing_zeros(), 16) as u16).unwrap();

		// We can't/shouldn't downsize textures in cases that cause undesirable side effects:
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
		if minimum_mipmap_level_keeping_dimension >= dimension
			|| !can_change_color_type
			|| is_auxiliary_shader_target_texture
		{
			return Ok(None);
		}

		Ok(self.as_pixel_array()?.and_then(|pixel_array| {
			let pixels = pixel_array.as_slice();
			pixels.iter().all_equal().then(|| {
				PixelArray {
					width: minimum_mipmap_level_keeping_dimension,
					height: minimum_mipmap_level_keeping_dimension,
					buf: iter::repeat(pixels[0])
						.take(
							minimum_mipmap_level_keeping_dimension.get() as usize
								* minimum_mipmap_level_keeping_dimension.get() as usize
						)
						.flat_map(|pixel| <RGBA8 as Into<[u8; 4]>>::into(pixel).into_iter())
						.collect()
				}
				.into()
			})
		}))
	}

	/// Visually losslessly optimizes this image: any visible (i.e., non completely
	/// transparent) pixel will be decoded to exactly the same color and transparency
	/// values, and returns the resulting encoded PNG. This may be a pessimization in
	/// cases where the input image was compressed better than we can came up with, or
	/// if the optimizer assumptions turn out to be wrong (for example, using a color
	/// palette is not always more efficient than storing the color values for each
	/// pixel).
	pub fn visually_lossless_optimize(
		self,
		zopfli_compression_iterations: u8,
		can_change_color_type: bool,
		can_change_transparent_pixel_colors: bool,
		can_convert_to_grayscale: bool
	) -> Result<Vec<u8>, ImageProcessingError> {
		/// The strategies that OxiPNG will try on the image to find the scanline
		/// filter that minimizes file size the most.
		const FILTER_STRATEGIES: [RowFilter; 5] = [
			RowFilter::None,
			RowFilter::Bigrams,
			RowFilter::MinSum,
			RowFilter::BigEnt,
			RowFilter::Brute
		];

		let zopfli_iterations_model =
			ZopfliIterationsTimeModel::new(zopfli_compression_iterations, 2.0);

		let pixel_count = self.width().get() as u32 * self.height().get() as u32;

		let optimization_options = Options {
			optimize_alpha: can_change_transparent_pixel_colors,
			backup: false,
			bit_depth_reduction: can_change_color_type,
			color_type_reduction: can_change_color_type,
			// Compute an appropriate number of Zopfli compression iterations using our
			// model. If the number of iterations drops to zero, switch to the much faster,
			// but not so space-efficient, cloudflare-zlib deflater
			deflate: match zopfli_iterations_model.iterations_for_data_size(
				// OxiPNG does one Zopfli compression attempt per PNG filter configured below
				// and returns the combination that yields the best result, so it's desired to
				// consider that the data size was multiplied by the number of filters to bound
				// execution time properly
				pixel_count.saturating_mul(FILTER_STRATEGIES.len() as u32),
				0,
				15
			) {
				0 => Deflaters::Libdeflater {
					// Use the maximum compression level for the best compression.
					// This is still acceptably fast for bigger images of realistic
					// sizes
					compression: 12
				},
				zopfli_iterations => Deflaters::Zopfli {
					iterations: NonZeroU8::new(zopfli_iterations).unwrap()
				}
			},
			filter: FILTER_STRATEGIES.into_iter().collect(),
			fix_errors: true, // Ignore CRC for speed. We assume a reliable data source
			force: false,
			idat_recoding: true,
			interlace: None, // We provide raw pixel data to OxiPNG, so interlacing is not a concern
			palette_reduction: true,
			grayscale_reduction: can_convert_to_grayscale && can_change_color_type,
			preserve_attrs: false,
			pretend: false,
			strip: Headers::All,
			timeout: Some(Duration::from_secs(600)), // Bail out if the optimization takes too long
			fast_evaluation: true,
			check: false
		};

		Ok(match self {
			parsed_png @ Self::ParsedPng { .. } => oxipng::optimize_from_raw(
				PngImage {
					ihdr: IhdrData {
						width: parsed_png.width().get() as u32,
						height: parsed_png.height().get() as u32,
						color_type: ColorType::RGBA,
						bit_depth: BitDepth::Eight,
						interlaced: Interlacing::None
					},
					data: parsed_png.into_pixel_array()?.unwrap().into_byte_buf(),
					aux_headers: IndexMap::new()
				},
				&optimization_options
			),
			Self::RGBA8 { pixels } => oxipng::optimize_from_raw(
				PngImage {
					ihdr: IhdrData {
						width: pixels.width().get() as u32,
						height: pixels.height().get() as u32,
						color_type: ColorType::RGBA,
						bit_depth: BitDepth::Eight,
						interlaced: Interlacing::None
					},
					data: pixels.into_byte_buf(),
					aux_headers: IndexMap::new()
				},
				&optimization_options
			),
			Self::Indexed {
				width,
				height,
				palette,
				pixel_palette_indexes,
				..
			} => oxipng::optimize_from_raw(
				PngImage {
					ihdr: IhdrData {
						width: width.get() as u32,
						height: height.get() as u32,
						color_type: ColorType::Indexed { palette },
						bit_depth: BitDepth::Eight,
						interlaced: Interlacing::None
					},
					data: pixel_palette_indexes,
					aux_headers: IndexMap::new()
				},
				&optimization_options
			)
		}?)
	}
}

impl<R: Read> From<PixelArray> for ProcessedImage<R> {
	fn from(pixels: PixelArray) -> Self {
		Self::RGBA8 { pixels }
	}
}
