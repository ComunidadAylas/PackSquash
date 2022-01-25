use ahash::AHashSet;
use futures::{future, StreamExt};
use regex::Regex;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::lazy::SyncLazy;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::config::LegacyLanguageFileOptions;
use crate::pack_file::asset_type::PackFileAssetType;
use crate::pack_file::AsyncReadAndSizeHint;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_stream::Stream;
use tokio_util::codec::{FramedRead, LinesCodec, LinesCodecError};

use super::{OptimizedBytesChunk, PackFile, PackFileConstructor};

#[cfg(test)]
mod tests;

/// A regular expression that matches plausible Java format specifiers, used in format strings.
static FORMAT_SPECIFIER_REGEX: SyncLazy<Regex> = SyncLazy::new(|| {
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
	file_length_hint: usize,
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

/// A stream that decorates another stream, mapping its items to a pair that indicates whether
/// that item is the last in the stream or not. This is done by buffering the last item temporarily,
/// until a new one arrives (so it isn't the last one) or the end of the stream is reached (so the
/// last item is indeed the last).
///
/// Please note that the concept of "last item" is not well-defined for streams that are not
/// fused (i.e. that return some item after they signal the end of the stream).
struct MarkLastDecorator<T: Unpin, S: Stream<Item = T> + Unpin> {
	inner: S,
	previous_item: Option<T>
}

impl<T: Unpin, S: Stream<Item = T> + Unpin> MarkLastDecorator<T, S> {
	/// Creates a new [`IdentifyLast`] decorator stream for the specified stream.
	fn new(inner: S) -> Self {
		Self {
			inner,
			previous_item: None
		}
	}
}

impl<T: Unpin, S: Stream<Item = T> + Unpin> Stream for MarkLastDecorator<T, S> {
	type Item = (T, bool);

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		match self.inner.poll_next_unpin(cx) {
			Poll::Pending => {
				// We're either waiting for the next item of the stream, or to be notified that
				// it ended. Wait until we know what's next
				Poll::Pending
			}
			Poll::Ready(Some(item)) => {
				// The inner stream yielded a new item, but we don't know if it's the last one
				match self.previous_item.replace(item) {
					Some(previous_item) => {
						// The previous item is now known to not be the last, as we've just got
						// another
						Poll::Ready(Some((previous_item, false)))
					}
					None => {
						// This is the first item yielded by the underlying stream. We don't have
						// anything to return yet, so poll again. This will only recurse once, because
						// even if the stream yields another item immediately, we would then have a
						// previous item
						self.poll_next(cx)
					}
				}
			}
			Poll::Ready(None) => {
				// The stream has ended. The item we buffered is known to be the last one.
				// If there is no buffered item, either because the inner stream yielded no items
				// or we've already yielded the buffered item, propagate the end of the stream
				if let Some(previous_item) = self.previous_item.take() {
					Poll::Ready(Some((previous_item, true)))
				} else {
					Poll::Ready(None)
				}
			}
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}
}

/// An opaque type that maintains a text line counter that can be displayed.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct LineNumber(Option<NonZeroUsize>);

impl LineNumber {
	/// Creates a new line number counter that would display a line number of 1.
	const fn new() -> Self {
		Self(Some(NonZeroUsize::new(1).unwrap()))
	}

	/// Checks whether this line number counter would display a line number of 1
	/// (i.e. it was just created, or it still points to the first line).
	const fn is_first(&self) -> bool {
		if let Some(line_number) = self.0 {
			line_number.get() == 1
		} else {
			false
		}
	}

	/// Increments the line number counter to point to the next line. This method
	/// is overflow-safe.
	fn increment(&mut self) {
		self.0 = self.0.and_then(|line_number| line_number.checked_add(1));
	}
}

impl Display for LineNumber {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.0 {
			Some(line_number) => line_number.fmt(f),
			None => f.write_str("unknown")
		}
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFile for LegacyLanguageFile<T> {
	type ByteChunkType = Vec<u8>;
	type OptimizationError = OptimizationError;
	type OptimizedByteChunksStream =
		impl Stream<Item = OptimizedBytesChunk<Self::ByteChunkType, Self::OptimizationError>>;

	fn process(self) -> Self::OptimizedByteChunksStream {
		let mut line_number = LineNumber::new();
		let mut processed_keys = AHashSet::new();

		let minify = self.optimization_settings.minify;
		let strip_bom = self.optimization_settings.strip_bom;

		MarkLastDecorator::new(FramedRead::with_capacity(
			self.read,
			// Limit line length to 16 KiB to bound memory consumption and be nice to Minecraft.
			// Longer lines are probably an error, and will negatively affect Minecraft performance
			LinesCodec::new_with_max_length(16 * 1024),
			// FIXME consider limiting the value of the size hint in this and other parts of the code
			// depending on the global memory budget (see FIXME below)
			self.file_length_hint
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
		file_read_producer().map(|(read, file_length_hint)| Self {
			read,
			// The file is too big to fit in memory if this conversion fails anyway
			file_length_hint: file_length_hint.try_into().unwrap_or(usize::MAX),
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
	processed_keys: &mut AHashSet<Box<[u8]>>
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
	if line_number.is_first() && strip_bom && line.chars().next().map_or(false, |c| c == '\u{feff}') {
		line.remove(0);
	}

	// Check whether the line is a comment. If so, bail out by copying or skipping
	// it. It's copied only if we're not minifying
	if line.starts_with('#') {
		return (!minify).then(|| prepare_for_output(line, is_last, NOT_MINIFIED));
	}

	match line.split_once('=') {
		Some((key, value)) => {
			// Key-value pair line, with a valid delimiter, that's not a comment.
			// Check whether this key already has a value. Minecraft overwrites the
			// previous value of a key when a new one is found, but to guarantee that
			// our output is minimal and that pack authors are not surprised when a
			// line overrides another, consider duplicate keys as an error. We use the
			// fast smaz short string compression algorithm to save on memory for
			// language files with lots of entries with English-like keys.
			// FIXME when the spooled temporary files refactor to use a global byte
			// budget is complete, consider replacing this with a CDB insert and lookup,
			// using the galvanize crate on one of those
			if !processed_keys.insert(smaz::compress(key.as_bytes()).into_boxed_slice()) {
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

			Some(prepare_for_output(
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
				(!minify).then(|| prepare_for_output(line, is_last, NOT_MINIFIED))
			} else {
				Some(Err(OptimizationError::MissingSeparator(line_number)))
			}
		}
	}
}

/// Prepares a line for output to the processed representation of this file, adding a line break
/// if necessary.
fn prepare_for_output<L: Into<String>, D: Into<Cow<'static, str>>>(
	line: L,
	is_last: bool,
	description: D
) -> OptimizedBytesChunk<Vec<u8>, OptimizationError> {
	let mut line = line.into();

	// Add a Unix-style line break if there are more lines. We don't need a newline at the end
	if !is_last {
		line.push('\n');
	}

	Ok((description.into(), line.into_bytes()))
}
