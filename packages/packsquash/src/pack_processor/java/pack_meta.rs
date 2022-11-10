//! Contains types to gather Minecraft: Java Edition pack metadata relevant for optimization
//! purposes.

use std::fmt::Display;
use std::io;
use std::ops::RangeBounds;

use enumset::EnumSet;
use json_comments::StripComments;
use packsquash_options::{
	minecraft_version, MinecraftQuirk, MinecraftVersion, MinecraftVersionRange
};
use serde_json::Value;
use strum::IntoEnumIterator;
use thiserror::Error;

use crate::java::pack_meta::filter_section::ResourceFilterSection;
use crate::java::resource_location::ResourceLocation;
use crate::java::PackType;
use crate::util::range_bounds_intersect::RangeBoundsIntersectExt;
use crate::util::strip_utf8_bom::StripUtf8BomExt;
use crate::vfs::VirtualFileSystem;
use crate::RelativePath;

mod filter_section;

/// Metadata for a Java Edition resource or data pack, contained in the `pack.mcmeta` file in the
/// root folder of a pack.
///
/// References:
/// - <https://minecraft.fandom.com/wiki/Resource_Pack#Contents>
/// - <https://minecraft.fandom.com/wiki/Data_Pack#pack.mcmeta>
/// - Minecraft class `net.minecraft.server.packs.metadata.pack.PackMetadataSectionSerializer`
/// - Minecraft class `net.minecraft.server.packs.resources.ResourceFilterSection`
pub struct PackMeta {
	meta: Value,
	pack_type: PackType,
	game_version_range: MinecraftVersionRange,
	game_version_quirks: EnumSet<MinecraftQuirk>,
	resource_filter_section: Option<ResourceFilterSection<'static>>
}

