use pretty_assertions::assert_eq;
use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static PROPERTIES_DATA: &[u8] = include_bytes!("example.properties");
static MINIFIED_PROPERTIES_DATA: &[u8] = include_bytes!("example_minified.properties");
static PROPERTIES_WITH_UNICODE_DATA: &[u8] = include_bytes!("escaped_unicode_characters.properties");
static MINIFIED_PROPERTIES_WITH_UNICODE_DATA: &[u8] =
	include_bytes!("escaped_unicode_characters_minified.properties");

/// Processes the given input data as a [PropertiesFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input: &[u8],
	settings: PropertiesFileOptions,
	expected_result: &[u8]
) {
	let data_stream = PropertiesFile {
		read: Builder::new().read(input).build(),
		file_length_hint: input.len(),
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, ByteBuffer)> = data_stream
		.map(|result| result.expect("No error should happen while decoding"))
		.collect()
		.await;

	let mut data = Vec::with_capacity(input.len());
	for (_, partial_data) in process_result {
		data.extend_from_slice(partial_data.as_ref());
	}

	assert_eq!(&data, expected_result);
}

#[tokio::test]
async fn minifying_works() {
	successful_process_test(
		PROPERTIES_DATA,
		PropertiesFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_PROPERTIES_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_with_unicode_data_works() {
	successful_process_test(
		PROPERTIES_WITH_UNICODE_DATA,
		PropertiesFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_PROPERTIES_WITH_UNICODE_DATA
	)
	.await
}

#[tokio::test]
async fn passthrough_works() {
	successful_process_test(
		PROPERTIES_DATA,
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
	successful_process_test(b"", PropertiesFileOptions::default(), b"").await
}
