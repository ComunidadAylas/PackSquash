use std::{
	borrow::Cow,
	io::{self, Write},
	str::Utf8Error
};

use bytes::{BufMut, BytesMut};
use glsl::{
	parser::{Parse, ParseError},
	syntax::TranslationUnit
};
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use super::{util::bom_stripper, OptimizedBytes, ResourcePackFile};

#[cfg(test)]
mod tests;

/// Represents a GLSL shader file (more precisely, a translation unit or shader stage).
///
/// Vanilla Minecraft uses fragment and vertex shaders that can be replaced by resource
/// packs for several effects, like the "creeper vision" showed while spectating a Creeper,
/// and the "Super Secret Settings" button that was ultimately removed.
/// Minecraft mods may support more shaders that can be added or replaced via resource packs.
struct ShaderFile<'a, T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length: usize,
	extension: &'a str,
	optimization_settings: OptimizationSettings
}

/// Parameters that influence how a [ShaderFile] is optimized.
struct OptimizationSettings {
	/// If true, the shader source code will be minified, which normally
	/// improves compressibility, or even replaces it altogether for tiny
	/// files. If false, the data will be parsed, to check for errors, and
	/// then returned as-is if the validation is successful
	minify: bool
}

impl Default for OptimizationSettings {
	fn default() -> Self {
		Self { minify: true }
	}
}

/// Helper struct to treat a [std::io::Write] like a [std::fmt::Write],
/// bridging the gap between character-oriented and byte-oriented sinks.
struct FormatWrite<W: Write>(W);

impl<W: Write> std::fmt::Write for FormatWrite<W> {
	fn write_str(&mut self, s: &str) -> std::fmt::Result {
		self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
	}
}

/// Optimizer decoder that transforms shader source code files to an optimized
/// representation.
struct OptimizerDecoder {
	file_length: usize,
	optimization_settings: OptimizationSettings
}

#[derive(Error, Debug)]
enum OptimizationError {
	#[error("Invalid UTF-8 character encoding: {0}")]
	InvalidUtf8(#[from] Utf8Error),
	#[error("Invalid shader code: {0}")]
	InvalidShaderStage(#[from] ParseError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, OptimizedBytes<BytesMut>);
	type Error = OptimizationError;

	fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		// FIXME: actual framing?
		// (i.e. do not hold the entire file in memory before decoding, so that frame != file)

		// Check if we have the entire file (i.e. frame)
		if src.len() < self.file_length {
			return Ok(None);
		}

		// Parse the translation unit
		let translation_unit =
			TranslationUnit::parse(std::str::from_utf8(bom_stripper::strip_utf8_bom(src))?)?;

		if self.optimization_settings.minify {
			// Transpile the translation unit back to a more compact GLSL string
			let mut buf_writer = {
				let mut buf = src.split_off(0);
				buf.clear();
				buf.writer()
			};

			glsl::transpiler::glsl::show_translation_unit(
				&mut FormatWrite(&mut buf_writer),
				&translation_unit
			);

			Ok(Some((
				Cow::Borrowed("Minified"),
				OptimizedBytes(buf_writer.into_inner())
			)))
		} else {
			// The shader is okay, but we don't want to minify it, so just
			// return an owned view of the read bytes
			Ok(Some((
				Cow::Borrowed("Validated and copied"),
				OptimizedBytes(src.split_off(0))
			)))
		}
	}
}

impl<T: AsyncRead + Unpin + 'static>
	ResourcePackFile<BytesMut, OptimizationError, FramedRead<T, OptimizerDecoder>> for ShaderFile<'_, T>
{
	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				file_length: self.file_length,
				optimization_settings: self.optimization_settings
			},
			self.file_length
		)
	}

	fn canonical_extension(&self) -> &str {
		self.extension
	}

	fn is_compressed(&self) -> bool {
		false
	}
}
