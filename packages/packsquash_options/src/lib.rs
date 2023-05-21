//! Contains the configuration options for PackSquash.

#![feature(doc_cfg)]

mod minecraft_version;

use std::{
	borrow::Cow,
	fmt,
	fmt::{Display, Formatter},
	fs, io,
	io::ErrorKind,
	num::{NonZeroU32, NonZeroU8, NonZeroUsize},
	ops::Deref,
	thread::available_parallelism
};

use ahash::AHashMap;
use camino::Utf8Path;
use const_default::ConstDefault;
use enumset::{EnumSet, EnumSetType};
use field_types::FieldName;
use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};
pub use minecraft_version::{MinecraftVersion, MinecraftVersionRange};
use schemars::{
	gen::SchemaGenerator,
	schema::{Schema, SchemaObject},
	JsonSchema
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sysinfo::{RefreshKind, System, SystemExt};

macro_rules! file_options_default_impl {
	( $( $ty:ty => $imp:expr ),+ ) => {
		$(impl Default for $ty {
			fn default() -> Self {
				$imp
			}
		}

		impl<'a> Default for &'a $ty {
			fn default() -> Self {
				static DEFAULT: $ty = $imp;
				&DEFAULT
			}
		})+
	};
}

macro_rules! default_variant {
	($enum_ty:ident :: $variant:ident) => {
		impl ConstDefault for $enum_ty {
			const DEFAULT: Self = Self::$variant;
		}

		impl Default for $enum_ty {
			fn default() -> Self {
				<Self as ConstDefault>::DEFAULT
			}
		}
	};
}

/// Contains all the options that configure a pack processing operation. This is the
/// root level configuration struct for PackSquash.
#[derive(Clone, Deserialize, JsonSchema)]
#[cfg_attr(feature = "serializable-squash-options", derive(Serialize))]
#[schemars(deny_unknown_fields)]
pub struct SquashOptions<'data> {
	/// The directory where the pack that will be processed resides.
	// This option is handled in a special way by packsquash_gui.
	// Check out the packSquashOptions.tsx component for more details
	#[serde(borrow)]
	pub pack_directory: ExistingDirectoryPath<'data>,
	/// Global options that tweak how the operation is done at a pack scale.
	#[serde(flatten, borrow)]
	pub global_options: GlobalOptions<'data>,
	/// A map that relates glob patterns that match relative file paths within the
	/// pack to file options, to further customize how the files that match the
	/// pattern are processed.
	#[serde(flatten, borrow)]
	#[schemars(with = "::std::collections::HashMap<String, FileOptions>")]
	pub file_options: FileOptionsMap<'data>
}

#[derive(Clone, Deserialize, Serialize, JsonSchema)]
#[serde(try_from = "Cow<'_, Utf8Path>")]
#[repr(transparent)]
pub struct ExistingDirectoryPath<'data>(
	#[schemars(schema_with = "with_directory_path_format")]
	#[serde(borrow)]
	Cow<'data, Utf8Path>
);

impl<'data> TryFrom<Cow<'data, Utf8Path>> for ExistingDirectoryPath<'data> {
	type Error = io::Error;

	fn try_from(value: Cow<'data, Utf8Path>) -> Result<Self, Self::Error> {
		if fs::metadata(&*value)?.is_dir() {
			Ok(Self(value))
		} else {
			Err(io::Error::new(
				ErrorKind::Other,
				"Expected a path to an existing directory, not a file or non-existent directory"
			))
		}
	}
}

impl Deref for ExistingDirectoryPath<'_> {
	type Target = Utf8Path;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl AsRef<Utf8Path> for ExistingDirectoryPath<'_> {
	fn as_ref(&self) -> &Utf8Path {
		&self.0
	}
}

#[derive(Clone, Deserialize)]
#[serde(try_from = "AHashMap<Cow<'_, str>, FileOptions>")]
pub struct FileOptionsMap<'data> {
	globs: GlobSet,
	#[cfg(feature = "serializable-squash-options")]
	#[serde(borrow)]
	file_options_list: Vec<(Cow<'data, str>, FileOptions)>,
	#[cfg(not(feature = "serializable-squash-options"))]
	file_options_list: Vec<FileOptions>,
	#[cfg(not(feature = "serializable-squash-options"))]
	#[serde(borrow)]
	lifetime_marker: std::marker::PhantomData<&'data ()>
}

