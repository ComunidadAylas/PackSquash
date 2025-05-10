//! Contains code to optimize JSON files.

use std::borrow::Cow;

use bytes::{BufMut, BytesMut};
use json_comments::StripComments;
use serde_json::Value;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use crate::config::JsonFileOptions;
use crate::pack_file::AsyncReadAndSizeHint;
use crate::pack_file::asset_type::PackFileAssetType;

use super::{PackFile, PackFileConstructor, util::strip_utf8_bom};

use self::debloater::Debloater;
use self::unbounded_depth_json_value::UnboundedDepthJsonValue;

mod debloater;
mod unbounded_depth_json_value;

#[cfg(test)]
mod tests;

/// Represents a pack text file that contains a single JSON object. This file may
/// have several extensions (`json`, `jsonc`, `mcmeta`...), and its contents may
/// be interpreted differently by Minecraft according to the role of the file.
///
/// The optimization process may be customized via [JsonFileOptions].
pub struct JsonFile<T: AsyncRead + Send + Unpin + 'static> {
	read: T,
	file_length_hint: usize,
	asset_type: PackFileAssetType,
	optimization_settings: JsonFileOptions
}

/// Optimizer decoder that transforms JSON files to an optimized representation.
pub struct OptimizerDecoder {
	asset_type: PackFileAssetType,
	optimization_settings: JsonFileOptions,
	reached_eof: bool
}

/// Represents an error that may happen while optimizing JSON files.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OptimizationError {
	#[error("JSON error: {0}")]
	JsonSerde(#[from] serde_json::Error),
	#[error("Unexpected JSON value: {0}")]
	UnexpectedValue(&'static str),
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error)
}

thread_local!(static DEBLOATER: Debloater = const { Debloater::new() });

