use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static FRAGMENT_SHADER_DATA: &str = include_str!("example.fsh");

/// Processes the given input data as a [ShaderFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input_data: &str,
	extension: &str,
	settings: OptimizationSettings,
	expect_smaller_file_size: bool
) {
	let input_ast = TranslationUnit::parse(&input_data)
		.expect("The input data should be a valid translation unit");
	let input_data = input_data.as_bytes();

	let data_stream = ShaderFile {
		read: Builder::new().read(input_data).build(),
		extension,
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
	let processed_ast =
		TranslationUnit::parse(&data).expect("The result should be a valid translation unit");

	assert_eq!(
		input_ast, processed_ast,
		"The processed translation unit should represent the same AST as the input"
	);

	assert!(
		!expect_smaller_file_size || data.as_bytes().len() < input_data.len(),
		"The processed shader file should be smaller than the original"
	);
}

#[tokio::test]
async fn minifying_works() {
	successful_process_test(
		FRAGMENT_SHADER_DATA,
		"fsh",
		OptimizationSettings { minify: true },
		true // Smaller size
	)
	.await
}

#[tokio::test]
async fn passthrough_works() {
	successful_process_test(
		FRAGMENT_SHADER_DATA,
		"fsh",
		OptimizationSettings { minify: false },
		false // Same size
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