impl FileOptionsMap<'_> {
	pub fn get<P: AsRef<Utf8Path>>(&self, path: P) -> impl Iterator<Item = &FileOptions> {
		let iter = self.globs.matches(path.as_ref()).into_iter();

		#[cfg(feature = "serializable-squash-options")]
		{
			iter.map(|i| &self.file_options_list[i].1)
		}
		#[cfg(not(feature = "serializable-squash-options"))]
		{
			iter.map(|i| &self.file_options_list[i])
		}
	}
}

impl<'data> TryFrom<AHashMap<Cow<'data, str>, FileOptions>> for FileOptionsMap<'data> {
	type Error = globset::Error;

	fn try_from(
		file_options_map: AHashMap<Cow<'data, str>, FileOptions>
	) -> Result<Self, Self::Error> {
		let mut file_options_list = Vec::with_capacity(file_options_map.len());

		// Build glob patterns to match file paths with their options
		let mut globset_builder = GlobSetBuilder::new();
		for (glob_pattern, file_options) in file_options_map {
			globset_builder.add(compile_pack_file_glob_pattern(&glob_pattern)?);

			#[cfg(feature = "serializable-squash-options")]
			{
				file_options_list.push((glob_pattern, file_options));
			}
			#[cfg(not(feature = "serializable-squash-options"))]
			{
				file_options_list.push(file_options);
			}
		}

		Ok(Self {
			globs: globset_builder.build()?,
			file_options_list,
			#[cfg(not(feature = "serializable-squash-options"))]
			lifetime_marker: std::marker::PhantomData
		})
	}
}

#[cfg(feature = "serializable-squash-options")]
impl Serialize for FileOptionsMap<'_> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		use serde::ser::SerializeMap;

		let mut map = serializer.serialize_map(Some(self.file_options_list.len()))?;

		for (glob_pattern, file_options) in &self.file_options_list {
			map.serialize_entry(glob_pattern, file_options)?;
		}

		map.end()
	}
}