// FIXME: actual framing?
// (i.e. do not hold the entire file in memory before decoding, so that frame != file)
impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, BytesMut);
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

		// Parse the JSON, so we know how to serialize it again in a compact manner, and whether
		// it's valid. Check whether we should parse and discard comments, too. We also disable
		// recursion limits to support complex JSON structures, which may be found in some practical
		// packs
		let mut json_value = if self.optimization_settings.always_allow_comments
			|| asset_type_has_comments_extension(self.asset_type)
		{
			UnboundedDepthJsonValue::deserialize(|| {
				serde_json::de::IoRead::new(StripComments::new(strip_utf8_bom(src)))
			})?
		} else {
			UnboundedDepthJsonValue::deserialize(|| {
				serde_json::de::SliceRead::new(strip_utf8_bom(src))
			})?
		};

		// All concrete asset types start with a JSON object (aka struct, map)
		if self.asset_type != PackFileAssetType::GenericJson
			&& self.asset_type != PackFileAssetType::GenericJsonWithComments
			&& !json_value.with_safe_stack(Value::is_object)
		{
			return Err(OptimizationError::UnexpectedValue(
				"The root JSON element must be an object"
			));
		}

		// Now that we have the value struct, clear the input buffer to reuse it for
		// the optimized JSON serialization
		src.clear();

		// Debloat the read value
		let debloated = if self.optimization_settings.delete_bloat {
			DEBLOATER.with(|debloater| {
				json_value.with_safe_stack_mut(|value| debloater.debloat(value, self.asset_type))
			})
		} else {
			false
		};

		// Sort the keys of all JSON objects, if requested and the JSON is not so deep that it could
		// cause too much memory to be allocated. On a small corpus of 4 resource packs, this provided
		// ~0.005% space savings at negligible performance cost, in addition to unmeasurable improvements
		// in human readability and ease of diffing
		if self.optimization_settings.sort_object_keys && !json_value.has_deeply_nested_value() {
			json_value.with_safe_stack_mut(|value| value.sort_all_objects());
		}

		let mut json_writer = src.split_off(0).writer();

		macro_rules! concat_debloated_suffix {
			($str:expr) => {
				if debloated {
					concat!($str, " and debloated")
				} else {
					$str
				}
			};
		}

		// Serialize the JSON value to the buffer and get a nice description string
		let description = if self.optimization_settings.minify {
			json_value.serialize(&mut serde_json::Serializer::new(&mut json_writer))?;
			concat_debloated_suffix!("Minified")
		} else {
			json_value.serialize(&mut serde_json::Serializer::pretty(&mut json_writer))?;
			concat_debloated_suffix!("Prettified")
		};

		// Cheaply get an owned BytesMut with the serialized JSON data
		Ok(Some((
			Cow::Borrowed(description),
			json_writer.get_mut().split_off(0)
		)))
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFile for JsonFile<T> {
	type ByteChunkType = BytesMut;
	type OptimizationError = OptimizationError;
	type OptimizedByteChunksStream = FramedRead<T, OptimizerDecoder>;

	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				asset_type: self.asset_type,
				optimization_settings: self.optimization_settings,
				reached_eof: false
			},
			// FIXME consider refactoring this when we have a global memory budget
			self.file_length_hint
		)
	}

	fn is_compressed(&self) -> bool {
		false
	}

	fn may_be_read_and_provided_by_mods(&self) -> bool {
		#[cfg(feature = "optifine")]
		if matches!(
			self.asset_type,
			PackFileAssetType::OptifineCustomEntityModel
				| PackFileAssetType::OptifineCustomEntityModelWithComments
				| PackFileAssetType::OptifineCustomEntityModelPart
				| PackFileAssetType::OptifineCustomEntityModelPartWithComments
				| PackFileAssetType::OptifineVanillaItemModel
				| PackFileAssetType::OptifineVanillaItemModelWithComments
				| PackFileAssetType::OptifineVanillaTextureMetadata
				| PackFileAssetType::OptifineVanillaTextureMetadataWithComments
		) {
			return true;
		}

		#[cfg(feature = "mtr3")]
		if matches!(
			self.asset_type,
			PackFileAssetType::Mtr3CustomTrainModel
				| PackFileAssetType::Mtr3CustomTrainModelWithComments
		) {
			return true;
		}

		matches!(
			self.asset_type,
			PackFileAssetType::MinecraftTextureMetadata
				| PackFileAssetType::MinecraftTextureMetadataWithComments
				| PackFileAssetType::GenericJson
				| PackFileAssetType::GenericJsonWithComments
		)
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFileConstructor<T> for JsonFile<T> {
	type OptimizationSettings = JsonFileOptions;

	fn new(
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<T>>,
		asset_type: PackFileAssetType,
		optimization_settings: Self::OptimizationSettings
	) -> Option<Self> {
		file_read_producer().map(|(read, file_length_hint)| Self {
			read,
			// The file is too big to fit in memory if this conversion fails anyway
			file_length_hint: file_length_hint.try_into().unwrap_or(usize::MAX),
			asset_type,
			optimization_settings
		})
	}
}

/// Checks whether the specified asset type is an extension type whose file extension
/// signals that its JSON data might have comments.
#[cfg_attr(
	not(any(feature = "optifine", feature = "mtr3")),
	allow(clippy::match_like_matches_macro)
)]
const fn asset_type_has_comments_extension(asset_type: PackFileAssetType) -> bool {
	match asset_type {
		PackFileAssetType::MinecraftTextureMetadataWithComments
		| PackFileAssetType::MinecraftMetadataWithComments
		| PackFileAssetType::MinecraftModelWithComments
		| PackFileAssetType::GenericJsonWithComments => true,
		#[cfg(feature = "optifine")]
		PackFileAssetType::OptifineCustomEntityModelWithComments
		| PackFileAssetType::OptifineCustomEntityModelPartWithComments
		| PackFileAssetType::OptifineVanillaItemModelWithComments
		| PackFileAssetType::OptifineVanillaTextureMetadataWithComments => true,
		#[cfg(feature = "mtr3")]
		PackFileAssetType::Mtr3CustomTrainModelWithComments => true,
		_ => false
	}
}
