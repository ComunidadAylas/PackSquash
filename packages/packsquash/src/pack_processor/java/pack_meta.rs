//! Contains types to gather Minecraft: Java Edition pack metadata relevant for optimization
//! purposes.

use std::{borrow::Cow, fmt::Display, io, io::Read, ops::RangeBounds};

use ahash::AHashMap;
use enumset::EnumSet;
use json_comments::StripComments;
use packsquash_options::{
	minecraft_version, MinecraftQuirk, MinecraftVersion, MinecraftVersionRange
};
use packsquash_util::PrettySerdePathErrorWrapper;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use thiserror::Error;

use crate::{
	java::{
		pack_meta::filter_section::ResourceFilterSection, resource_location::ResourceLocation,
		PackType
	},
	pack_processor::java::pack_meta::{
		feature_flags_section::FeatureFlagsSection, language_section::LanguageSection,
		metadata_section::MetadataSection
	},
	util::{range_bounds_intersect::RangeBoundsIntersectExt, strip_utf8_bom::StripUtf8BomExt},
	vfs::VirtualFileSystem,
	RelativePath
};

mod feature_flags_section;
mod filter_section;
mod language_section;
mod metadata_section;

/// Metadata for a Java Edition resource or data pack, contained in the `pack.mcmeta` file in the
/// root folder of a pack.
///
/// References:
/// - <https://minecraft.wiki/w/Resource_Pack#Contents>
/// - <https://minecraft.wiki/w/Data_Pack#pack.mcmeta>
/// - Minecraft class `net.minecraft.server.packs.metadata.pack.PackMetadataSectionSerializer`
/// - Minecraft class `net.minecraft.server.packs.resources.ResourceFilterSection`
pub struct PackMeta {
	meta: SerializedPackMeta<'static>,
	pack_type: PackType,
	game_version_range: MinecraftVersionRange,
	game_version_quirks: EnumSet<MinecraftQuirk>
}

#[derive(Deserialize, Serialize, Debug)]
struct SerializedPackMeta<'data> {
	#[serde(rename = "pack")]
	#[serde(borrow)]
	metadata: MetadataSection<'data>,
	#[serde(rename = "language")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	#[serde(borrow)]
	additional_languages: Option<LanguageSection<'data>>,
	#[serde(rename = "filter")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	#[serde(borrow)]
	filtered_resources: Option<ResourceFilterSection<'data>>,
	#[serde(rename = "features")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	#[serde(borrow)]
	feature_flags: Option<FeatureFlagsSection<'data>>,
	#[serde(flatten, borrow)]
	bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

