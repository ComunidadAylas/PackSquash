//! Contains rules that define the constraints a pack type of a given format version should be
//! processed with.

use std::ops::{Range, RangeFrom, RangeInclusive, RangeTo};

use enumset::{EnumSet, enum_set};

use crate::{
	config::MinecraftQuirk,
	pack_file::asset_type::PackFileAssetType,
	pack_metadata::{PackFormatVersion, PackMetadata, PackType}
};

impl PackFormatVersion {
	/// The pack format version used in Minecraft versions from 1.13 to 1.14.4, for both resource
	/// and data packs.
	const RELEASE_1_13: Self = Self::single_component(4);

	/// The pack format version used in Minecraft versions from 1.15 to 1.16.1, for both resource
	/// and data packs.
	const RELEASE_1_15: Self = Self::single_component(5);

	/// The pack format version used in Minecraft versions from 1.17 to 1.17.1, for both resource
	/// and data packs.
	const RELEASE_1_17: Self = Self::single_component(7);

	/// The resource pack format version used in Minecraft versions from 23w17a to 1.20.1.
	const SNAPSHOT_23W_17A_RESOURCES: Self = Self::single_component(15);

	/// The resource pack format version used in Minecraft versions from 21w39a (1.18 snapshot) to
	/// 1.18.2.
	const RELEASE_1_18_RESOURCES: Self = Self::single_component(8);

	/// The resource pack format version used in Minecraft versions from 24w13a (1.20.5 snapshot)
	/// to 1.20.5-pre3.
	const SNAPSHOT_24W_13A_RESOURCES: Self = Self::single_component(31);

	/// The data pack format version used in Minecraft versions from 24w21a (1.21 snapshot)
	/// to 1.21-pre1.
	const SNAPSHOT_24W_21A_DATA: Self = Self::single_component(45);

	/// The resource pack format version used in Minecraft version 24w40a (1.21.2 snapshot).
	const SNAPSHOT_24W_40A_RESOURCES: Self = Self::single_component(40);
}

impl PackMetadata {
	/// Returns a maybe pessimistic set of Minecraft quirks that will need to be worked around to
	/// guarantee that the pack will work as expected.
	///
	/// This is done by looking at the format versions targeted by pack layers in the pack metadata,
	/// as those versions specify a range of Minecraft versions that the pack is meant to be
	/// compatible with. If only a subset of Minecraft versions targeted are affected by a quirk,
	/// that quirk will be returned in the set. Similarly, if the Minecraft versions targeted may or
	/// may not be affected by some quirk, that quirk will be returned too.
	pub fn applicable_minecraft_quirks(&self) -> EnumSet<MinecraftQuirk> {
		let versions_range = self.bounding_format_version_range();
		match self.ty {
			PackType::ClientResources => {
				Self::applicable_minecraft_resource_pack_quirks(versions_range)
			}
			PackType::ServerData => EnumSet::empty() // None for now
		}
	}

	fn applicable_minecraft_resource_pack_quirks(
		versions_range: RangeInclusive<PackFormatVersion>
	) -> EnumSet<MinecraftQuirk> {
		let mut quirks = EnumSet::empty();

		if versions_range.overlaps(..PackFormatVersion::RELEASE_1_13) {
			quirks |= MinecraftQuirk::GrayscaleImagesGammaMiscorrection;
			quirks |= MinecraftQuirk::RestrictiveBannerLayerTextureFormatCheck;
			quirks |= MinecraftQuirk::PngObfuscationIncompatibility;
		}

		if versions_range.overlaps(..PackFormatVersion::RELEASE_1_15)
			|| versions_range.overlaps(PackFormatVersion::SNAPSHOT_24W_13A_RESOURCES..)
		{
			// Minecraft 1.14 is compatible with this feature, but we can't tell
			// it apart from 1.13 due to it sharing the same version number, so
			// err on the safe side. For the time being, 24w14a is the last version
			// to support this feature, but it shares a version number with 24w13a
			quirks |= MinecraftQuirk::OggObfuscationIncompatibility;
		}

		if versions_range.overlaps(..PackFormatVersion::RELEASE_1_17) {
			quirks |= MinecraftQuirk::Java8ZipParsing;
		}

		if versions_range.overlaps(..PackFormatVersion::SNAPSHOT_24W_40A_RESOURCES) {
			// 24w39a is the first snapshot to have this fixed, but we can't tell it
			// apart from 24w38a due to it sharing the same pack format version number,
			// so err on the safe side
			quirks |= MinecraftQuirk::BadEntityEyeLayerTextureTransparencyBlending;
		}

		quirks
	}

