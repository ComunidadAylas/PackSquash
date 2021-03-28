use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static FRAGMENT_SHADER_DATA: &str = include_str!("example.fsh");
static MINIFIED_FRAGMENT_SHADER_DATA: &str = include_str!("example_minified.fsh");

/// Processes the given input data as a [ShaderFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input_data: &str,
	extension: &str,
	settings: OptimizationSettings,
	expected_result: &str
) {
	let input_data = input_data.as_bytes();

	let data_stream = ShaderFile {
		read: Builder::new().read(input_data).build(),
		extension: extension,
		file_length: input_data.len(),
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, OptimizedBytes<BytesMut>)> = data_stream
		.map(|result| result.expect("No error should happen while decoding"))
		.collect()
		.await;

	assert!(
		!process_result.is_empty(),
		"Some data was expected for this input"
	);

	let mut data = Vec::with_capacity(input_data.len());
	for (_, partial_data) in process_result {
		data.extend_from_slice(&partial_data);
	}

	let data = String::from_utf8(data).expect("The result should be a UTF-8 string");
	assert_eq!(&data, expected_result);
}

#[tokio::test]
async fn minifying_works() {
	successful_process_test(
		FRAGMENT_SHADER_DATA,
		"fsh",
		OptimizationSettings { minify: true },
		MINIFIED_FRAGMENT_SHADER_DATA
	)
	.await
}

#[tokio::test]
async fn passthrough_works() {
	successful_process_test(
		FRAGMENT_SHADER_DATA,
		"fsh",
		OptimizationSettings { minify: false },
		FRAGMENT_SHADER_DATA
	)
	.await
}

#[tokio::test]
async fn invalid_input_is_handled() {
	let mut data_stream = ShaderFile {
		read: Builder::new().read(&[]).build(),
		extension: "fsh",
		file_length: 0,
		optimization_settings: Default::default()
	}
	.process();

	data_stream
		.next()
		.await
		.expect("Expected some result for this input")
		.expect_err("Expected an error for this input");
}
