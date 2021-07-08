use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static PROPERTIES_DATA: &str = include_str!("example.properties");
static MINIFIED_PROPERTIES_DATA: &str = include_str!("example_minified.properties");

/// Processes the given input data as a [PropertiesFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input_data: &str,
	settings: PropertiesFileOptions,
	expected_result: &str
) {
	let input_data = input_data.as_bytes();

	let data_stream = PropertiesFile {
		read: Builder::new().read(input_data).build(),
		file_length: input_data.len(),
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, OptimizedBytes<ByteBuffer>)> = data_stream
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
	let mut properties_data_with_bom = String::from(PROPERTIES_DATA);
	properties_data_with_bom.insert(0, '\u{feff}');

	successful_process_test(
		&properties_data_with_bom,
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
	successful_process_test("", PropertiesFileOptions::default(), "").await
}
