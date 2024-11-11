//! Contains code to optimize files that will just be copied without optimizations.

use std::{borrow::Cow, io::Error};

use bytes::BytesMut;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use super::{AsyncReadAndSizeHint, PackFile, PackFileAssetType, PackFileConstructor};

#[cfg(test)]
mod tests;

/// Represents a pack file that will be passed through (copied) exactly as-is,
/// without any check or validation whatsoever.
///
/// This struct is mainly useful for files that PackSquash knows that are part
/// of a pack but have a format that can't be handled better (yet), or that
/// the user does not want to be processed.
pub struct PassthroughFile<T: AsyncRead + Send + Unpin + 'static> {
	read: T,
	optimization_strategy_message: &'static str,
	is_compressed: bool
}

/// Passthrough decoder that always returns the bytes it receives without changes or checks.
pub struct PassthroughDecoder {
	pub optimization_strategy_message: &'static str
}

impl Decoder for PassthroughDecoder {
	type Item = (Cow<'static, str>, BytesMut);
	type Error = Error;

	fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		if src.is_empty() {
			Ok(None)
		} else {
			Ok(Some((
				Cow::Borrowed(self.optimization_strategy_message),
				src.split_off(0)
			)))
		}
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFile for PassthroughFile<T> {
	type ByteChunkType = BytesMut;
	type OptimizationError = Error;
	type OptimizedByteChunksStream = FramedRead<T, PassthroughDecoder>;

	fn process(self) -> FramedRead<T, PassthroughDecoder> {
		FramedRead::new(
			self.read,
			PassthroughDecoder {
				optimization_strategy_message: self.optimization_strategy_message
			}
		)
	}

	fn is_compressed(&self) -> bool {
		self.is_compressed
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFileConstructor<T> for PassthroughFile<T> {
	type OptimizationSettings = ();

	fn new(
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<T>>,
		asset_type: PackFileAssetType,
		_: Self::OptimizationSettings
	) -> Option<Self> {
		match asset_type {
			PackFileAssetType::TrueTypeOrOpenTypeFont | PackFileAssetType::TrueTypeFont => {
				file_read_producer().map(|(read, _)| Self {
					read,
					optimization_strategy_message: "Copied, but might be optimized manually. \
					More information: <https://packsquash.page.link/Optimizing-TTF-fonts>",
					is_compressed: false
				})
			}
			// TODO: ZippedUnifontHex could be optimized by dropping any non-`.hex` file inside it
			//       and recompressing the remaining `.hex` files with our efficient Zopfli algorithm
			PackFileAssetType::ZippedUnifontHex
			| PackFileAssetType::LegacyUnicodeFontCharacterSizes => {
				file_read_producer().map(|(read, _)| Self {
					read,
					optimization_strategy_message: "Copied",
					is_compressed: false
				})
			}
			PackFileAssetType::Text
			| PackFileAssetType::ClosingCreditsText
			| PackFileAssetType::LegacyTextCredits => file_read_producer().map(|(read, _)| Self {
				read,
				optimization_strategy_message: "Copied",
				is_compressed: false
			}),
			// FIXME: these should have file-specific optimizations, and this is not difficult
			// to do. This is a temporary solution for PackSquash to work with data packs
			PackFileAssetType::LegacyNbtStructure | PackFileAssetType::NbtStructure => {
				file_read_producer().map(|(read, _)| Self {
					read,
					optimization_strategy_message: "Copied",
					is_compressed: true
				})
			}
			PackFileAssetType::Custom => file_read_producer().map(|(read, _)| Self {
				read,
				optimization_strategy_message: "Copied (custom asset)",
				is_compressed: false
			}),
			_ => unreachable!("Passing through unexpected asset type: {:?}", asset_type)
		}
	}
}
