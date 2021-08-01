use std::{
	borrow::Cow,
	convert::TryInto,
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

use crate::config::ShaderFileOptions;

use super::{
	util::bom_stripper, util::to_ascii_lowercase_extension, OptimizedBytes, PackFile,
	PackFileConstructor, PackFileConstructorArgs
};

#[cfg(test)]
mod tests;

/// Represents a GLSL shader file (more precisely, a translation unit or shader stage).
///
/// Vanilla Minecraft uses fragment and vertex shaders that can be replaced by resource
/// packs for several effects, like the "creeper vision" showed while spectating a Creeper,
/// and the "Super Secret Settings" button that was ultimately removed.
/// Minecraft mods may support more shaders that can be added or replaced via resource packs.
pub struct ShaderFile<T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length: usize,
	extension: String,
	optimization_settings: ShaderFileOptions
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
pub struct OptimizerDecoder {
	optimization_settings: ShaderFileOptions,
	reached_eof: bool
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OptimizationError {
	#[error("Invalid UTF-8 character encoding: {0}")]
	InvalidUtf8(#[from] Utf8Error),
	#[error("Shader parse error: {0}")]
	InvalidShaderStage(#[from] ParseError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

// FIXME: actual framing?
// (i.e. do not hold the entire file in memory before decoding, so that frame != file)
impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, OptimizedBytes<BytesMut>);
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

impl<T: AsyncRead + Unpin + 'static> PackFile for ShaderFile<T> {
	type ByteChunkType = BytesMut;
	type OptimizationError = OptimizationError;
	type OptimizedBytesChunksStream = FramedRead<T, OptimizerDecoder>;

	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				optimization_settings: self.optimization_settings,
				reached_eof: false
			},
			self.file_length
		)
	}

	fn canonical_extension(&self) -> &str {
		&self.extension
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFileConstructor<T> for ShaderFile<T> {
	type OptimizationSettings = ShaderFileOptions;

	fn new<F: FnMut() -> Option<(T, u64)>>(
		mut file_read_producer: F,
		args: PackFileConstructorArgs<'_, ShaderFileOptions>
	) -> Option<Self> {
		let extension = to_ascii_lowercase_extension(args.path.as_ref());

		if matches!(&*extension, "fsh" | "vsh") {
			file_read_producer().map(|(read, file_length)| Self {
				read,
				// The file is too big to fit in memory if this conversion fails anyway
				file_length: file_length.try_into().unwrap_or(usize::MAX),
				extension: extension.into_owned(),
				optimization_settings: args.optimization_settings
			})
		} else {
			None
		}
	}
}