/// Global options that affect how the entire pack is processed. The default values for
/// these options are meant to be the most reasonable that achieve good compression for
/// a wide range of use cases without using protection, compression or compressibility-improving
/// techniques that may pose interoperability problems.
#[serde_as]
#[derive(Clone, Deserialize, Serialize, JsonSchema, FieldName)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct GlobalOptions<'data> {
	#[serde_as(as = "Option<serde_with::DisplayFromStr>")]
	#[schemars(with = "Option<String>")]
	#[schemars(regex(pattern = r"^[1-9][0-9]*\.[1-9][0-9]*(?:\.[1-9][0-9]*)?$"))]
	pub target_minecraft_version: Option<MinecraftVersion>,
	/// In some cases, the `pack.png` pack icon won't be shown in Minecraft, even if it
	/// is present. This optimization will skip the `pack.png` entirely, not adding it
	/// to the result ZIP file, which saves some space. This is completely harmless if
	/// Minecraft did not show the icon anyway.
	///
	/// **Default value**: `false`
	pub skip_pack_icon: bool,
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
	/// metadata in the generated ZIP files and don't care about potential speed ups in later runs.
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
	// This option is handled in a special way by packsquash_gui.
	// Check out the packSquashOptions.tsx component for more details
	pub treat_asset_warnings_as_errors: bool,
	// TODO turn to file-specific option
	pub missing_reference_action: MissingReferenceAction,
	pub accept_references_to_unknown_packs_and_mods: bool,
	pub always_allow_json_comments: bool,
	/// Some Minecraft versions have some quirks that affect how pack files can be optimized. If these
	/// quirks are ignored, it may happen that those files are no longer correctly interpreted by the
	/// game. PackSquash can work around these quirks, but doing so may come at the cost of some
	/// drawbacks. Therefore, this option is not set explicitly, PackSquash tries to deduce a suitable
	/// set of quirks from the pack files, and the value of this option is ignored. Setting
	/// `work_around_minecraft_quirks` overrides the set of quirks that will be worked around,
	/// preventing its automatic detection by PackSquash.
	///
	/// **Default value**: `None` (detect the set of quirks to work around automatically)
	pub work_around_minecraft_quirks: Option<EnumSet<MinecraftQuirk>>,
	/// PackSquash supports pack files added by mods, but, in the interest of keeping its output as
	/// lean as possible by default, you should indicate what mods do you want to support and include
	/// in the result ZIP file.
	///
	/// **Default value**: empty set (do not add any mod-specific files)
	#[cfg(any(feature = "optifine-support", feature = "mtr3-support"))]
	#[doc(cfg(any(feature = "optifine-support", feature = "mtr3-support")))]
	pub allow_mods: EnumSet<MinecraftMod>,
	pub process_not_self_referenced_and_non_vanilla_assets: bool,
	/// The bitrate control mode that will be used for transcoding audio files that do not explicitly
	/// override the bitrate control mode via file-specific options. Different bitrate control modes
	/// have different trade-offs between audio quality, file size, bandwidth predictability and
	/// encoding speed.
	///
	/// Changing the bitrate control mode usually requires changing its target metrics, too. See the
	/// `non_positional_audio_target_bitrate_control_metric` and
	/// `positional_audio_target_bitrate_control_metric` options for details.
	///
	/// **Default value**: constant quality factor (CQF)
	pub audio_bitrate_control_mode: AudioBitrateControlMode,
	/// The metric that will be used as a target for the specified bitrate control mode. Depending
	/// on the selected bitrate control mode, this will be interpreted as a quality factor, average
	/// bitrate, approximate bitrate, or maximum bitrate.
	///
	/// This metric will be used for non-positional (i.e., stereo) sounds only. For mono sounds,
	/// please check out the `positional_audio_target_bitrate_control_metric` option. The
	/// rationale of using different metrics for each sound type is that positional sounds tend to
	/// be shorter and combined with others, so players are expected to be less sensitive to them
	/// being encoded with a slightly lower quality, which helps saving space.
	///
	/// **Default value**: `0.25` (interpreted as a quality factor, ≈68 kbit/s for stereo, 44.1 kHz
	/// audio)
	pub non_positional_audio_target_bitrate_control_metric: f32,
	/// The sampling frequency that non-positional (i.e., stereo) audio will be resampled to.
	/// Downsampling helps saving space, at the cost of potentially introducing aliasing artifacts
	/// if the input audio contains frequencies higher than half the new sampling rate and narrowing
	/// margins for filters and further signal processing work. If the specified sampling frequency
	/// is higher than the sampling frequency of the input audio file, no resampling will be done.
	///
	/// This metric is used for non-positional sounds only. For mono sounds, please check out the
	/// `positional_audio_sampling_frequency` option.
	///
	/// **Default value**: `40050` (40.05 kHz)
	pub non_positional_audio_sampling_frequency: NonZeroU32,
	/// The metric that will be used as a target for the specified bitrate control mode. Depending
	/// on the selected bitrate control mode, this will be interpreted as a quality factor, average
	/// bitrate, approximate bitrate, or maximum bitrate.
	///
	/// This metric will be used for positional (i.e., mono) sounds only. For stereo sounds,
	/// please check out the `non_positional_audio_target_bitrate_control_metric` option. The
	/// rationale of using different metrics for each sound type is that positional sounds tend to
	/// be shorter and combined with others, so players are expected to be less sensitive to them
	/// being encoded with a slightly lower quality, which helps saving space.
	///
	/// **Default value**: `0.0` (interpreted as a quality factor)
	pub positional_audio_target_bitrate_control_metric: f32,
	/// The sampling frequency that positional (i.e., mono) audio will be resampled to.
	/// Downsampling helps saving space, at the cost of potentially introducing aliasing artifacts
	/// if the input audio contains frequencies higher than half the new sampling rate and narrowing
	/// margins for filters and further signal processing work. If the specified sampling frequency
	/// is higher than the sampling frequency of the input audio file, no resampling will be done.
	///
	/// This metric is used for positional sounds only. For stereo sounds, please check out the
	/// `non_positional_audio_sampling_frequency` option.
	///
	/// **Default value**: `32000` (32 kHz)
	pub positional_audio_sampling_frequency: NonZeroU32,
	/// The output file path where the result ZIP will be written to. This path must not point to a
	/// folder.
	///
	/// Depending on how other options are configured, PackSquash may use this ZIP file, if it exists,
	/// to reuse its processed data and speed up squash operations.
	///
	/// **Default value**: `pack.zip` (file `pack.zip` in the current working directory)
	// This option is handled in a special way by packsquash_gui.
	// Check out the packSquashOptions.tsx component for more details
	#[serde(borrow)]
	pub output_file_path: MaybeNonexistentFilePath<'data>,
	/// The number of concurrent threads that PackSquash will use to process the pack. Several
	/// threads allow processing more stuff at once, improving speed substantially. PackSquash
	/// may end up spawning more threads than this for internal reasons.
	///
	/// **Default value**: `number of available physical CPU threads`
	pub threads: NonZeroUsize,
	/// Internally, PackSquash uses buffers to work with the files it processes in-memory as much as
	/// possible. However, doing so naively also limits its scalability to bigger packs, depending on the
	/// available memory of the environment it is running on. This option sets the total size **in MiB**
	/// those buffers will be able to grow up to before being rolling over their contents disk, freeing
	/// memory and diverting the operations to temporary files, which is slower.
	///
	/// **Default value**: `half of the available memory reported by the OS`
	pub maximum_scratch_files_buffers_size: usize
}

