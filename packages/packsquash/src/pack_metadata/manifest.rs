//! Type definitions for serialized pack metadata.
//!
//! The entry point for this module is the [`PackMetadata`] struct. Internally, such struct is
//! deserialized from a given [`PackType`], and the right [`PackType`] to use is guessed by the
//! [`PackMetadata::read()`] method.

use std::{
	cmp, fmt,
	hash::{Hash, Hasher},
	io,
	ops::{Deref, RangeInclusive},
	path::Path
};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use arcstr::ArcStr;
use json_comments::StripComments;
use serde::{
	Deserialize, Deserializer, Serialize, Serializer,
	de::{self, DeserializeSeed, Visitor, value::MapAccessDeserializer}
};
use strum::VariantArray;
use thiserror::Error;
use tokio::io::AsyncReadExt;

use crate::{pack_file::strip_utf8_bom, vfs::VirtualFileSystem};

/// The format version a pack can declare compatibility with.
///
/// Before Minecraft 25w31a, pack format versions were a single version number. From Minecraft
/// 25w31a onwards, a pack format version has both a major and minor version number.
///
/// References:
/// - <https://minecraft.wiki/w/Pack.mcmeta>
/// - Minecraft 26.2 `net.minecraft.server.packs.metadata.pack.PackFormat` class
#[derive(Clone, Copy, Debug)]
pub struct PackFormatVersion {
	/// The major pack format version targeted. For pack format versions where the concept of a
	/// minor version did not exist, this represents the only version number.
	///
	/// Minecraft versions such as 26.2 treat a major version of `i32::MAX` as indicating
	/// compatibility with an unknown version.
	pub major_version: i32,
	/// The minor pack format version targeted. When such a version is not set or not applicable, a
	/// default value appropriate to the context is assumed.
	pub minor_version: i32,
	ctx: PackFormatVersionSerdeContext
}

impl PackFormatVersion {
	/// The minimum possible pack format version.
	pub(super) const MIN: Self = Self::single_component(i32::MIN);

	/// The maximum possible pack format version, which can also be interpreted as an unknown
	/// version by the game.
	pub(super) const MAX_OR_UNKNOWN: Self = Self::single_component(i32::MAX);

	/// Creates a pack format version for a single-component (i.e., legacy) version string, without
	/// a defined minor version.
	pub(super) const fn single_component(version: i32) -> Self {
		Self {
			major_version: version,
			minor_version: 0,
			ctx: PackFormatVersionSerdeContext::LEGACY
		}
	}
}

impl Serialize for PackFormatVersion {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		if self.ctx.single_version_integer_expected
			|| self.minor_version == self.ctx.default_minor_version
		{
			self.major_version.serialize(serializer)
		} else {
			(self.major_version, self.minor_version).serialize(serializer)
		}
	}
}

impl PartialEq for PackFormatVersion {
	fn eq(&self, other: &Self) -> bool {
		self.major_version == other.major_version && self.minor_version == other.minor_version
	}
}

impl Eq for PackFormatVersion {}

impl PartialOrd for PackFormatVersion {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PackFormatVersion {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		(self.major_version, self.minor_version).cmp(&(other.major_version, other.minor_version))
	}
}

impl Hash for PackFormatVersion {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.major_version, self.minor_version).hash(state)
	}
}

/// Serialization-relevant tidbits of information about a [`PackFormatVersion`].
#[derive(Debug, Clone, Copy)]
struct PackFormatVersionSerdeContext {
	/// Whether a single version integer is expected (i.e., two major and minor version integers are
	/// not allowed). This is the case for the versions under the `pack_format` and
	/// `supported_formats` keys.
	single_version_integer_expected: bool,
	/// Whether negative version integers are allowed. This is the case for the versions under the
	/// `pack_format` and `supported_formats` keys.
	negative_version_integers_allowed: bool,
	/// What minor version to default to in case the minor version integer is absent, either because
	/// we are expecting a single version integer, or because the minor version integer is missing.
	/// Minecraft assumes a minor version of `i32::MAX` for `max_format`, and of zero for
	/// `min_format` and elsewhere.
	default_minor_version: i32
}

