//! Contains the configuration options needed to create a `PackSquasher` run.

use std::num::{NonZeroU8, NonZeroU16, NonZeroU32};
use std::thread::available_parallelism;
use std::{num::NonZeroUsize, path::PathBuf};

use enumset::{EnumSet, EnumSetType};
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sysinfo::{MemoryRefreshKind, RefreshKind, System};

use crate::squash_zip::SquashZipSettings;

/// Contains all the options that configure a `PackSquasher` operation.
///
/// This is the root level configuration struct for PackSquash, so it is a
/// good starting point  to read the API documentation, after the `PackSquasher`
/// struct.
#[derive(Clone, Deserialize)]
pub struct SquashOptions {
	/// The directory where the pack that will be processed resides.
	pub pack_directory: PathBuf,
	#[serde(flatten)]
	/// Global options that tweak how the squash operation works at a pack scale.
	pub global_options: GlobalOptions,
	/// A map that relates glob patterns that match relative file paths within the
	/// pack to file options, to further customize how the files that match the
	/// pattern are processed.
	#[serde(flatten)]
	pub file_options: IndexMap<String, FileOptions>
}

/// An opaque struct that contains validated and processed [`SquashOptions`] ready
/// to be used with `PackSquasher`.
#[derive(Clone)]
pub struct ProcessedSquashOptions {
	pub(super) options: SquashOptions,
	pub(super) file_options_globs: GlobSet
}

impl TryFrom<SquashOptions> for ProcessedSquashOptions {
	type Error = globset::Error;

	fn try_from(squash_options: SquashOptions) -> Result<Self, Self::Error> {
		// Build glob patterns to match file paths with their options
		let mut globset_builder = GlobSetBuilder::new();
		for glob_pattern in squash_options.file_options.keys() {
			globset_builder.add(compile_pack_file_glob_pattern(glob_pattern)?);
		}

		Ok(ProcessedSquashOptions {
			options: squash_options,
			file_options_globs: globset_builder.build()?
		})
	}
}