impl Default for GlobalOptions<'_> {
	fn default() -> Self {
		let available_memory =
			System::new_with_specifics(RefreshKind::new().with_memory()).available_memory();

		Self {
			target_minecraft_version: None,
			skip_pack_icon: false,
			zip_spec_conformance_level: ZipSpecConformanceLevel::DEFAULT,
			size_increasing_zip_obfuscation: false,
			percentage_of_zip_structures_tuned_for_obfuscation_discretion: PercentageInteger(0),
			never_store_squash_times: false,
			recompress_compressed_files: false,
			zip_compression_iterations: 20,
			treat_asset_warnings_as_errors: false,
			missing_reference_action: MissingReferenceAction::DEFAULT,
			// TODO default to true
			accept_references_to_unknown_packs_and_mods: false,
			always_allow_json_comments: true,
			work_around_minecraft_quirks: None,
			#[cfg(any(feature = "optifine-support", feature = "mtr3-support"))]
			allow_mods: EnumSet::empty(),
			// TODO default to true
			process_not_self_referenced_and_non_vanilla_assets: false,
			audio_bitrate_control_mode: Default::default(),
			non_positional_audio_target_bitrate_control_metric: 0.25, // ~ 68 kbit/s for stereo, 44.1 kHz signals
			non_positional_audio_sampling_frequency: NonZeroU32::new(40_050).unwrap(),
			positional_audio_target_bitrate_control_metric: 0.0,
			positional_audio_sampling_frequency: NonZeroU32::new(32_000).unwrap(),
			threads: available_parallelism().unwrap_or(NonZeroUsize::MIN),
			output_file_path: MaybeNonexistentFilePath(Utf8Path::new("pack.zip").into()),
			// In MiB. By default, half of available memory
			maximum_scratch_files_buffers_size: (available_memory / 2097152)
				// usize is defined to be able to represent any location in memory, so the conversion
				// seems infallible. However, the available system memory may be greater than the
				// virtual memory space given to the process by the OS. This can happen on 32-bit
				// targets with PAE and more than 4 GiB of available memory, and 64-bit targets that
				// run a 32-bit executable
				.try_into()
				.unwrap_or(0)
		}
	}
}

// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(try_from = "Cow<'_, Utf8Path>")]
#[repr(transparent)]
pub struct MaybeNonexistentFilePath<'data>(
	#[schemars(schema_with = "with_maybe_nonexistent_file_path_format")]
	#[serde(borrow)]
	Cow<'data, Utf8Path>
);

impl<'data> TryFrom<Cow<'data, Utf8Path>> for MaybeNonexistentFilePath<'data> {
	type Error = io::Error;

	fn try_from(value: Cow<'data, Utf8Path>) -> Result<Self, Self::Error> {
		match fs::metadata(&*value) {
			// Consider everything that causes I/O errors or is not a directory as a file.
			// This allows non-regular files to slip through (named pipes, devices...)
			Ok(metadata) if !metadata.is_dir() => Ok(Self(value)),
			Ok(_) => Err(io::Error::new(
				ErrorKind::Other,
				"Expected a path to a file, not a directory"
			)),
			Err(err) if err.kind() == ErrorKind::NotFound => Ok(Self(value)),
			Err(err) => Err(err)
		}
	}
}