impl PackFormatVersionSerdeContext {
	/// A legacy pack format version serialization context.
	const LEGACY: Self = Self {
		single_version_integer_expected: true,
		negative_version_integers_allowed: true,
		default_minor_version: 0
	};

	/// A `min_format` pack format version serialization context.
	const MINIMUM_FORMAT: Self = Self {
		single_version_integer_expected: false,
		negative_version_integers_allowed: false,
		default_minor_version: 0
	};

	/// A `max_format` pack format version serialization context.
	const MAXIMUM_FORMAT: Self = Self {
		single_version_integer_expected: false,
		negative_version_integers_allowed: false,
		default_minor_version: i32::MAX
	};
}

impl PackFormatVersionSerdeContext {
	/// Validates whether the negativity (i.e., quality of being less than zero) of the given
	/// version integer is consistent with the expectations for this serialization context.
	fn validate_negativity<E: de::Error>(&self, version: i32) -> Result<i32, E> {
		(self.negative_version_integers_allowed || version >= 0)
			.then_some(version)
			.ok_or_else(|| E::custom("a version integer must be non-negative in this context"))
	}
}

impl<'de> DeserializeSeed<'de> for PackFormatVersionSerdeContext {
	type Value = PackFormatVersion;

	fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
		if self.single_version_integer_expected {
			let major_version = self.validate_negativity(<i32>::deserialize(deserializer)?)?;

			return Ok(Self::Value {
				major_version,
				minor_version: self.default_minor_version,
				ctx: self
			});
		}

		/// A `deserialize_any`-friendly visitor for the formats a pack format version can be
		/// expressed in. We use a custom visitor instead of an untagged enum for better
		/// diagnostics and performance.
		struct MaybeIntegerPairVersionVisitor {
			ctx: PackFormatVersionSerdeContext
		}

		impl<'de> Visitor<'de> for MaybeIntegerPairVersionVisitor {
			type Value = PackFormatVersion;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str(
					"a pair of major and minor version integers, or a major version integer"
				)
			}

			fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
				self.visit_i64(
					v.try_into()
						// A u64 that does not fit in i64 cannot fit in i32 either
						.map_err(|_| {
							E::custom("integer is out of range for a 32-bit signed integer")
						})?
				)
			}

			fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
				Ok(PackFormatVersion {
					major_version: self.ctx.validate_negativity(v.try_into().map_err(|_| {
						E::custom("integer is out of range for a 32-bit signed integer")
					})?)?,
					minor_version: self.ctx.default_minor_version,
					ctx: self.ctx
				})
			}

			fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
				let major_version = self
					.ctx
					.validate_negativity(seq.next_element()?.ok_or_else(|| {
						de::Error::invalid_length(0, &"either one or two version integers")
					})?)?;
				let minor_version = self.ctx.validate_negativity(
					seq.next_element()?
						.unwrap_or(self.ctx.default_minor_version)
				)?;

				// The vanilla game accepts up to 254 more integers here, which are ignored. This
				// opens up the technically correct possibility of mods and pack authors storing
				// additional version numbers in this array to track e.g. build numbers, protocol
				// versions, and so on. However, we take the interoperability-preserving stance of
				// rejecting such trailing numbers because they contribute to format ossification,
				// and tracking additional version metadata is more robustly done on custom,
				// extension key-value pairs rather than overloading values with a well-defined
				// meaning
				if seq.next_element::<i32>()?.is_some() {
					return Err(de::Error::custom(
						"found three or more version integers, but only the first two have a known meaning"
					));
				}

				Ok(PackFormatVersion {
					major_version,
					minor_version,
					ctx: self.ctx
				})
			}
		}

		deserializer.deserialize_any(MaybeIntegerPairVersionVisitor { ctx: self })
	}
}

