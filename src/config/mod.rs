//! TODO

use std::{
	convert::{TryFrom, TryInto},
	num::NonZeroUsize,
	path::PathBuf
};

use enumset::{EnumSet, EnumSetType};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sysinfo::{RefreshKind, System, SystemExt};

use crate::squash_zip::SquashZipSettings;

/// Contains all the options that configure a `PackSquasher` operation. This is the
/// root level configuration struct for PackSquash, so it is a good starting point
/// to read the API documentation, after the `PackSquasher` struct.
#[derive(Deserialize)]
#[non_exhaustive]
pub struct SquashOptions {
	/// The directory where the pack that will be processed resides. If `None`, the
	/// directory will have to be provided when the squash operation begins, or it
	/// will return an error. If `Some`, this value will take priority over the
	/// directory provided at squash operation time, if any.
	pub pack_directory: Option<PathBuf>,
	#[serde(flatten)]
	/// Global options that tweak how the squash operation works at a pack scale.
	pub global_options: GlobalOptions,
	/// A map that relates glob patterns that match relative file paths within the
	/// pack to file options, to further customize how the files that match the
	/// pattern are processed.
	#[serde(flatten)]
	pub file_options: IndexMap<String, FileOptions>
}

/// Global options that affect how the entire pack is processed. The default values for
/// these options are meant to be the most reasonable that achieve good compression for
/// a wide range of use cases without using protection, compression or compressibility-improving
/// techniques that may pose interoperability problems.
#[derive(Deserialize)]
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
	/// PackSquash uses a custom ZIP compressor that is able to balance ZIP file
	/// interoperability and specification intent conformance with increased space savings,
	/// compressibility and protection against programs other than Minecraft being able
	/// to extract files from the ZIP. This option lets you choose the tradeoff that is
	/// most suitable to your situation and objectives.
	///
	/// **Default value**: [ZipSpecConformance::Pedantic]
	pub zip_spec_conformance: ZipSpecConformance,
	/// If `zip_spec_conformance` is set to [ZipSpecConformance::Disregard], enabling this
	/// option will add extra protections to the ZIP file that will slightly increase its
	/// size. Otherwise, the value of this option will be ignored. This setting does not
	/// affect whether protections that do not increase the file size are added or not.
	///
	/// **Default value**: `false`
	pub size_increasing_zip_obfuscation: bool,
	/// If `zip_spec_conformance` is set to [ZipSpecConformance::Disregard], this option
	/// sets the probability for each internal ZIP record to be stored in a way that favors
	/// additional discretion of the fact that protection techniques were used, as opposed to
	/// a way that favors increased compressibility of the result ZIP file. Additional
	/// compressibility may be useful if the ZIP file is to be achieved or distributed from a
	/// web server that supports static content compression, for example.
	///
	/// When this setting is 0 (minimum value), every ZIP record will be stored favoring
	/// increased compressibility. When this setting is 100 (maximum value), every ZIP record
	/// will be stored favoring increased discretion instead. Values in the middle combine
	/// increased discretion and compressibility, which may exacerbate both qualities.
	///
	/// This setting has no effect if `zip_spec_conformance` is not set to
	/// [ZipSpecConformance::Disregard].
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
	/// less than linear). This means that PackSquash will be a bit "hesitant" to reduce the number
	/// of iterations for bigger files to meet the target time, so it will exceed it, but not by too
	/// much.
	///
	/// The number of iterations actually used is clamped to the `[1, 20]` interval, so using values
	/// outside that interval for this option will only change the magnitude threshold where iterations
	/// start being reduced to meet a target time.
	///
	/// **Default value**: `20`
	pub zip_compression_iterations: u8,
	/// Some Minecraft versions have some quirks that limit how some files can be compressed, because
	/// otherwise they are not correctly interpreted by the game. PackSquash can work around these
	/// quirks, but doing so may come at the cost of reduced space savings or increased processing
	/// times, so this should only be done when your pack is affected by them. This option specifies
	/// the quirks that will worked around.
	///
	/// **Default value**: empty set (no quirks worked around)
	pub workaround_minecraft_quirks: EnumSet<MinecraftQuirk>,
	/// This option controls whether Minecraft will ignore system and hidden files (i.e. whose name
	/// starts with a dot), not even trying to process them. Under most circumstances, you shouldn't
	/// need to disable this option.
	///
	/// **Default value**: `true`
	pub ignore_system_and_hidden_files: bool,
	/// PackSquash supports pack files added by mods, but, in the interest of keeping its output as
	/// lean as possible by default, you should indicate what mods do you want to support and include
	/// in the result ZIP file.
	///
	/// This option does not have any effect if PackSquash was compiled without mod support.
	///
	/// **Default value**: empty set (do not add any mod-specific files)
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
	/// Currently, PackSquash creates one spooling buffer per thread, plus one extra spooling buffer for
	/// the output ZIP file.
	///
	/// **Default value**: `half of the available memory reported by the OS / (number of CPU hardware threads + 1)`
	pub spooling_buffers_size: usize,
	/// The tentative maximum number of files that PackSquash will keep open at the same time. The default is
	/// almost always fine, but if lots of threads are spawned to process pack files (more than 64) there can be
	/// problems, because most operating systems limit the number of open files a process can have, and each thread
	/// will keep the pack file it is processing open, at least.
	///
	/// Under normal circumstances, you should only reduce this value when you hit an open file limit. Conversely,
	/// you should only increase it if you have configured your operating system to increase this limit and `threads`
	/// is less than this value, because otherwise the concurrency will be effectively limited by this limit.
	///
	/// The default value is a conservative limit that won't hurt performance under most circumstances, neither risk
	/// hitting limitations.
	///
	/// **Default value**: `512`
	pub open_file_limit: NonZeroUsize
}