impl Deref for MaybeNonexistentFilePath<'_> {
	type Target = Utf8Path;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl AsRef<Utf8Path> for MaybeNonexistentFilePath<'_> {
	fn as_ref(&self) -> &Utf8Path {
		&self.0
	}
}

/// A ZIP specification intent conformance level that a squash operation can adhere to.
// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Clone, Copy, Deserialize, JsonSchema, Serialize)]
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

default_variant!(ZipSpecConformanceLevel::Pedantic);

// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Clone, Copy, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MissingReferenceAction {
	Ignore,
	Warn,
	ErrorOut
}

default_variant!(MissingReferenceAction::Warn);

/// A helper struct that contains an integer guaranteed to be in the `[0, 100]` interval.
// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Deserialize, JsonSchema, Serialize, Clone, Copy)]
#[serde(try_from = "u8")]
#[repr(transparent)]
pub struct PercentageInteger(#[schemars(range(min = 0, max = 100))] u8);

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
#[derive(Deserialize, Serialize, EnumSetType, JsonSchema)]
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
	/// All currently known Minecraft versions overlay entity layer textures in a way that
	/// does not account for transparency properly, by taking into account their color and
	/// not only their transparency values as blending coefficients to use for overlying
	/// that texture. PackSquash can change the color of transparent pixels, and as such it
	/// can trigger this behavior.
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
	/// The audio decoding code of the latest Minecraft versions, from 1.14 onwards, was
	/// refactored in such a way that the PackSquash Ogg Vorbis obfuscation techniques were
	/// made possible. Older Minecraft versions do not support these obfuscated files, showing
	/// errors in the console and freezing the game when they are played.
	///
	/// This workaround ensures that no obfuscation is done to any Ogg Vorbis file generated
	/// by PackSquash, so at least the pack will work. Keep in mind that, due to 1.13 and
	/// 1.14 sharing the same pack format version, the autodetection code for this quirk will
	/// err on the safe side and only consider Minecraft versions starting from 1.15 to be
	/// compatible.
	OggObfuscationIncompatibility
}

impl Display for MinecraftQuirk {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		// End-users should appreciate quirk strings being displayed with
		// the same casing rules used for serialization
		self.serialize(f)
	}
}

/// A Minecraft modification supported by PackSquash that adds file types to packs.
#[derive(Deserialize, Serialize, EnumSetType, JsonSchema)]
#[enumset(serialize_deny_unknown, serialize_repr = "list")]
#[non_exhaustive]
#[cfg(any(feature = "optifine-support", feature = "mtr3-support"))]
#[doc(cfg(any(feature = "optifine-support", feature = "mtr3-support")))]
pub enum MinecraftMod {
	/// OptiFine.
	///
	/// Currently, this adds support for the following file types:
	/// - Properties files (`.properties`).
	/// - Custom entity model files (`.jem`, and `.jpm`).
	#[serde(rename = "OptiFine")]
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	Optifine,
	/// Minecraft Transit Railway, version 3.0 and compatibles.
	///
	/// Currently, this adds support for the following file types:
	/// - Blockbench modded entity model projects for custom train models (`.bbmodel`).
	#[serde(rename = "Minecraft Transit Railway 3")]
	#[cfg(feature = "mtr3-support")]
	#[doc(cfg(feature = "mtr3-support"))]
	MinecraftTransitRailway3
}

/// Options that customize how some file, of a certain file type, is processed.
#[derive(Deserialize, Serialize, JsonSchema, Clone, Copy)]
#[serde(untagged)]
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
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
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