/// Global options that affect how the entire pack is processed.
///
/// The default values for  these options are meant to be the most reasonable that achieve good
/// compression for  a wide range of use cases without using protection, compression or
/// compressibility-improving techniques that may pose interoperability problems.
#[derive(Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct GlobalOptions {
	/// In some cases, the `pack.png` pack icon won't be shown in Minecraft, even if it
	/// is present. This optimization will skip the `pack.png` entirely, not adding it
	/// to the result ZIP file, which saves some space. This is completely harmless if
	/// Minecraft did not show the icon anyway.
	///
	/// **Default value**: `false`
	pub skip_pack_icon: bool,
	/// This option controls whether the pack metadata file, `pack.mcmeta`, will be validated
	/// or not. Validating this file is a good thing in most circumstances, and you should
	/// only disable this option if you are extremely concerned about performance, need to
	/// add a `pack.mcmeta` file to the generated ZIP file later on, want to use PackSquash
	/// with files that are not a Minecraft pack, or similar reasons.
	///
	/// Even if this option is set to `false`, `pack.mcmeta` may still be validated if
	/// `automatic_minecraft_quirks_detection` or `automatic_asset_types_mask_detection` are
	/// enabled. To guarantee that file is not validated no matter what, both options should be
	/// set to `false`.
	///
	/// **Default value**: `true`
	pub validate_pack_metadata_file: bool,
	/// PackSquash uses a custom ZIP compressor that is able to balance ZIP file
	/// interoperability and specification intent conformance with increased space savings,
	/// compressibility and protection against external programs being able to extract files
	/// from the ZIP. This option lets you choose the tradeoff that is most suitable to your
	/// situation and objectives.
	///
	/// **Default value**: [ZipSpecConformanceLevel::Pedantic]
	pub zip_spec_conformance_level: ZipSpecConformanceLevel,
	/// If `zip_spec_conformance` is set to [ZipSpecConformanceLevel::Disregard], enabling this
	/// option will add extra protections to the ZIP file that will slightly increase its
	/// size. Otherwise, the value of this option will be ignored. This setting does not
	/// affect whether protections that do not increase the file size are added or not.
	///
	/// **Default value**: `false`
	pub size_increasing_zip_obfuscation: bool,
	/// If `zip_spec_conformance` is set to [ZipSpecConformanceLevel::Disregard], this option
	/// sets the approximate probability for each internal ZIP record to be stored in a way that
	/// favors additional discretion of the fact that protection techniques were used, as opposed
	/// to a way that favors increased compressibility of the result ZIP file. Additional
	/// compressibility may be useful if the ZIP file is to be achieved or distributed from a
	/// web server that supports static content compression, for example.
	///
	/// When this setting is 0 (minimum value), every ZIP record will be stored favoring
	/// increased compressibility. When this setting is 100 (maximum value), every ZIP record
	/// will be stored favoring increased discretion instead. Values in the middle combine
	/// increased discretion and compressibility, which may exacerbate both qualities.
	///
	/// This setting has no effect if `zip_spec_conformance` is not set to
	/// [ZipSpecConformanceLevel::Disregard].
	///
	/// **Default value**: `0`
	pub percentage_of_zip_structures_tuned_for_obfuscation_discretion: PercentageInteger,
	/// If `zip_spec_conformance` is set to a value that allows storing the time metadata needed to
	/// reuse the generated ZIP files in future runs, this option controls whether that time metadata
	/// will actually be stored or not. If `true`, the metadata won't be stored no matter what, which
	/// means that the ZIP file won't be able to be reused. Otherwise, if `false`, whether this metadata
	/// is stored or not depends on the value of `zip_spec_conformance`.
	///
	/// You might want to set this to `true` if you are concerned about the presence of encrypted
	/// metadata in the generated ZIP files and don't care about potential speed-ups in later runs.
	/// In fact, if you won't run PackSquash anymore on this pack, for example because you will
	/// distribute it to players after this run, it is recommended to set this to `true`, as this
	/// improves compressibility a bit and removes the now unnecessary metadata.
	///
	/// **Default value**: `false`
	pub never_store_squash_times: bool,
	/// If its value is true, this option instructs PackSquash to try compressing files that
	/// are already compressed by design, like audio and PNG files, before storing them in the
	/// result ZIP file. This can squeeze in some extra savings, at the cost of noticeably increased
	/// processing times.
	///
	/// **Default value**: `false`
	pub recompress_compressed_files: bool,
	/// The number of Zopfli compression iterations that PackSquash will do when compressing a file
	/// of magnitude 1 MiB just before it is stored in the ZIP file. This affects files that are not
	/// compressed by design, or all files if `recompress_compressed_files` is enabled. A higher
	/// number of iterations means that bigger files will be subject to more compression iterations,
	/// which can lead to higher space savings, but is slower. Lower numbers mean that, in general,
	/// less compression iterations will be done, which is quicker, but reduces space savings.
	///
	/// Note that PackSquash calculates the exact number of iterations for a file depending on its
	/// size, so this number of iterations only guides that computation. In particular, PackSquash
	/// targets a reference compression time, so smaller files will be compressed more, and larger
	/// files will be compressed less. Also, the file size is not actually taken into account for
	/// this; what is really used is a non-linear magnitude that grows slower than the file size
	/// (in mathematical terms, the order of the function is that of a fractional power, which is
	/// less than linear). This means that PackSquash will be "hesitant" to reduce the number of
	/// iterations for bigger files to meet the target time, so it will exceed it, but not by too
	/// much.
	///
	/// Unless set to zero, the number of iterations is clamped to the `[1, 20]` interval, so using
	/// values outside that interval for this option will only change the magnitude threshold where
	/// iterations start being reduced to meet a target time.
	///
	/// However, zero is a special case, because no file will be compressed, no matter its size. This
	/// is useful to speed up the process without sacrificing file-specific optimization techniques,
	/// while usually speeding up the loading speed of your pack by Minecraft clients (because they
	/// won't have to decompress any file, which is a bottleneck, especially with the advent of fast
	/// storage devices in these years). The obvious downside is that the generated ZIP files may take
	/// more space, which increases bandwidth requirements. Also, if the decompression speed is much
	/// greater than the storage device speed (i.e. a beefy CPU is paired with a slow HDD, for example),
	/// Minecraft clients may actually take longer to load the pack.
	///
	/// **Default value**: `20`
	pub zip_compression_iterations: u8,
	/// By default, PackSquash will try to automatically deduce an appropriate set of Minecraft quirks
	/// that affect how pack files can be optimized, by looking at the pack files. This automatic
	/// detection works fine in most circumstances, but because quirks affect specific Minecraft
	/// versions, and maybe only under some conditions, it might be inexact.
	///
	/// If you know exactly what quirks affect your pack and do not want PackSquash to come up with its
	/// own set of quirks to work around, set this option to `false`, and configure
	/// `work_around_minecraft_quirks` accordingly. Otherwise, you can set it to `true`.
	///
	/// When this option is set to `true`, the `pack.mcmeta` file may be read and validated, even if
	/// `validate_pack_metadata_file` and `automatic_asset_types_mask_detection` are set to `false`.
	/// To guarantee that file is not read no matter what, these options should be all set to `false`.
	///
	/// **Default value**: `true`
	pub automatic_minecraft_quirks_detection: bool,
	/// Some Minecraft versions have some quirks that affect how pack files can be optimized. If these
	/// quirks are ignored, it may happen that those files are no longer correctly interpreted by the
	/// game. PackSquash can work around these quirks, but doing so may come at the cost of some
	/// drawbacks. Therefore, when `automatic_minecraft_quirks_detection` is set to `true`, the default
	/// value, PackSquash tries to deduce a suitable set of quirks from the pack files, and the value
	/// of this option is ignored. Only if `automatic_minecraft_quirks_detection` is set to `false`
	/// this option will specify the exact set of quirks that will be worked around.
	///
	/// **Default value**: empty set (no quirks worked around, unless
	/// `automatic_minecraft_quirks_detection` is set to `true` and quirks were detected)
	pub work_around_minecraft_quirks: EnumSet<MinecraftQuirk>,
	/// By default, PackSquash will try to automatically deduce the appropriate set of pack files to
	/// include in the generated ZIP by checking what Minecraft versions it targets, according to the
	/// pack format version. This works fine in most circumstances, and saves space if the pack contains
	/// legacy or too new files for the targeted Minecraft version, but it might be not desirable
	/// sometimes.
	///
	/// If you want PackSquash to include every pack file it recognizes and is enabled in `allow_mods`
	/// no matter what, set this option to `false`. Otherwise, leave it set to `true` to let it
	/// exclude files that are known to be not relevant.
	///
	/// When this option is set to `true`, the `pack.mcmeta` file may be read and validated, even if
	/// `validate_pack_metadata_file` and `automatic_minecraft_quirks_detection` are set to `false`.
	/// To guarantee that file is not read no matter what, these options should be all set to `false`.
	///
	/// **Default value**: `true`
	pub automatic_asset_types_mask_detection: bool,
	/// This option controls whether PackSquash will ignore system and hidden files (i.e. whose name
	/// starts with a dot), not even trying to process them. Under most circumstances, you shouldn't
	/// need to disable this option.
	///
	/// **Default value**: `true`
	pub ignore_system_and_hidden_files: bool,
	/// PackSquash supports pack files added by mods, but, in the interest of keeping its output as
	/// lean as possible by default, you should indicate what mods do you want to support and include
	/// in the result ZIP file.
	///
	/// **Default value**: empty set (do not add any mod-specific files)
	#[cfg(any(feature = "optifine", feature = "mtr3"))]
	#[doc(cfg(any(feature = "optifine", feature = "mtr3")))]
	pub allow_mods: EnumSet<MinecraftMod>,
	/// The output file path where the result ZIP will be written to. This path must not point to a
	/// folder.
	///
	/// Depending on how other options are configured, PackSquash may use this ZIP file, if it exists,
	/// to reuse its processed data and speed up squash operations.
	///
	/// **Default value**: `pack.zip` (file `pack.zip` in the current working directory)
	pub output_file_path: PathBuf,
	/// The number of concurrent threads that PackSquash will use to process the resource pack files.
	/// Several threads allow processing several files at once, improving speed substantially. PackSquash
	/// may end up spawning slightly more threads than this for internal reasons.
	///
	/// **Default value**: `number of available physical CPU threads`
	pub threads: NonZeroUsize,
	/// Internally, PackSquash uses spooling buffers to work with the files it processes in-memory as
	/// much as possible. However, doing so naively also limits what PackSquash can do, depending on the
	/// available memory of the environment it is running on. This option sets, **in MiB**, the size that
	/// each of the spooling buffers used will be able to grow up to before being rolled over to disk,
	/// freeing memory and diverting the operations to a temporary file, which is slower.
	///
	/// Currently, PackSquash creates what amounts to one spooling buffer per thread, plus one extra
	/// spooling buffer for the output ZIP file.
	///
	/// **Default value**: `half of the available memory reported by the OS / (number of CPU hardware threads + 1)`
	pub spooling_buffers_size: usize
}

impl Default for GlobalOptions {
	fn default() -> Self {
		let available_memory = System::new_with_specifics(
			RefreshKind::nothing().with_memory(MemoryRefreshKind::nothing().with_ram())
		)
		.available_memory();

		let hardware_threads = available_parallelism().unwrap_or(NonZeroUsize::MIN);

		Self {
			skip_pack_icon: false,
			validate_pack_metadata_file: true,
			zip_spec_conformance_level: Default::default(),
			size_increasing_zip_obfuscation: false,
			percentage_of_zip_structures_tuned_for_obfuscation_discretion: PercentageInteger(0),
			never_store_squash_times: false,
			recompress_compressed_files: false,
			zip_compression_iterations: 20,
			automatic_minecraft_quirks_detection: true,
			work_around_minecraft_quirks: EnumSet::empty(),
			automatic_asset_types_mask_detection: true,
			ignore_system_and_hidden_files: true,
			#[cfg(any(feature = "optifine", feature = "mtr3"))]
			allow_mods: EnumSet::empty(),
			threads: hardware_threads,
			output_file_path: PathBuf::from("pack.zip"),
			// In MiB. By default, half of available memory / (hardware threads + 1 for the output ZIP)
			spooling_buffers_size: (available_memory / 2097152 / (hardware_threads.get() as u64 + 1))
				.try_into()
				.unwrap_or(usize::MAX)
		}
	}
}

