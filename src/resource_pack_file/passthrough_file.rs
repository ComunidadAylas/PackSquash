use std::{borrow::Cow, io::Error};

use bytes::BytesMut;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use super::{OptimizedBytes, ResourcePackFile};

#[cfg(test)]
mod tests;

/// Represents a resource pack file that will be passed through (copied) exactly as-is,
/// without any check or validation whatsoever.
///
/// This struct is mainly useful for files that PackSquash knows that are part
/// of a resource pack but have a format that can't be handled better (yet), or that
/// the user does not want to be processed.
struct PassthroughFile<'a, T: AsyncRead + Unpin + 'static> {
	read: T,
	canonical_extension: &'a str,
	optimization_strategy_message: &'static str,
	is_compressed: bool
}

/// Passthrough decoder that always returns the bytes it receives without changes or checks.
pub(super) struct PassthroughDecoder {
	pub(super) optimization_strategy_message: &'static str
}

impl Decoder for PassthroughDecoder {
	type Item = (Cow<'static, str>, OptimizedBytes<BytesMut>);
	type Error = Error;

	fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		if src.is_empty() {
			Ok(None)
		} else {
			Ok(Some((
				Cow::Borrowed(self.optimization_strategy_message),
				OptimizedBytes(src.split_off(0))
			)))
		}
	}
}

impl<T: AsyncRead + Unpin + 'static>
	ResourcePackFile<BytesMut, Error, FramedRead<T, PassthroughDecoder>> for PassthroughFile<'_, T>
{
	fn process(self) -> FramedRead<T, PassthroughDecoder> {
		FramedRead::new(
			self.read,
			PassthroughDecoder {
				optimization_strategy_message: self.optimization_strategy_message
			}
		)
	}

	fn canonical_extension(&self) -> &str {
		self.canonical_extension
	}

	fn is_compressed(&self) -> bool {
		self.is_compressed
	}
}