/// Parameters that influence how a audio file is optimized.
#[derive(Deserialize, Serialize, JsonSchema, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct AudioFileOptions {
	/// If `true`, input audio files that are already Ogg won't be transcoded again. This preserves
	/// their original quality and improves performance, but may come at a cost in space savings.
	///
	/// **Default value**: `true`
	pub transcode_ogg: bool,
	/// If `true`, an additional, fast two-pass optimization and validation step will be executed
	/// on the generated Ogg Vorbis file before adding it to the pack, no matter if it was
	/// transcoded or not. This enables PackSquash to ensure that the generated file will work fine
	/// in Minecraft, losslessly reduce its size by 5% on average, and optionally obfuscate it to
	/// thwart its use outside of Minecraft.
	///
	/// Due to how fast and unobtrusive this step is, it's usually best to leave it enabled. Good
	/// reasons for disabling it include troubleshooting and wanting a slightly faster execution
	/// at the cost of missing out on the aforementioned features.
	///
	/// **Default value**: `true`
	pub two_pass_vorbis_optimization_and_validation: bool,
	/// If `true`, empty audio files (i.e., with no audio data, or full of complete silence)
	/// will be replaced with a specially-crafted, but valid empty audio file that is
	/// optimized for size and contains no audio data. This kind of files work fine in
	/// Minecraft and most media players, but some might consider the lack of audio data as
	/// an error.
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
	/// Sets whether this sound should be treated as positional (i.e., mono) or non-positional
	/// (i.e., stereo) for bitrate control and resampling purposes, without affecting its channel
	/// count. This is useful to override the channel count based classification heuristic used
	/// by PackSquash in cases where the different treatment is warranted (e.g., a mono sound
	/// being used prominently in the foreground, or a stereo sound being a faint background
	/// detail).
	///
	/// This option only has any effect when the audio file is transcoded.
	///
	/// **Default value**: `None` (let PackSquash decide according to the number of channels)
	pub is_positional_audio: Option<bool>,
	/// Overrides the sampling frequency to resample this audio file to, ignoring whether it is
	/// positional or not and the global `positional_audio_sampling_frequency` and
	/// `non_positional_audio_sampling_frequency` options.
	///
	/// This option only has any effect when the audio file is transcoded.
	///
	/// **Default value**: `None` (let PackSquash decide based on the value of the global options
	/// and whether this sound is positional or not)
	pub sampling_frequency_override: Option<NonZeroU32>,
	/// Overrides the bitrate control mode used for encoding this audio file, ignoring the global
	/// `audio_bitrate_control_mode` option.
	///
	/// **Default value**: `None` (use the global bitrate control mode)
	pub audio_bitrate_control_mode_override: Option<AudioBitrateControlMode>,
	/// Overrides the target bitrate control metric to use for encoding this audio file, ignoring
	/// whether it is positional or not and the global
	/// `positional_audio_target_bitrate_control_metric` and
	/// `non_positional_audio_target_bitrate_control_metric` options.
	///
	/// This option only has any effect when the audio file is transcoded.
	///
	/// **Default value**: `None` (let PackSquash decide based on the value of the global options
	/// and whether this sound is positional or not)
	pub target_bitrate_control_metric_override: Option<f32>,
	/// Sets the pitch shift coefficient that will have to be used to play back the sound
	/// at the original pitch. This pitch shift coefficient can be used directly in Minecraft
	/// commands like `/playsound`.
	///
	/// This option is mainly useful to make sound ripping harder. If you're just looking into
	/// saving space by making the audio faster (as it has less samples and is shorter) and
	/// then speeding it up in Minecraft, it's better to just change the sampling frequency,
	/// which results in homologous quality and space tradeoffs.
	///
	/// **Default value**: `1.0` (the audio pitch is not shifted)
	pub target_pitch: f32,
	/// If `true`, the generated Ogg Vorbis files will be mangled in a way meant to make them
	/// harder to play outside of Minecraft. The obfuscation technique used is not robust
	/// against being reversed by an expert and does not affect file sizes.
	///
	/// This option only has any effect if the two-pass optimization and validation step is
	/// enabled, i.e., the `two_pass_vorbis_optimization_and_validation` option is set to `true`.
	///
	/// **Default value**: `true`
	pub ogg_obfuscation: bool
}

file_options_default_impl!(AudioFileOptions => AudioFileOptions {
	transcode_ogg: true,
	two_pass_vorbis_optimization_and_validation: true,
	channels: ChannelMixingOption::DEFAULT,
	is_positional_audio: None,
	sampling_frequency_override: None,
	audio_bitrate_control_mode_override: None,
	target_bitrate_control_metric_override: None,
	target_pitch: 1.0,
	empty_audio_optimization: true,
	ogg_obfuscation: false
});

/// A channel mixing strategy for some audio file, contained in [`AudioFileOptions`].
// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Deserialize, JsonSchema, Serialize, Clone, Copy)]
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
	Skip
}