impl GlobalOptions {
	/// Returns the [`SquashZipSettings`] contained within these options, which are used to configure
	/// the SquashZip compressor.
	pub(crate) fn as_squash_zip_settings(&self) -> SquashZipSettings {
		SquashZipSettings {
			zopfli_iterations: self.zip_compression_iterations,
			store_squash_time: !self.never_store_squash_times
				&& !matches!(
					self.zip_spec_conformance_level,
					ZipSpecConformanceLevel::Pedantic
				),
			enable_obfuscation: matches!(
				self.zip_spec_conformance_level,
				ZipSpecConformanceLevel::Disregard
			),
			enable_deduplication: matches!(
				self.zip_spec_conformance_level,
				ZipSpecConformanceLevel::Balanced | ZipSpecConformanceLevel::Disregard
			),
			enable_size_increasing_obfuscation: self.size_increasing_zip_obfuscation,
			percentage_of_records_tuned_for_obfuscation_discretion: self
				.percentage_of_zip_structures_tuned_for_obfuscation_discretion,
			workaround_old_java_obfuscation_quirks: self
				.work_around_minecraft_quirks
				.contains(MinecraftQuirk::Java8ZipParsing),
			spool_buffer_size: self.spooling_buffers_size.saturating_mul(1024 * 1024)
		}
	}
}

/// A ZIP specification intent conformance level that a squash operation can adhere to.
#[derive(Default, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ZipSpecConformanceLevel {
	/// The generated ZIP files will follow the ZIP file specification to the letter, so they
	/// can be extracted with virtually any ZIP file manipulation program.
	///
	/// This option prevents a squash operation from doing anything that may render its ZIP
	/// file format unconventional:
	/// - No deduplication: files that are identical after processing and compression
	///   won't be stored only once in the result ZIP file.
	/// - No protection against being opened with ZIP file manipulation tools.
	/// - No compressibility improvements that pose interoperability problems.
	/// - No ability to reuse ZIP files generated in previous runs, because this requires
	///   using metadata in a way that is technically not within the ZIP file specification.
	///
	/// Output ZIP files generated with this conformance level **MUST NOT** be reused in runs
	/// configured to expect and generate output ZIP files with a less strict conformance
	/// level. Reusing them with runs configured with this same conformance level does not have
	/// any effect.
	#[default]
	Pedantic,
	/// Like [ZipSpecConformanceLevel::Pedantic], but stores the metadata needed to reuse this ZIP file
	/// to speed up future runs, and expects this metadata from the specified output ZIP file, if it
	/// already exists.
	///
	/// This conformance level is compatible with most ZIP file manipulation programs, even
	/// though some metadata is out of specification, so any compliant ZIP file manipulation
	/// program is, in theory, free to reject the file or warn about it. They almost universally
	/// don't do this, though.
	///
	/// Output ZIP files generated with this conformance level can only be safely reused with runs
	/// configured to expect and generate output ZIP files with the [ZipSpecConformanceLevel::Balanced]
	/// level and, of course, [ZipSpecConformanceLevel::High] level.
	High,
	/// Like [ZipSpecConformanceLevel::High], but allows deduplicating identical processed files in
	/// the output ZIP file. This yields significant space savings in case the pack contains
	/// repeated assets.
	///
	/// This makes a class of ZIP file manipulation programs choke when trying to extract from
	/// the file, but no additional protections or compressibility improvements are applied.
	///
	/// If your pack does not contain assets that process to the same contents, this conformance
	/// level is equivalent to [ZipSpecConformanceLevel::High].
	///
	/// Output ZIP files generated with this conformance level can only be safely reused with runs
	/// configured to expect and generate output ZIP files with the [ZipSpecConformanceLevel::High]
	/// level and, of course, [ZipSpecConformanceLevel::Balanced] level.
	Balanced,
	/// PackSquash will apply every technique it knows to protect your pack, improve its compressibility
	/// and reduce its size, without any regard that the result ZIP file can be used outside of Minecraft
	/// (in fact, the protection techniques make this harder). This is the complete opposite to
	/// [ZipSpecConformanceLevel::Pedantic].
	///
	/// Currently, the above paragraph means:
	/// - Deduplication is allowed.
	/// - Protections to make it less easy to be used outside of Minecraft are added.
	/// - Compressibility improving techniques that pose interoperability problems, but are accepted by
	///   Minecraft, are used.
	/// - ZIP files generated in previous runs may be reused to speed up the current run.
	///
	/// Output ZIP files generated with this conformance level can only be safely reused with runs
	/// configured to expect and generate output ZIP files with the [ZipSpecConformanceLevel::Disregard]
	/// level.
	Disregard
}

/// A helper struct that contains an integer guaranteed to be in the `[0, 100]` interval.
#[derive(Deserialize, Clone, Copy)]
#[serde(try_from = "u8")]
#[repr(transparent)]
pub struct PercentageInteger(u8);

impl TryFrom<u8> for PercentageInteger {
	type Error = &'static str;

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0..=100 => Ok(PercentageInteger(value)),
			_ => Err("The specified percentage value is not between 0 and 100, inclusive")
		}
	}
}

impl From<PercentageInteger> for u8 {
	fn from(value: PercentageInteger) -> Self {
		value.0
	}
}

