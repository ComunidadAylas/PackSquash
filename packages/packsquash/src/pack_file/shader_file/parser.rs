//! GLSL parsing and transpilation code, based on `glsl_lang` and its companion crates.

use crate::pack_file::strip_utf8_bom;
use aho_corasick::AhoCorasick;
use const_format::concatcp;
use const_random::const_random;
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
use std::fmt::Write;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::sync::LazyLock;
use std::{fmt, path};
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

/// The path prefix used in [`MOJ_IMPORT_PLACEHOLDER_PRAGMA`] to mark a given path as
/// included via `<...>` syntax (i.e., relative to the `include` folder, a.k.a.
/// "system include" in C/GLSL preprocessor literature), as opposed to `"..."` syntax.
const MOJ_IMPORT_PLACEHOLDER_PRAGMA_RELATIVE_MARKER_PATH_PREFIX: &str =
	// The # character prevents this prefix from forming a valid resource location:
	// Minecraft parses import paths as resource locations
	concatcp!("__packsquash_internal_relative_marker#", const_random!(u16));

/// A placeholder, to be used only by PackSquash `#pragma` preprocessor directive that
/// signals that there was a `#moj_import` preprocessor directive at its position in
/// the original source.
///
/// This directive is replaced by PackSquash before outputting transpiled shader code
/// with its corresponding `#moj_import`. It should not be used by any external application
/// under any circumstances.
static MOJ_IMPORT_PLACEHOLDER_PRAGMA: &str = concatcp!(
	"#pragma __PACKSQUASH_INTERNAL_MOJ_IMPORT_PLACEHOLDER_",
	const_random!(u16),
	" "
);

/// An error that may happen while parsing a GLSL grammar symbol.
#[derive(Error, Debug)]
pub enum ParseError {
	#[error("Syntax error: {0}")]
	Syntax(#[from] glsl_lang::parse::ParseError<LexicalError<Infallible>>),
	#[error("Non-included shaders must define a main function, but it was not defined")]
	MissingMainFunction,
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

		let inserted_moj_import_placeholder_pragma = Cell::new(false);

		let mut preprocessor = Processor::new_with_fs(ImportFileSystem::new(
			&inserted_moj_import_placeholder_pragma
		));

		// Imports using the <...> syntax are known as "system imports", and the preprocessor needs
		// to know at least one system include directory to resolve them
		preprocessor
			.system_paths_mut()
			.push(MOJ_IMPORT_PLACEHOLDER_PRAGMA_RELATIVE_MARKER_PATH_PREFIX.into());

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
					return Err(ParseError::MissingMainFunction);
				}