/// A range of legacy pack format versions (i.e., without a minor version integer component) a pack
/// can declare compatibility with.
///
/// This was used between Minecraft 23w31a (inclusive, 1.20.2) and 25w31a (exclusive, 1.21.9) to
/// make a pack explicitly compatible with different pack format versions, potentially in
/// combination with overlays. After 25w31a, this was superseded by separate `min_format` and
/// `max_format` fields.
///
/// This struct is cheap to clone.
///
/// References:
/// - <https://minecraft.wiki/w/Pack.mcmeta>
/// - Minecraft 26.2 `net.minecraft.server.packs.metadata.pack.PackFormat` class
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
struct SupportedLegacyPackFormatVersionRange(RangeInclusive<PackFormatVersion>);

impl<'de> Deserialize<'de> for SupportedLegacyPackFormatVersionRange {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		/// A `deserialize_any`-friendly visitor for the formats a supported legacy pack format
		/// version range can be expressed in. We use a custom visitor instead of an untagged enum
		/// for better diagnostics and performance.
		struct SupportedLegacyPackFormatVersionRangeVisitor;

		impl<'de> Visitor<'de> for SupportedLegacyPackFormatVersionRangeVisitor {
			type Value = SupportedLegacyPackFormatVersionRange;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str(
					"a supported version integer, \
					a pair of minimum and maximum supported version integers, \
					or an object with `min_inclusive` and `max_inclusive` keys containing supported version integers"
				)
			}

			fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
				self.visit_i64(
					v.try_into()
						// A u64 that does not fit in i64 cannot fit in i32 either
						.map_err(|_| {
							E::custom("integer is out of range for a 32-bit signed integer")
						})?
				)
			}

			fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
				let version = v
					.try_into()
					.map_err(|_| E::custom("integer is out of range for a 32-bit signed integer"))?;

				Ok(Self::Value {
					0: PackFormatVersion::single_component(version)
						..=PackFormatVersion::single_component(version)
				})
			}

			fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
				let min_version = seq.next_element()?.ok_or_else(|| {
					de::Error::invalid_length(0, &"either one or two version integers")
				})?;
				let max_version = seq.next_element()?.unwrap_or(min_version);

				if seq.next_element::<i32>()?.is_some() {
					return Err(de::Error::custom(
						"found three or more version integers, but only two are expected"
					));
				}

				if min_version > max_version {
					return Err(de::Error::custom(
						"the minimum supported format version cannot be greater than the maximum supported format version"
					));
				}

				Ok(Self::Value {
					0: PackFormatVersion::single_component(min_version)
						..=PackFormatVersion::single_component(max_version)
				})
			}

			fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
				#[derive(Deserialize)]
				#[serde(deny_unknown_fields)]
				struct RangeMap {
					min_inclusive: i32,
					max_inclusive: i32
				}

				let range_map = <RangeMap>::deserialize(MapAccessDeserializer::new(map))?;

				if range_map.min_inclusive > range_map.max_inclusive {
					return Err(de::Error::custom(
						"`min_inclusive` cannot be greater than `max_inclusive`"
					));
				}

				Ok(Self::Value {
					0: PackFormatVersion::single_component(range_map.min_inclusive)
						..=PackFormatVersion::single_component(range_map.max_inclusive)
				})
			}
		}

		deserializer.deserialize_any(SupportedLegacyPackFormatVersionRangeVisitor)
	}
}

impl Serialize for SupportedLegacyPackFormatVersionRange {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let min_version = self.start();
		let max_version = self.end();

		if min_version == max_version {
			min_version.serialize(serializer)
		} else {
			(min_version, max_version).serialize(serializer)
		}
	}
}

