//! GLSL parsing and transpilation code, based on `glsl_lang` and its companion crates.

use crate::pack_file::strip_utf8_bom;
use aho_corasick::AhoCorasick;
use glsl_lang::ast::{
	Expr, ExternalDeclarationData, FileId, Statement, TranslationUnit, TypeSpecifierNonArrayData
};
use glsl_lang::parse::{Extractable, Parse};
use glsl_lang::transpiler::glsl::{FormattingSettings, FormattingState};
use glsl_lang_lexer::v2_full::fs::PreprocessorExt;
use glsl_lang_lexer::v2_full::LexicalError;
use glsl_lang_lexer::ParseOptions;
use glsl_lang_pp::processor::fs::{FileSystem, Processor};
use regex::Regex;
use std::any::TypeId;
use std::borrow::Cow;
use std::cell::Cell;
use std::convert::Infallible;
use std::fmt;
use std::fmt::Write;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::sync::LazyLock;
use thiserror::Error;

/// A GLSL lexer that does not preprocess its input before parsing, considering
/// preprocessor directives as symbols of the external declaration production rule.
/// This lexer retains the preprocessor directives in the AST it produces, but it
/// doesn't support inputs that contain preprocessor directives out of external
/// declaration position.
type NonPreprocessingLexer<'i> = glsl_lang_lexer::v2_min::str::Lexer<'i>;
/// A preprocessing GLSL lexer that preprocesses its input before parsing, expanding
/// preprocessor directives and resolving imports. This lexer does not retain preprocessor
/// directive information in the produced AST, but it supports inputs that use preprocessor
/// directives in any position. As a compromise, it is possible to inject preprocessor directives
/// back into the AST in external declaration position after parsing, but this is only guaranteed
/// to yield correct results if such directives originally were in such position.
type PreprocessingLexer<'r, 'p, 'flag> =
	glsl_lang_lexer::v2_full::fs::Lexer<'r, 'p, ImportFileSystem<'flag>>;

/// An error that may happen while parsing a GLSL grammar symbol.
#[derive(Error, Debug)]
pub enum ParseError {
	#[error("Syntax error: {error}")]
	Syntax {
		error: glsl_lang::parse::ParseError<LexicalError<Infallible>>,
		/// Represents whether this syntax error happened when the source file contained
		/// an unresolvable `#moj_import` directive. This indicates that the syntax error
		/// might be a false positive, as it can be caused due to preprocessor directives
		/// being inappropriately expanded or masked: imported shaders can change the
		/// outcome of the preprocessing stage.
		found_unresolvable_moj_import: bool
	},
	#[error("Non-included shaders must define a main function, but it was not defined")]
	MissingMainFunction {
		/// Represents whether this error happened when the source file contained an
		/// unresolvable `#moj_import` directive. This indicates that the error might
		/// be a false positive, as it can be caused due to changes in the outcome of
		/// the preprocessing stage compared to during runtime.
		found_unresolvable_moj_import: bool
	},
	#[error("Invalid encoding: {0}")]
	InvalidEncoding(#[from] Utf8Error)
}

/// A GLSL parser capable of parsing a GLSL grammar symbol.
pub struct Parser {
	parse_options: ParseOptions
}

impl Parser {
	/// Creates a new GLSL parser.
	pub fn new() -> Self {
		Self {
			parse_options: ParseOptions {
				// GLSL specification, section 3.3: "Version 1.10 of the language does not
				// require shaders to include this directive, and shaders that do not
				// include a #version directive will be treated as targeting version 1.10."
				default_version: 110,
				target_vulkan: false,
				source_id: FileId::default(),
				allow_rs_ident: false
			}
		}
	}