/// A Minecraft file parsing quirk that negatively affects the perceived correctness of
/// the generated ZIP files and that can be worked around.
#[derive(Deserialize, Serialize, EnumSetType)]
#[serde(rename_all = "snake_case")]
#[enumset(serialize_deny_unknown, serialize_repr = "list")]
#[non_exhaustive]
pub enum MinecraftQuirk {
	/// Older versions of Minecraft (probably all versions since 1.6 until 1.13 are affected)
	/// assume that grayscale images are in a rather uncommon color space, instead of the more
	/// common sRGB it assumes for color images. Because PackSquash can compress grayscale
	/// color images to actual grayscale format to save space, affected Minecraft versions
	/// display those images with colors that look "washed-out".
	///
	/// This workaround stops PackSquash from reducing color images to grayscale, which may
	/// hurt compression. This has no effect for input images that already are in grayscale.
	GrayscaleImagesGammaMiscorrection,
	/// Older versions of Minecraft (probably all versions from 1.6 until 1.13 are affected)
	/// require banner and shield layer textures to be stored in RGBA format, or else the
	/// layers they represent won't be applied at all, even if the palette contains
	/// transparency data. As PackSquash can convert images encoded in RGBA format to palette
	/// format to save space, it can trigger this quirky behavior.
	///
	/// This workaround stops PackSquash from changing the color format of the affected
	/// textures to a palette, which includes color quantization, as it is used to generate
	/// a palette. This incurs some space costs.
	RestrictiveBannerLayerTextureFormatCheck,
	/// All known Minecraft versions before snapshot 24w39a (1.12.2) overlay entity layer
	/// textures in a way that  does not account for transparency properly, by taking into
	/// account their color and not only their transparency values as blending coefficients
	/// to use for overlying that texture. PackSquash can change the color of transparent
	/// pixels, and as such it can trigger this behavior.
	///
	/// This workaround stops PackSquash from changing the color of transparent pixels and
	/// quantizing the pixels to a palette to reduce texture file size, as both optimizations
	/// do not guarantee that the color of transparent pixels will stay the same.
	BadEntityEyeLayerTextureTransparencyBlending,
	/// The latest Minecraft versions, from 1.17 onwards, are compiled for Java 16+, which
	/// means that they do not support older Java versions. On the other hand, Java 8 was
	/// used almost ubiquitously with older Minecraft clients, especially in modded
	/// environments. However, a lot of things have changed in newer Java versions, including
	/// low-level details of how ZIP files are parsed.
	///
	/// When a ZIP specification conformance level that adds extraction protection is used,
	/// this workaround tells PackSquash to use obfuscation techniques that work fine with
	/// Java 8. This comes at the cost of a protection that is a bit different, but the small
	/// differences will extremely likely not matter in protection strength. Compressibility
	/// can be impacted negatively, though. This quirk does not have any effect if an affected
	/// ZIP specification conformance level is not used, or if the Minecraft client is run
	/// using newer Java versions.
	Java8ZipParsing,
	/// The audio decoding code of a range of Minecraft versions, from 1.14 to 24w14a (1.20.5
	/// snapshot), both inclusive, made possible the PackSquash Ogg Vorbis obfuscation techniques.
	/// Older and newer Minecraft versions do not support these obfuscated files, showing errors
	/// in the console and even freezing the game when they are played.
	///
	/// This workaround ensures that no obfuscation is done to any Ogg Vorbis file generated
	/// by PackSquash, so at least the pack will work. Keep in mind that, due to 1.13 and
	/// 1.14 sharing the same pack format version, the autodetection code for this quirk will
	/// err on the safe side and only consider Minecraft versions starting from 1.15 to be
	/// compatible. Likewise, the quirk will also be applied to Minecraft versions beginning from
	/// 24w13a, two snapshots before the incompatibility was really introduced.
	OggObfuscationIncompatibility,
	/// The PNG decoding code of all known Minecraft versions since resource packs were introduced
	/// until 1.13 did not support the PackSquash PNG obfuscation techniques, causing the game to
	/// show errors when the affected textures were loaded.
	///
	/// This workaround ensures that no obfuscation is done to any PNG file generated by PackSquash,
	/// so at least the pack will work.
	PngObfuscationIncompatibility
}

impl MinecraftQuirk {
	pub(super) const fn as_str(&self) -> &'static str {
		match self {
			Self::GrayscaleImagesGammaMiscorrection => "grayscale_images_gamma_miscorrection",
			Self::RestrictiveBannerLayerTextureFormatCheck => {
				"restrictive_banner_layer_texture_format_check"
			}
			Self::BadEntityEyeLayerTextureTransparencyBlending => {
				"bad_entity_eye_layer_texture_transparency_blending"
			}
			Self::Java8ZipParsing => "java8_zip_parsing",
			Self::OggObfuscationIncompatibility => "ogg_obfuscation_incompatibility",
			Self::PngObfuscationIncompatibility => "png_obfuscation_incompatibility"
		}
	}
}

/// A Minecraft modification supported by PackSquash that adds file types to packs.
#[derive(Deserialize, Serialize, EnumSetType)]
#[enumset(serialize_deny_unknown, serialize_repr = "list")]
#[non_exhaustive]
#[cfg(any(feature = "optifine", feature = "mtr3"))]
#[doc(cfg(any(feature = "optifine", feature = "mtr3")))]
pub enum MinecraftMod {
	/// OptiFine.
	///
	/// Currently, this adds support for the following file types:
	/// - Properties files (`.properties`).
	/// - Custom entity model files (`.jem`, `.jemc`, `.jpm` and `.jpmc`).
	#[serde(rename = "OptiFine")]
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	Optifine,
	/// Minecraft Transit Railway, version 3.0 and compatibles.
	///
	/// Currently, this adds support for the following file types:
	/// - Blockbench modded entity model projects for custom train models (`.bbmodel` and `.bbmodelc`).
	#[serde(rename = "Minecraft Transit Railway 3")]
	#[cfg(feature = "mtr3")]
	#[doc(cfg(feature = "mtr3"))]
	MinecraftTransitRailway3
}

/// Options that customize how some file, of a certain file type, is processed.
// When adding new variants to this enum, please update the lib.rs file too, so
// the default options are used for new file types too
#[derive(Deserialize, Clone, Copy)]
#[serde(
	untagged,
	expecting = "some options did not match the expected global or file-specific options.\n\
	Are you using file-specific options as global options, or migrating from a PackSquash version that recognized other options?"
)]
#[non_exhaustive]
pub enum FileOptions {
	/// The options for transcoding audio files to a more space-efficient format that is
	/// accepted by Minecraft.
	AudioFileOptions(AudioFileOptions),
	/// The options that control how JSON files are optimized.
	JsonFileOptions(JsonFileOptions),
	/// The options that customize how PNG files are optimized to a more space-efficient
	/// representation.
	PngFileOptions(PngFileOptions),
	/// Options that influence how shader files are converted to a more distribution-friendly
	/// representation.
	ShaderFileOptions(ShaderFileOptions),
	/// Options that influence how property files are converted to a more distribution-friendly
	/// representation.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	PropertiesFileOptions(PropertiesFileOptions),
	/// Options that influence how legacy language files are converted to a more
	/// distribution-friendly representation.
	LegacyLanguageFileOptions(LegacyLanguageFileOptions),
	/// Options that influence how command function files are converted to a more
	/// distribution-friendly representation.
	CommandFunctionFileOptions(CommandFunctionFileOptions),
	/// Options that influence how custom files that the user explicitly wants to include in the
	/// pack are processed.
	// For better style, keep this variant last
	CustomFileOptions(CustomFileOptions)
}

impl FileOptions {
	/// Tweaks the value of the crate-private fields that are used to enforce global options
	/// contained in the [`GlobalOptions`] struct.
	///
	/// It is recommended to execute this method just after the default or user provided
	/// file settings for some pack file were found, before actually using them.
	pub(crate) fn tweak_from_global_options(mut self, global_options: &GlobalOptions) -> Self {
		if let FileOptions::PngFileOptions(file_options) = &mut self {
			file_options.working_around_grayscale_reduction_quirk = global_options
				.work_around_minecraft_quirks
				.contains(MinecraftQuirk::GrayscaleImagesGammaMiscorrection);
			file_options.working_around_color_type_change_quirk = global_options
				.work_around_minecraft_quirks
				.contains(MinecraftQuirk::RestrictiveBannerLayerTextureFormatCheck);
			file_options.working_around_transparent_pixel_colors_change_quirk = global_options
				.work_around_minecraft_quirks
				.contains(MinecraftQuirk::BadEntityEyeLayerTextureTransparencyBlending);
			file_options.minecraft_version_supports_png_obfuscation = !global_options
				.work_around_minecraft_quirks
				.contains(MinecraftQuirk::PngObfuscationIncompatibility);
		}

		if let FileOptions::AudioFileOptions(file_options) = &mut self {
			file_options.minecraft_version_supports_ogg_obfuscation = !global_options
				.work_around_minecraft_quirks
				.contains(MinecraftQuirk::OggObfuscationIncompatibility);
		}

		self
	}
}