impl GlobalOptions {
	/// TODO
	pub(crate) fn as_squash_zip_settings(&self) -> SquashZipSettings {
		SquashZipSettings {
			zopfli_iterations: self.zip_compression_iterations,
			store_squash_time: !self.never_store_squash_times
				&& !matches!(self.zip_spec_conformance, ZipSpecConformance::Pedantic),
			enable_obfuscation: matches!(self.zip_spec_conformance, ZipSpecConformance::Disregard),
			enable_deduplication: matches!(
				self.zip_spec_conformance,
				ZipSpecConformance::Balanced | ZipSpecConformance::Disregard
			),
			enable_size_increasing_obfuscation: self.size_increasing_zip_obfuscation,
			percentage_of_structures_tuned_for_obfuscation_discretion: self
				.percentage_of_zip_structures_tuned_for_obfuscation_discretion,
			spool_buffer_size: self.spooling_buffers_size.saturating_mul(1024 * 1024)
		}
	}
}

impl Default for GlobalOptions {
	fn default() -> Self {
		// The "k" in "kB" here has an SI-compliant meaning (1000 and not 1024 bytes)
		let available_memory_kb =
			System::new_with_specifics(RefreshKind::new().with_memory()).available_memory();

		let hardware_threads = num_cpus::get();

		Self {
			skip_pack_icon: false,
			zip_spec_conformance: Default::default(),
			size_increasing_zip_obfuscation: false,
			percentage_of_zip_structures_tuned_for_obfuscation_discretion: PercentageInteger(0),
			never_store_squash_times: false,
			recompress_compressed_files: false,
			zip_compression_iterations: 20,
			workaround_minecraft_quirks: EnumSet::empty(),
			ignore_system_and_hidden_files: true,
			allow_mods: EnumSet::empty(),
			threads: hardware_threads.try_into().unwrap(),
			output_file_path: PathBuf::from("pack.zip"),
			// In MiB. By default, half of available memory / hardware threads
			spooling_buffers_size: (available_memory_kb * 125
				/ 262144 / (hardware_threads as u64 + 1))
				.try_into()
				.unwrap_or(usize::MAX),
			open_file_limit: 512.try_into().unwrap()
		}
	}
}

