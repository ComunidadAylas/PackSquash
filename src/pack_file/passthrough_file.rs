use std::{borrow::Cow, io::Error};

use bytes::BytesMut;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use super::{
	util::to_ascii_lowercase_extension, OptimizedBytes, PackFile, PackFileConstructor,
	PackFileConstructorArgs
};

#[cfg(test)]
mod tests;

/// Represents a pack file that will be passed through (copied) exactly as-is,
/// without any check or validation whatsoever.
///
/// This struct is mainly useful for files that PackSquash knows that are part
/// of a pack but have a format that can't be handled better (yet), or that
/// the user does not want to be processed.
pub struct PassthroughFile<T: AsyncRead + Unpin + 'static> {
	read: T,
	canonical_extension: &'static str,
	optimization_strategy_message: &'static str,
	is_compressed: bool
}

/// Passthrough decoder that always returns the bytes it receives without changes or checks.
pub struct PassthroughDecoder {
	pub optimization_strategy_message: &'static str
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

impl<T: AsyncRead + Unpin + 'static> PackFile for PassthroughFile<T> {
	type ByteChunkType = BytesMut;
	type OptimizationError = Error;
	type OptimizedBytesChunksStream = FramedRead<T, PassthroughDecoder>;

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

impl<T: AsyncRead + Unpin + 'static> PackFileConstructor<T> for PassthroughFile<T> {
	type OptimizationSettings = ();

	fn new<F: FnMut() -> Option<(T, u64)>>(
		mut file_read_producer: F,
		args: PackFileConstructorArgs<'_, ()>
	) -> Option<Self> {
		let extension = &*to_ascii_lowercase_extension(args.path.as_ref());

		match extension {
			"ttf" => file_read_producer().map(|(read, _)| Self {
				read,
				canonical_extension: "ttf",
				optimization_strategy_message: "Copied, but might be optimized manually \
					(more information: <https://packsquash.page.link/Optimizing-TTF-fonts>)",
				is_compressed: false
			}),
			"bin" => file_read_producer().map(|(read, _)| Self {
				read,
				canonical_extension: "bin",
				optimization_strategy_message: "Copied",
				is_compressed: false
			}),
			// Plain text files that are known to be used by Minecraft
			"txt"
				if matches!(
					args.path.as_ref(),
					"assets/minecraft/texts/end.txt"
						| "assets/minecraft/texts/credits.txt"
						| "assets/minecraft/texts/splashes.txt"
				) =>
			{
				file_read_producer().map(|(read, _)| Self {
					read,
					canonical_extension: "txt",
					optimization_strategy_message: "Copied",
					is_compressed: false
				})
			}
			// FIXME: these two should have file-specific optimizations, and they are not difficult
			// to do. This is a temporary solution for PackSquash to work with data packs
			"nbt" => file_read_producer().map(|(read, _)| Self {
				read,
				canonical_extension: "nbt",
				optimization_strategy_message: "Copied",
				is_compressed: true
			}),
			"mcfunction" => file_read_producer().map(|(read, _)| Self {
				read,
				canonical_extension: "mcfunction",
				optimization_strategy_message: "Copied",
				is_compressed: false
			}),
			_ => None
		}
	}
}