/// Parameters that influence how an audio file is optimized.
#[derive(Deserialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct AudioFileOptions {
	/// If `true`, input audio files that are already Ogg won't be transcoded again. This preserves
	/// their original quality and improves performance, but may come at a cost in space savings.
	///
	/// **Default value**: `true`
	pub transcode_ogg: bool,
	/// If `true`, an additional fast two-pass optimization and validation step will be performed
	/// on the generated Ogg Vorbis file before it is added to the pack, regardless of whether it
	/// has been transcoded. This enables PackSquash to ensure that the generated file will work
	/// fine in Minecraft, losslessly reduce its size by an average of 5%, and optionally
	/// obfuscate it to thwart its playback outside of Minecraft.
	///
	/// Due to how fast and unobtrusive this step is, it's usually best to leave it enabled. Good
	/// reasons to disable it include troubleshooting and wanting a slightly faster execution
	/// at the cost of missing out on the features described above.
	///
	/// **Default value**: `true`
	pub two_pass_vorbis_optimization_and_validation: bool,
	/// If `true`, empty audio files (i.e., with no audio data, or full of complete silence)
	/// will be replaced with a special empty audio file that is optimized for size and contains
	/// no audio data. This kind of file works fine in Minecraft and most media players, but
	/// some might consider the lack of audio data an error.
	///
	/// This option is only honored if the audio file is being transcoded, which is always the
	/// case when the `transcode_ogg` option is set to `true`.
	///
	/// **Default value**: `true`
	pub empty_audio_optimization: bool,
	/// Sets a number of channels that the audio file will be mixed to. Downmixing stereo sounds
	/// to mono may save a bit of space, and also affects how Minecraft calculates positional
	/// audio effects. On the other hand, it may be desirable to upmix mono sounds to stereo to
	/// get the different positional effects. If `is_positional_audio` is set to `None`, this
	/// channel mixing also influences whether the sound is considered positional or not.
	///
	/// **Default value**: do not downmix or upmix (keep the channels of the input file)
	pub channels: ChannelMixingOption,
	/// The bitrate control mode that will be used for transcoding the audio file. Different bitrate
	/// control modes have different trade-offs between audio quality, file size, bandwidth
	/// predictability and encoding speed.
	///
	/// **Default value**: constant quality factor (CQF)
	pub bitrate_control_mode: AudioBitrateControlMode,
	/// The metric to use as a target for the specified bitrate control mode when trancoding.
	/// Depending on the selected bitrate control mode, this will be interpreted as a quality
	/// factor, average bitrate, approximate bitrate, or maximum bitrate.
	///
	/// **Default value**: `0.25` for stereo audio (interpreted as a quality factor, ≈68 kbit/s for
	/// stereo, 44.1 kHz audio) and `0.0` for mono audio, interpreted as quality factors
	pub target_bitrate_control_metric: Option<f32>,
	/// The sampling frequency that the audio will be resampled to, in Hz. Downsampling helps to save
	/// space, at the cost of potentially introducing aliasing artifacts if the input audio contains
	/// frequencies higher than half the new sampling rate and narrowing margins for filters and
	/// further signal processing work. If the specified sampling frequency is higher than the
	/// sampling frequency of the input audio file, no resampling will be done.
	///
	/// **Default value**: `40050` (40.05 kHz) for stereo audio, `32000` for mono audio
	pub sampling_frequency: Option<NonZeroU32>,
	/// Sets the pitch shift coefficient that will have to be used to play back the sound
	/// at the original pitch. This pitch shift coefficient can be used directly in Minecraft
	/// commands like `/playsound`.
	///
	/// This option is mainly useful to make sound ripping harder. If you're just looking into
	/// saving space by making the audio faster (as it has fewer samples and is shorter) and
	/// then speeding it up in Minecraft, it's better to just change the sampling frequency,
	/// which results in homologous quality and space tradeoffs.
	///
	/// **Default value**: `1.0` (the audio pitch is not shifted)
	pub target_pitch: f32,
	/// If `true`, the generated Ogg Vorbis files will be mangled in a way so that they will be
	/// harder to play outside of Minecraft. The obfuscation technique used is not robust against
	/// some scenarios or expert knowledge, but it does not increase file size.
	///
	/// This option is only taken into account if the two-pass optimization and validation step is
	/// enabled, i.e., the `two_pass_vorbis_optimization_and_validation` option is set to `true`,
	/// and the `ogg_obfuscation_incompatibility` quirk is not being worked around.
	///
	/// **Default value**: `false`
	pub ogg_obfuscation: bool,
	/// Crate-private option set by the [MinecraftQuirk::OggObfuscationIncompatibility]
	/// workaround to not obfuscate Ogg Vorbis files.
	///
	/// **Default value**: `true`
	#[serde(skip)]
	pub(crate) minecraft_version_supports_ogg_obfuscation: bool
}

impl Default for AudioFileOptions {
	fn default() -> Self {
		Self {
			transcode_ogg: true,
			two_pass_vorbis_optimization_and_validation: true,
			empty_audio_optimization: true,
			channels: Default::default(),
			bitrate_control_mode: Default::default(),
			target_bitrate_control_metric: None,
			sampling_frequency: None,
			target_pitch: 1.0,
			ogg_obfuscation: false,
			minecraft_version_supports_ogg_obfuscation: true
		}
	}
}

/// A channel mixing strategy for some audio file, contained in [`AudioFileOptions`].
#[derive(Deserialize, Clone, Copy, Default)]
#[serde(untagged)]
pub enum ChannelMixingOption {
	/// Downmix or upmix the sound channels in the input file to generate an output
	/// file with the specified number of channels.
	///
	/// Currently, only `1` or `2` channels make sense for a resource pack audio
	/// file, as other channel counts are rejected by Minecraft.
	ToChannels(ChannelCount),
	/// Do not change the number or layout of the sound channels of the input file
	/// in any way.
	#[default]
	Skip
}

