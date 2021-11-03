use std::{borrow::Cow, lazy::SyncLazy};

use bytes::{BufMut, BytesMut};
use globset::{Glob, GlobSet};
use json_comments::StripComments;
use serde_json::Value;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};

use crate::{config::JsonFileOptions, pack_file::util::compile_hardcoded_pack_file_glob_pattern};

use self::debloater::Debloater;

use super::{
	pack_file_asset_type_globset,
	util::{best_asset_type_match, strip_utf8_bom},
	OptimizedBytes, PackFile, PackFileAssetType, PackFileConstructor, PackFileConstructorArgs
};

use enum_iterator::IntoEnumIterator;
use num_enum::TryFromPrimitive;

mod debloater;

#[cfg(test)]
mod tests;

/// Represents a pack text file that contains a single JSON object. This file may
/// have several extensions (`json`, `jsonc`, `mcmeta`...), and its contents may
/// be interpreted differently by Minecraft according to the role of the file.
///
/// The optimization process may be customized via [JsonFileOptions].
pub struct JsonFile<T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length_hint: usize,
	asset_type: JsonFileAssetType,
	optimization_settings: JsonFileOptions
}

/// Optimizer decoder that transforms JSON files to an optimized representation.
pub struct OptimizerDecoder {
	asset_type: JsonFileAssetType,
	optimization_settings: JsonFileOptions,
	reached_eof: bool
}

/// The type of asset contained in a [`JsonFile`], used to influence how it is processed.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoEnumIterator)]
#[repr(usize)]
enum JsonFileAssetType {
	/// Any asset in JSON format, with `.json` or `.jsonc` extension. Because this is a
	/// generic asset type, no optimizations specific to a particular JSON structure will
	/// be done.
	Generic,
	/// A Minecraft metadata asset, with `.mcmeta` extension. These files describe properties
	/// of textures and the pack itself.
	MinecraftMetadata,
	/// A Minecraft block or entity model in vanilla format, with `.json` or `.jsonc`
	/// extension.
	MinecraftModel,
	/// An OptiFine custom entity model, with `.jem` extension.
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	OptifineCustomEntityModel,
	/// An OptiFine custom entity model part, with `.jpm` extension.
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	OptifineCustomEntityModelPart,
	/// A Blockbench modded entity model project which contains custom train models for
	/// the Minecraft Transit Railway 3 mod, with `.bbmodel` extension.
	#[cfg(feature = "mtr3-support")]
	#[doc(cfg(feature = "mtr3-support"))]
	Mtr3CustomTrainModel
}

/// A lazily initialized set of glob patterns that will be used to identify the JSON file
/// asset type from its path.
static JSON_ASSET_TYPE_GLOBSET: SyncLazy<GlobSet> =
	pack_file_asset_type_globset::<JsonFileAssetType>();

impl PackFileAssetType for JsonFileAssetType {
	fn to_glob_pattern(&self) -> Glob {
		match self {
			Self::Generic => {
				compile_hardcoded_pack_file_glob_pattern("{assets,data}/*/**/?*.{json,jsonc}")
			}
			Self::MinecraftMetadata => compile_hardcoded_pack_file_glob_pattern(
				"{pack.mcmeta,assets/*/textures/**/?*.mcmeta}"
			),
			Self::MinecraftModel => compile_hardcoded_pack_file_glob_pattern(
				// It is technically possible in vanilla resource packs to have a model file
				// in folders other than "block" and "item", and in namespaces other than
				// "minecraft". However, mods can define their own namespaces which use
				// similar locations for models, but store them in an entirely different
				// format. The following pattern is a compromise between pack authors'
				// freedom to structure their pack as they wish and accurately identifying
				// vanilla block and item models as such, with few false positives. Mods can
				// define a subfolder like "modname_block" to signal that their models are
				// not to be parsed as vanilla models
				"assets/*/models/{block,item}/**/?*.{json,jsonc}"
			),
			#[cfg(feature = "optifine-support")]
			Self::OptifineCustomEntityModel => {
				compile_hardcoded_pack_file_glob_pattern("assets/minecraft/optifine/cem/?*.jem")
			}
			#[cfg(feature = "optifine-support")]
			Self::OptifineCustomEntityModelPart => {
				compile_hardcoded_pack_file_glob_pattern("assets/minecraft/optifine/cem/?*.jpm")
			}
			#[cfg(feature = "mtr3-support")]
			Self::Mtr3CustomTrainModel => {
				// MTR can read train models from any resource location, but to keep things tidy and
				// ensure that no conflicts with other mods can happen, confine them to the MTR
				// namespace
				compile_hardcoded_pack_file_glob_pattern("assets/mtr/**/?*.bbmodel")
			}
		}
	}

