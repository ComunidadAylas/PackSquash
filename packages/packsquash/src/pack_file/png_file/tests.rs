use futures::FutureExt;
use rgb::FromSlice;
use spng::{ContextFlags, CrcAction, DecodeFlags, Format};
use std::panic::AssertUnwindSafe;
use std::{env, fs};
use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use crate::config::ColorQuantizationTarget;

use super::*;

static PNG_DATA: &[u8] = include_bytes!("example.png");
static ENDERMAN_EYES_DATA: &[u8] = include_bytes!("enderman_eyes.png");
/// Somewhat extreme but realistic example of a texture whose size increases
/// 7x when color quantized and dithered with the default options.
static DITHERBOMB_DATA: &[u8] = include_bytes!("ditherbomb.png");
static SINGLE_BLUE_COLOR: &[u8] = include_bytes!("blue.png");

/// Processes the given input data as a [PngFile], using the provided settings,
/// expecting a successful result.
#[allow(clippy::too_many_arguments)]
async fn successful_process_test(
	input_data: &[u8],
	settings: PngFileOptions,
	expect_same_pixels: bool,
	expect_smaller_file_size: bool,
	expect_same_color_type: bool,
	expected_resolution: Option<(u32, u32)>,
	lenient_decode: bool,
	asset_type: PackFileAssetType,
	test_name: &str
) {
	let input_data_len = input_data.len();

	let data_stream = PngFile {
		read: Builder::new().read(input_data).build(),
		asset_type,
		file_length_hint: input_data_len,
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, Vec<u8>)> = data_stream
		.map(|result| result.expect("No error should happen while decoding"))
		.collect()
		.await;

	assert!(
		!process_result.is_empty(),
		"Some data was expected for this input"
	);

	let mut decoded_pixels;
	let processed_data_size;
	let processed_color_type;
	let processed_resolution;
	{
		let mut data = Vec::with_capacity(input_data_len);
		for (_, partial_data) in process_result {
			data.extend(&*partial_data);
		}
		processed_data_size = data.len();

		if env::var("WRITE_PNG_TEST_RESULTS").unwrap_or_else(|_| String::from("0")) == "1" {
			fs::write(
				format!("../../target/png_test_result_{test_name}.png"),
				&data
			)
			.expect("No error should happen while writing a test result to disk")
		}

		let mut png_decoder = spng::Decoder::new(&*data)
			.with_decode_flags(DecodeFlags::TRANSPARENCY)
			.with_output_format(Format::Rgba8);

		if lenient_decode {
			png_decoder = png_decoder
				.with_context_flags(ContextFlags::IGNORE_ADLER32)
				.with_crc_actions(CrcAction::Use, CrcAction::Use);
		}

		let mut png_reader = png_decoder
			.read_info()
			.expect("No error should happen while decoding processed PNG");
		let image_info = png_reader.info();

		processed_color_type = image_info.color_type;
		processed_resolution = (image_info.width, image_info.height);

		decoded_pixels = vec![0; png_reader.output_buffer_size()];
		png_reader
			.next_frame(&mut decoded_pixels)
			.expect("No error should happen while reading processed PNG frame");
	}
	let decoded_pixels = decoded_pixels.as_rgba();

	let mut original_pixels;
	let original_color_type;
	{
		let mut png_reader = spng::Decoder::new(input_data)
			.with_decode_flags(DecodeFlags::GAMMA | DecodeFlags::TRANSPARENCY)
			.with_output_format(Format::Rgba8)
			.read_info()
			.expect("No error should happen while decoding example PNG");
		let image_info = png_reader.info();

		original_color_type = image_info.color_type;

		original_pixels = vec![0; png_reader.output_buffer_size()];
		png_reader
			.next_frame(&mut original_pixels)
			.expect("No error should happen while reading example PNG frame");
	}
	let original_pixels = original_pixels.as_rgba();

	assert!(
		expected_resolution.is_none() || expected_resolution == Some(processed_resolution),
		"The processed PNG does not have the expected resolution: {:?} actual, {:?} expected",
		processed_resolution,
		expected_resolution.unwrap()
	);

	assert!(
		!expect_same_pixels || original_pixels == decoded_pixels,
		"The processed PNG should have the same pixels as the original PNG"
	);

	eprintln!("Original PNG size: {input_data_len} bytes, color type: {original_color_type:?}");
	eprintln!(
		"Processed PNG size: {processed_data_size} bytes, color type: {processed_color_type:?}"
	);
	assert!(
		!expect_smaller_file_size || processed_data_size < input_data_len,
		"The processed PNG should be smaller than the original PNG"
	);

	assert!(
		!expect_same_color_type || original_color_type == processed_color_type,
		"The processed PNG should have the same color type as the original PNG"
	);
}

