use std::{env, fs};

use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use crate::config::ColorQuantizationTarget;

use super::*;

static PNG_DATA: &[u8] = include_bytes!("example.png");

/// Processes the given input data as a [PngFile], using the provided settings,
/// expecting a successful result.
async fn successful_process_test(
	input_data: &[u8],
	settings: PngFileOptions,
	expect_same_pixels: bool,
	expect_smaller_file_size: bool,
	test_name: &str
) {
	let input_data_len = input_data.len();

	let data_stream = PngFile {
		read: Builder::new().read(input_data).build(),
		asset_type: PackFileAssetType::GenericTexture,
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
	{
		let mut data = Vec::with_capacity(input_data_len);
		for (_, partial_data) in process_result {
			data.extend(&*partial_data);
		}
		processed_data_size = data.len();

		if env::var("WRITE_PNG_TEST_RESULTS").unwrap_or(String::from("0")) == "1" {
			fs::write(format!("target/png_test_result_{}.png", test_name), &data).unwrap();
		}

		let (image_info, mut png_reader) = spng::Decoder::new(&*data)
			.with_decode_flags(DecodeFlags::TRANSPARENCY)
			.with_output_format(Format::Rgba8)
			.read_info()
			.expect("No error should happen while decoding processed PNG");
		decoded_pixels = vec![0; image_info.buffer_size];
		png_reader
			.next_frame(&mut decoded_pixels)
			.expect("No error should happen while reading processed PNG frame");
	}
	let decoded_pixels = decoded_pixels.as_rgba();

	let mut original_pixels;
	{
		let (image_info, mut png_reader) = spng::Decoder::new(input_data)
			.with_decode_flags(DecodeFlags::GAMMA | DecodeFlags::TRANSPARENCY)
			.with_output_format(Format::Rgba8)
			.read_info()
			.expect("No error should happen while decoding example PNG");
		original_pixels = vec![0; image_info.buffer_size];
		png_reader
			.next_frame(&mut original_pixels)
			.expect("No error should happen while reading example PNG frame");
	}
	let original_pixels = original_pixels.as_rgba();

	assert_eq!(
		original_pixels.len(),
		decoded_pixels.len(),
		"The processed PNG should have the same resolution as the original PNG"
	);

	assert!(
		!expect_same_pixels || original_pixels == decoded_pixels,
		"The processed PNG should have the same pixels as the original PNG"
	);

	eprintln!("Original PNG size: {} bytes", input_data_len);
	eprintln!("Processed PNG size: {} bytes", processed_data_size);
	assert!(
		!expect_smaller_file_size || processed_data_size < input_data_len,
		"The processed PNG should be smaller than the original PNG"
	);
}

#[tokio::test]
async fn lossless_optimization_works() {
	successful_process_test(
		PNG_DATA,
		PngFileOptions {
			color_quantization_target: ColorQuantizationTarget::None,
			..Default::default()
		},
		true, // Same pixels
		true, // Smaller size
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
		false, // Not necessarily the same pixels
		true,  // Smaller size
		"lossy_optimization_works"
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
