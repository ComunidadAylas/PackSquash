use std::{borrow::Cow, convert::TryInto};

use bytes::{BufMut, BytesMut};
use json_comments::StripComments;
use serde_json::Value;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use crate::config::JsonFileOptions;

use self::debloater::{DebloatError, Debloater};

use super::{
	util::bom_stripper, util::to_ascii_lowercase_extension, OptimizedBytes, PackFile,
	PackFileConstructor, PackFileConstructorArgs
};

mod debloater;

#[cfg(test)]
mod tests;

/// Represents a pack text file that contains a single JSON object. This file may
/// have several extensions (`json`, `jsonc`, `mcmeta`...), and its contents may
/// be interpreted differently by Minecraft according to the role of the file.
///
/// The optimization process may be customized via [JsonFileOptions].
pub struct JsonFile<T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length: usize,
	extension: String,
	optimization_settings: JsonFileOptions
}

/// Optimizer decoder that transforms JSON files to an optimized representation.
pub struct OptimizerDecoder {
	optimization_settings: JsonFileOptions,
	reached_eof: bool
}

/// Represents an error that may happen while optimizing JSON files.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OptimizationError {
	#[error("JSON error: {0}")]
	JsonSerde(#[from] serde_json::Error),
	#[error("Debloat error: {0}")]
	Debloat(#[from] DebloatError),
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error)
}

thread_local!(static DEBLOATER: Debloater = Debloater::new());

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

		// Parse the JSON so we know how to serialize it again in a compact manner,
		// and we know it's valid. Also remove its comments
		let mut json_value: Value =
			serde_json::from_reader(StripComments::new(bom_stripper::strip_utf8_bom(src)))?;

		// Now that we have the value struct, clear the input buffer to reuse it for
		// the optimized JSON serialization
		src.clear();

		// Debloat the read value
		if self.optimization_settings.delete_bloat {
			DEBLOATER.with(|debloater| -> Result<(), DebloatError> {
				debloater.debloat(&mut json_value)
			})?;
		}

		let mut json_writer = src.split_off(0).writer();

		// Serialize the JSON value to the buffer
		let description;
		if self.optimization_settings.minify {
			serde_json::ser::to_writer(&mut json_writer, &json_value)?;
			description = "Minified";
		} else {
			serde_json::ser::to_writer_pretty(&mut json_writer, &json_value)?;
			description = "Prettified";
		}

		// Cheaply get an owned BytesMut with the serialized JSON data
		Ok(Some((
			Cow::Borrowed(description),
			OptimizedBytes(json_writer.get_mut().split_off(0))
		)))
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFile for JsonFile<T> {
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
		match &*self.extension {
			// .jsonc is converted to .json, because we strip comments
			"jsonc" => "json",
			// Other extensions (e.g. .mcmeta) are passed through
			_ => &self.extension
		}
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFileConstructor<T> for JsonFile<T> {
	type OptimizationSettings = JsonFileOptions;

	fn new<F: FnMut() -> Option<(T, u64)>>(
		mut file_read_producer: F,
		args: PackFileConstructorArgs<'_, JsonFileOptions>
	) -> Option<Self> {
		let extension = to_ascii_lowercase_extension(args.path.as_ref());

		let is_json_file = matches!(&*extension, "json" | "jsonc" | "mcmeta")
			|| (args.optimization_settings.allow_optifine_extensions
				&& matches!(&*extension, "jem" | "jpm"));

		if is_json_file {
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