impl Deref for SupportedLegacyPackFormatVersionRange {
	type Target = RangeInclusive<PackFormatVersion>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

/// The name of the directory containing a pack layer, relative to the root directory of the pack. A
/// pack layer can be an overlay, in which case this matches the name of the directory of such
/// overlay, or the base pack in a pack, in which case an empty name is used. In other words, a
/// layer represents a set of pack objects the game reads at the same override hierarchy level.
///
/// Such directory names can only contain the following characters: `a-zA-Z0-9_-.`
pub type PackLayerDirectoryName = ArcStr;

/// Defines the serialized format of overlays contained in a pack.
///
/// Overlays are logical packs within a pack that are conditionally applied on top of the base pack
/// if their target pack format version range matches that of the running client. Applying a pack on
/// top of another has the effect of the top pack overriding the files it has in common with lower
/// packs, similarly to copying a filesystem directory on top of another while overwriting files.
///
/// References:
/// - <https://minecraft.wiki/w/Pack.mcmeta>
/// - Minecraft 26.2 `net.minecraft.server.packs.OverlayMetadataSection` class
/// - Minecraft 26.2 `net.minecraft.server.packs.CompositePackResources` class
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
struct OverlaysMetadataJson {
	/// The overlays of a pack.
	///
	/// The game applies the overlays listed here in reverse iteration order, i.e. the last overlay
	/// takes priority.
	entries: Vec<OverlayEntryJson>
}

/// The serialized format of a pack overlay definition.
///
/// The same overlay directory can appear in several entries, opening up the possibility of
/// restoring the content of an overlay after others are applied under a potentially different range
/// of supported formats.
///
/// References:
/// - <https://minecraft.wiki/w/Pack.mcmeta>
/// - Minecraft 26.2 `net.minecraft.server.packs.OverlayMetadataSection` class
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
struct OverlayEntryJson {
	/// The name component of the directory containing this overlay.
	///
	/// Overlay directories are relative to the root directory of the base pack (i.e., where the
	/// `pack.mcmeta` file is).
	#[serde(deserialize_with = "deserialize_overlay_entry_directory")]
	directory: PackLayerDirectoryName,
	#[serde(default, deserialize_with = "deserialize_min_format_version")]
	min_format: Option<PackFormatVersion>,
	#[serde(default, deserialize_with = "deserialize_max_format_version")]
	max_format: Option<PackFormatVersion>,
	#[serde(default, rename = "formats")]
	supported_formats: Option<SupportedLegacyPackFormatVersionRange>
}

fn deserialize_overlay_entry_directory<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<PackLayerDirectoryName, D::Error> {
	let directory = <ArcStr>::deserialize(deserializer)?;

	// Now validate the directory as the game would. Note that we add an extra validation that the
	// directory is not `..`, because the game, as of 26.3-snapshot-7, builds paths with this string
	// in a way susceptible to mostly harmless and useless directory traversal attacks
	if directory.is_empty()
		|| directory == ".."
		|| directory
			.chars()
			.any(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.'))
	{
		Err(de::Error::custom(
			"an overlay directory name cannot be empty, `..`, or contain characters outside `a-zA-Z0-9_-.`"
		))
	} else {
		Ok(directory)
	}
}

fn deserialize_legacy_format_version<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<Option<PackFormatVersion>, D::Error> {
	Ok(Some(
		PackFormatVersionSerdeContext::LEGACY.deserialize(deserializer)?
	))
}

fn deserialize_min_format_version<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<Option<PackFormatVersion>, D::Error> {
	Ok(Some(
		PackFormatVersionSerdeContext::MINIMUM_FORMAT.deserialize(deserializer)?
	))
}

fn deserialize_max_format_version<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<Option<PackFormatVersion>, D::Error> {
	Ok(Some(
		PackFormatVersionSerdeContext::MAXIMUM_FORMAT.deserialize(deserializer)?
	))
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct PackSectionJson {
	/// A human-friendly description text component for the pack, which can be formatted and shown
	/// to users in the game.
	#[serde(deserialize_with = "deserialize_loose_text_component")]
	description: serde_json::Value,
	/// The pack format version this pack targets.
	///
	/// When overlays were added in Minecraft 23w31a, this field began representing the "main" pack
	/// format version targeted by the base layer of the pack, which previous game versions still
	/// took into account. However, game versions that read `supported_formats`/`min_format`/
	/// `max_format` no longer have an internal concept of a single supported pack format version
	/// for a pack layer, reducing `pack_format`'s purpose mainly to backwards compatibility. After
	/// the switch to major and minor format versions, `pack_format` is no longer accepted, unless
	/// targeting older game versions.
	#[serde(default, deserialize_with = "deserialize_legacy_format_version")]
	pack_format: Option<PackFormatVersion>,
	#[serde(default, deserialize_with = "deserialize_min_format_version")]
	min_format: Option<PackFormatVersion>,
	#[serde(default, deserialize_with = "deserialize_max_format_version")]
	max_format: Option<PackFormatVersion>,
	#[serde(default)]
	supported_formats: Option<SupportedLegacyPackFormatVersionRange>
}

/// The serialized format of a pack metadata manifest.
///
/// This is an intermediate format, used internally by [`PackType`] during the deserialization of a
/// high-level [`PackMetadata`] struct other modules are coupled with.
///
/// References:
/// - <https://minecraft.wiki/w/Resource_Pack#Contents>
/// - <https://minecraft.wiki/w/Data_Pack#pack.mcmeta>
/// - <https://minecraft.wiki/w/Pack.mcmeta>
/// - Minecraft classes `net.minecraft.server.packs.metadata.pack.PackMetadataSectionSerializer`
///   (older Minecraft versions), `net.minecraft.server.packs.metadata.PackMetadataSection` (Minecraft
///   26.1 and 26.2).
#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct PackMetadataJson {
	#[serde(rename = "pack")]
	pack_section: PackSectionJson,
	#[serde(default)]
	overlays: Option<OverlaysMetadataJson>,
	/// Additional pieces of pack metadata not processed by PackSquash at this time that
	/// should be kept as-is, be it because they are extensions users or modifications came
	/// up with, or PackSquash does not have dedicated processing logic for them yet.
	///
	/// As of Minecraft 26.2, vanilla packs may contain extra `features`, `filter`, and
	/// `language` fields that are used by the game.
	#[serde(flatten)]
	extra: HashMap<String, serde_json::Value>
}

fn deserialize_loose_text_component<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<serde_json::Value, D::Error> {
	let text_component = <serde_json::Value>::deserialize(deserializer)?;

	// We don't optimize text components for now, and they have an intricate format that has been
	// routinely extended over time, so let's only validate we don't have an egregiously wrong value
	// type here for maintainability
	if matches!(
		text_component,
		serde_json::Value::String(_) | serde_json::Value::Object(_) | serde_json::Value::Array(_)
	) {
		Ok(text_component)
	} else {
		Err(de::Error::custom(
			"the pack description must be a text component"
		))
	}
}

/// Represents a Minecraft pack type.
///
/// References:
/// - Minecraft 26.2 `net.minecraft.server.packs.PackType` class
/// - Minecraft 26.2 `net.minecraft.server.packs.metadata.pack.PackFormat` class
#[derive(VariantArray, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackType {
	/// A resource pack, containing game resource assets for the client.
	ClientResources,
	/// A data pack, containing data-driven gameplay definitions for the server.
	ServerData
}

impl PackType {
	/// Returns the relative path from the root folder of a pack layer to the directory all the
	/// payload files of a pack layer of this type are placed under.
	///
	/// The vanilla game infers the type of a pack by context (i.e., taking into account from where
	/// it is loaded from), but history has shown that we can reliably use such a marker directory
	/// to make an accurate guess about the intended type of a pack.
	pub(super) const fn prefix_directory(&self) -> &'static str {
		match self {
			Self::ClientResources => "assets",
			Self::ServerData => "data"
		}
	}