	/// Parses the specified `source` as the GLSL grammar symbol `T`. `is_top_level_translation_unit`
	/// signals whether the translation unit parsed as such at `source` is to be considered top-level
	/// (i.e., parsed by Minecraft as such, being the only source snippet added to a shader executable
	/// with `glShaderSource` before calling `glCompileShader`, after custom `#moj_import` preprocessing).
	///
	/// Returns `Ok(Some(_))` when `source` can be parsed as `T` and is safe to transpile (i.e., the AST
	/// completely defines the shader, including preprocessor directives that must be kept for the game
	/// to consume). `Ok(None)` is returned when `source` can be parsed as `T`, but it is not safe to
	/// transpile. `Err(_)` is returned when `source` cannot be parsed as `T`.
	pub fn parse<T: Extractable<TranslationUnit> + 'static>(
		&self,
		source: &[u8],
		is_top_level_translation_unit: bool
	) -> Result<Option<ParsedSymbol<T>>, ParseError> {
		// Wrap the input GLSL symbol into something that parses as a translation unit
		let source = T::wrap(std::str::from_utf8(strip_utf8_bom(source))?);

		let found_unresolvable_moj_import = Cell::new(false);

		let mut preprocessor =
			Processor::new_with_fs(ImportFileSystem::new(&found_unresolvable_moj_import));

		// Imports using the <...> syntax are known as "system imports", and the preprocessor needs
		// to know at least one system include directory to resolve them
		preprocessor.system_paths_mut().push(PathBuf::new());

		TranslationUnit::parse_with_options::<PreprocessingLexer>(
			preprocessor.open_source(&source, ""),
			&self.parse_options
		)
		.map(|(mut translation_unit, _, lexer_iter)| {
			let preprocessor_directives = lexer_iter.into_directives();
			let lacks_preprocessor_directives = preprocessor_directives.directives().is_empty();

			if TypeId::of::<T>() == TypeId::of::<TranslationUnit>() && is_top_level_translation_unit {
				// GLSL 4.60 specification, section 6.1: "A shader need not contain a function named
				// main, but one shader in a set of shaders linked together to form a single shader
				// executable must, or a link-time error results. This function takes no arguments,
				// returns no value, and must be declared as type void".
				// Minecraft builds each top-level shader to its own shader executable, so such shaders
				// must contain a main function
				if !translation_unit.0.iter().any(|external_declaration| {
					if let ExternalDeclarationData::FunctionDefinition(function_definition) =
						&external_declaration.content
					{
						let function_prototype = &function_definition.prototype;

						function_prototype.name.0 == "main"
							&& function_prototype.ty.ty.ty.content == TypeSpecifierNonArrayData::Void
							&& function_prototype.ty.ty.array_specifier.is_none()
							&& function_prototype.ty.qualifier.is_none()
							&& function_prototype.parameters.is_empty()
					} else {
						false
					}
				}) {
					return Err(ParseError::MissingMainFunction {
						found_unresolvable_moj_import: found_unresolvable_moj_import.get()
					});
				}

				// Expanding and removing preprocessor directives in top-level shaders does not alter
				// their semantics, as long as we are able to resolve every #moj_import to its source
				// code, because shaders imported with #moj_import may define preprocessor variables
				// that affect the source code compiled in the top-level shader.
				//
				// In general, we may not be able to resolve #moj_import to source files. But we can
				// keep semantics if we check that we could resolve all the #moj_import directives, and
				// the AST is capable of holding preprocessor directives that cannot be expanded in the
				// same position as they originally were in the source. If that's not the case, then
				// it's definitely not safe to transform this code
				let safe_to_transpile = lacks_preprocessor_directives
					|| (!found_unresolvable_moj_import.get()
						&& self.lacks_injectable_pp_directives_out_of_external_declaration_position(
							&source
						));

				Ok(safe_to_transpile.then(|| {
					preprocessor_directives.inject(&mut translation_unit);

					// Wrapping and extracting a TU into itself is a no-op, so this always returns Some
					T::extract(translation_unit)
				}))
			} else {
				// - Non-TUs are parsed by extracting their AST node from a TU, which discards any
				//   preprocessor directive nodes, and may thus change the semantics of the source code.
				// - Non-top-level TUs should never get their preprocessor directives expanded because
				//   they might be relied on by the top-level TU when preprocessing its source code.
				Ok(lacks_preprocessor_directives.then(|| T::extract(translation_unit)))
			}
		})
		.map_or_else(
			|error| {
				Err(ParseError::Syntax {
					error,
					found_unresolvable_moj_import: found_unresolvable_moj_import.get()
				})
			},
			|map_result| {
				map_result.map(|symbol| symbol.flatten().map(|symbol| ParsedSymbol { symbol }))
			}
		)
	}

	/// Checks whether the specified valid GLSL source lacks injectable preprocessor directives
	/// out of external declaration position. Currently, the injectable preprocessor directives
	/// are `#version`, `#extension`, `#pragma` and `#moj_import`, which are necessary to preserve
	/// (or, in the case of `#moj_import`, expand) in the generated GLSL code for the game to
	/// parse shaders as intended.
	///
	/// Currently, this method is implemented by removing preprocessor directives that do not get
	/// injected (i.e., are expanded) and trying to parse the result with a non-preprocessing lexer
	/// that only accepts preprocessor directives in external declaration position. Therefore, if
	/// it is best to avoid calling it if it is known that there are no preprocessor directives.
	/// False negatives are returned if the source cannot be parsed when removing preprocessor
	/// directives whose expansion is necessary for syntax correctness, which given the usage of
	/// this function is a safe but inefficient fallback.
	fn lacks_injectable_pp_directives_out_of_external_declaration_position(
		&self,
		source: &str
	) -> bool {
		/// The newline escapes accepted by our parser and the GLSL specification.
		const NEWLINE_ESCAPES: &[&str] = &["\\\n", "\\\r\n"];
		/// The preprocessing matchers used in this method.
		static MATCHERS: LazyLock<(AhoCorasick, Regex)> = LazyLock::new(|| {
			(
				AhoCorasick::new(NEWLINE_ESCAPES).unwrap(),
				// When this function is called we know that any present preprocessor directives
				// are valid and follow their expected syntax, so this regex can be a bit lenient
				Regex::new("(?m:^[[:space:]]*#[[:space:]]*(?:|define.*|undef.*|if.*|else.*|elif.*|endif.*|error.*|line.*|moj_import.*)$)").unwrap()
			)
		});

		let (newline_escapes_matcher, uninjectable_preprocessor_directives_matcher) = &*MATCHERS;

		// Before preprocessing, section 3.1 of the GLSL specification mandates that newline escapes
		// must be removed
		let source_without_newline_escapes =
			newline_escapes_matcher.replace_all(source, &[""; NEWLINE_ESCAPES.len()]);

		// Preprocessor directives always take a single line after newline escapes have been removed,
		// so matching them now with a line regex is correct
		let source_without_uninjectable_preprocessor_directives =
			uninjectable_preprocessor_directives_matcher
				.replace_all(&source_without_newline_escapes, "");

		TranslationUnit::parse_with_options::<NonPreprocessingLexer>(
			&source_without_uninjectable_preprocessor_directives,
			&self.parse_options
		)
		.is_ok()
	}
}