#[tokio::test]
async fn lossless_optimization_works() {
	successful_process_test(
		PNG_DATA,
		PngFileOptions {
			color_quantization_target: ColorQuantizationTarget::None,
			skip_alpha_optimizations: true,
			..Default::default()
		},
		true,           // Same pixels
		true,           // Smaller size
		false,          // Not necessarily the same color type
		Some((16, 16)), // Same resolution
		false,          // The PNG datastream should be standards-compliant
		PackFileAssetType::GenericTexture,
		"lossless_optimization_works"
	)
	.await
}

#[tokio::test]
async fn lossy_optimization_works() {
	successful_process_test(
		PNG_DATA,
		PngFileOptions {
			color_quantization_target: ColorQuantizationTarget::FourBitDepth,
			..Default::default()
		},
		false,          // Not necessarily the same pixels
		true,           // Smaller size
		false,          // Not necessarily the same color type
		Some((16, 16)), // Same resolution
		false,          // The PNG datastream should be standards-compliant
		PackFileAssetType::GenericTexture,
		"lossy_optimization_works"
	)
	.await
}

#[tokio::test]
async fn entity_eye_blending_workaround_works() {
	successful_process_test(
		ENDERMAN_EYES_DATA,
		PngFileOptions {
			color_quantization_target: ColorQuantizationTarget::EightBitDepth,
			working_around_transparent_pixel_colors_change_quirk: true,
			..Default::default()
		},
		true,           // Same pixels
		false,          // Not necessarily smaller
		false,          // Not necessarily the same color type
		Some((64, 32)), // Same resolution
		false,          // The PNG datastream should be standards-compliant
		PackFileAssetType::EyeLayer,
		"entity_eye_blending_workaround_works"
	)
	.await
}

#[tokio::test]
async fn banner_layer_check_workaround_works() {
	successful_process_test(
		PNG_DATA,
		PngFileOptions {
			color_quantization_target: ColorQuantizationTarget::EightBitDepth,
			working_around_color_type_change_quirk: true,
			..Default::default()
		},
		false,          // Not necessarily the same pixels
		false,          // Not necessarily smaller
		true,           // Same color type
		Some((16, 16)), // Same resolution
		false,          // The PNG datastream should be standards-compliant
		PackFileAssetType::BannerLayer,
		"banner_layer_check_workaround_works"
	)
	.await
}

#[tokio::test]
#[cfg_attr(
	all(ci, target_arch = "aarch64", target_os = "linux"),
	ignore = "Due to QEMU emulation, quite slow on CI"
)]
async fn ditherbomb_does_not_get_bigger() {
	successful_process_test(
		DITHERBOMB_DATA,
		PngFileOptions {
			color_quantization_target: ColorQuantizationTarget::Auto,
			color_quantization_dithering_level: 1.0.try_into().unwrap(),
			..Default::default()
		},
		true,               // Should fall back to the first pass result
		true,               // The first pass strips some non-critical chunks
		true,               // Should fall back to the first pass result
		Some((4395, 6598)), // Same resolution
		false,              // The PNG datastream should be standards-compliant
		PackFileAssetType::GenericTexture,
		"ditherbomb_does_not_get_bigger"
	)
	.await
}