	/// Deserializes the pack metadata manifest for a pack of this type from the given deserializer.
	fn deserialize_meta<'de, D: Deserializer<'de>>(
		&self,
		deserializer: D
	) -> Result<PackMetadata, D::Error> {
		let pack_meta = <PackMetadataJson>::deserialize(deserializer)?;

		let mut pack_layers = HashMap::with_capacity(
			1 + pack_meta
				.overlays
				.as_ref()
				.map(|overlays_holder| overlays_holder.entries.len())
				.unwrap_or(0)
		);

		// Read the format version range for the base layer of this pack (i.e., its non-overlay objects),
		// which also determines the format version range the pack supports
		let base_layer_format_version_range = match (
			pack_meta.pack_section.pack_format,
			pack_meta.pack_section.min_format,
			pack_meta.pack_section.max_format,
			pack_meta.pack_section.supported_formats.as_ref()
		) {
			(pack_format, Some(min_format), Some(max_format), supported_formats) => self
				.deserialize_new_format_version_range::<D>(
					false,
					pack_format,
					min_format,
					max_format,
					supported_formats
				),
			(_, Some(_), None, _) | (_, None, Some(_), _) => Err(de::Error::custom(
				"when setting `min_format` or `max_format`, both must be set"
			)),
			(pack_format, None, None, Some(supported_formats)) => {
				self.deserialize_old_format_version_range::<D>(false, pack_format, supported_formats)
			}
			(Some(pack_format), None, None, None) => {
				self.deserialize_ancient_format_version_range::<D>(pack_format)
			}
			(None, None, None, None) => Err(de::Error::custom(
				"missing pack format version information: \
				`min_format`/`max_format`, `supported_formats`, or `pack_format` must be set \
				according to the supported game version range"
			))
		}?;

		pack_layers.insert(
			arcstr::literal!(""), // Resolves to root pack directory
			HashSet::from_iter([base_layer_format_version_range])
		);

		// Now read the overlay layers. Due to the proper design of the game's code, this can reuse
		// the same deserialization helpers as the base layer above (it could be even better if we
		// didn't have to deal with this overlay and minor version madness in the first place,
		// though...)
		for (i, overlay) in pack_meta
			.overlays
			.iter()
			.flat_map(|overlays_holder| &overlays_holder.entries)
			.enumerate()
		{
			let overlay_layer_version_range = match (
				overlay.min_format,
				overlay.max_format,
				overlay.supported_formats.as_ref()
			) {
				(Some(min_format), Some(max_format), supported_formats) => self
					.deserialize_new_format_version_range::<D>(
						true,
						None,
						min_format,
						max_format,
						supported_formats
					),
				(Some(_), None, _) | (None, Some(_), _) => Err(de::Error::custom(
					"when setting `min_format` or `max_format`, both must be set"
				)),
				(None, None, Some(supported_formats)) => {
					self.deserialize_old_format_version_range::<D>(true, None, supported_formats)
				}
				(None, None, None) => Err(de::Error::custom(
					"missing overlay format version information: \
					`min_format`/`max_format` or `formats` must be set according to the supported game version range"
				))
			}
			.map_err(|err| de::Error::custom(format_args!("on overlay {i}: {err}")))?;

			pack_layers
				.entry(overlay.directory.clone())
				.or_insert_with(|| HashSet::with_capacity(1))
				.insert(overlay_layer_version_range);
		}

		Ok(PackMetadata {
			ty: *self,
			layers: pack_layers,
			json: pack_meta
		})
	}