/// The virtual filesystem used by [`PreprocessingLexer`] to expand `#moj_import`
/// directives to the contents of the files they reference.
///
/// This filesystem records whether any `#moj_import` directive was expanded.
struct ImportFileSystem<'flag> {
	found_unresolvable_moj_import: &'flag Cell<bool>
}

impl<'flag> ImportFileSystem<'flag> {
	/// Creates a new virtual filesystem used by the preprocessing shader lexer.
	/// `found_unresolvable_moj_import` will be set to `true` if any `#moj_import`
	/// directive is found, because it is not possible to expand them at the
	/// moment.
	fn new(found_unresolvable_moj_import: &'flag Cell<bool>) -> Self {
		Self {
			found_unresolvable_moj_import
		}
	}
}

impl FileSystem for ImportFileSystem<'_> {
	type Error = Infallible;

	fn canonicalize(&self, _: &Path) -> Result<PathBuf, Self::Error> {
		Ok(PathBuf::new())
	}

	fn exists(&self, _: &Path) -> bool {
		true
	}

	fn read(&self, _: &Path) -> Result<Cow<'_, str>, Self::Error> {
		// For now, just record that a #moj_import was found. A #moj_import'ed file
		// may contain preprocessor directives whose knowledge may be necessary to properly
		// expand other preprocessor directives, and thus keep the GLSL source semantics
		// when doing AST transformations and transpiling. We can't resolve #moj_import
		// at the moment because we can't access pack files here, so if such a directive
		// is found, we know we should not proceed with any transformations: doing AST
		// transformations and transpiling is appropriate if and only if we can expand all
		// the preprocessor directives. Also consider that not expanding can cause failure
		// to correctly parse shaders that depend on imported text to be syntactically
		// correct, but those should be not that common to find in practice
		self.found_unresolvable_moj_import.set(true);
		Ok("".into())
	}
}

/// A parsed GLSL grammar symbol returned by a [`Parser`].
#[derive(Debug, Eq, PartialEq)]
pub struct ParsedSymbol<T> {
	symbol: T
}

/// Represents a GLSL grammar symbol that can be transpiled back to GLSL.
pub trait Transpilable {
	/// Transpiles this GLSL symbol to GLSL. The output code is minified
	/// if `minify` is `true`; else, it is prettified.
	fn transpile(&self, minify: bool) -> String;
}

impl<T> Transpilable for ParsedSymbol<T>
where
	ParsedSymbol<T>: TranspilableInner
{
	fn transpile(&self, minify: bool) -> String {
		let mut output = String::new();

		TranspilableInner::transpile(
			self,
			&mut output,
			&mut FormattingState::from(&if minify {
				FormattingSettings::minifying()
			} else {
				FormattingSettings::default()
			})
		)
		.unwrap();

		output
	}
}

impl<T> Deref for ParsedSymbol<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.symbol
	}
}

impl<T> DerefMut for ParsedSymbol<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.symbol
	}
}

/// Internal trait used to encapsulate GLSL symbol-specific transpilation logic.
trait TranspilableInner {
	fn transpile<W: Write + ?Sized>(
		&self,
		f: &mut W,
		formatting_state: &mut FormattingState
	) -> fmt::Result;
}

impl TranspilableInner for ParsedSymbol<TranslationUnit> {
	fn transpile<W: Write + ?Sized>(
		&self,
		f: &mut W,
		formatting_state: &mut FormattingState
	) -> fmt::Result {
		glsl_lang::transpiler::glsl::show_translation_unit(f, self, *formatting_state)
	}
}

impl TranspilableInner for ParsedSymbol<Vec<Statement>> {
	fn transpile<W: Write + ?Sized>(
		&self,
		f: &mut W,
		formatting_state: &mut FormattingState
	) -> fmt::Result {
		for statement in &self.symbol {
			glsl_lang::transpiler::glsl::show_statement(f, statement, formatting_state)?;
		}

		Ok(())
	}
}

impl TranspilableInner for ParsedSymbol<Expr> {
	fn transpile<W: Write + ?Sized>(
		&self,
		f: &mut W,
		formatting_state: &mut FormattingState
	) -> fmt::Result {
		glsl_lang::transpiler::glsl::show_expr(f, self, formatting_state)
	}
}