/// Represents a bitrate control mode that can be used by the PackSquash Vorbis encoder,
/// a modified version of the reference encoder with the aoTuV and Lancer patches applied.
#[derive(Default, Deserialize, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum AudioBitrateControlMode {
	/// *Constant Quality Factor*: the encoder will interpret the target metric as a quality
	/// factor, attempting to keep the perceived subjective quality constant for all the audio
	/// segments. The encoder will not have hard pressures to limit the bitrate in any way,
	/// although the quality metric tends to be strongly correlated with an average bitrate for
	/// typical signals.
	///
	/// Some advantages of this bitrate control mode include:
	/// - It adapts well to different sampling frequencies and channel counts: the encoder knows
	///   that it requires fewer bits to encode mono signals than stereo signals at the same quality
	///   level, for example.
	/// - Unlike with bitrates, it's not possible to ask for too high or low quality levels.
	/// - Easy to encode audio segments are stored in minimal space, with consistent quality: there
	///   is no pressure to meet any average bitrate.
	/// - Performance is significantly higher than when using ABR or CABR modes, because the Vorbis
	///   bitrate management engine logic is skipped.
	///
	/// Some disadvantages of this bitrate control mode include:
	/// - The relationship between quality factor and actual average bitrate is harder to predict.
	/// - There is no hard guarantee against hard to encode segments bumping the average bitrate
	///   up.
	///
	/// The quality factor is expected to be in the [-2, 10] range, where -2 is the worst audio
	/// quality, and 10 is the best.
	#[default]
	Cqf,
	/// *Variable BitRate*: the encoder will interpret the target metric as an approximate bitrate,
	/// internally translating it to a quality factor. Therefore, this mode is equivalent to
	/// [CQF](AudioBitrateControlMode::Cqf), but with the quality factor selected in a different way.
	///
	/// Some advantages of this bitrate control mode over CQF include:
	/// - The relationship between quality factor and actual average bitrate is more predictable.
	///
	/// Some disadvantages of this bitrate control mode over CQF include:
	/// - The same bitrate may yield different quality levels for different audio signals, or be
	///   too high or too low for the possible quality factors.
	///
	/// The bitrate is interpreted in kbit/s.
	Vbr,
	/// **Average BitRate**: the encoder will interpret the target metric as an average bitrate,
	/// and will be coerced to meet that bitrate for the entire audio signal by using a bitrate
	/// management engine. No specific subjective quality level will be targeted.
	///
	/// Some advantages of this bitrate control mode over CQF and VBR include:
	/// - The actual average stream bitrate is guaranteed to very closely meet the specified average
	///   bitrate. Therefore, the resulting file sizes are more predictable.
	/// - The maximum instantaneous bitrate for an audio segment might be higher than average for a
	///   small time window, as long as it doesn't affect the long-term average.
	///
	/// Some disadvantages of this bitrate control mode over CQF and VBR include:
	/// - Setting too low bitrates for the input signal may severely degrade audio quality, while
	///   setting too high bitrates may waste space on padding the data to maintain the average.
	/// - Easy-to-encode audio segments may be stored using more bits than necessary for a certain
	///   quality level in order to meet the average bitrate. CConversely, harder-to-encode segments
	///   may sound worse when the encoder is already outputting at a high bitrate, as it will be
	///   deprived of bits to devote to them. The resulting subjective quality will be more
	///   inconsistent.
	/// - Performance is significantly worse than when using CQF or VBR due to the bitrate
	///   management engine being engaged.
	///
	/// The bitrate is interpreted in kbit/s.
	Abr,
	/// **Constrained Average BitRate**: the encoder will interpret the target metric as a hard
	/// maximum bitrate and internally select a slightly lower average bitrate than the maximum
	/// to meet. This mode is similar to [ABR](AudioBitrateControlMode::Abr), but with the addition
	/// of a maximum bitrate.
	///
	/// Some advantages of this bitrate control mode over ABR include:
	/// - The actual average bitrate is guaranteed to never exceed the specified maximum bitrate,
	///   which limits the maximum file size with certainty.
	///
	/// Some disadvantages of this bitrate control mode over ABR include:
	/// - To ensure that the hard maximum bitrate is never exceeded, a lower average bitrate will
	///   be targeted, which provides headroom for hard to encode segments, but usually results in
	///   inferior quality.
	///
	/// The bitrate is interpreted in kbit/s.
	#[serde(rename = "CABR")]
	ConstrainedAbr
}

/// A helper struct that contains an integer that must be a valid number of
/// audio channels accepted by Minecraft.
#[derive(Deserialize, Clone, Copy)]
#[serde(try_from = "NonZeroU8")]
#[repr(transparent)]
pub struct ChannelCount(NonZeroU8);

impl TryFrom<NonZeroU8> for ChannelCount {
	type Error = &'static str;

	fn try_from(value: NonZeroU8) -> Result<Self, Self::Error> {
		if (1..=2).contains(&value.get()) {
			Ok(ChannelCount(value))
		} else {
			Err("The specified number of channels is not valid")
		}
	}
}

impl From<ChannelCount> for NonZeroU8 {
	fn from(value: ChannelCount) -> Self {
		value.0
	}
}

/// Parameters that influence how a JSON file is optimized.
#[derive(Deserialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct JsonFileOptions {
	/// If `true`, the JSON file will be minified (i.e. unnecessary white space, line breaks
	/// and comments will be removed) to save space and improve parsing performance. If `false`,
	/// the JSON file will be prettified instead, but comments will still be removed.
	///
	/// **Default value**: `true` (minify)
	#[serde(rename = "minify_json")]
	pub minify: bool,
	/// If `true`, PackSquash will delete known-superfluous keys from JSON files, like credits
	/// added by pack authoring tools, that are ignored by Minecraft. If `false`, those keys
	/// will be left alone.
	///
	/// **Default value**: `true` (delete superfluous keys)
	#[serde(rename = "delete_bloat_keys")]
	pub delete_bloat: bool,
	/// If `true`, PackSquash will allow comments in JSON files whose usual extension does not end
	/// with an extra `c` letter, which explicitly marks the file as following an extended JSON
	/// format that can contain comments. If `false`, comments will only be allowed in JSON files
	/// with those specific extensions.
	///
	/// **Default value**: `true` (allow comments in the JSON file, no matter its extension)
	#[serde(rename = "always_allow_json_comments")]
	pub always_allow_comments: bool
}

impl Default for JsonFileOptions {
	fn default() -> Self {
		Self {
			minify: true,
			delete_bloat: true,
			always_allow_comments: true
		}
	}
}