	/// Returns the pack format of the last Minecraft version that used single-component pack format
	/// versions, 1.21.8, inclusive.
	const fn last_single_component_format_version(&self) -> PackFormatVersion {
		match self {
			Self::ClientResources => PackFormatVersion::single_component(64),
			Self::ServerData => PackFormatVersion::single_component(81)
		}
	}

	// The validating deserialization logic below is meant to be functionally equivalent to that in
	// the Minecraft 26.2 `net.minecraft.server.packs.metadata.pack.PackFormat` class

	fn deserialize_new_format_version_range<'de, D: Deserializer<'de>>(
		&self,
		for_overlay: bool,
		pack_format: Option<PackFormatVersion>,
		min_format: PackFormatVersion,
		max_format: PackFormatVersion,
		supported_formats: Option<&SupportedLegacyPackFormatVersionRange>
	) -> Result<RangeInclusive<PackFormatVersion>, D::Error> {
		let supported_formats_field_name = if for_overlay {
			"formats"
		} else {
			"supported_formats"
		};

		if max_format < min_format {
			return Err(de::Error::custom(
				"`max_format` must be greater than or equal to `min_format`"
			));
		}

		if min_format >= self.last_single_component_format_version() && !for_overlay {
			if supported_formats.is_some() {
				return Err(de::Error::custom(format_args!(
					"`supported_formats` cannot be set when `min_format` targets pack format versions that reject `{supported_formats_field_name}`"
				)));
			}

			if let Some(pack_format) = pack_format
				&& (!(min_format..=max_format).contains(&pack_format)
					|| pack_format < PackFormatVersion::single_component(15))
			{
				return Err(de::Error::custom(
					"`pack_format` must be within the range defined by `min_format` and `max_format`, \
					and target Minecraft 23w17a/23w18a or newer (version 15), as older versions \
					lack multi-version pack support"
				));
			}
		} else if let Some(supported_formats) = supported_formats {
			// Deviation from the game logic: we check that `min_format` matches the start of
			// `supported_formats` with minor version granularity too, as we want to standardize on
			// a minor of zero for single-component versions
			if supported_formats.start() != &min_format
				|| (supported_formats.end().major_version != max_format.major_version
					&& supported_formats.end().major_version
						!= self.last_single_component_format_version().major_version)
			{
				return Err(de::Error::custom(format_args!(
					"`min_format` and `max_format` must match the versions targeted by `{supported_formats_field_name}`"
				)));
			}

			match (for_overlay, pack_format) {
				(false, Some(pack_format))
					if !(min_format..=max_format).contains(&pack_format)
						|| pack_format < PackFormatVersion::single_component(15) =>
				{
					return Err(de::Error::custom(
						"`pack_format` must be within the range defined by `min_format` and `max_format`, \
						and target Minecraft 23w17a/23w18a or newer (version 15), as older versions \
						lack multi-version pack support"
					));
				}
				(_, Some(_)) | (true, None) => (), // All good. Overlays don't deserialize a `pack_format`
				(false, None) => {
					return Err(de::Error::custom(format_args!(
						"`pack_format` must be set when targeting pack format versions that expect `{supported_formats_field_name}` or older"
					)));
				}
			}
		} else {
			return Err(de::Error::custom(format_args!(
				"`supported_formats` must be set when `min_format` targets pack format versions that expect `{supported_formats_field_name}`"
			)));
		}

		Ok(min_format..=max_format)
	}