	/// Returns a maybe pessimistic set of pack file asset types that Minecraft and
	/// its mods can read from a pack.
	///
	/// This is done by looking at the format versions targeted by pack layers in the pack metadata,
	/// as those versions specify a range of Minecraft versions that the pack is meant to be
	/// compatible with. If only a subset of Minecraft versions targeted use some asset type,
	/// that type will be returned in the set. Similarly, if the Minecraft versions targeted may or
	/// may not use some asset type, that type will be returned too.
	pub fn applicable_asset_type_mask(&self) -> EnumSet<PackFileAssetType> {
		let versions_range = self.bounding_format_version_range();

		// Initialize a mask with asset types common to all pack types and all game versions
		let mut asset_type_mask = enum_set!(
			PackFileAssetType::MinecraftMetadata
				| PackFileAssetType::MinecraftMetadataWithComments
				| PackFileAssetType::GenericJson
				| PackFileAssetType::GenericJsonWithComments
				| PackFileAssetType::Custom
		);

		match self.ty {
			PackType::ClientResources => {
				Self::applicable_resource_pack_asset_type_mask(versions_range, &mut asset_type_mask)
			}
			PackType::ServerData => {
				Self::applicable_data_pack_asset_type_mask(versions_range, &mut asset_type_mask)
			}
		}

		asset_type_mask
	}

	fn applicable_resource_pack_asset_type_mask(
		versions_range: RangeInclusive<PackFormatVersion>,
		asset_type_mask: &mut EnumSet<PackFileAssetType>
	) {
		// Add asset types common to resource packs for all game versions
		*asset_type_mask |= PackFileAssetType::MinecraftTextureMetadata;
		*asset_type_mask |= PackFileAssetType::MinecraftTextureMetadataWithComments;
		*asset_type_mask |= PackFileAssetType::MinecraftModel;
		*asset_type_mask |= PackFileAssetType::MinecraftModelWithComments;

		#[cfg(feature = "optifine")]
		{
			*asset_type_mask |= PackFileAssetType::OptifineCustomEntityModel;
			*asset_type_mask |= PackFileAssetType::OptifineCustomEntityModelWithComments;
			*asset_type_mask |= PackFileAssetType::OptifineCustomEntityModelPart;
			*asset_type_mask |= PackFileAssetType::OptifineCustomEntityModelPartWithComments;
			*asset_type_mask |= PackFileAssetType::OptifineVanillaItemModel;
			*asset_type_mask |= PackFileAssetType::OptifineVanillaItemModelWithComments;
			*asset_type_mask |= PackFileAssetType::OptifineVanillaTextureMetadata;
			*asset_type_mask |= PackFileAssetType::OptifineVanillaTextureMetadataWithComments;

			*asset_type_mask |= PackFileAssetType::OptifineTexture;

			*asset_type_mask |= PackFileAssetType::GenericProperties;
		}

		#[cfg(feature = "mtr3")]
		{
			*asset_type_mask |= PackFileAssetType::Mtr3CustomTrainModel;
			*asset_type_mask |= PackFileAssetType::Mtr3CustomTrainModelWithComments;

			*asset_type_mask |= PackFileAssetType::Mtr3CustomGenericTexture;
		}

		*asset_type_mask |= PackFileAssetType::GenericOggVorbisAudio;
		*asset_type_mask |= PackFileAssetType::GenericAudio;

		*asset_type_mask |= PackFileAssetType::PackIcon;
		*asset_type_mask |= PackFileAssetType::BannerLayer;
		*asset_type_mask |= PackFileAssetType::EyeLayer;
		*asset_type_mask |= PackFileAssetType::AuxiliaryShaderTargetTexture;
		*asset_type_mask |= PackFileAssetType::GenericTexture;

		*asset_type_mask |= PackFileAssetType::VertexShader;
		*asset_type_mask |= PackFileAssetType::FragmentShader;

		*asset_type_mask |= PackFileAssetType::Text;

		// Now add asset types that used by only a subset of versions

		if versions_range.overlaps(..PackFormatVersion::RELEASE_1_13) {
			*asset_type_mask |= PackFileAssetType::LegacyLanguageFile;
			*asset_type_mask |= PackFileAssetType::TrueTypeFont;
		}
		if versions_range.overlaps(PackFormatVersion::RELEASE_1_13..) {
			*asset_type_mask |= PackFileAssetType::TrueTypeOrOpenTypeFont;
		}

		if versions_range.overlaps(PackFormatVersion::SNAPSHOT_23W_17A_RESOURCES..) {
			*asset_type_mask |= PackFileAssetType::ZippedUnifontHex;
		}
		if versions_range.overlaps(..PackFormatVersion::SNAPSHOT_23W_17A_RESOURCES) {
			*asset_type_mask |= PackFileAssetType::LegacyUnicodeFontCharacterSizes;
		}

		if versions_range.overlaps(..PackFormatVersion::RELEASE_1_17) {
			*asset_type_mask |= PackFileAssetType::LegacyTextCredits;
		}
		if versions_range.overlaps(PackFormatVersion::RELEASE_1_17..) {
			*asset_type_mask |= PackFileAssetType::TranslationUnitSegment;
		}

		if versions_range.overlaps(PackFormatVersion::RELEASE_1_18_RESOURCES..) {
			*asset_type_mask |= PackFileAssetType::ClosingCreditsText;
		}
	}