/// A ZIP specification intent conformance target that a squash operation can adhere to.
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ZipSpecConformance {
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
	/// Output ZIP files generated with this conformance target **MUST NOT** be reused in runs
	/// configured to expect and generate output ZIP files with a less strict conformance
	/// target. Reusing them with runs configured with this same conformance target does not have
	/// any effect.
	Pedantic,
	/// Like [ZipSpecConformance::Pedantic], but stores the metadata needed to reuse this ZIP file
	/// to speed up future runs, and expects this metadata from the specified output ZIP file, if it
	/// already exists.
	///
	/// This conformance target is compatible with most ZIP file manipulation programs, even
	/// though some metadata is out of specification, so any compliant ZIP file manipulation
	/// program is, in theory, free to reject the file or warn about it. They almost universally
	/// don't do this, though.
	///
	/// Output ZIP files generated with this conformance target can only be safely reused with runs
	/// configured to expect and generate output ZIP files with the [ZipSpecConformance::Balanced]
	/// target and, of course, [ZipSpecConformance::High] target.
	High,
	/// Like [ZipSpecConformance::High], but allows deduplicating identical processed files in
	/// the output ZIP file. This yields significant space savings in case the pack contains
	/// repeated assets.
	///
	/// This makes a class of ZIP file manipulation programs choke when trying to extract from
	/// the file, but no additional protections or compressibility improvements are applied.
	///
	/// If your pack does not contain assets that process to the same contents, this conformance
	/// target is equivalent to [ZipSpecConformance::High].
	///
	/// Output ZIP files generated with this conformance target can only be safely reused with runs
	/// configured to expect and generate output ZIP files with the [ZipSpecConformance::High]
	/// target and, of course, [ZipSpecConformance::Balanced] target.
	Balanced,
	/// PackSquash will apply every technique it knows to protect your pack, improve its compressibility
	/// and reduce its size, without any regard that the result ZIP file can be used outside of Minecraft
	/// (in fact, the protection techniques make this harder). This is the complete opposite to
	/// [ZipSpecConformance::Pedantic].
	///
	/// Currently, the above paragraph means:
	/// - Deduplication is allowed.
	/// - Protections to make it less easy to be used outside of Minecraft are added.
	/// - Compressibility improving techniques that pose interoperability problems, but are accepted by
	///   Minecraft, are used.
	/// - ZIP files generated in previous runs may be reused to speed up the current run.
	///
	/// Output ZIP files generated with this conformance target can only be safely reused with runs
	/// configured to expect and generate output ZIP files with the [ZipSpecConformance::Disregard]
	/// target.
	Disregard
}

impl Default for ZipSpecConformance {
	fn default() -> Self {
		Self::Pedantic
	}
}

/// A helper struct that contains an integer guaranteed to be in the `[0, 100]` interval.
#[derive(Deserialize, Clone, Copy)]
#[serde(try_from = "u8")]
pub struct PercentageInteger(u8);

impl PercentageInteger {
	/// Creates a [`PercentageInteger`] struct that holds the specified integer. This will
	/// fail if the specified integer is not in the expected interval.
	pub fn new(percentage: u8) -> Result<Self, <Self as TryFrom<u8>>::Error> {
		percentage.try_into()
	}
}

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

/// A Minecraft file parsing quirk that affects the perceived correctness of the PackSquash
/// output ZIP and that can be worked around.
#[derive(Deserialize, Serialize, EnumSetType)]
#[serde(rename_all = "snake_case")]
#[enumset(serialize_deny_unknown, serialize_as_list)]
#[non_exhaustive]
pub enum MinecraftQuirk {
	/// Older versions of Minecraft (1.12.2 was confirmed to be affected and 1.16.2 was
	/// confirmed not to be; probably all versions until 1.13 are affected) assume that
	/// grayscale images are in a rather uncommon color space, instead of the more common
	/// sRGB it assumes for color images. Because PackSquash can compress grayscale color
	/// images to actual grayscale format to save space, affected Minecraft versions
	/// display those images with colors that look "washed-out".
	///
	/// This workaround stops PackSquash from reducing color images to grayscale, which may
	/// hurt compression. This has no effect for input images that already are in grayscale.
	GrayscaleTexturesGammaMiscorrection
}

/// A Minecraft modification supported by PackSquash that adds file types to packs.
#[derive(Deserialize, Serialize, EnumSetType)]
#[enumset(serialize_deny_unknown, serialize_as_list)]
#[non_exhaustive]
#[cfg(feature = "mod-support")]
#[doc(cfg(feature = "mod-support"))]
pub enum MinecraftMod {
	/// OptiFine.
	///
	/// Currently, this adds support for the following file types:
	/// - Properties files (`.properties`).
	/// - Custom entity model files (`.jem` and `.jpm`).
	#[serde(rename = "OptiFine")]
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	Optifine
}

/// Options that customize how some file, of a certain file type, is processed.
#[derive(Deserialize, Clone, Copy)]
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
	PropertiesFileOptions(PropertiesFileOptions)
}

/// Helper trait that every file options struct contained in [`FileOptions`] variants must
/// implement.
pub(crate) trait FileOptionsTrait {
	/// Tweaks the value of the crate-private fields that are used to enforce global options
	/// contained in the [`GlobalOptions`] struct.
	///
	/// It is recommended to execute this method just after the default or user provided
	/// file settings for some pack file were found, before actually using them.
	fn tweak_from_global_options(self, global_options: &GlobalOptions) -> Self;
}