	fn deserialize_old_format_version_range<'de, D: Deserializer<'de>>(
		&self,
		for_overlay: bool,
		pack_format: Option<PackFormatVersion>,
		supported_formats: &SupportedLegacyPackFormatVersionRange
	) -> Result<RangeInclusive<PackFormatVersion>, D::Error> {
		let supported_formats_field_name = if for_overlay {
			"formats"
		} else {
			"supported_formats"
		};

		if supported_formats.end() > &self.last_single_component_format_version() {
			return Err(de::Error::custom(format_args!(
				"`{supported_formats_field_name}` cannot be set when targeting pack format versions that expect `min_format`/`max_format` instead"
			)));
		}

		match (for_overlay, pack_format) {
			(false, Some(pack_format))
				if !supported_formats.contains(&pack_format)
					|| pack_format < PackFormatVersion::single_component(15) =>
			{
				return Err(de::Error::custom(
					"`pack_format` must be within the range defined by `supported_formats`, \
					and target Minecraft 23w17a/23w18a or newer (version 15), as older versions \
					lack multi-version pack support"
				));
			}
			(_, Some(_)) | (true, None) => (), // All good. Overlays don't deserialize a `pack_format`
			(false, None) => {
				return Err(de::Error::custom(format_args!(
					"`{supported_formats_field_name}` must be set when `min_format` targets pack format versions \
					that expect `{supported_formats_field_name}`"
				)));
			}
		}

		Ok(supported_formats.0.clone())
	}