				// Expanding and removing preprocessor directives in top-level shaders does not alter
				// their semantics, as long as we are able to resolve every #moj_import to its source
				// code, or no shader imported with #moj_import defines preprocessor variables that
				// affect the source code compiled in the top-level shader.
				//
				// In general, we may not be able to resolve #moj_import to source files. But we can
				// keep potential semantic alterations to a minimum if we check that the AST is capable
				// of holding preprocessor directives that cannot be expanded in the same position as
				// they originally were in the source. If that's not the case, then it's definitely not
				// safe to transform this code
				Ok((lacks_preprocessor_directives
					|| self.lacks_injectable_pp_directives_out_of_external_declaration_position(
						&source
					))
				.then(|| {
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
			|err| Err(err.into()),
			|map_result| {
				map_result.map(|symbol| {
					symbol.flatten().map(|symbol| ParsedSymbol {
						symbol,
						inserted_moj_import_placeholder_pragma:
							inserted_moj_import_placeholder_pragma.into_inner()
					})
				})
			}
		)
	}

	/// Checks whether the specified GLSL source lacks injectable preprocessor directives
	/// out of external declaration position. Currently, the injectable preprocessor directives
	/// are `#version`, `#extension`, `#pragma` and `#moj_import`, which are necessary to
	/// preserve in the generated GLSL code for the game to parse shaders as intended.
	///
	/// Currently, this method is implemented by removing preprocessor directives that do not get
	/// injected (i.e., are expanded) and trying to parse the result with a non-preprocessing lexer
	/// that only accepts preprocessor directives in external declaration position. Therefore, if
	/// it is best to avoid calling it if it is known that there are no preprocessor directives.
	fn lacks_injectable_pp_directives_out_of_external_declaration_position(
		&self,
		source: &str
	) -> bool {
		/// The newline escapes accepted by our parser and the GLSL specification.
		const NEWLINE_ESCAPES: &[&str] = &["\\\n", "\\\r\n"];
		/// The preprocessing matchers used in this method.
		static MATCHERS: LazyLock<(AhoCorasick, Regex)> = LazyLock::new(|| {
			(
				AhoCorasick::new_auto_configured(NEWLINE_ESCAPES),
				// When this function is called we know that any present preprocessor directives
				// are valid and follow their expected syntax, so this regex can be a bit lenient
				Regex::new("(?m:^[[:space:]]*#[[:space:]]*(?:|define.*|undef.*|if.*|else.*|elif.*|endif.*|error.*|line.*)$)").unwrap()
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

	/// Replaces internal, temporary `#moj_import` placeholder pragmas with their corresponding
	/// `#moj_import` directive in the provided `source`, which is assumed to have been generated
	/// by the GLSL transpiler used by PackSquash. See [`MOJ_IMPORT_PLACEHOLDER_PRAGMA`] for more
	/// details.
	///
	/// The implementation of this method is optimized for the case where there is at least one
	/// placeholder `#pragma` to replace.
	fn replace_moj_import_placeholder_pragmas(source: &str) -> String {
		source
			.split('\n') // Our GLSL transpiler always uses LF and doesn't insert newline escapes
			.map(|line| {
				if let Some(import_path) = line
					.trim_start_matches(|c: char| c.is_ascii_whitespace())
					.strip_prefix(MOJ_IMPORT_PLACEHOLDER_PRAGMA)
				{
					if let Some(import_path) = import_path.strip_prefix(concatcp!(
						MOJ_IMPORT_PLACEHOLDER_PRAGMA_RELATIVE_MARKER_PATH_PREFIX,
						path::MAIN_SEPARATOR
					)) {
						// Luckily, Minecraft does not support escaping > or " in #moj_import paths
						Cow::Owned(format!("#moj_import <{import_path}>"))
					} else {
						Cow::Owned(format!("#moj_import \"{import_path}\""))
					}
				} else {
					Cow::Borrowed(line)
				}
			})
			.intersperse(Cow::Borrowed("\n"))
			.collect()
	}
}

/// The virtual filesystem used by [`PreprocessingLexer`] to expand `#moj_import`
/// directives to the contents of the files they reference.
///
/// This filesystem records whether any `#moj_import` directive was expanded.
struct ImportFileSystem<'flag> {
	inserted_moj_import_placeholder_pragma: &'flag Cell<bool>
}

impl<'flag> ImportFileSystem<'flag> {
	/// Creates a new virtual filesystem used by the preprocessing shader lexer.
	/// `inserted_moj_import_placeholder_pragma` will be set to `true` if any
	/// `#moj_import` directive is expanded.
	fn new(inserted_moj_import_placeholder_pragma: &'flag Cell<bool>) -> Self {
		Self {
			inserted_moj_import_placeholder_pragma
		}
	}
}

impl FileSystem for ImportFileSystem<'_> {
	type Error = Infallible;

	fn canonicalize(&self, path: &Path) -> Result<PathBuf, Self::Error> {
		Ok(path.into())
	}

	fn exists(&self, _: &Path) -> bool {
		true
	}

	fn read(&self, path: &Path) -> Result<Cow<'_, str>, Self::Error> {
		// For now, always expand included files to placeholder #pragmas, to replace them
		// back with #moj_imports when transpiling. For v0.4.0, we could try to actually
		// expand the files for parsing, which should support more inputs (e.g., shaders
		// that depend on included tokens to be syntactically correct, or rely on included
		// preprocessor variables to select code to compile). If the referenced file does
		// not exist on this pack, then we could fall back to accepting potentially invalid
		// input: it could become valid after resolving imports
		self.inserted_moj_import_placeholder_pragma.set(true);
		Ok(format!("{MOJ_IMPORT_PLACEHOLDER_PRAGMA}{}", path.to_str().unwrap()).into())
	}
}

/// A parsed GLSL grammar symbol returned by a [`Parser`].
#[derive(Debug)]
pub struct ParsedSymbol<T> {
	symbol: T,
	inserted_moj_import_placeholder_pragma: bool
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

		if self.inserted_moj_import_placeholder_pragma {
			Parser::replace_moj_import_placeholder_pragmas(&output)
		} else {
			output
		}
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

impl<T: PartialEq> PartialEq for ParsedSymbol<T> {
	fn eq(&self, other: &Self) -> bool {
		self.symbol.eq(&other.symbol)
	}
}

impl<T: Eq> Eq for ParsedSymbol<T> {}

/// Internal trait used to mark types that can carry out a part of the
/// transpilation steps for GLSL symbols.
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

impl TranspilableInner for ParsedSymbol<Statement> {
	fn transpile<W: Write + ?Sized>(
		&self,
		f: &mut W,
		formatting_state: &mut FormattingState
	) -> fmt::Result {
		glsl_lang::transpiler::glsl::show_statement(f, self, formatting_state)
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