/// Represents an error that may happen while getting pack metadata.
#[derive(Error, Debug)]
pub enum PackMetaError {
	#[error("JSON error: {0}")]
	JsonSerde(#[from] serde_json::Error),
	#[error("Syntax error: {0}")]
	MalformedMeta(&'static str),
	#[error("Could not infer the pack type. Is it a valid pack?")]
	UnknownPackType,
	#[error("Minecraft version {actual_game_version} is not in the {expected_game_version_range} range implied by pack_format")]
	InvalidGameVersion {
		actual_game_version: MinecraftVersion,
		expected_game_version_range: MinecraftVersionRange
	},
	#[error("JSON error at {0}")]
	JsonSerdeWithPath(#[from] serde_path_to_error::Error<serde_json::Error>),
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
		const PACK_FORMAT_VERSION_IS_NOT_VALID_INTEGER: &str =
			"pack_format version is too low or not a Java integer";

		let pack_format_version;

		// TODO try .mcmetac
		let meta_file = vfs
			.open(&RelativePath::from_inner("pack.mcmeta"))?
			.reader
			.strip_utf8_bom()?;

		let pack_type = PackType::iter()
			.try_find(|pack_type| vfs.is_dir(&pack_type.root_directory()))?
			.ok_or(PackMetaError::UnknownPackType)?;

		// Parse the pack metadata to get its format version and do some basic validation.
		// We do this parsing manually, instead of using auxiliary structs that derive
		// deserialization traits, because it is faster and provides better error
		// information
		let meta = if allow_json_comments {
			serde_json::from_reader(StripComments::new(meta_file))
		} else {
			serde_json::from_reader(meta_file)
		}?;

		let expected_game_version_range;
		let game_version_range;
		let resource_filter_section;

		match &meta {
			Value::Object(root_object) => {
				match root_object.get("pack").ok_or(PackMetaError::MalformedMeta(
					"Missing pack key in root object"
				))? {
					Value::Object(pack_meta_object) => {
						match pack_meta_object.get("pack_format").ok_or(
							PackMetaError::MalformedMeta(
								"Missing pack_format key in pack metadata object"
							)
						)? {
							Value::Number(pack_format_version_number) => {
								// Minecraft always reads this field as a Java integer,
								// so a conversion to an i32 should be successful
								pack_format_version =
									i32::try_from(pack_format_version_number.as_i64().ok_or(
										PackMetaError::MalformedMeta(
											PACK_FORMAT_VERSION_IS_NOT_VALID_INTEGER
										)
									)?)
									.map_err(|_| {
										PackMetaError::MalformedMeta(
											PACK_FORMAT_VERSION_IS_NOT_VALID_INTEGER
										)
									})?;

								// The least valid pack format version for resource packs
								// is 1, and for data packs 4. Smaller values are effectively
								// reserved and should not be used
								let initial_pack_format_version = match pack_type {
									PackType::ResourcePack => 1,
									PackType::DataPack => 4
								};

								if pack_format_version < initial_pack_format_version {
									return Err(PackMetaError::MalformedMeta(
										PACK_FORMAT_VERSION_IS_NOT_VALID_INTEGER
									));
								}
							}
							_ => {
								return Err(PackMetaError::MalformedMeta(
									PACK_FORMAT_VERSION_IS_NOT_VALID_INTEGER
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
									"The description key value cannot be a text component"
								))
							}
							None => {
								return Err(PackMetaError::MalformedMeta(
									"Missing description key in pack metadata object"
								))
							}
						};
					}
					_ => {
						return Err(PackMetaError::MalformedMeta(
							"The pack key value is not a JSON object"
						))
					}
				};

				// Get the appropriate game version range according to the metadata we've just read
				expected_game_version_range =
					possible_game_version_range(pack_type, pack_format_version);
				game_version_range = match target_game_version {
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

				// Newer Minecraft versions support filtering (e.g., ignoring) files provided
				// by packs loaded before this pack, including default packs
				resource_filter_section = (minecraft_version!(1, 19, 0)..)
					.intersects(&game_version_range)
					.then(|| {
						root_object
							.get("filter")
							// TODO avoid this clone. Maybe refactor deserialization to use Serde traits?
							.cloned()
							.map(serde_path_to_error::deserialize)
					})
					.flatten()
					.transpose()?;
			}
			_ => {
				return Err(PackMetaError::MalformedMeta(
					"The JSON value is not an object"
				))
			}
		};

		Ok(Self {
			meta,
			pack_type,
			game_version_quirks: get_game_version_quirks(pack_type, &game_version_range),
			game_version_range,
			resource_filter_section
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
		self.resource_filter_section.as_ref().map_or_else(
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
) -> MinecraftVersionRange {
	match (pack_type, pack_format_version) {
		(PackType::ResourcePack, 1) => {
			MinecraftVersionRange::from(minecraft_version!(1, 6, 1)..minecraft_version!(1, 9))
		}
		(PackType::ResourcePack, 2) => {
			MinecraftVersionRange::from(minecraft_version!(1, 9)..minecraft_version!(1, 11))
		}
		(PackType::ResourcePack, 3) => {
			MinecraftVersionRange::from(minecraft_version!(1, 11)..minecraft_version!(1, 13))
		}
		(PackType::ResourcePack | PackType::DataPack, 4) => {
			MinecraftVersionRange::from(minecraft_version!(1, 13)..minecraft_version!(1, 15))
		}
		(PackType::ResourcePack | PackType::DataPack, 5) => {
			MinecraftVersionRange::from(minecraft_version!(1, 15)..minecraft_version!(1, 16, 2))
		}
		(PackType::ResourcePack | PackType::DataPack, 6) => {
			MinecraftVersionRange::from(minecraft_version!(1, 16, 2)..minecraft_version!(1, 17))
		}
		(PackType::ResourcePack | PackType::DataPack, 7) => {
			MinecraftVersionRange::from(minecraft_version!(1, 17)..minecraft_version!(1, 18))
		}
		(PackType::ResourcePack, 8) => {
			MinecraftVersionRange::from(minecraft_version!(1, 18)..minecraft_version!(1, 19))
		}
		(PackType::ResourcePack, 9) => {
			MinecraftVersionRange::from(minecraft_version!(1, 19)..minecraft_version!(1, 20))
		}
		(PackType::DataPack, 8) => {
			MinecraftVersionRange::from(minecraft_version!(1, 18)..minecraft_version!(1, 18, 2))
		}
		(PackType::DataPack, 9) => {
			MinecraftVersionRange::from(minecraft_version!(1, 18, 2)..=minecraft_version!(1, 18, 2))
		}
		(PackType::DataPack, 10) => {
			MinecraftVersionRange::from(minecraft_version!(1, 19)..minecraft_version!(1, 20))
		}
		// Future Minecraft versions at the time of writing
		_ => MinecraftVersionRange::from(minecraft_version!(1, 20)..)
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