	fn deserialize_ancient_format_version_range<'de, D: Deserializer<'de>>(
		&self,
		pack_format: PackFormatVersion
	) -> Result<RangeInclusive<PackFormatVersion>, D::Error> {
		if pack_format > self.last_single_component_format_version() {
			return Err(de::Error::custom(
				"`pack_format` cannot be set when targeting pack format versions that expect `min_format`/`max_format` instead"
			));
		}

		Ok(pack_format..=pack_format)
	}
}

/// A view of the pack metadata manifest in a form suitable for further processing.
///
/// Minecraft places the pack metadata manifest at the `pack.mcmeta` file in the root folder of a
/// pack. As an extension, PackSquash also allows reading such a manifest from a `pack.mcmetac`
/// file.
#[derive(Serialize)]
pub struct PackMetadata {
	/// The type of the pack described by this metadata.
	#[serde(skip)]
	pub ty: PackType,
	/// The layers that compose this pack.
	///
	/// The concept of "layer" is defined at the documentation for [`PackLayerDirectoryName`]. Every
	/// pack contains at least one base layer for non-overlay directories.
	#[serde(skip)]
	pub layers: HashMap<PackLayerDirectoryName, HashSet<RangeInclusive<PackFormatVersion>>>,
	/// The original manifest this struct was deserialized from, to allow for eventual optimization
	/// and lossless re-serialization of it.
	#[serde(flatten)]
	json: PackMetadataJson
}

/// Represents an error that may happen while fetching pack metadata.
#[derive(Error, Debug)]
pub enum PackMetadataError {
	#[error(
		"The root directory of the pack contains prefix directories for multiple pack types. \
		Please ensure your pack contains only the prefix directory for the desired pack type"
	)]
	AmbiguousPackType,
	#[error(
		"The root directory of the pack lacks prefix directories for any known pack type. \
		Please add at least one prefix directory to indicate the desired pack type"
	)]
	UnknownPackType,
	#[error("Manifest error: {0}")]
	JsonSerde(#[from] serde_json::Error),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

impl PackMetadata {
	/// Reads the metadata for a pack at `root_path` on the given `vfs`.
	pub async fn read(
		vfs: &impl VirtualFileSystem,
		root_path: impl AsRef<Path>
	) -> Result<Self, PackMetadataError> {
		let root_path = root_path.as_ref();

		let mut pack_type = None;
		for candidate_pack_type in PackType::VARIANTS {
			match (
				vfs.file_type(root_path.join(candidate_pack_type.prefix_directory()))
					.map(|marker_file_type| marker_file_type.is_dir())
					.ok()
					.unwrap_or(false),
				&mut pack_type
			) {
				(true, pack_type @ None) => *pack_type = Some(candidate_pack_type),
				(true, Some(_)) => return Err(PackMetadataError::AmbiguousPackType),
				(false, _) => ()
			}
		}

		let Some(pack_type) = pack_type else {
			return Err(PackMetadataError::UnknownPackType);
		};

		let mut manifest_buf = vec![];
		vfs.open(root_path.join("pack.mcmetac"))
			.or_else(|_| vfs.open(root_path.join("pack.mcmeta")))?
			.file_read
			.read_to_end(&mut manifest_buf)
			.await?;

		Ok(
			pack_type.deserialize_meta(&mut serde_json::Deserializer::from_reader(
				StripComments::new(strip_utf8_bom(&manifest_buf))
			))?
		)
	}

	/// Returns the smallest format version range that encompasses all the format version subranges
	/// targeted by the defined pack layers.
	pub fn bounding_format_version_range(&self) -> RangeInclusive<PackFormatVersion> {
		let (min_format_version, max_format_version) = self
			.layers
			.values()
			.flatten()
			.cloned() // Cheap bitwise copy
			.map(RangeInclusive::into_inner)
			.reduce(|(min_start, max_end), (start, end)| {
				(cmp::min(min_start, start), cmp::max(max_end, end))
			})
			.unwrap_or((PackFormatVersion::MIN, PackFormatVersion::MAX_OR_UNKNOWN));

		min_format_version..=max_format_version
	}
}
