//! Contains miscellaneous small helper functions that are used for processing pack files.

use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::StreamExt;
use tokio_stream::Stream;

use super::{OptimizationError, OptimizedBytesChunk};

/// The Unicode byte order mark character (BOM).
pub static BOM: char = '\u{feff}';
/// The UTF-8 representation of the Unicode byte order mark character.
pub static BOM_UTF8: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Returns a slice of the input buffer that skips the initial byte order
/// mark character that some programs (most notably, Microsoft software,
/// like Notepad) add to UTF-8 encoded files. If no BOM is present, the
/// buffer is returned as-is.
pub fn strip_utf8_bom(buf: &[u8]) -> &[u8] {
	let start_index = if starts_with_bom(buf) {
		BOM_UTF8.len()
	} else {
		0
	};

	&buf[start_index..]
}

/// Checks whether the specified byte buffer begins with a byte order mark
/// character.
pub fn starts_with_bom<T: AsRef<[u8]>>(buf: T) -> bool {
	let buf = buf.as_ref();

	buf.len() >= BOM_UTF8.len() && buf[..BOM_UTF8.len()] == BOM_UTF8
}

/// An opaque type that maintains a text line counter that can be displayed.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct LineNumber(Option<NonZeroUsize>);

impl LineNumber {
	/// Creates a new line number counter that would display a line number of 1.
	pub const fn new() -> Self {
		Self(Some(NonZeroUsize::new(1).unwrap()))
	}

	/// Checks whether this line number counter would display a line number of 1
	/// (i.e. it was just created, or it still points to the first line).
	pub const fn is_first(&self) -> bool {
		if let Some(line_number) = self.0 {
			line_number.get() == 1
		} else {
			false
		}
	}

	/// Increments the line number counter to point to the next line. This method
	/// is overflow-safe.
	pub fn increment(&mut self) {
		self.0 = self.0.and_then(|line_number| line_number.checked_add(1));
	}
}

impl Display for LineNumber {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self.0 {
			Some(line_number) => Display::fmt(&line_number, f),
			None => f.write_str("unknown")
		}
	}
}

/// A stream that decorates another stream, mapping its items to a pair that indicates whether
/// that item is the last in the stream or not. This is done by buffering the last item temporarily,
/// until a new one arrives (so it isn't the last one) or the end of the stream is reached (so the
/// last item is indeed the last).
///
/// Please note that the concept of "last item" is not well-defined for streams that are not
/// fused (i.e. that return some item after they signal the end of the stream).
pub struct MarkLastDecorator<T: Unpin, S: Stream<Item = T> + Unpin> {
	inner: S,
	previous_item: Option<T>
}

impl<T: Unpin, S: Stream<Item = T> + Unpin> MarkLastDecorator<T, S> {
	/// Creates a new [`IdentifyLast`] decorator stream for the specified stream.
	pub fn new(inner: S) -> Self {
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

/// Prepares a line for output to the processed representation of a text file, adding a line break
/// if necessary.
pub fn prepare_line_for_output<
	L: Into<String>,
	D: Into<Cow<'static, str>>,
	E: Into<OptimizationError> + Debug + Display
>(
	line: L,
	is_last: bool,
	description: D
) -> OptimizedBytesChunk<Vec<u8>, E> {
	let mut line = line.into();

	// Add a Unix-style line break if there are more lines. We don't need a newline at the end
	if !is_last {
		line.push('\n');
	}

	Ok((description.into(), line.into_bytes()))
}
