use crate::pack_file::util::BOM;
use pretty_assertions::assert_eq;
use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static JSON_DATA: &str = include_str!("example.json");
static MINIFIED_JSON_DATA: &str = include_str!("example_minified.json");
static MINIFIED_AND_DEBLOATED_JSON_DATA: &str = include_str!("example_minified_and_debloated.json");
static PRETTIFIED_JSON_DATA: &str = include_str!("example_prettified.json");
static PRETTIFIED_SORTED_JSON_DATA: &str = include_str!("example_prettified_sorted.json");
static DEEPLY_NESTED_JSON_DATA: &str = include_str!("deeply_nested.json");
static MINIFIED_DEEPLY_NESTED_JSON_DATA: &str = include_str!("deeply_nested_minified.json");

/// Processes the given input data as a [JsonFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input_data: &str,
	asset_type: PackFileAssetType,
	settings: JsonFileOptions,
	expected_result: &str
) {
	let input_data = input_data.as_bytes();

	let data_stream = JsonFile {
		read: Builder::new().read(input_data).build(),
		file_length_hint: input_data.len(),
		asset_type,
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, BytesMut)> = data_stream
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

/// Processes the given input data as a [JsonFile], using the provided settings,
/// expecting an unsuccessful result.
async fn unsuccessful_process_test(
	input_data: &[u8],
	asset_type: PackFileAssetType,
	settings: JsonFileOptions
) {
	let mut data_stream = JsonFile {
		read: Builder::new().read(input_data).build(),
		file_length_hint: input_data.len(),
		asset_type,
		optimization_settings: settings
	}
	.process();

	data_stream
		.next()
		.await
		.expect("Expected some result for this input")
		.expect_err("Expected an error for this input");
}

#[tokio::test]
async fn minifying_works() {
	successful_process_test(
		JSON_DATA,
		PackFileAssetType::GenericJson,
		JsonFileOptions {
			minify: true,
			sort_object_keys: false,
			..Default::default()
		},
		MINIFIED_JSON_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_with_bom_works() {
	let mut json_data_with_bom = String::from(JSON_DATA);
	json_data_with_bom.insert(0, BOM);

	successful_process_test(
		&json_data_with_bom,
		PackFileAssetType::GenericJson,
		JsonFileOptions {
			minify: true,
			sort_object_keys: false,
			..Default::default()
		},
		MINIFIED_JSON_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_with_comments_works() {
	let mut json_data_with_comment = String::from(JSON_DATA);
	json_data_with_comment.insert_str(0, "// This is a comment\n");

	successful_process_test(
		&json_data_with_comment,
		PackFileAssetType::GenericJson,
		JsonFileOptions {
			minify: true,
			always_allow_comments: true,
			sort_object_keys: false,
			..Default::default()
		},
		MINIFIED_JSON_DATA
	)
	.await
}

#[tokio::test]
async fn prettifying_works() {
	successful_process_test(
		JSON_DATA,
		PackFileAssetType::GenericJson,
		JsonFileOptions {
			minify: false,
			sort_object_keys: false,
			..Default::default()
		},
		PRETTIFIED_JSON_DATA
	)
	.await
}

#[tokio::test]
async fn prettifying_with_key_sorting_works() {
	successful_process_test(
		JSON_DATA,
		PackFileAssetType::GenericJson,
		JsonFileOptions {
			minify: false,
			sort_object_keys: true,
			..Default::default()
		},
		PRETTIFIED_SORTED_JSON_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_and_debloating_model_works() {
	successful_process_test(
		JSON_DATA,
		PackFileAssetType::MinecraftModel,
		JsonFileOptions {
			minify: true,
			sort_object_keys: false,
			..Default::default()
		},
		MINIFIED_AND_DEBLOATED_JSON_DATA
	)
	.await;
}

#[tokio::test]
async fn comments_are_always_allowed_for_specific_extensions() {
	let mut json_data_with_comment = String::from(JSON_DATA);
	json_data_with_comment.insert_str(0, "// This is a comment\n");

	successful_process_test(
		&json_data_with_comment,
		PackFileAssetType::GenericJsonWithComments,
		JsonFileOptions {
			minify: true,
			delete_bloat: false,
			always_allow_comments: false,
			sort_object_keys: false,
			..Default::default()
		},
		MINIFIED_JSON_DATA
	)
	.await;
}

#[tokio::test]
async fn comments_are_rejected_when_not_allowed() {
	let mut json_data_with_comment = String::from(JSON_DATA);
	json_data_with_comment.insert_str(0, "// This is a comment\n");

	unsuccessful_process_test(
		json_data_with_comment.as_bytes(),
		PackFileAssetType::GenericJson,
		JsonFileOptions {
			always_allow_comments: false,
			..Default::default()
		}
	)
	.await;
}

#[tokio::test]
async fn empty_input_is_handled_with_error() {
	unsuccessful_process_test(&[], PackFileAssetType::GenericJson, Default::default()).await;
}

#[tokio::test]
async fn unexpected_value_is_handled_with_error() {
	const NULL_VALUE: &str = "null";

	unsuccessful_process_test(
		NULL_VALUE.as_bytes(),
		PackFileAssetType::MinecraftModel,
		JsonFileOptions {
			// Debloat, to test debloating too
			delete_bloat: true,
			..Default::default()
		}
	)
	.await;
}

#[tokio::test]
async fn strange_value_is_handled_consistently() {
	// Non-object or array values are allowed by the RFC 8259 (which obsoletes RFC 4627) and ECMA-404
	// standards. Programs that strictly conform to RFC 4627 (for example, Gson in strict mode) will
	// reject such values. PackSquash follows the latest standards, so it allows such values. If
	// Minecraft or some mod actually needs them, then a parser capable of reading them will be used.
	//
	// The only concern here is if interoperability with software that parses JSON according to RFC
	// 4627 is needed. In such cases PackSquash would not make the JSON files less interoperable than
	// before, and such files are hopelessly unusable with such software anyway. Our goal is to
	// enforce JSON correctness, not specific file schemas or restrictions imposed by obsolete
	// standards
	successful_process_test(
		"null",
		PackFileAssetType::GenericJson,
		Default::default(),
		"null"
	)
	.await;
}

#[tokio::test]
async fn deeply_nested_value_works() {
	successful_process_test(
		DEEPLY_NESTED_JSON_DATA,
		PackFileAssetType::MinecraftModel,
		JsonFileOptions {
			minify: true,
			delete_bloat: true,
			sort_object_keys: false, // Actually sorting keys in this file is prone to allocation errors
			..JsonFileOptions::default()
		},
		MINIFIED_DEEPLY_NESTED_JSON_DATA
	)
	.await;
}
