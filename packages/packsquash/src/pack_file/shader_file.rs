//! Contains code to optimize shader files.

use std::{borrow::Cow, io, str::Utf8Error};

use bytes::BytesMut;
use glsl_lang::ast::{Expr, Statement, TranslationUnit};
use glsl_lang::parse::Extractable;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use crate::config::{ShaderFileOptions, ShaderSourceTransformationStrategy};
use crate::pack_file::shader_file::parser::{ParsedSymbol, Transpilable};
use parser::{ParseError, Parser};

use super::{AsyncReadAndSizeHint, PackFile, PackFileAssetType, PackFileConstructor};

mod parser;

#[cfg(test)]
mod tests;

/// Represents a GLSL shader file (more precisely, a translation unit or shader stage, or a
/// segment of it).
///
/// Vanilla Minecraft uses fragment and vertex shaders that can be replaced by resource
/// packs for several effects, like the "creeper vision" showed while spectating a Creeper,
/// and the "Super Secret Settings" button that was ultimately removed. Minecraft mods may
/// support more shaders that can be added or replaced via resource packs.
pub struct ShaderFile<T: AsyncRead + Send + Unpin + 'static> {
	read: T,
	file_length_hint: usize,
	is_vertex_or_fragment_shader: bool,
	optimization_settings: ShaderFileOptions
}

/// Optimizer decoder that transforms shader source code files to an optimized
/// representation.
pub struct OptimizerDecoder {
	optimization_settings: ShaderFileOptions,
	is_vertex_or_fragment_shader: bool,
	reached_eof: bool
}

/// Represents an error that may happen while optimizing GLSL shader files.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OptimizationError {
	#[error("Invalid encoding: {0}")]
	InvalidEncoding(#[from] Utf8Error),
	#[error("Shader error: {0}")]
	InvalidShader(#[from] ParseError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, BytesMut);
	type Error = OptimizationError;

	fn decode(&mut self, _: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		Ok(None)
	}

	fn decode_eof(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		// This method will be called when EOF is reached until it returns None. Because we
		// will only ever output a single item in the stream, always return None if we have
		// executed once already
		if self.reached_eof {
			return Ok(None);
		}
		self.reached_eof = true;

		let shader_parser = Parser::new();
		let source_transformation_strategy =
			self.optimization_settings.source_transformation_strategy;

		if self.is_vertex_or_fragment_shader {
			// Vertex or fragment shaders must be parseable as translation units
			process_shader_as::<TranslationUnit>(
				src,
				&shader_parser,
				self.optimization_settings
					.is_top_level_shader
					.unwrap_or(true),
				source_transformation_strategy
			)
		} else {
			// Include shaders may not necessarily be a translation unit. In fact, they technically
			// can be any text, as long as its inclusion in a top-level shader yields valid GLSL. As
			// a compromise between rejecting technically valid inputs and allowing potential nonsensical
			// text in them, which hampers validation and minification/prettifying capabilities,
			// also accept standalone statements and expressions
			process_shader_as::<TranslationUnit>(
				src,
				&shader_parser,
				false,
				source_transformation_strategy
			)
			.or_else(|_| {
				process_shader_as::<Statement>(
					src,
					&shader_parser,
					false,
					source_transformation_strategy
				)
			})
			.or_else(|_| {
				process_shader_as::<Expr>(src, &shader_parser, false, source_transformation_strategy)
			})
		}
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFile for ShaderFile<T> {
	type ByteChunkType = BytesMut;
	type OptimizationError = OptimizationError;
	type OptimizedByteChunksStream = FramedRead<T, OptimizerDecoder>;

	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				optimization_settings: self.optimization_settings,
				is_vertex_or_fragment_shader: self.is_vertex_or_fragment_shader,
				reached_eof: false
			},
			self.file_length_hint
		)
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFileConstructor<T> for ShaderFile<T> {
	type OptimizationSettings = ShaderFileOptions;

	fn new(
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<T>>,
		asset_type: PackFileAssetType,
		optimization_settings: Self::OptimizationSettings
	) -> Option<Self> {
		file_read_producer().map(|(read, file_length_hint)| Self {
			read,
			// The file is too big to fit in memory if this conversion fails anyway
			file_length_hint: file_length_hint.try_into().unwrap_or(usize::MAX),
			is_vertex_or_fragment_shader: !matches!(
				asset_type,
				PackFileAssetType::TranslationUnitSegment
			),
			optimization_settings
		})
	}
}

/// Processes the shader code at the specified source buffer, trying to parse it as `T`.
/// An error is returned when the source can't be parsed as the specified symbol, which
/// may or may not be a format error depending on how the source is interpreted by Minecraft.
fn process_shader_as<T: Extractable<TranslationUnit> + 'static>(
	src: &mut BytesMut,
	shader_parser: &Parser,
	is_top_level_translation_unit: bool,
	source_transformation_strategy: ShaderSourceTransformationStrategy
) -> Result<Option<<OptimizerDecoder as Decoder>::Item>, OptimizationError>
where
	ParsedSymbol<T>: Transpilable
{
	if let (
		Some(symbol),
		ShaderSourceTransformationStrategy::Minify | ShaderSourceTransformationStrategy::Prettify
	) = (
		shader_parser.parse::<T>(src, is_top_level_translation_unit)?,
		source_transformation_strategy
	) {
		// The shader is valid and safe to transform
		let minify = matches!(
			source_transformation_strategy,
			ShaderSourceTransformationStrategy::Minify
		);

		let mut buf = src.split_off(0);
		buf.clear();

		buf.extend_from_slice(symbol.transpile(minify).as_bytes());

		Ok(Some((
			Cow::Borrowed(if minify { "Minified" } else { "Prettified" }),
			buf
		)))
	} else {
		// The shader is valid, but not safe to transform, or we don't want to transform it
		Ok(Some((
			Cow::Borrowed("Validated and copied"),
			src.split_off(0)
		)))
	}
}