/// Parameters that influence how a PNG file is optimized.
///
/// Note that, in any case, any PNG chunks (e.g. metadata) that are not used by Minecraft
/// to display the image will not be copied over from the original file.
#[derive(Deserialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct PngFileOptions {
	/// The number of Zopfli compression iterations that PackSquash will do to compress raw
	/// pixel data that amounts to a magnitude of 1 MiB. This option is similar to
	/// `zip_compression_iterations`, and is used to feed the same linear model, but with
	/// different parameters better suited for image compression.
	///
	/// When the number of compression iterations drops to zero, which happens when this option
	/// is set to zero or the texture is pretty big, a much faster DEFLATE compression algorithm
	/// is used instead of Zopfli. This extra performance may come at the cost of file
	/// size. On the other side, the number of iterations is limited to a maximum of 15. Values
	/// greater than 15 are still useful for this setting, because they change the threshold
	/// where iterations start being reduced in order to keep acceptable performance levels.
	///
	/// **Default value**: `5`
	pub image_data_compression_iterations: u8,
	/// Controls how the colors of the image will be quantized.
	///
	/// Color quantization is a lossy process if and only if the image contains more colors than
	/// the target bit depth. However, due to the usage of a color palette specific to the image,
	/// the loss of quality is usually not so noticeable. Keep in mind that vanilla-sized textures
	/// usually contain few colors, because they are so small (a 16x16 size means at most 256
	/// different colors).
	///
	/// **Default value**: [ColorQuantizationTarget::EightBitDepth] (quantize to 256 colors)
	pub color_quantization_target: ColorQuantizationTarget,
	/// The level of dithering that will be applied when quantizing colors, between 0 and 1. This
	/// option has no effect if `color_quantization_target` is set to not perform color quantization.
	///
	/// Dithering is a technique that improves the perceived color depth of color-quantized images
	/// by diffusing the color palette in areas that lost color information. The dithered areas
	/// appear more like they had their original colors, reducing color banding artifacts. However,
	/// dithering can introduce noisy, hard-to-compress diffusion patterns.
	///
	/// In most images, especially natural-looking ones, color quantization saves enough space to
	/// compensate for the decreased dithered areas compressibility, so lots of dithering is a good
	/// idea because it both gives better-looking results and reduces file sizes. Extreme
	/// counterexamples are images composed of big blocks of plain colors whose color is not in the
	/// quantized palette: in this case dithering will negatively affect compression while yielding a
	/// worse look than only completely replacing those plain colors by colors in the palette, so
	/// reducing the dithering level is warranted.
	///
	/// **Default value**: `0.85`
	pub color_quantization_dithering_level: UnitIntervalFloat,
	/// The maximum width and height of the images that will be accepted. This parameter
	/// sets a high bound of memory usage by PackSquash and helps to author packs with
	/// reasonable texture sizes.
	///
	/// **Default value**: 8192
	pub maximum_width_and_height: NonZeroU16,
	/// If `true`, this option prevents the color values of completely transparent pixels from being
	/// changed in order to achieve better compression. This optimization is visually lossless,
	/// because completely transparent pixels are invisible no matter their color, and does not
	/// affect images that lack an alpha channel. However, if the image is meant to be edited
	/// further or contains steganographic data, this optimization may have undesirable side
	/// effects. Disabling alpha optimizations also reduces the time it takes to optimize an
	/// image, at the cost of a maybe increased file size.
	///
	/// **Default value**: `false`
	pub skip_alpha_optimizations: bool,
	/// If `true`, single-color textures that are estimated to be safe to resize will be downsized
	/// to the minimum resolution that maintains the current maximum mipmap level. This can provide
	/// significant space savings for this kind of textures, but in some edge cases (using
	/// modifications, dealing with animated textures or fonts) breakage may occur. If you are
	/// setting this to `false` to work around such problems, please let us know.
	///
	/// **Default value**: `false`
	pub downsize_if_single_color: bool,
	/// Controls whether PackSquash should assume that this texture may be stitched by the game as
	/// a part of an internal or custom atlas that is directory-listed. For performance reasons,
	/// Minecraft stitches most game textures into atlases, including those of item and block models,
	/// but there are a few exceptions, such as main menu panorama textures, or those sampled
	/// directly from shaders.
	///
	/// Currently, this option only affects how PackSquash protects images when
	/// [`size_increasing_zip_obfuscation`](GlobalOptions::size_increasing_zip_obfuscation) is in
	/// effect. `true` is a safe default value that causes textures to be less protected. Setting
	/// it to `false` for better protection is, however, only recommended if you have detailed
	/// knowledge of how the game stitches textures and are willing to test the correctness of this
	/// assumption on a case-by-case basis.
	///
	/// This option may be changed in the future to have more side effects. It may also be removed,
	/// depending on how PackSquash improves its atlas texture detection capabilities.
	///
	/// **Default value**: `true`
	pub may_be_directory_listed_atlas_sprite: bool,
	/// If `true`, the generated PNG files will be mangled in a way so that they will be harder to
	/// view outside of Minecraft. The obfuscation technique used is not robust against some
	/// scenarios or expert knowledge, but it does not increase file size.
	///
	/// **Default value**: `false`
	pub png_obfuscation: bool,
	/// Crate-private option set by the [MinecraftQuirk::GrayscaleImagesGammaMiscorrection]
	/// workaround to not reduce color images to grayscale.
	///
	/// **Default value**: `false`
	#[serde(skip)]
	pub(crate) working_around_grayscale_reduction_quirk: bool,
	/// Crate-private option set by the [MinecraftQuirk::RestrictiveBannerLayerTextureFormatCheck]
	/// workaround to not change the texture color type.
	///
	/// **Default value**: `false`
	#[serde(skip)]
	pub(crate) working_around_color_type_change_quirk: bool,
	/// Crate-private option set by the [MinecraftQuirk::BadEntityEyeLayerTextureTransparencyBlending]
	/// workaround to not change the texture color type.
	///
	/// **Default value**: `false`
	#[serde(skip)]
	pub(crate) working_around_transparent_pixel_colors_change_quirk: bool,
	/// Crate-private option set by the [MinecraftQuirk::PngObfuscationIncompatibility]
	/// workaround to not obfuscate PNG files.
	///
	/// **Default value**: `true`
	#[serde(skip)]
	pub(crate) minecraft_version_supports_png_obfuscation: bool
}

impl Default for PngFileOptions {
	fn default() -> Self {
		Self {
			image_data_compression_iterations: 5,
			color_quantization_target: Default::default(),
			color_quantization_dithering_level: UnitIntervalFloat(0.85),
			maximum_width_and_height: NonZeroU16::new(8192).unwrap(),
			skip_alpha_optimizations: false,
			downsize_if_single_color: false,
			may_be_directory_listed_atlas_sprite: true,
			png_obfuscation: false,
			working_around_grayscale_reduction_quirk: false,
			working_around_color_type_change_quirk: false,
			working_around_transparent_pixel_colors_change_quirk: false,
			minecraft_version_supports_png_obfuscation: true
		}
	}
}

/// Possible targets the colors of a PNG file will be quantized to.
#[derive(Default, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ColorQuantizationTarget {
	/// No quantization will be done. The image will be losslessly compressed.
	None,
	/// The image will be quantized to at most 2 colors.
	OneBitDepth,
	/// The image will be quantized to at most 4 colors.
	TwoBitDepth,
	/// The image will be quantized to at most 16 colors.
	FourBitDepth,
	/// The image will be quantized to at most 256 colors.
	EightBitDepth,
	/// The image will be quantized to at most 256 colors if and
	/// only if doing so saves space.
	#[default]
	Auto
}

impl ColorQuantizationTarget {
	/// Gets the bit depth of this color quantization target.
	pub(crate) const fn depth(&self) -> u8 {
		match self {
			Self::None => 0,
			Self::OneBitDepth => 1,
			Self::TwoBitDepth => 2,
			Self::FourBitDepth => 4,
			Self::EightBitDepth => 8,
			Self::Auto => 8
		}
	}

	/// Gets the maximum number of colors that an image with this color
	/// quantization target can possibly have.
	pub(crate) const fn max_colors(&self) -> u32 {
		1 << self.depth()
	}

	/// Returns whether color quantization should be performed at all
	/// to meet this color quantization target.
	pub(crate) const fn should_quantize(&self) -> bool {
		self.depth() > 0
	}

	/// Returns whether this quantization target requires quantization to
	/// be performed, even if it negatively impacts file size.
	pub(crate) const fn is_quantization_required(&self) -> bool {
		self.should_quantize() && !matches!(self, Self::Auto)
	}
}

/// A helper struct that contains an 32-bit floating point number guaranteed to be
/// in the `[0, 1]` interval.
#[derive(Deserialize, Clone, Copy)]
#[serde(try_from = "f32")]
#[repr(transparent)]
pub struct UnitIntervalFloat(f32);

impl TryFrom<f32> for UnitIntervalFloat {
	type Error = &'static str;

