use std::sync::LazyLock;

use ahash::AHashSet;
use futures::{future, StreamExt};
use patricia_tree::PatriciaSet;
use regex::Regex;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_stream::Stream;
use tokio_util::codec::{FramedRead, LinesCodec, LinesCodecError};

use crate::config::LegacyLanguageFileOptions;
use crate::pack_file::asset_type::PackFileAssetType;
use crate::pack_file::util::{prepare_line_for_output, LineNumber, MarkLastDecorator, BOM};
use crate::pack_file::AsyncReadAndSizeHint;

use super::{OptimizedBytesChunk, PackFile, PackFileConstructor};

#[cfg(test)]
mod tests;

/// A regular expression that matches plausible Java format specifiers, used in format strings.
static FORMAT_SPECIFIER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
	// The format specifier follows this syntax
	// (from https://docs.oracle.com/en/java/javase/11/docs/api/java.base/java/util/Formatter.html#summary):
	// %[argument_index$][flags][width][.precision]conversion
	//
	// This regex is adapted from:
	// https://github.com/AdoptOpenJDK/openjdk-jdk11/blob/19fb8f93c59dfd791f62d41f332db9e306bc1422/src/java.base/share/classes/java/util/Formatter.java#L2689-L2691
	//
	// The list of valid flags and conversions was deduced from:
	// https://github.com/AdoptOpenJDK/openjdk-jdk11/blob/19fb8f93c59dfd791f62d41f332db9e306bc1422/src/java.base/share/classes/java/util/Formatter.java#L4553-L4806
	//
	// Note that this regex is not context sensitive: it can't check whether the objects to be formatted
	// are compatible with the conversion, and it doesn't check that the flags, width, precision and
	// argument index make sense according to the objects to format and the conversion. It's only meant
	// to catch blatantly invalid specifiers
	Regex::new(
		"%(?:\\d+\\$)?\
		[-#+ 0,(<]*\
		\\d*\
		(?:\\.\\d+)?\
		(?:[doxXeEgGfaAcCbBsShHn%]|[tT][HIklMNLQpsSTzZaAbBCdehjmuUVwWyYrRcDF])"
	)
	.unwrap()
});

/// Represents a line-oriented text file that contains pairs of string keys and their value for a
/// given locale. The format of these files looks similar to a standard Java properties text file,
/// but they are not parsed according to that format; instead, Minecraft uses its own, more
/// restrictive parser.
///
/// The language strings files in the format represented by this struct were superseded by JSON
/// files in Minecraft 1.13. Only Minecraft 1.12 and older versions use this type of files.
///
/// References:
/// - <https://minecraft.fandom.com/wiki/Resource_Pack?oldid=1257552#Language>
/// - Minecraft class `net.minecraft.client.resources.Locale` (MCP 1.12.2 name)
/// - OpenJDK 11 implementation of class `java.util.Formatter`
pub struct LegacyLanguageFile<T: AsyncRead + Send + Unpin + 'static> {
	read: T,
	optimization_settings: LegacyLanguageFileOptions
}