/// Parameters that influence how a audio file is optimized.
#[derive(Deserialize, Clone, Copy)]
#[serde(default, deny_unknown_fields)]
#[non_exhaustive]
pub struct AudioFileOptions {
	/// If `true`, input audio files that are already OGG won't be transcoded again. This preserves
	/// their original quality and improves performance, but may come at a cost in space savings.
	///
	/// **Default value**: `true`
	pub transcode_ogg: bool,
	/// Sets a number of channels that the audio file will be mixed to. Downmixing stereo sounds
	/// to mono may save a bit of space, but may also affect how Minecraft calculates positional
	/// audio effects. On the other hand, it may be desireable to upmix mono sounds to stereo
	/// to get the different positional effects.
	///
	/// **Default value**: do not downmix or upmix (keep the channels of the input file)
	pub channels: ChannelMixingOption,
	/// Changes the sampling frequency of the resulting audio file. Higher sampling frequencies
	/// can represent audio signals with higher bandwidth properly, but also take more space
	/// (because more audio samples are recorded and stored).
	///
	/// **Default value**: `32000` (Hz)
	pub sampling_frequency: PositiveI32,
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
	/// The minimum bitrate that the OGG Vorbis encoder will try to use to represent the audio
	/// signal. Higher values provide higher quality at the expense of file size.
	///
	/// **Default value**: 40000 (bit/s)
	pub minimum_bitrate: PositiveI32,
	/// The maximum bitrate that the OGG Vorbis encoder will try to use to represent the audio
	/// signal. Higher values provide higher quality at the expense of file size.
	///
	/// **Default value**: 96000 (bit/s)
	pub maximum_bitrate: PositiveI32
}

impl Default for AudioFileOptions {
	fn default() -> Self {
		Self {
			transcode_ogg: true,
			channels: Default::default(),
			sampling_frequency: 32_000.try_into().unwrap(),
			target_pitch: 1.0,
			minimum_bitrate: 40_000.try_into().unwrap(),
			maximum_bitrate: 96_000.try_into().unwrap()
		}
	}
}

impl FileOptionsTrait for AudioFileOptions {
	fn tweak_from_global_options(self, _: &GlobalOptions) -> Self {
		self
	}
}

/// A channel mixing strategy for some audio file, contained in [`AudioFileOptions`].
#[derive(Deserialize, Clone, Copy)]
#[serde(untagged)]
pub enum ChannelMixingOption {
	/// Downmix or upmix the sound channels in the input file to generate an output
	/// file with the specified number of channels.
	///
	/// At the time this was written, only `1` or `2` channels make sense for a
	/// resource pack audio file, but other numbers are accepted.
	ToChannels(PositiveI32),
	/// Do not change the number or layout of the sound channels of the input file
	/// in any way.
	Skip
}

impl Default for ChannelMixingOption {
	fn default() -> Self {
		ChannelMixingOption::Skip
	}
}

/// A helper struct that contains an integer that must be strictly positive (i.e.
/// greater, and not equal to, zero).
#[derive(Deserialize, Clone, Copy)]
#[serde(try_from = "i32")]
pub struct PositiveI32(i32);

impl PositiveI32 {
	/// Creates a [`PositiveI32`] struct that holds the specified integer. This will
	/// fail if the specified integer is not in the expected interval.
	pub fn new(value: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
		value.try_into()
	}
}

impl TryFrom<i32> for PositiveI32 {
	type Error = &'static str;

	fn try_from(value: i32) -> Result<Self, Self::Error> {
		if value > 0 {
			Ok(PositiveI32(value))
		} else {
			Err("The specified integer should be greater than zero")
		}
	}
}

