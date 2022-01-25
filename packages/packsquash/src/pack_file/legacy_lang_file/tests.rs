use crate::pack_file::util::BOM;
use pretty_assertions::assert_eq;
use tokio_test::io::Builder;

use super::*;

static LANG_DATA: &str = include_str!("base64.lang");
static MINIFIED_LANG_DATA: &str = include_str!("base64_minified.lang");
static LANG_DATA_WITH_MISSING_SEPARATOR: &str = include_str!("base64_missing_separator.lang");
static LANG_DATA_WITH_DUPLICATE_KEY: &str = include_str!("base64_duplicate_key.lang");
static LANG_DATA_WITH_BAD_FORMAT_STRING: &str = include_str!("base64_bad_format_string.lang");

/// Processes the given input data as a [LegacyLanguageFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input_text: &str,
	add_bom: bool,
	settings: LegacyLanguageFileOptions,
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

	let data_stream = LegacyLanguageFile {
		read: Builder::new().read(input_data).build(),
		file_length_hint: input_data.len(),
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

/// Processes the given input data as a [LegacyLanguageFile], using the provided settings,
/// expecting some error to happen.
async fn unsuccessful_process_test(
	input_text: &str,
	add_bom: bool,
	settings: LegacyLanguageFileOptions,
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

	let data_stream = LegacyLanguageFile {
		read: Builder::new().read(input_data).build(),
		file_length_hint: input_data.len(),
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
		LANG_DATA,
		false,
		LegacyLanguageFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_LANG_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_with_bom_and_bom_stripping_works() {
	successful_process_test(
		LANG_DATA,
		true,
		LegacyLanguageFileOptions {
			minify: true,
			strip_bom: true,
			..Default::default()
		},
		MINIFIED_LANG_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_with_bom_and_no_bom_stripping_works() {
	unsuccessful_process_test(
		LANG_DATA,
		true,
		LegacyLanguageFileOptions {
			minify: true,
			strip_bom: false,
			..Default::default()
		},
		// The first line would no longer be interpreted as a comment, and it contains no separator
		|err| matches!(err, OptimizationError::MissingSeparator(_)),
		""
	)
	.await
}

#[tokio::test]
async fn passthrough_without_bom_works() {
	successful_process_test(
		LANG_DATA,
		false,
		LegacyLanguageFileOptions {
			minify: false,
			strip_bom: false,
			..Default::default()
		},
		LANG_DATA
	)
	.await
}

#[tokio::test]
async fn passthrough_with_bom_and_bom_stripping_works() {
	successful_process_test(
		LANG_DATA,
		true,
		LegacyLanguageFileOptions {
			minify: false,
			strip_bom: true,
			..Default::default()
		},
		LANG_DATA
	)
	.await
}

#[tokio::test]
async fn line_with_missing_separator_is_handled() {
	unsuccessful_process_test(
		LANG_DATA_WITH_MISSING_SEPARATOR,
		false,
		Default::default(),
		|err| matches!(err, OptimizationError::MissingSeparator(_)),
		"Expected a missing separator error"
	)
	.await
}

#[tokio::test]
async fn duplicate_key_is_handled() {
	unsuccessful_process_test(
		LANG_DATA_WITH_DUPLICATE_KEY,
		false,
		Default::default(),
		|err| matches!(err, OptimizationError::DuplicateKey(_, _)),
		"Expected a duplicate key error"
	)
	.await
}

#[tokio::test]
async fn invalid_format_string_is_handled() {
	unsuccessful_process_test(
		LANG_DATA_WITH_BAD_FORMAT_STRING,
		false,
		Default::default(),
		|err| matches!(err, OptimizationError::InvalidFormatString(_)),
		"Expected an invalid format string error"
	)
	.await
}