/// Represents an error that may happen while getting pack metadata.
#[derive(Error, Debug)]
pub enum PackMetaError {
	#[error("JSON error: {0}")]
	Json(#[from] PrettySerdePathErrorWrapper<serde_json::Error>),
	#[error("The pack format version {actual_pack_format_version} is too low (expected at least {expected_minimum_pack_format_version})")]
	TooLowPackFormatVersion {
		actual_pack_format_version: i32,
		expected_minimum_pack_format_version: i32
	},
	#[error("The pack format version {pack_format_version} is invalid for a {pack_type}")]
	InvalidPackFormatVersion {
		pack_format_version: i32,
		pack_type: PackType
	},
	#[error("Could not infer the pack type. Is it a valid pack?")]
	UnknownPackType,
	#[error("Minecraft version {actual_game_version} is not in the {expected_game_version_range} range implied by pack_format")]
	InvalidGameVersion {
		actual_game_version: MinecraftVersion,
		expected_game_version_range: MinecraftVersionRange
	},
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

impl PackMeta {
	/// Gathers the metadata from the pack contained in a virtual filesystem.
	pub fn new(
		vfs: &(impl VirtualFileSystem + ?Sized),
		target_game_version: Option<MinecraftVersion>,
		allow_json_comments: bool
	) -> Result<Self, PackMetaError> {
		let mut meta_file = vfs
			.open(&RelativePath::from_inner("pack.mcmeta"))?
			.reader
			.strip_utf8_bom()?;

		let mut stripped_comments_file_reader;
		let meta_reader;
		if allow_json_comments {
			stripped_comments_file_reader = StripComments::new(meta_file);
			meta_reader = &mut stripped_comments_file_reader as &mut dyn Read;
		} else {
			meta_reader = &mut meta_file;
		}

		let mut meta = packsquash_util::deserialize_with_pretty_path_on_error(
			&mut serde_json::Deserializer::from_reader(meta_reader)
		)?;

		let pack_type = infer_pack_type_from_meta_sections(&meta).map_or_else(
			|| {
				PackType::iter()
					.try_find(|pack_type| vfs.is_dir(&pack_type.root_directory()))?
					.ok_or(PackMetaError::UnknownPackType)
			},
			Ok
		)?;

		// The least valid pack format version for resource packs
		// is 1, and for data packs 4. Smaller values are effectively
		// reserved and should not be used
		let pack_format_version = meta.metadata.pack_format_version;
		let initial_pack_format_version = match pack_type {
			PackType::ResourcePack => 1,
			PackType::DataPack => 4
		};

		if pack_format_version < initial_pack_format_version {
			return Err(PackMetaError::TooLowPackFormatVersion {
				actual_pack_format_version: pack_format_version,
				expected_minimum_pack_format_version: initial_pack_format_version
			});
		}

		// Get the appropriate game version range according to the metadata we've just read
		let expected_game_version_range =
			possible_game_version_range(pack_type, pack_format_version)?;
		let game_version_range = match target_game_version {
			Some(target_game_version) => {
				if expected_game_version_range.contains(&target_game_version) {
					// The specified game version is plausible, so narrow the range
					Ok((target_game_version..=target_game_version).into())
				} else {
					Err(PackMetaError::InvalidGameVersion {
						actual_game_version: target_game_version,
						expected_game_version_range
					})
				}
			}
			None => Ok(expected_game_version_range)
		}?;

		// Resource filtering was introduced in 1.19
		if !(minecraft_version!(1, 19, 0)..).intersects(&game_version_range) {
			meta.filtered_resources = None;
		}

		// Datapack feature flags were introduced in 1.19.3
		if !(minecraft_version!(1, 19, 3)..).intersects(&game_version_range) {
			meta.feature_flags = None;
		}

		Ok(Self {
			meta,
			pack_type,
			game_version_quirks: get_game_version_quirks(pack_type, &game_version_range),
			game_version_range
		})
	}

	pub fn game_version_range(&self) -> &(impl RangeBounds<MinecraftVersion> + Display) {
		&self.game_version_range
	}

	// TODO update these docs
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
	pub fn game_version_quirks(&self) -> EnumSet<MinecraftQuirk> {
		self.game_version_quirks
	}

	pub fn pack_type(&self) -> PackType {
		self.pack_type
	}

	pub fn is_resource_location_filtered_out(
		&self,
		resource_location: &ResourceLocation<'_>
	) -> bool {
		self.meta.filtered_resources.as_ref().map_or_else(
			|| false,
			|resource_filter_section| {
				resource_filter_section
					.block_filters
					.iter()
					.any(|filter| filter.matches_resource_location(resource_location))
			}
		)
	}
}

fn possible_game_version_range(
	pack_type: PackType,
	pack_format_version: i32
) -> Result<MinecraftVersionRange, PackMetaError> {
	match (pack_type, pack_format_version) {
		(PackType::ResourcePack, 1) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 6, 1)..minecraft_version!(1, 9)
		)),
		(PackType::ResourcePack, 2) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 9)..minecraft_version!(1, 11)
		)),
		(PackType::ResourcePack, 3) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 11)..minecraft_version!(1, 13)
		)),
		(PackType::ResourcePack | PackType::DataPack, 4) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 13)..minecraft_version!(1, 15)
		)),
		(PackType::ResourcePack | PackType::DataPack, 5) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 15)..minecraft_version!(1, 16, 2)
		)),
		(PackType::ResourcePack | PackType::DataPack, 6) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 16, 2)..minecraft_version!(1, 17)
		)),
		(PackType::ResourcePack | PackType::DataPack, 7) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 17)..minecraft_version!(1, 18)
		)),
		// Data pack and resource pack format versions diverged here
		(PackType::ResourcePack, 8) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 18)..minecraft_version!(1, 19)
		)),
		(PackType::ResourcePack, 9) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 19)..minecraft_version!(1, 19, 3)
		)),
		(PackType::ResourcePack, 10) => {
			// For some strange reason, Mojang skipped number 10 for resource pack format versions. See:
			// https://www.reddit.com/r/Minecraft/comments/yug9b1/why_is_the_resource_pack_pack_format_now_12_in/
			Err(PackMetaError::InvalidPackFormatVersion {
				pack_type,
				pack_format_version
			})
		}
		(PackType::ResourcePack, 11) => Ok(MinecraftVersionRange::from(
			// Pack format version 11 was assigned to three 1.19.3 snapshots. We can consider such
			// packs as targeting 1.19.3 for our purposes
			minecraft_version!(1, 19, 3)..=minecraft_version!(1, 19, 3)
		)),
		(PackType::ResourcePack, 12) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 19, 3)..minecraft_version!(1, 19, 4)
		)),
		(PackType::ResourcePack, 13) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 19, 4)..minecraft_version!(1, 20)
		)),
		(PackType::DataPack, 8) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 18)..minecraft_version!(1, 18, 2)
		)),
		(PackType::DataPack, 9) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 18, 2)..=minecraft_version!(1, 18, 2)
		)),
		(PackType::DataPack, 10) => Ok(MinecraftVersionRange::from(
			minecraft_version!(1, 19)..minecraft_version!(1, 19, 4)
		)),
		(PackType::DataPack, 11 | 12) => Ok(MinecraftVersionRange::from(
			// Mojang assigned version 11 to three 1.19.4 snapshots. For our purposes,
			// we can consider such data packs as 1.19.4 datapacks
			minecraft_version!(1, 19, 4)..minecraft_version!(1, 20)
		)),
		// Future Minecraft versions at the time of writing
		_ => Ok(MinecraftVersionRange::from(minecraft_version!(1, 20)..))
	}
}

