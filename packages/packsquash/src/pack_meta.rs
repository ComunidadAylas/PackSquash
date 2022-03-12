//! Contains helper structs to parse the pack metadata file for information relevant for
//! optimization purposes.

use std::io;
use std::path::Path;

use enumset::EnumSet;
use json_comments::StripComments;
use serde_json::Value;
use thiserror::Error;
use tokio::io::AsyncReadExt;

use crate::pack_file::asset_type::PackFileAssetType;
use crate::pack_file::strip_utf8_bom;
use crate::{config::MinecraftQuirk, vfs::VirtualFileSystem};

#[cfg(test)]
mod tests;

/// The pack format version used in Minecraft versions from 1.13 to 1.14.4.
pub const PACK_FORMAT_VERSION_1_13: i32 = 4;
/// The pack format version used in Minecraft versions from 1.17 to 1.17.1.
pub const PACK_FORMAT_VERSION_1_17: i32 = 7;

/// Metadata for a resource or data pack, contained in the `pack.mcmeta` or
/// `pack.mcmetac` file in the root folder of a pack.
///
/// References:
/// - <https://minecraft.fandom.com/wiki/Resource_Pack#Contents>
/// - <https://minecraft.fandom.com/wiki/Data_Pack#pack.mcmeta>
/// - Minecraft class `net.minecraft.server.packs.metadata.pack.PackMetadataSectionSerializer`
pub struct PackMeta {
	pack_format_version: i32
}

/// Represents an error that may happen while parsing pack metadata files.
#[derive(Error, Debug)]
pub enum PackMetaError {
	#[error("JSON error: {0}")]
	JsonSerde(#[from] serde_json::Error),
	#[error("Syntax error: {0}")]
	MalformedMeta(&'static str),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

impl PackMeta {
	/// Creates a new pack metadata struct from a virtual filesystem and its root path.
	pub async fn new<F: VirtualFileSystem, P: AsRef<Path>>(
		vfs: &F,
		root_path: P
	) -> Result<Self, PackMetaError> {
		const PACK_FORMAT_VERSION_IS_NOT_INTEGER: &str =
			"\"pack_format\" version is not a Java integer";

		let pack_format_version;

		let mut file = vfs
			.open(root_path.as_ref().join("pack.mcmetac"))
			.or_else(|_| vfs.open(root_path.as_ref().join("pack.mcmeta")))?;
		let mut pack_meta_value =
			Vec::with_capacity(file.file_size_hint.try_into().unwrap_or(usize::MAX));

		file.file_read.read_to_end(&mut pack_meta_value).await?;

		// Parse the pack metadata to get its format version and do some basic validation.
		// We do this parsing manually, instead of using auxiliary structs that derive
		// deserialization traits, because it is faster, provides more relevant error
		// information, and we only need to parse a few things that are unlikely to change
		match serde_json::from_reader(StripComments::new(strip_utf8_bom(&*pack_meta_value)))? {
			Value::Object(root_object) => {
				match root_object.get("pack").ok_or(PackMetaError::MalformedMeta(
					"Missing \"pack\" key in root object"
				))? {
					Value::Object(pack_meta_object) => {
						match pack_meta_object.get("pack_format").ok_or(
							PackMetaError::MalformedMeta(
								"Missing \"pack_format\" key in pack metadata object"
							)
						)? {
							Value::Number(pack_format_version_number) => {
								// Minecraft always reads this field as a Java integer,
								// so a conversion to an i32 should be successful
								pack_format_version = i32::try_from(
									pack_format_version_number.as_i64().ok_or(
										PackMetaError::MalformedMeta(
											PACK_FORMAT_VERSION_IS_NOT_INTEGER
										)
									)?
								)
								.map_err(|_| {
									PackMetaError::MalformedMeta(PACK_FORMAT_VERSION_IS_NOT_INTEGER)
								})?;
							}
							_ => {
								return Err(PackMetaError::MalformedMeta(
									PACK_FORMAT_VERSION_IS_NOT_INTEGER
								))
							}
						};

						// Also validate the pack description, because it is required by Minecraft
						match pack_meta_object.get("description") {
							Some(Value::String(_))
							| Some(Value::Object(_))
							| Some(Value::Array(_)) => {
								// This can possibly be a Minecraft text component, parsed by the
								// static class Serializer at net.minecraft.network.chat.Component
							}
							Some(_) => {
								return Err(PackMetaError::MalformedMeta(
									"The \"description\" key value is not a text component"
								))
							}
							None => {
								return Err(PackMetaError::MalformedMeta(
									"Missing \"description\" key in pack metadata object"
								))
							}
						};
					}
					_ => {
						return Err(PackMetaError::MalformedMeta(
							"The \"pack\" key value is not a JSON object"
						))
					}
				}
			}
			_ => {
				return Err(PackMetaError::MalformedMeta(
					"The JSON value is not an object"
				))
			}
		};

		Ok(Self {
			pack_format_version
		})
	}

	/// Returns a maybe pessimistic set of Minecraft quirks that will need to be
	/// worked around to guarantee that the pack will work as expected.
	///
	/// This is done by looking at the `pack_format` version in the pack metadata,
	/// as that version specifies a range of Minecraft versions that the pack is
	/// meant to be compatible with. If only a subset of Minecraft versions that
	/// a `pack_format` version targets are affected by a quirk, that quirk will be
	/// returned in the set. Similarly, if the Minecraft versions a `pack_format`
	/// version targets may or may not be affected by some quirk, that quirk will
	/// be returned too.
	pub fn target_minecraft_versions_quirks(&self) -> EnumSet<MinecraftQuirk> {
		let mut quirks = EnumSet::empty();

		if self.pack_format_version < PACK_FORMAT_VERSION_1_13 {
			quirks |= MinecraftQuirk::GrayscaleImagesGammaMiscorrection;
			quirks |= MinecraftQuirk::RestrictiveBannerLayerTextureFormatCheck;
		}

		if self.pack_format_version < PACK_FORMAT_VERSION_1_17 {
			quirks |= MinecraftQuirk::Java8ZipParsing;
		}

		// All known Minecraft versions are affected by this quirk
		quirks |= MinecraftQuirk::BadEntityEyeLayerTextureTransparencyBlending;

		quirks
	}

	/// Returns a maybe pessimistic set of pack file asset types that Minecraft and
	/// its mods can read from a pack.
	///
	/// This is done by looking at the `pack_format` version in the pack metadata,
	/// as that version specifies a range of Minecraft versions that the pack is
	/// meant to be compatible with. If only a subset of Minecraft versions that
	/// a `pack_format` version targets use some asset type, that type will be
	/// returned in the set. Similarly, if the Minecraft versions a `pack_format`
	/// version targets may or may not use some asset type, that asset type will
	/// be returned too.
	pub fn target_minecraft_version_asset_type_mask(&self) -> EnumSet<PackFileAssetType> {
		let mut asset_type_mask = EnumSet::all();

		if self.pack_format_version >= PACK_FORMAT_VERSION_1_13 {
			asset_type_mask -= PackFileAssetType::LegacyLanguageFile;
		}

		if self.pack_format_version >= PACK_FORMAT_VERSION_1_17 {
			asset_type_mask -= PackFileAssetType::LegacyTextCredits;
		} else {
			asset_type_mask -= PackFileAssetType::TranslationUnitSegment;
		}

		asset_type_mask
	}
}