#[tokio::test]
#[cfg_attr(
	all(ci, target_arch = "aarch64", target_os = "linux"),
	ignore = "Due to QEMU emulation, flaky on CI when targeting musl, and quite slow"
)]
async fn ditherbomb_can_be_defused() {
	successful_process_test(
		DITHERBOMB_DATA,
		PngFileOptions {
			color_quantization_target: ColorQuantizationTarget::Auto,
			color_quantization_dithering_level: 0.0.try_into().unwrap(),
			..Default::default()
		},
		false,              // Not necessarily the same pixels
		true,               // No dithering is enough to make the optimizations work as expected
		false,              // Not necessarily the same color type
		Some((4395, 6598)), // Same resolution
		false,              // The PNG datastream should be standards-compliant
		PackFileAssetType::GenericTexture,
		"ditherbomb_can_be_defused"
	)
	.await
}

#[tokio::test]
async fn single_color_image_is_downsized() {
	successful_process_test(
		SINGLE_BLUE_COLOR,
		PngFileOptions {
			downsize_if_single_color: true,
			..Default::default()
		},
		false,        // Not the same pixels
		true,         // Smaller file size
		false,        // Maybe different color type
		Some((1, 1)), // Not a power of two, so vanilla Minecraft doesn't do mipmaps
		false,        // The PNG datastream should be standards-compliant
		PackFileAssetType::GenericTexture,
		"single_color_image_is_downsized"
	)
	.await
}

#[tokio::test]
async fn invalid_input_is_handled() {
	let mut data_stream = PngFile {
		read: Builder::new().read(&[]).build(),
		asset_type: PackFileAssetType::GenericTexture,
		file_length_hint: 0,
		optimization_settings: Default::default()
	}
	.process();

	data_stream
		.next()
		.await
		.expect("Expected some result for this input")
		.expect_err("Expected an error for this input");
}

#[tokio::test]
async fn png_data_with_trailing_bytes_is_handled() {
	let png_data = Vec::from_iter(PNG_DATA.iter().copied().chain(std::iter::once(0)));

	successful_process_test(
		&png_data,
		PngFileOptions {
			color_quantization_target: ColorQuantizationTarget::None,
			skip_alpha_optimizations: true,
			..Default::default()
		},
		true,           // Same pixels
		true,           // Smaller size
		false,          // Not necessarily the same color type
		Some((16, 16)), // Same resolution
		false,          // The PNG datastream should be standards-compliant
		PackFileAssetType::GenericTexture,
		"png_data_with_trailing_bytes_is_handled"
	)
	.await
}

#[tokio::test]
async fn png_obfuscation_works() {
	// AssertUnwindSafe can be used because catch_unwind will not witness any invalid
	// state at the expected panic points
	let conforming_decoder_test_result = AssertUnwindSafe(successful_process_test(
		PNG_DATA,
		PngFileOptions {
			image_data_compression_iterations: 0,
			color_quantization_target: ColorQuantizationTarget::None,
			skip_alpha_optimizations: true,
			png_obfuscation: true,
			..Default::default()
		},
		true,           // Same pixels
		false,          // Non necessarily smaller
		false,          // Not necessarily the same color type
		Some((16, 16)), // Same resolution
		false,          // Decode strictly
		PackFileAssetType::GenericTexture,
		"png_obfuscation_works"
	))
	.catch_unwind()
	.await;

	assert!(
		conforming_decoder_test_result.is_err(),
		"Decoding an obfuscated PNG datastream with a conforming decoder should fail"
	);

	successful_process_test(
		PNG_DATA,
		PngFileOptions {
			image_data_compression_iterations: 0,
			color_quantization_target: ColorQuantizationTarget::None,
			skip_alpha_optimizations: true,
			png_obfuscation: true,
			..Default::default()
		},
		true,           // Same pixels
		false,          // Non necessarily smaller
		false,          // Not necessarily the same color type
		Some((16, 16)), // Same resolution
		true,           // Decode leniently
		PackFileAssetType::GenericTexture,
		"png_obfuscation_works"
	)
	.await
}
