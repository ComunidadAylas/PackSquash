use crate::pack_file::util::BOM_UTF8;
use pretty_assertions::assert_eq;
use std::fmt::Debug;
use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static FRAGMENT_SHADER_DATA: &[u8] = include_bytes!("example.fsh");
static NON_TRANSFORMABLE_SHADER_DATA: &[u8] = include_bytes!("example_non_transformable.glsl");
static FALSE_POSITIVE_PARSE_ERROR: &[u8] = include_bytes!("example_false_positive_parse_error.glsl");

/// Processes the given input data as a [ShaderFile], using the provided settings,
/// expecting a successful result that equals the expected string.
#[allow(clippy::too_many_arguments)]
async fn successful_process_test<T: Extractable<TranslationUnit> + PartialEq + Debug + 'static>(
	input_data: &[u8],
	add_bom: bool,
	settings: ShaderFileOptions,
	is_vertex_or_fragment_shader: bool,
	is_top_level_translation_unit: bool,
	expect_parseable_input_and_equal_ast_after_processing: bool,
	expect_smaller_file_size: bool,
	expect_output_equal_to_input: bool
) {
	let shader_parser = Parser::new();

	let input_data_str = std::str::from_utf8(input_data);

	let input_data_with_maybe_bom = if add_bom {
		let mut input_data = input_data.to_vec();
		input_data.splice(0..0, BOM_UTF8);
		Cow::Owned(input_data)
	} else {
		Cow::Borrowed(input_data)
	};

	let data_stream = ShaderFile {
		read: Builder::new().read(&input_data_with_maybe_bom).build(),
		file_length_hint: input_data_with_maybe_bom.len(),
		is_vertex_or_fragment_shader,
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, BytesMut)> = data_stream
		.map(|result| result.expect("No error should happen while decoding"))
		.collect()
		.await;

	let mut data = Vec::with_capacity(input_data_with_maybe_bom.len());
	for (_, partial_data) in process_result {
		data.extend_from_slice(&partial_data);
	}

	let output_data_str = std::str::from_utf8(&data);

	eprintln!(
		"Processed shader source:\n--------------------\n{}\n--------------------",
		output_data_str.as_ref().unwrap_or(&"<NOT UTF-8>")
	);

	assert!(!data.is_empty(), "Some data was expected for this input");

	if expect_parseable_input_and_equal_ast_after_processing {
		let input_ast = shader_parser
			.parse::<T>(input_data, is_top_level_translation_unit)
			.expect("The test input data should be a valid GLSL symbol");

		let processed_ast = shader_parser
			.parse::<T>(&data, is_top_level_translation_unit)
			.expect("The result should be a valid GLSL symbol");

		assert_eq!(
			input_ast, processed_ast,
			"The processed translation unit should represent the same AST as the input"
		);
	}

	assert!(
		!expect_smaller_file_size || data.len() < input_data.len(),
		"The processed shader file should be smaller than the original"
	);

	assert!(
		!expect_output_equal_to_input || input_data_str == output_data_str,
		"The processed shader file should be the same as the original"
	);
}

#[tokio::test]
async fn minifying_works() {
	successful_process_test::<TranslationUnit>(
		FRAGMENT_SHADER_DATA,
		false, // No BOM
		ShaderFileOptions {
			source_transformation_strategy: ShaderSourceTransformationStrategy::Minify,
			..Default::default()
		},
		true,  // Fragment shader
		true,  // Top-level TU
		true,  // The output AST should match the input AST
		true,  // Smaller size
		false  // Different output text
	)
	.await
}

#[tokio::test]
async fn minifying_with_bom_works() {
	successful_process_test::<TranslationUnit>(
		FRAGMENT_SHADER_DATA,
		true, // Add BOM
		ShaderFileOptions {
			source_transformation_strategy: ShaderSourceTransformationStrategy::Minify,
			..Default::default()
		},
		true,  // Fragment shader
		true,  // Top-level TU
		true,  // The output AST should match the input AST
		true,  // Smaller size
		false  // Different output text
	)
	.await
}

#[tokio::test]
async fn prettifying_works() {
	successful_process_test::<TranslationUnit>(
		FRAGMENT_SHADER_DATA,
		false, // No BOM
		ShaderFileOptions {
			source_transformation_strategy: ShaderSourceTransformationStrategy::Prettify,
			..Default::default()
		},
		true,  // Fragment shader
		true,  // Top-level TU
		true,  // The output AST should match the input AST
		false, // Same/bigger size
		false  // Different output text
	)
	.await
}

#[tokio::test]
async fn passthrough_works() {
	successful_process_test::<TranslationUnit>(
		FRAGMENT_SHADER_DATA,
		false, // No BOM
		ShaderFileOptions {
			source_transformation_strategy: ShaderSourceTransformationStrategy::KeepAsIs,
			..Default::default()
		},
		true,  // Fragment shader
		true,  // Top-level TU
		true,  // The output AST should match the input AST
		false, // Same size
		true   // Same output text
	)
	.await
}

#[tokio::test]
async fn invalid_input_is_handled() {
	let mut data_stream = ShaderFile {
		read: Builder::new().read(&[]).build(),
		file_length_hint: 0,
		is_vertex_or_fragment_shader: true,
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
async fn minifying_is_averted_when_preprocessor_expansion_would_be_incomplete() {
	successful_process_test::<TranslationUnit>(
		NON_TRANSFORMABLE_SHADER_DATA,
		false, // No BOM
		ShaderFileOptions {
			source_transformation_strategy: ShaderSourceTransformationStrategy::Minify,
			..Default::default()
		},
		true,  // Fragment shader
		true,  // Top-level TU
		true,  // The output AST should match the input AST
		false, // Same size
		true   // Same output text
	)
	.await
}

#[tokio::test]
async fn tentative_parsing_errors_are_not_fatal() {
	successful_process_test::<TranslationUnit>(
		FALSE_POSITIVE_PARSE_ERROR,
		false, // No BOM
		ShaderFileOptions {
			source_transformation_strategy: ShaderSourceTransformationStrategy::Minify,
			..Default::default()
		},
		true,  // Fragment shader
		true,  // Top-level TU
		false, // The input AST cannot be constructed
		false, // Same size
		true   // Same output text
	)
	.await
}

#[tokio::test]
async fn include_shaders_may_not_be_translation_units() {
	successful_process_test::<Expr>(
		b"1 + 1",
		false, // No BOM
		ShaderFileOptions {
			source_transformation_strategy: ShaderSourceTransformationStrategy::Minify,
			..Default::default()
		},
		false, // Include shader
		false, // Not top-level
		true,  // The output AST should match the input AST
		true,  // Smaller size
		false  // Different output text
	)
	.await
}

#[tokio::test]
async fn include_shaders_with_preprocessor_directives_are_passed_through() {
	successful_process_test::<Expr>(
		b"#define LOVE:\nyou + me",
		false, // No BOM
		ShaderFileOptions {
			source_transformation_strategy: ShaderSourceTransformationStrategy::Minify,
			..Default::default()
		},
		false, // Include shader
		false, // Not top-level
		true,  // The output AST should match the input AST
		false, // Smaller size
		true   // Same output text
	)
	.await
}
