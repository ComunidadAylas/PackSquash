use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static JSON_DATA: &str = include_str!("example.json");
static MINIFIED_JSON_DATA: &str = include_str!("example_minified.json");
static MINIFIED_AND_DEBLOATED_JSON_DATA: &str = include_str!("example_minified_and_debloated.json");
static PRETTIFIED_JSON_DATA: &str = include_str!("example_prettified.json");

/// Processes the given input data as a [JsonFile], using the provided settings,
/// expecting a successful result that equals the expected string.
async fn successful_process_test(
	input_data: &str,
	asset_type: JsonFileAssetType,
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
		JSON_DATA,
		JsonFileAssetType::Generic,
		JsonFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_JSON_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_with_bom_works() {
	let mut json_data_with_bom = String::from(JSON_DATA);
	json_data_with_bom.insert(0, '\u{feff}');

	successful_process_test(
		&json_data_with_bom,
		JsonFileAssetType::Generic,
		JsonFileOptions {
			minify: true,
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
		JsonFileAssetType::Generic,
		JsonFileOptions {
			minify: false,
			..Default::default()
		},
		PRETTIFIED_JSON_DATA
	)
	.await
}

#[tokio::test]
async fn minifying_and_debloating_model_works() {
	successful_process_test(
		JSON_DATA,
		JsonFileAssetType::MinecraftModel,
		JsonFileOptions {
			minify: true,
			..Default::default()
		},
		MINIFIED_AND_DEBLOATED_JSON_DATA
	)
	.await;
}

#[tokio::test]
async fn empty_input_is_handled_with_error() {
	let mut data_stream = JsonFile {
		read: Builder::new().read(&[]).build(),
		file_length_hint: 0,
		asset_type: JsonFileAssetType::Generic,
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
async fn strange_value_is_handled_consistently() {
	successful_process_test(
		"null",
		JsonFileAssetType::MinecraftModel,
		JsonFileOptions {
			// Debloat, to test debloating too
			delete_bloat: true,
			..Default::default()
		},
		"null"
	)
	.await;
}