default_variant!(ChannelMixingOption::Skip);

/// Represents a bitrate control mode that can be used by the PackSquash Vorbis encoder,
/// a modified version of the reference encoder with the aoTuV and Lancer patches applied.
// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Deserialize, JsonSchema, Serialize, Clone, Copy)]
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
	///   that it requires less bits to encode mono signals than stereo signals at the same quality
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
	/// The quality factor is expected to be in the [-1, 10] range, where -1 means the worst audio
	/// quality, and 10 the best.
	Cqf,
	/// *Variable BitRate*: the encoder will interpret the target metric as an approximate bitrate,
	/// internally translating it to a quality factor. Therefore, this mode is equivalent to
	/// [CQF](AudioBitrateControlMode::Cqf), but with the quality factor selected in another way.
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
	/// and will be pressured to meet that bitrate for the whole audio signal. No specific subjective
	/// quality level will be targeted.
	///
	/// Some advantages of this bitrate control mode include:
	/// - The actual average stream bitrate is guaranteed to very closely meet the specified average
	///   bitrate. Therefore, the resulting file sizes are easier to predict.
	/// - The maximum instantaneous bitrate for an audio segment might be higher than average in a
	///   small time window, as long as it doesn't affect the average.
	///
	/// Some disadvantages of this bitrate control mode include:
	/// - Setting too low bitrates for the input signal may severely degrade audio quality, and
	///   setting too high bitrates will waste space on padding the data to meet the average.
	/// - Easy to encode audio segments may be stored using more bits than necessary for a certain
	///   quality level in order to meet the average bitrate. Conversely, harder to encode segments
	///   may sound worse due to the encoder being deprived of bits to dedicate to them, if running
	///   high on bitrate. The resulting subjective quality will be more inconsistent.
	/// - Performance is significantly worse than when using CQF or VBR modes, because the Vorbis
	///   bitrate management engine logic is used.
	///
	/// The bitrate is interpreted in kbit/s.
	Abr,
	/// **Constrained Average BitRate**: the encoder will interpret the target metric as a hard
	/// maximum bitrate, internally selecting a slightly lower average bitrate than the maximum
	/// to meet. This mode is similar to [ABR](AudioBitrateControlMode::Abr), but with the addition
	/// of a maximum bitrate.
	///
	/// Some advantages of this bitrate control mode over ABR include:
	/// - The actual average bitrate is guaranteed to never exceed the specified maximum bitrate,
	///   allowing to bound the maximum file size with certainty.
	///
	/// Some disadvantages of this bitrate control mode over ABR include:
	/// - To make sure that the hard maximum bitrate is never exceeded, a lower average bitrate
	///   will be targeted, which ensures headroom for hard to encode segments, but usually yields
	///   inferior quality.
	///
	/// The bitrate is interpreted in kbit/s.
	#[serde(rename = "CABR")]
	ConstrainedAbr
}

default_variant!(AudioBitrateControlMode::Cqf);

/// A helper struct that contains an integer that must be a valid number of
/// audio channels accepted by Minecraft.
// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Deserialize, JsonSchema, Serialize, Clone, Copy)]
#[serde(try_from = "NonZeroU8")]
#[repr(transparent)]
pub struct ChannelCount(#[schemars(range(min = 1, max = 2))] NonZeroU8);

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
#[derive(Deserialize, Serialize, JsonSchema, Clone, Copy)]
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
	// TODO docs
	pub assume_only_childless_models_are_baked: bool
}

file_options_default_impl!(JsonFileOptions => JsonFileOptions {
	minify: true,
	delete_bloat: true,
	assume_only_childless_models_are_baked: false
});

/// Parameters that influence how a PNG file is optimized.
///
/// Note that, in any case, any PNG chunks (e.g. metadata) that are not used by Minecraft
/// to display the image will not be copied over from the original file.
// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Deserialize, JsonSchema, Serialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct PngFileOptions {
	/// The number of Zopfli compression iterations that PackSquash will do to compress raw
	/// pixel data that amounts to a magnitude of 1 MiB. This option is similar to
	/// `zip_compression_iterations`, and is used to feed the same linear model, but with
	/// different parameters better suited for image compression.
	///
	/// When the number of compression iterations drops to zero, which happens when this option
	/// is set to zero or the texture is pretty big, a much more faster DEFLATE compression
	/// algorithm is used instead of Zopfli. This extra performance may come at the cost of file
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
	/// sets a high bound of memory usage by PackSquash and helps authoring packs with
	/// reasonable texture sizes.
	///
	/// **Default value**: 8192
	pub maximum_width_and_height: u16,
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
	/// **Default value**: `true`
	pub downsize_if_single_color: bool
}