	fn canonical_extension(&self) -> &str {
		match self {
			Self::Generic | Self::MinecraftModel => "json",
			Self::MinecraftMetadata => "mcmeta",
			#[cfg(feature = "optifine-support")]
			Self::OptifineCustomEntityModel => "jem",
			#[cfg(feature = "optifine-support")]
			Self::OptifineCustomEntityModelPart => "jpm",
			#[cfg(feature = "mtr3-support")]
			Self::Mtr3CustomTrainModel => "bbmodel"
		}
	}
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

thread_local!(static DEBLOATER: Debloater = Debloater::new());

// FIXME: actual framing?
// (i.e. do not hold the entire file in memory before decoding, so that frame != file)
impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, OptimizedBytes<BytesMut>);
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

		// Parse the JSON so we know how to serialize it again in a compact manner,
		// and we know it's valid. Also remove its comments
		let mut json_value: Value = serde_json::from_reader(StripComments::new(strip_utf8_bom(src)))?;

		// All concrete asset types we know start with a JSON object (aka struct, map)
		if self.asset_type != JsonFileAssetType::Generic && !json_value.is_object() {
			return Err(OptimizationError::UnexpectedValue(
				"The root JSON element must be an object"
			));
		}

		// Now that we have the value struct, clear the input buffer to reuse it for
		// the optimized JSON serialization
		src.clear();

		// Debloat the read value
		let debloated = if self.optimization_settings.delete_bloat {
			DEBLOATER.with(|debloater| debloater.debloat(&mut json_value, self.asset_type))
		} else {
			false
		};

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
		let description;
		if self.optimization_settings.minify {
			serde_json::ser::to_writer(&mut json_writer, &json_value)?;
			description = concat_debloated_suffix!("Minified");
		} else {
			serde_json::ser::to_writer_pretty(&mut json_writer, &json_value)?;
			description = concat_debloated_suffix!("Prettified");
		}

		// Cheaply get an owned BytesMut with the serialized JSON data
		Ok(Some((
			Cow::Borrowed(description),
			OptimizedBytes(json_writer.get_mut().split_off(0))
		)))
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFile for JsonFile<T> {
	type ByteChunkType = BytesMut;
	type OptimizationError = OptimizationError;
	type OptimizedBytesChunksStream = FramedRead<T, OptimizerDecoder>;

	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				asset_type: self.asset_type,
				optimization_settings: self.optimization_settings,
				reached_eof: false
			},
			self.file_length_hint
		)
	}

	fn canonical_extension(&self) -> &str {
		self.asset_type.canonical_extension()
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFileConstructor<T> for JsonFile<T> {
	type OptimizationSettings = JsonFileOptions;

	fn new<F: FnMut() -> Option<(T, u64)>>(
		mut file_read_producer: F,
		args: PackFileConstructorArgs<'_, JsonFileOptions>
	) -> Option<Self> {
		let asset_type =
			best_asset_type_match(
				&JSON_ASSET_TYPE_GLOBSET,
				args.path,
				|asset_type| match asset_type {
					#[cfg(feature = "optifine-support")]
					JsonFileAssetType::OptifineCustomEntityModel
					| JsonFileAssetType::OptifineCustomEntityModelPart => {
						args.optimization_settings.allow_optifine_files
					}
					#[cfg(feature = "mtr3-support")]
					JsonFileAssetType::Mtr3CustomTrainModel => args.optimization_settings.allow_mtr3_files,
					_ => true
				}
			);

		asset_type
			.map(|asset_type| {
				file_read_producer().map(|(read, file_length_hint)| Self {
					read,
					// The file is too big to fit in memory if this conversion fails anyway
					file_length_hint: file_length_hint.try_into().unwrap_or(usize::MAX),
					asset_type,
					optimization_settings: args.optimization_settings
				})
			})
			.flatten()
	}
}
