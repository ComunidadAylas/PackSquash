use std::borrow::Cow;

use bytes::{BufMut, BytesMut};
use json_comments::StripComments;
use serde_json::Value;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use self::debloater::{DebloatError, Debloater};

use super::{util::bom_stripper, OptimizedBytes, ResourcePackFile};

mod debloater;

#[cfg(test)]
mod tests;

/// Represents a resource pack text file that contains a single JSON object.
/// This file may have several extensions (`json`, `jsonc`, `mcmeta`...), and its
/// contents may be interpreted differently by Minecraft according to the role
/// of the file.
///
/// The optimization process may be customized via [OptimizationSettings].
struct JsonFile<'a, T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length: usize,
	extension: &'a str,
	optimization_settings: OptimizationSettings
}

/// Parameters that influence how a [JsonFile] is optimized.
struct OptimizationSettings {
	/// If true, the JSON data will be minified, which normally improves
	/// compressibility a fair amount, or even replaces it altogether for
	/// tiny files. If false, the data will be prettified instead
	minify: bool,
	/// Removes JSON values that, while ignored by Minecraft, are fairly
	/// common to see in input files. This can be because some program
	/// automatically adds them, or other reasons.
	delete_bloat: bool
}

impl Default for OptimizationSettings {
	fn default() -> Self {
		Self {
			minify: true,
			delete_bloat: true
		}
	}
}

/// Optimizer decoder that transforms JSON files to an optimized representation.
struct OptimizerDecoder {
	file_length: usize,
	optimization_settings: OptimizationSettings
}

/// Represents an error that may happen while optimizing JSON files.
#[derive(Error, Debug)]
#[non_exhaustive]
enum OptimizationError {
	#[error("JSON object serialization or desarialization error: {0}")]
	JsonSerde(#[from] serde_json::Error),
	#[error("Debloat error: {0}")]
	Debloat(#[from] DebloatError),
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error)
}

thread_local!(static DEBLOATER: Debloater = Debloater::new());

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

		// Parse the JSON so we know how to serialize it again in a compact manner,
		// and we know it's valid. Also, remove its comments
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

impl<T: AsyncRead + Unpin + 'static>
	ResourcePackFile<BytesMut, OptimizationError, FramedRead<T, OptimizerDecoder>> for JsonFile<'_, T>
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
		match self.extension {
			// .jsonc is converted to .json, because we strip comments
			"jsonc" => "json",
			// Other extensions (e.g. .mcmeta) are passed through
			_ => self.extension
		}
	}

	fn is_compressed(&self) -> bool {
		false
	}
}