	fn try_from(value: f32) -> Result<Self, Self::Error> {
		(0.0..=1.0)
			.contains(&value)
			.then_some(UnitIntervalFloat(value))
			.ok_or("The specified value is not a decimal number between 0 and 1, inclusive")
	}
}

impl From<UnitIntervalFloat> for f32 {
	fn from(value: UnitIntervalFloat) -> Self {
		value.0
	}
}

/// Parameters that influence how a shader file is optimized.
#[derive(Deserialize, Clone, Copy, Default)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct ShaderFileOptions {
	/// Defines how the GLSL source code of this shader will be transformed to a more optimized
	/// but semantically equivalent representation.
	///
	/// **Default value**: [`ShaderSourceTransformationStrategy::Minify`]
	#[serde(rename = "shader_source_transformation_strategy")]
	pub source_transformation_strategy: ShaderSourceTransformationStrategy,
	/// If `true`, PackSquash will consider this shader to be a top-level one (i.e., parsed
	/// by Minecraft as a translation unit and never included in other shaders). When `false`,
	/// this shader will not be considered top-level.
	///
	/// The value of this option is only honored for vertex and fragment shaders. By definition,
	/// include shaders never are top-level, so they can't be marked as top-level via this option.
	///
	/// In the vast majority of scenarios, vertex and fragment shaders are not meant to be
	/// included in other shaders (i.e., they are top-level). However, Minecraft technically allows
	/// such inclusion to happen, and it is exceedingly difficult for PackSquash to determine whether
	/// a vertex or fragment shader is being included by other shader. PackSquash needs to know
	/// whether a shader is top-level to decide whether it is appropriate to expand and remove
	/// preprocessor directives: if it is not top-level but PackSquash thinks it is, removing
	/// preprocessor directives may change the preprocessor behavior in the top-level shader,
	/// yielding potentially different GLSL source code.
	///
	/// It is strongly recommended to only explicitly set this option to `false` for the exceedingly
	/// rare cases where the default behavior generates semantically altered GLSL code, as doing so
	/// will make it harder for PackSquash to optimize the affected shaders. Otherwise, omitting
	/// it is the most appropriate course of action.
	///
	/// **Default value**: `None` (`true` for every vertex and fragment shader)
	pub is_top_level_shader: Option<bool>
}

/// A strategy that may be used to transform GLSL shader source code.
///
/// Please note that PackSquash may not be able to transform some shaders due to limitations
/// in its GLSL parsing code. When this happens, the optimization strategy text for affected
/// shaders will highlight that situation. These limitations might be removed in the future,
/// rendering PackSquash capable of transforming more shaders according to the selected
/// strategy.
#[derive(Deserialize, Copy, Clone, Default)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ShaderSourceTransformationStrategy {
	/// Attempt to minify the shader (i.e., remove unnecessary whitespace, line breaks,
	/// comments and preprocessor directives) to save space and improve parsing performance.
	#[default]
	Minify,
	/// Attempt to prettify the shader (i.e., print it in an indented, human-readable form),
	/// while expanding and removing preprocessor directives and comments.
	Prettify,
	/// Add the source code as-is to the generated ZIP file, without removing or expanding
	/// preprocessor directives.
	///
	/// PackSquash will automatically fall back to this strategy when it is not possible to
	/// execute other strategies for a given shader.
	KeepAsIs
}

/// Parameters that influence how a legacy language file is optimized.
#[derive(Deserialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct LegacyLanguageFileOptions {
	/// If `true`, the legacy language file will be minified: empty lines and comments will be
	/// removed. This saves space and improves parsing performance. If `false`, the file will
	/// still be validated for errors, but left as-is. Line endings are normalized to Unix style
	/// (using a single LF character) no matter what.
	///
	/// **Default value**: `true` (minify)
	#[serde(rename = "minify_legacy_language")]
	pub minify: bool,
	/// If `true`, the BOM in the first line of the file will be stripped. This normally saves
	/// space and avoids user confusion, as the BOM is normally introduced inadvertently, and
	/// Minecraft interprets the BOM as being part of the line. Therefore, the BOM may undesirably
	/// become part of the key of the first language string, causing it not to work, or prevent
	/// a comment from being parsed as such.
	///
	/// However, if a pack relies on the BOM being there because it refers to the first language
	/// string with a BOM, this behavior might have undesirable consequences. In such cases, set
	/// this option to `false` to leave the BOM alone.
	///
	/// **Default value**: `true` (strip the BOM from the first line of the file)
	#[serde(rename = "strip_legacy_language_bom")]
	pub strip_bom: bool
}

impl Default for LegacyLanguageFileOptions {
	fn default() -> Self {
		Self {
			minify: true,
			strip_bom: true
		}
	}
}

/// Parameters that influence how a command function file is optimized.
#[derive(Deserialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct CommandFunctionFileOptions {
	/// If `true`, the command function file will be minified: empty lines and comments will be
	/// removed. This saves space and improves parsing performance. If `false`, the file will
	/// still be validated for errors, but left as-is. Line endings are normalized to Unix style
	/// (using a single LF character) no matter what.
	///
	/// **Default value**: `true` (minify)
	#[serde(rename = "minify_command_function")]
	pub minify: bool
}

impl Default for CommandFunctionFileOptions {
	fn default() -> Self {
		Self { minify: true }
	}
}

/// Parameters that influence how a properties file is optimized.
///
/// These files are only supported if PackSquash was compiled with OptiFine mod support. Otherwise,
/// these parameters are read and parsed but ignored afterward.
#[derive(Deserialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
#[cfg(feature = "optifine")]
#[doc(cfg(feature = "optifine"))]
pub struct PropertiesFileOptions {
	/// If `true`, the properties file will be minified (i.e. unnecessary white space, line breaks
	/// and comments will be removed) to save space and improve parsing performance. If `false`,
	/// the properties file will still be validated for errors, but left as-is.
	///
	/// **Default value**: `true` (minify)
	#[serde(rename = "minify_properties")]
	pub minify: bool
}

#[cfg(feature = "optifine")]
impl Default for PropertiesFileOptions {
	fn default() -> Self {
		Self { minify: true }
	}
}

/// Parameters that define a custom pack file, which PackSquash doesn't expect
/// and skips by default, but that the pack author desires to put in the
/// generated ZIP file.
#[derive(Deserialize, Clone, Copy)]
#[serde(deny_unknown_fields)]
#[non_exhaustive]
pub struct CustomFileOptions {
	/// If `true`, the custom file will be copied to the generated ZIP file as-is,
	/// without any specific optimizations. A `false` value explicitly asks for
	/// the default behavior of skipping the file.
	pub force_include: bool
}

/// Compiles the specified glob pattern to a matcher that is ready to consume
/// any relative pack file path, preventing `*` and `?` from matching path
/// separators, considering `/` as the path separator independently of the
/// current platform, and enabling backslash escaping. An error will be
/// returned if the provided glob pattern is invalid.
pub(crate) fn compile_pack_file_glob_pattern(glob_pattern: &str) -> Result<Glob, globset::Error> {
	GlobBuilder::new(glob_pattern)
		.literal_separator(true)
		.backslash_escape(true)
		.build()
}
