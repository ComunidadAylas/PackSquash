use std::borrow::Cow;

use bytes::BytesMut;
use java_properties::{LineEnding, PropertiesError, PropertiesIter, PropertiesWriter};
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use super::{OptimizedBytes, ResourcePackFile};

#[cfg(test)]
mod tests;

/// Represents a [Java properties line-oriented text file][1] that contains pairs of
/// keys and values. These files are easy to parse and generate in the Java ecosystem,
/// and as such fairly common.
///
/// Vanilla Minecraft does not use property files in resource packs. Currently, only
/// the OptiFine mod is known to do so. In such mod, properties files are used for
/// several features, like Alternate Blocks, Connected Textures, and so on; see
/// [the official documentation][2].
///
/// The optimization process may be customized via [OptimizationSettings].
///
/// [1]: https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/Properties.html#load(java.io.Reader)
/// [2]: https://github.com/sp614x/optifine/tree/master/OptiFineDoc/doc
struct PropertiesFile<'a, T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length: usize,
	extension: &'a str,
	optimization_settings: OptimizationSettings
}

/// Parameters that influence how a [PropertiesFile] is optimized.
struct OptimizationSettings {
	/// If true, the JSON data will be normalized and minified, which normally
	/// improves compressibility a fair amount, or even replaces it altogether for
	/// tiny files. If false, the data will be parsed, to check for errors, and
	/// then returned as-is if the validation is successful
	minify: bool
}

impl Default for OptimizationSettings {
	fn default() -> Self {
		Self { minify: true }
	}
}

/// Optimizer decoder that transforms properties files to an optimized representation.
struct OptimizerDecoder {
	file_length: usize,
	optimization_settings: OptimizationSettings
}

/// Helper enum to allow clients of [PropertiesFile] consume bytes from different
/// owned representations, which skips costly conversions.
#[derive(Debug)]
enum ByteBuffer {
	BytesMut(BytesMut),
	Vec(Vec<u8>)
}

impl AsRef<[u8]> for ByteBuffer {
	fn as_ref(&self) -> &[u8] {
		match self {
			ByteBuffer::BytesMut(buf) => buf,
			ByteBuffer::Vec(buf) => buf
		}
	}
}

impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, OptimizedBytes<ByteBuffer>);
	type Error = PropertiesError;

	fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		// FIXME: actual framing?
		// (i.e. do not hold the entire file in memory before decoding, so that frame != file)

		// Check if we have the entire file (i.e. frame). Empty files do not generate any frame
		let read_bytes = src.len();
		if read_bytes == 0 || read_bytes < self.file_length {
			return Ok(None);
		}

		if self.optimization_settings.minify {
			// Re-write the properties file, using the terse and normalized
			// format that the writer outputs
			let mut minified_file_buf = Vec::with_capacity(self.file_length);
			let mut minified_properties_writer = PropertiesWriter::new(&mut minified_file_buf);
			minified_properties_writer.set_line_ending(LineEnding::LF);
			minified_properties_writer.set_kv_separator("=").unwrap();

			PropertiesIter::new(&**src).read_into(|key, value| {
				minified_properties_writer.write(&key, &value).unwrap();
			})?;

			// We should clear the source buffer when handing off a decoded frame
			src.clear();

			Ok(Some((
				Cow::Borrowed("Minified"),
				OptimizedBytes(ByteBuffer::Vec(minified_file_buf))
			)))
		} else {
			// Parse the properties file to check its correctness,
			// and then just copy the original file
			PropertiesIter::new(&**src).read_into(|_, _| ())?;

			Ok(Some((
				Cow::Borrowed("Validated and copied"),
				OptimizedBytes(ByteBuffer::BytesMut(src.split_off(0)))
			)))
		}
	}
}

impl<T: AsyncRead + Unpin + 'static>
	ResourcePackFile<ByteBuffer, PropertiesError, FramedRead<T, OptimizerDecoder>>
	for PropertiesFile<'_, T>
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