/// Represents an error that may happen while optimizing legacy language files.
#[derive(Error, Debug)]
pub enum OptimizationError {
	#[error("Format error: Missing \"=\" separator at line {0}")]
	MissingSeparator(LineNumber),
	#[error("Format error: Duplicate key \"{0}\" at line {1}")]
	DuplicateKey(String, LineNumber),
	#[error("Format error: Invalid format string in value of line {0}")]
	InvalidFormatString(LineNumber),
	#[error("Error while reading a line: {0}")]
	TextLineRead(#[from] LinesCodecError)
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFile for LegacyLanguageFile<T> {
	type ByteChunkType = Vec<u8>;
	type OptimizationError = OptimizationError;
	type OptimizedByteChunksStream =
		impl Stream<Item = OptimizedBytesChunk<Self::ByteChunkType, Self::OptimizationError>>;

	fn process(self) -> Self::OptimizedByteChunksStream {
		let mut line_number = LineNumber::new();
		let mut processed_keys = PatriciaSet::new();

		let minify = self.optimization_settings.minify;
		let strip_bom = self.optimization_settings.strip_bom;

		MarkLastDecorator::new(FramedRead::new(
			self.read,
			// Limit line length to 16 KiB to bound memory consumption and be nice to Minecraft.
			// Longer lines are probably an error, and will negatively affect Minecraft performance
			LinesCodec::new_with_max_length(16 * 1024)
		))
		.filter_map(move |(line_result, is_last)| {
			let processed_line_result = line_result.map_or_else(
				|err| Some(Err(err.into())),
				|line| {
					process_line(
						line,
						is_last,
						line_number,
						minify,
						strip_bom,
						&mut processed_keys
					)
				}
			);

			line_number.increment();

			future::ready(processed_line_result)
		})
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFileConstructor<T> for LegacyLanguageFile<T> {
	type OptimizationSettings = LegacyLanguageFileOptions;

	fn new(
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<T>>,
		_: PackFileAssetType,
		optimization_settings: Self::OptimizationSettings
	) -> Option<Self> {
		file_read_producer().map(|(read, _)| Self {
			read,
			optimization_settings
		})
	}
}

/// Processes the specified line of text, returning an optimized bytes chunk with its optimized
/// representation. `None` is returned to signal that the line should not be copied to the output
/// file at all.
fn process_line<L: Into<String>>(
	line: L,
	is_last: bool,
	line_number: LineNumber,
	minify: bool,
	strip_bom: bool,
	processed_keys: &mut PatriciaSet
) -> Option<OptimizedBytesChunk<Vec<u8>, OptimizationError>> {
	const MINIFIED: &str = "Minified";
	const NOT_MINIFIED: &str = "Validated and copied";

	let mut line = line.into();

	// The lines codec takes care of stripping line breaks from the line, even if
	// Windows line endings (CR + LF) are used. However, we should remove the BOM in
	// the first line if present. This would remove the BOM prefix from the key of
	// the first locale string, avoiding pack author confusion and doing a better
	// work than Mojang ;). This might break packs that dealt with the BOM by
	// including it in the key references elsewhere, so only do it if the option
	// is enabled
	if line_number.is_first() && strip_bom && line.chars().next().map_or(false, |c| c == BOM) {
		line.remove(0);
	}

	// Check whether the line is a comment. If so, bail out by copying or skipping
	// it. It's copied only if we're not minifying
	if line.starts_with('#') {
		return (!minify).then(|| prepare_line_for_output(line, is_last, NOT_MINIFIED));
	}

	match line.split_once('=') {
		Some((key, value)) => {
			// Key-value pair line, with a valid delimiter, that's not a comment.
			// Check whether this key already has a value. Minecraft overwrites the
			// previous value of a key when a new one is found, but to guarantee that
			// our output is minimal and that pack authors are not surprised when a
			// line overrides another, consider duplicate keys as an error. We use a
			// Patricia tree to substantially save on memory for big files in common
			// cases
			if !processed_keys.insert(key) {
				return Some(Err(OptimizationError::DuplicateKey(
					String::from(key),
					line_number
				)));
			}

			// Minecraft will use the value as a format string. Check that its format
			// specifiers are plausible: if we find a format specifier delimiter
			// character that was not matched by the regex, then we know that some
			// format specifier is not valid. This is the idea used by OpenJDK 11's
			// private method java.util.Formatter#parse(String s)
			if FORMAT_SPECIFIER_REGEX
				.split(value)
				.any(|literal_text| literal_text.contains('%'))
			{
				return Some(Err(OptimizationError::InvalidFormatString(line_number)));
			}

			Some(prepare_line_for_output(
				line,
				is_last,
				if minify { MINIFIED } else { NOT_MINIFIED }
			))
		}
		None => {
			// The line does not contain a key-value separator and is not a comment.
			// This is a format error, unless the line is blank. The game would ignore
			// this line, but be more strict about it because this clutters the file
			// for no good reason
			if line.trim().is_empty() {
				(!minify).then(|| prepare_line_for_output(line, is_last, NOT_MINIFIED))
			} else {
				Some(Err(OptimizationError::MissingSeparator(line_number)))
			}
		}
	}
}
