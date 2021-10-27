use std::{
	borrow::Cow,
	error::Error,
	fmt::{self, Display, Formatter},
	io
};

use bytes::BytesMut;
use java_properties::{LineEnding, PropertiesError, PropertiesIter, PropertiesWriter};
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use crate::config::PropertiesFileOptions;

use super::{
	util::{strip_utf8_bom, to_ascii_lowercase_extension},
	OptimizedBytes, PackFile, PackFileConstructor, PackFileConstructorArgs
};

#[cfg(test)]
mod tests;

/// Represents a [Java properties line-oriented text file][1] that contains pairs of
/// keys and values. These files are easy to parse and generate in the Java ecosystem,
/// and as such fairly common.
///
/// Vanilla Minecraft does not use property files in resource packs. Currently, only the
/// OptiFine mod is known to do so. In such mod, properties files are used for several features,
/// like Alternate Blocks, Connected Textures, and so on; see [the official documentation][2].
///
/// The optimization process may be customized via [PropertiesFileOptions].
///
/// [1]: https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/Properties.html#load(java.io.Reader)
/// [2]: https://github.com/sp614x/optifine/tree/master/OptiFineDoc/doc
pub struct PropertiesFile<T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length_hint: usize,
	optimization_settings: PropertiesFileOptions
}

/// Optimizer decoder that transforms properties files to an optimized representation.
pub struct OptimizerDecoder {
	optimization_settings: PropertiesFileOptions,
	reached_eof: bool
}

/// Represents an error that may happen while optimizing properties files.
#[derive(Debug)]
pub struct OptimizationError {
	description: String
}

impl Error for OptimizationError {}

impl Display for OptimizationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!("Parse or I/O error: {}", self.description))
	}
}

impl From<PropertiesError> for OptimizationError {
	fn from(err: PropertiesError) -> Self {
		Self {
			description: err.to_string()
		}
	}
}

impl From<io::Error> for OptimizationError {
	fn from(err: io::Error) -> Self {
		Self {
			description: err.to_string()
		}
	}
}

/// Helper enum to allow clients of [PropertiesFile] consume bytes from different
/// owned representations, which skips costly conversions.
#[derive(Debug)]
pub enum ByteBuffer {
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

// FIXME: actual framing?
// (i.e. do not hold the entire file in memory before decoding, so that frame != file)
impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, OptimizedBytes<ByteBuffer>);
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

		if self.optimization_settings.minify {
			// Re-write the properties file, using the terse and normalized
			// format that the writer outputs
			let mut minified_file_buf = Vec::with_capacity(src.len());
			let mut minified_properties_writer = PropertiesWriter::new(&mut minified_file_buf);
			minified_properties_writer.set_line_ending(LineEnding::LF);
			minified_properties_writer.set_kv_separator("=").unwrap();

			PropertiesIter::new(strip_utf8_bom(src)).read_into(|key, value| {
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

impl<T: AsyncRead + Unpin + 'static> PackFile for PropertiesFile<T> {
	type ByteChunkType = ByteBuffer;
	type OptimizationError = OptimizationError;
	type OptimizedBytesChunksStream = FramedRead<T, OptimizerDecoder>;

	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				optimization_settings: self.optimization_settings,
				reached_eof: false
			},
			self.file_length_hint
		)
	}

	fn canonical_extension(&self) -> &str {
		"properties"
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFileConstructor<T> for PropertiesFile<T> {
	type OptimizationSettings = PropertiesFileOptions;

	fn new<F: FnMut() -> Option<(T, u64)>>(
		mut file_read_producer: F,
		args: PackFileConstructorArgs<'_, PropertiesFileOptions>
	) -> Option<Self> {
		if !args.optimization_settings.skip
			&& matches!(&*to_ascii_lowercase_extension(args.path), "properties")
		{
			file_read_producer().map(|(read, file_length_hint)| Self {
				read,
				// The file is too big to fit in memory if this conversion fails anyway
				file_length_hint: file_length_hint.try_into().unwrap_or(usize::MAX),
				optimization_settings: args.optimization_settings
			})
		} else {
			None
		}
	}
}