	fn applicable_data_pack_asset_type_mask(
		versions_range: RangeInclusive<PackFormatVersion>,
		asset_type_mask: &mut EnumSet<PackFileAssetType>
	) {
		if versions_range.overlaps(..PackFormatVersion::SNAPSHOT_24W_21A_DATA) {
			*asset_type_mask |= PackFileAssetType::LegacyNbtStructure;
			*asset_type_mask |= PackFileAssetType::LegacyCommandFunction;
		}
		if versions_range.overlaps(PackFormatVersion::SNAPSHOT_24W_21A_DATA..) {
			*asset_type_mask |= PackFileAssetType::NbtStructure;
			*asset_type_mask |= PackFileAssetType::CommandFunction;
		}
	}
}

trait RangeInclusiveExt<Other> {
	/// Checks whether this range overlaps some other range, i.e. that their intersection is not
	/// contains at least a single point.
	///
	/// Implementations of this method assume that ranges are well-formed: the start bound is less
	/// than or equal to the end bound.
	fn overlaps(&self, other: Other) -> bool;
}

impl RangeInclusiveExt<Self> for RangeInclusive<PackFormatVersion> {
	fn overlaps(&self, other: Self) -> bool {
		self.start().max(other.start()) <= self.end().min(other.end())
	}
}

impl RangeInclusiveExt<Range<PackFormatVersion>> for RangeInclusive<PackFormatVersion> {
	fn overlaps(&self, other: Range<PackFormatVersion>) -> bool {
		let max_start_bound = self.start().max(&other.start);
		max_start_bound <= self.end() && max_start_bound < &other.end
	}
}

impl RangeInclusiveExt<RangeFrom<PackFormatVersion>> for RangeInclusive<PackFormatVersion> {
	fn overlaps(&self, other: RangeFrom<PackFormatVersion>) -> bool {
		&other.start <= self.end()
	}
}

impl RangeInclusiveExt<RangeTo<PackFormatVersion>> for RangeInclusive<PackFormatVersion> {
	fn overlaps(&self, other: RangeTo<PackFormatVersion>) -> bool {
		self.start() < &other.end
	}
}