impl From<PositiveI32> for i32 {
	fn from(value: PositiveI32) -> Self {
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
	/// Crate-private option set when [MinecraftMod::Optifine] was configured.
	///
	/// **Default value**: `false`
	#[serde(skip)]
	pub(crate) allow_optifine_extensions: bool
}

impl Default for JsonFileOptions {
	fn default() -> Self {
		Self {
			minify: true,
			delete_bloat: true,
			allow_optifine_extensions: false
		}
	}
}

impl FileOptionsTrait for JsonFileOptions {
	fn tweak_from_global_options(mut self, global_options: &GlobalOptions) -> Self {
		#[cfg(feature = "optifine-support")]
		{
			self.allow_optifine_extensions =
				global_options.allow_mods.contains(MinecraftMod::Optifine);
		}

		self
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
	/// The maximum width and height of the images that will be accepted. This parameter
	/// sets a high bound of memory usage by PackSquash and helps authoring packs with
	/// reasonable texture sizes.
	///
	/// **Default value**: 4096
	// TODO: put this text in the wiki
	// Internally, Minecraft builds a texture atlas with every texture it needs to draw,
	// including those of packs, for efficiency reasons.
	// Assuming a maximum texture size of 65536x65536 (which as of 2021 is an upper bound
	// for consumer GPUs), 256 individual textures of 4096x4096 would fit in such an atlas,
	// which is a pretty low number and not enough to hold all vanilla textures. Resource
	// packs with few textures may get away with textures that are even bigger than this,
	// maybe only on "beefy" GPUs, but it does not make much sense to mix and match small
	// textures with such big textures. I've not seen packs with textures bigger
	// than 2048x2048, and even that is quite a stretch. Therefore, establish an upper limit
	// of 4096x4096 to set an upper bound to memory usage per decoded RGBA8 image of ~64 MiB,
	// and reject textures that are impractically big, introducing a kind of "validation" that
	// may help limit how much pack creators can shoot themselves on the foot
	pub maximum_width_and_height: u32,
	/// Crate-private option set by the [MinecraftQuirk::GrayscaleTexturesGammaMiscorrection]
	/// workaround to not reduce color images to grayscale.
	///
	/// **Default value**: `false`
	#[serde(skip)]
	pub(crate) do_not_reduce_to_grayscale: bool,
	/// Crate-private option set when the `skip_pack_icon` configuration parameter is set.
	///
	/// **Default value**: `false`
	#[serde(skip)]
	pub(crate) skip_pack_icon: bool
}

impl Default for PngFileOptions {
	fn default() -> Self {
		Self {
			color_quantization_target: Default::default(),
			maximum_width_and_height: 4096,
			do_not_reduce_to_grayscale: false,
			skip_pack_icon: false
		}
	}
}

impl FileOptionsTrait for PngFileOptions {
	fn tweak_from_global_options(mut self, global_options: &GlobalOptions) -> Self {
		self.do_not_reduce_to_grayscale = global_options
			.workaround_minecraft_quirks
			.contains(MinecraftQuirk::GrayscaleTexturesGammaMiscorrection);
		self.skip_pack_icon = global_options.skip_pack_icon;

		self
	}
}

/// Possible targets the colors of a PNG file will be quantized to.
#[derive(Deserialize, Copy, Clone)]
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
	EightBitDepth
}

impl Default for ColorQuantizationTarget {
	fn default() -> Self {
		Self::EightBitDepth
	}
}

impl ColorQuantizationTarget {
	/// Gets the bit depth of this color quantization target.
	pub(crate) const fn depth(&self) -> u8 {
		match self {
			Self::None => 0,
			Self::OneBitDepth => 1,
			Self::TwoBitDepth => 2,
			Self::FourBitDepth => 4,
			Self::EightBitDepth => 8
		}
	}

	/// Gets the maximum number of colors that an image with this color
	/// quantization target can possibly have.
	pub(crate) const fn max_colors(&self) -> u32 {
		u32::checked_pow(2, self.depth() as u32).unwrap()
	}

	/// Returns whether color quantization should be performed at all
	/// to meet this color quantization target.
	pub(crate) const fn should_quantize(&self) -> bool {
		self.depth() > 0
	}
}

/// Parameters that influence how a shader file is optimized.
#[derive(Deserialize, Clone, Copy)]
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

impl Default for ShaderFileOptions {
	fn default() -> Self {
		Self { minify: true }
	}
}

impl FileOptionsTrait for ShaderFileOptions {
	fn tweak_from_global_options(self, _: &GlobalOptions) -> Self {
		self
	}
}

/// Parameters that influence how a properties file is optimized.
///
/// These files are only supported if PackSquash was compiled with OptiFine mod support. Otherwise,
/// these parameters are read and parsed but ignored afterwards.
#[derive(Deserialize, Clone, Copy)]
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
impl Default for PropertiesFileOptions {
	fn default() -> Self {
		Self { minify: true }
	}
}

#[cfg(feature = "optifine-support")]
impl FileOptionsTrait for PropertiesFileOptions {
	fn tweak_from_global_options(self, _: &GlobalOptions) -> Self {
		self
	}
}