file_options_default_impl!(PngFileOptions => PngFileOptions {
	image_data_compression_iterations: 5,
	color_quantization_target: ColorQuantizationTarget::DEFAULT,
	color_quantization_dithering_level: UnitIntervalFloat(0.85),
	maximum_width_and_height: 8192,
	skip_alpha_optimizations: false,
	downsize_if_single_color: true
});

/// Possible targets the colors of a PNG file will be quantized to.
// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Deserialize, JsonSchema, Serialize, Copy, Clone)]
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
	Auto
}

default_variant!(ColorQuantizationTarget::Auto);

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
// Derive Serialize to work around schemars quirk: https://github.com/GREsau/schemars/issues/140
#[derive(Deserialize, JsonSchema, Serialize, Clone, Copy)]
#[serde(try_from = "f32")]
#[repr(transparent)]
pub struct UnitIntervalFloat(#[schemars(range(min = 0, max = 1))] f32);

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
#[derive(Deserialize, Serialize, JsonSchema, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct ShaderFileOptions {
	/// If `true`, the shader file will be minified (i.e. unnecessary white space, line breaks
	/// and comments will be removed) to save space and improve parsing performance. If `false`,
	/// the shader file will still be validated for errors, but left as-is.
	///
	/// **Default value**: `true` (minify)
	#[serde(rename = "minify_shader")]
	pub minify: bool
}

file_options_default_impl!(ShaderFileOptions => ShaderFileOptions {
	minify: true
});

/// Parameters that influence how a legacy language file is optimized.
#[derive(Deserialize, Serialize, JsonSchema, Clone, Copy)]
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

file_options_default_impl!(LegacyLanguageFileOptions => LegacyLanguageFileOptions {
	minify: true,
	strip_bom: true
});

/// Parameters that influence how a command function file is optimized.
#[derive(Deserialize, Serialize, JsonSchema, Clone, Copy)]
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

file_options_default_impl!(CommandFunctionFileOptions => CommandFunctionFileOptions {
	minify: true
});

/// Parameters that influence how a properties file is optimized.
///
/// These files are only supported if PackSquash was compiled with OptiFine mod support. Otherwise,
/// these parameters are read and parsed but ignored afterwards.
#[derive(Deserialize, Serialize, JsonSchema, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
#[cfg(feature = "optifine-support")]
#[doc(cfg(feature = "optifine-support"))]
pub struct PropertiesFileOptions {
	/// If `true`, the properties file will be minified (i.e. unnecessary white space, line breaks
	/// and comments will be removed) to save space and improve parsing performance. If `false`,
	/// the properties file will still be validated for errors, but left as-is.
	///
	/// **Default value**: `true` (minify)
	#[serde(rename = "minify_properties")]
	pub minify: bool
}

#[cfg(feature = "optifine-support")]
file_options_default_impl!(PropertiesFileOptions => PropertiesFileOptions {
	minify: true
});

/// Parameters that define a custom pack file, which PackSquash doesn't expect
/// and skips by default, but that the pack author desires to put in the
/// generated ZIP file.
#[derive(Deserialize, Serialize, JsonSchema, Clone, Copy)]
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
pub fn compile_pack_file_glob_pattern(glob_pattern: &str) -> Result<Glob, globset::Error> {
	GlobBuilder::new(glob_pattern)
		.literal_separator(true)
		.backslash_escape(true)
		.build()
}

fn with_maybe_nonexistent_file_path_format(gen: &mut SchemaGenerator) -> Schema {
	with_format(gen, "maybe_nonexistent_file_path")
}

fn with_directory_path_format(gen: &mut SchemaGenerator) -> Schema {
	with_format(gen, "directory_path")
}

fn with_format(gen: &mut SchemaGenerator, format: impl Into<String>) -> Schema {
	let mut schema: SchemaObject = <str>::json_schema(gen).into();
	schema.format = Some(format.into());
	schema.into()
}
