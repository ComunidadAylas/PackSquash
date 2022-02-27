use std::borrow::Cow;

use pretty_assertions::assert_eq;
use tokio_test::io::Builder;

use crate::pack_file::util::BOM;

use super::*;

static FUNCTION_DATA: &str = include_str!("example.mcfunction");
static MINIFIED_FUNCTION_DATA: &str = include_str!("example_minified.mcfunction");
static FUNCTION_DATA_LEADING_SLASH: &str = include_str!("leading_slash.mcfunction");
static FUNCTION_DATA_LEADING_DOUBLE_SLASH: &str = include_str!("leading_double_slash.mcfunction");

/// Processes the given input data as a [CommandFunctionFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input_text: &str,
	add_bom: bool,
	settings: CommandFunctionFileOptions,
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

	let data_stream = CommandFunctionFile {
		read: Builder::new().read(input_data).build(),
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, Vec<u8>)> = data_stream
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

/// Processes the given input data as a [CommandFunctionFile], using the provided settings,
/// expecting some error to happen.
async fn unsuccessful_process_test(
	input_text: &str,
	add_bom: bool,
	settings: CommandFunctionFileOptions,
	mut error_matcher: impl FnMut(OptimizationError) -> bool,
	failed_assertion_message: &'static str
) {
	let input_text = {
		let mut input_data = Cow::Borrowed(input_text);

		if add_bom {
			input_data.to_mut().insert(0, BOM);
		}

		input_data
	};
	let input_data = input_text.as_bytes();

	let data_stream = CommandFunctionFile {
		read: Builder::new().read(input_data).build(),
		optimization_settings: settings
	}
	.process();

	assert!(
		data_stream
			.any(|result| future::ready(match result {
				Ok(_) => false,
				Err(err) => error_matcher(err)
			}))
			.await,
		"{}",
		failed_assertion_message
	);
}

#[tokio::test]
async fn minifying_works() {
	successful_process_test(
		FUNCTION_DATA,
		false,
		CommandFunctionFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_FUNCTION_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_with_bom_works() {
	successful_process_test(
		FUNCTION_DATA,
		true,
		CommandFunctionFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_FUNCTION_DATA
	)
	.await
}

#[tokio::test]
async fn passthrough_works() {
	successful_process_test(
		FUNCTION_DATA,
		false,
		CommandFunctionFileOptions {
			minify: false,
			..Default::default()
		},
		FUNCTION_DATA
	)
	.await
}

#[tokio::test]
async fn passthrough_with_bom_works() {
	successful_process_test(
		FUNCTION_DATA,
		true,
		CommandFunctionFileOptions {
			minify: false,
			..Default::default()
		},
		FUNCTION_DATA
	)
	.await
}

#[tokio::test]
async fn command_with_leading_slash_is_handled() {
	unsuccessful_process_test(
		FUNCTION_DATA_LEADING_SLASH,
		false,
		Default::default(),
		|err| matches!(err, OptimizationError::GratuitousLeadingSlash(_)),
		"Expected a gratuitous leading slash error"
	)
	.await
}

#[tokio::test]
async fn comment_with_double_slash_delimiter_is_handled() {
	unsuccessful_process_test(
		FUNCTION_DATA_LEADING_DOUBLE_SLASH,
		false,
		Default::default(),
		|err| matches!(err, OptimizationError::DoubleSlashComment(_)),
		"Expected a double slash comment error"
	)
	.await
}
