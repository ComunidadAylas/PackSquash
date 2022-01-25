use crate::pack_file::util::BOM;
use pretty_assertions::assert_eq;
use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static PROPERTIES_DATA: &str = include_str!("example.properties");
static MINIFIED_PROPERTIES_DATA: &str = include_str!("example_minified.properties");

/// Processes the given input data as a [PropertiesFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input_text: &str,
	add_bom: bool,
	settings: PropertiesFileOptions,
	expected_result: &str
) {
	let input_text = {
		let mut input_data = Cow::Borrowed(input_text);

		if add_bom {
			input_data.to_mut().insert(0, BOM);
		}

		input_data
	};
	let input_data = input_text.as_bytes();

	let data_stream = PropertiesFile {
		read: Builder::new().read(input_data).build(),
		file_length_hint: input_data.len(),
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, ByteBuffer)> = data_stream
		.map(|result| result.expect("No error should happen while decoding"))
		.collect()
		.await;

	let mut data = Vec::with_capacity(input_data.len());
	for (_, partial_data) in process_result {
		data.extend_from_slice(partial_data.as_ref());
	}

	let data = String::from_utf8(data).expect("The result should be a UTF-8 string");
	assert_eq!(&data, expected_result);
}

#[tokio::test]
async fn minifying_works() {
	successful_process_test(
		PROPERTIES_DATA,
		false,
		PropertiesFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_PROPERTIES_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_with_bom_works() {
	successful_process_test(
		PROPERTIES_DATA,
		true,
		PropertiesFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_PROPERTIES_DATA
	)
	.await
}

#[tokio::test]
async fn passthrough_works() {
	successful_process_test(
		PROPERTIES_DATA,
		false,
		PropertiesFileOptions {
			minify: false,
			..Default::default()
		},
		PROPERTIES_DATA
	)
	.await
}

#[tokio::test]
async fn passthrough_with_bom_works() {
	successful_process_test(
		PROPERTIES_DATA,
		true,
		PropertiesFileOptions {
			minify: false,
			..Default::default()
		},
		PROPERTIES_DATA
	)
	.await
}

#[tokio::test]
async fn empty_input_is_handled_properly() {
	successful_process_test("", false, PropertiesFileOptions::default(), "").await
}