fn get_game_version_quirks(
	pack_type: PackType,
	target_minecraft_version: &impl RangeBounds<MinecraftVersion>
) -> EnumSet<MinecraftQuirk> {
	let mut quirks = EnumSet::empty();

	match pack_type {
		PackType::ResourcePack => {
			if (..minecraft_version!(1, 13)).intersects(target_minecraft_version) {
				quirks |= MinecraftQuirk::GrayscaleImagesGammaMiscorrection;
				quirks |= MinecraftQuirk::RestrictiveBannerLayerTextureFormatCheck;
			}

			if (..minecraft_version!(1, 14)).intersects(target_minecraft_version) {
				quirks |= MinecraftQuirk::OggObfuscationIncompatibility;
			}

			// All known Minecraft versions are affected by this quirk
			quirks |= MinecraftQuirk::BadEntityEyeLayerTextureTransparencyBlending;
		}
		PackType::DataPack => {
			// Data packs have no quirks to work around yet
		}
	}

	if (..minecraft_version!(1, 17)).intersects(target_minecraft_version) {
		quirks |= MinecraftQuirk::Java8ZipParsing;
	}

	quirks
}

fn infer_pack_type_from_meta_sections(meta: &SerializedPackMeta<'_>) -> Option<PackType> {
	if meta.feature_flags.is_some() {
		// Only data packs should define feature flags
		Some(PackType::DataPack)
	} else if meta.additional_languages.is_some() {
		// Only resource packs should define additional languages
		Some(PackType::ResourcePack)
	} else {
		// The meta sections are inconclusive to the pack type
		None
	}
}
