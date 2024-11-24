//! Contains code to optimize PNG files.

use std::borrow::Cow;

use bytes::BytesMut;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use crate::config::PngFileOptions;

use super::{AsyncReadAndSizeHint, PackFile, PackFileAssetType, PackFileConstructor};

use image_processor::{ImageProcessingError, ProcessedImage};

mod image_processor;

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
	#[error("{0}")]
	OptimizationError(#[from] ImageProcessingError),
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
		// the next passes. It's relatively cheap to do this, although not free.
		// This pass also guarantees that both the width and height are not greater than
		// maximum_width_and_height
		let first_pass_png = image_processor::strip_unnecessary_chunks(
			src.split_off(0),
			self.optimization_settings.maximum_width_and_height
		)?;
		let mut first_pass_image = ProcessedImage::read(&*first_pass_png)?;

		// Second pass: downsize most textures that consist of a single color to the minimum
		// size that does not cause side effects in Minecraft. If that can't be done, then
		// perform quantization if desired and useful (i.e., there are more pixels than
		// possible colors, and no option stops us from doing so). It does not make sense to
		// do both downsizing and quantization: there are no colors to quantize when downsizing
		// is successful, and quantizing is only useful when the image has many colors. Note that
		// both of these operations may change the color type (i.e., turn an indexed image to RGBA,
		// or vice versa)
		let second_pass_image = match (can_change_color_type
			&& self.optimization_settings.downsize_if_single_color)
			.then(|| {
				first_pass_image
					.downsize_single_color(can_change_color_type, is_auxiliary_shader_target_texture)
			})
			.transpose()?
			.flatten()
		{
			// Downsizing was successful
			Some(downsized_png) => Some(downsized_png),
			None if color_quantization_target.should_quantize()
				&& first_pass_image.width().get() as u32 * first_pass_image.height().get() as u32
					> color_quantization_target.max_colors()
				&& can_change_color_type
				&& can_change_transparent_pixel_colors =>
			{
				// The image could not be downsized, but it could be quantized
				first_pass_image.quantize_color(
					color_quantization_target,
					self.optimization_settings
						.color_quantization_dithering_level
						.into()
				)?
			}
			// No downsizing or quantization is appropriate
			None => None
		};

		let quantization_quality = second_pass_image
			.as_ref()
			.and_then(|image| image.quantization_quality());
		let have_second_pass_result = second_pass_image.is_some();
		let must_use_second_pass_result =
			color_quantization_target.is_quantization_required() && quantization_quality.is_some();

		// Third pass: complete lossless optimization of the second pass PNG, if quantization
		// or downsizing was done, or else the first pass PNG
		let third_pass_png = second_pass_image
			.unwrap_or(first_pass_image)
			.visually_lossless_optimize(
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
		//   PNG files with big areas of flat colors that are turned into big areas of noisy
		//   dithering. This is relevant when the second pass is used as input for the third
		//   pass.
		// - Downsizing yields smaller files except in some extreme edge cases due to the
		//   heuristics used by the optimizer in the third pass being inappropriate. This may be
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
		let (mut optimized_png, optimization_strategy_message) =
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
					if let Some(quantization_quality) = quantization_quality {
						Cow::Owned(format!(
							"Optimized with {quantization_quality}% quality color quantization"
						))
					} else if have_second_pass_result {
						Cow::Borrowed("Downsized and optimized")
					} else if can_change_transparent_pixel_colors {
						Cow::Borrowed("Optimized with no visible color loss")
					} else {
						Cow::Borrowed("Optimized")
					}
				)
			};

		// Final pass: apply obfuscation to the optimized result if possible and desired
		if self
			.optimization_settings
			.minecraft_version_supports_png_obfuscation
			&& self.optimization_settings.png_obfuscation
		{
			image_processor::obfuscate_png(&mut optimized_png);
		}

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

	fn may_be_directory_listed_atlas_texture_sprite(&self) -> bool {
		self.optimization_settings
			.may_be_directory_listed_atlas_sprite
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
