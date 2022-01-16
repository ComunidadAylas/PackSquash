//! Contains code to identify the asset type of a pack file, which is used to define and enhance the
//! optimizations that can be done to a file.

use std::{convert::TryFrom, fmt::Debug};

use enum_iterator::IntoEnumIterator;
use futures::StreamExt;
use globset::{Glob, GlobSet, GlobSetBuilder};
use num_enum::TryFromPrimitive;
use tokio::io::AsyncRead;

#[cfg(feature = "audio-transcoding")]
use crate::pack_file::audio_file::AudioFile;
use crate::pack_file::json_file::JsonFile;
use crate::pack_file::passthrough_file::PassthroughFile;
use crate::pack_file::png_file::PngFile;
#[cfg(feature = "optifine-support")]
use crate::pack_file::properties_file::PropertiesFile;
use crate::pack_file::shader_file::ShaderFile;
use crate::{
	config::{compile_pack_file_glob_pattern, FileOptions},
	RelativePath
};

use super::{AsyncReadAndSizeHint, PackFile, PackFileConstructor, PackFileProcessData};

/// Represents a relevant pack file asset type, stored in a pack file. A [`PackFile`] can
/// represent assets of several types. An asset type adds constraints on the data format
/// of a [`PackFile`], or otherwise has characteristics that are relevant for optimization.
// When adding or removing variants from this enumeration, make sure to update the PackFileAssetTypeMatches
// implementation too
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, IntoEnumIterator)]
#[repr(usize)]
pub(super) enum PackFileAssetType {
	/// A Minecraft metadata asset, with `.mcmeta` extension. These files describe properties
	/// of textures and the pack itself.
	MinecraftMetadata,
	/// A Minecraft block or entity model in vanilla format, with `.json` or `.jsonc` extension.
	MinecraftModel,
	/// An OptiFine custom entity model, with `.jem` extension.
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	OptifineCustomEntityModel,
	/// An OptiFine custom entity model part, with `.jpm` extension.
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	OptifineCustomEntityModelPart,
	/// A Blockbench modded entity model project which contains custom train models for the
	/// Minecraft Transit Railway 3 mod, with `.bbmodel` extension.
	#[cfg(feature = "mtr3-support")]
	#[doc(cfg(feature = "mtr3-support"))]
	Mtr3CustomTrainModel,
	/// Any asset in JSON format, with `.json` or `.jsonc` extension. Because this is a
	/// generic asset type, no optimizations specific to a particular JSON structure will
	/// be done.
	GenericJson,

	/// Any audio asset in Ogg Vorbis format. Minecraft expects the `.ogg` extension for them.
	GenericOggVorbisAudio,
	/// Any audio asset in a format supported by GStreamer, other than Ogg Vorbis. To make
	/// PackSquash easier to distribute and test, only audio files with the extensions `.mp3`,
	/// `.opus`, `.flac` and `.wav` are supported. As Minecraft does not support these formats,
	/// they will be converted to Ogg Vorbis, with `.ogg` extension.
	#[cfg(feature = "audio-transcoding")]
	#[doc(cfg(feature = "audio-transcoding"))]
	GenericAudio,

	/// The `pack.png` pack icon file, located at the root directory of the pack.
	PackIcon,
	/// A banner or shield layer texture, with `.png` extension. This asset type is relevant
	/// due to a quirk on how older Minecraft versions (<= 1.12.2) processed them.
	BannerLayer,
	/// A Enderman, Ender Dragon, Phantom or spider eye layer texture, with `.png` extension.
	/// These textures are rendered by the eyes render type, which by default has some problems
	/// with alpha blending that are exacerbated with the optimizations PackSquash does. As of
	/// 15th November, 2021, all released Minecraft versions are affected by these problems.
	EyeLayer,
	/// An OptiFine-specific texture, with `.png` extension.
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	OptifineTexture,
	/// Any texture in PNG format, with `.png` extension.
	GenericTexture,

	/// A generic properties file added by OptiFine, with `.properties` extension.
	#[cfg(feature = "optifine-support")]
	#[doc(cfg(feature = "optifine-support"))]
	GenericProperties,

	/// A GLSL vertex shader, with `.vsh` extension.
	VertexShader,
	/// A GLSL fragment shader, with `.fsh` extension.
	FragmentShader,
	/// A segment of GLSL code that is included verbatim in another shader, with `.glsl`
	/// extension. Also known as "include shader", even though these assets are not
	/// standalone shaders by themselves.
	TranslationUnitSegment,

	/// A font in TrueType format, with `.ttf` extension.
	TrueTypeFont,
	/// A binary file that describes the start and end positions of individual characters in
	/// legacy Unicode fonts, with `.bin` extension.
	FontCharacterSizes,
	/// A UTF-8 plain text file that is shown in-game in some form, with `.txt` extension.
	/// These texts are currently used for the End Poem and splash texts. In previous versions
	/// they were also used for credit text.
	Text,
	/// A structure file in NBT format, compressed with gzip. Used by data packs. Its extension
	/// is `.nbt`.
	NbtStructure,
	/// A vanilla Minecraft data pack function, which is a list of commands that can be executed
	/// and referred to as a whole. Its extension is `.mcfunction`.
	CommandsFunction
}

impl PackFileAssetType {
	/// Compiles a glob pattern that matches [`RelativePath`]s and can be used to identify the
	/// [`PackFileConstructor`] belonging to this pack file asset type.
	fn to_glob_pattern(self) -> Glob {
		match self {
			Self::MinecraftMetadata => compile_hardcoded_pack_file_glob_pattern(
				"{pack.mcmeta,assets/*/textures/**/?*.mcmeta}"
			),
			Self::MinecraftModel => {
				// It is technically possible in vanilla resource packs to have a model file
				// in folders other than "block" and "item", and in namespaces other than
				// "minecraft". However, mods can define their own namespaces which use
				// similar locations for models, but store them in an entirely different
				// format. The following pattern is a compromise between pack authors'
				// freedom to structure their pack as they wish and accurately identifying
				// vanilla block and item models as such, with few false positives. Mods can
				// define a subfolder like "modname_block" to signal that their models are
				// not to be parsed as vanilla models
				compile_hardcoded_pack_file_glob_pattern(
					"assets/*/models/{block,item}/**/?*.{json,jsonc}"
				)
			}
			#[cfg(feature = "optifine-support")]
			Self::OptifineCustomEntityModel => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/{mcpatcher,optifine}/cem/?*.jem"
			),
			#[cfg(feature = "optifine-support")]
			Self::OptifineCustomEntityModelPart => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/{mcpatcher,optifine}/cem/?*.jpm"
			),
			#[cfg(feature = "mtr3-support")]
			Self::Mtr3CustomTrainModel => {
				// MTR can read train models from any resource location, but to keep things tidy and
				// ensure that no conflicts with other mods can happen, confine them to the MTR
				// namespace
				compile_hardcoded_pack_file_glob_pattern("assets/mtr/**/?*.bbmodel")
			}
			Self::GenericJson => {
				// This is really generic on purpose, as exhaustively matching all the JSON
				// files a Minecraft resource pack can contain, even if we limit ourselves
				// to vanilla, is a recipe for extreme maintenance effort
				compile_hardcoded_pack_file_glob_pattern("{assets,data}/*/**/?*.{json,jsonc}")
			}

			Self::GenericOggVorbisAudio => compile_hardcoded_pack_file_glob_pattern(
				"assets/*/sounds/**/?*.{ogg,oga}"
			),
			#[cfg(feature = "audio-transcoding")]
			Self::GenericAudio => compile_hardcoded_pack_file_glob_pattern(
				"assets/*/sounds/**/?*.{mp3,opus,flac,wav}"
			),

			Self::PackIcon => compile_hardcoded_pack_file_glob_pattern("pack.png"),
			Self::BannerLayer => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/textures/entity/{banner/?*.png,banner_base.png,shield/?*.png,shield_base.png,shield_base_nopattern.png}"
			),
			Self::EyeLayer => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/textures/entity/{enderman/enderman_eyes.png,enderdragon/dragon_eyes.png,spider_eyes.png,phantom_eyes.png}"
			),
			#[cfg(feature = "optifine-support")]
			Self::OptifineTexture => {
				// OptiFine looks for PNGs in specific locations within its folder, but users can
				// define paths to files in different folders in OptiFine files. As a compromise,
				// let any PNG file within OptiFine go through
				compile_hardcoded_pack_file_glob_pattern("assets/*/{mcpatcher,optifine}/**/?*.png")
			}
			Self::GenericTexture => {
				// Some mods might accept textures in any resource location, but to keep things tidier
				// and do some potentially unwanted PNG file cleanup, enforce them to be within a
				// "textures" directory
				compile_hardcoded_pack_file_glob_pattern("assets/*/textures/**/?*.png")
			}

			// Whitelist of OptiFine properties files. The following features are supported:
			//
			// - Custom Panorama (background.properties)
			// - Better Grass (bettergrass.properties)
			// - Custom Biome Palettes (color.properties, properties files in "colormap/blocks").
			//   Note that this feature also reads some PNG files from resource packs:
			//   https://github.com/sp614x/optifine/blob/eeb28cf187ff8d9beb3fb5442a7631d0029fd34e/OptiFineDoc/doc/color.properties#L301-L327
			// - Custom Blocks (block.properties)
			// - Custom Items (cit.properties, properties files in "cit")
			// - Connected Textures (files in "ctm")
			// - Custom Animations (files in "anim")
			// - Custom GUIs (files in "gui/container")
			// - Dynamic Lights (dynamic_lights.properties)
			// - Emissive Textures (emissive.properties)
			// - HD Fonts (font/ascii.properties, font/ascii_sga.properties)
			// - Custom Loading Screens (gui/loading/loading.properties)
			// - Natural Textures (natural.properties)
			// - Random Entities (properties files in "mob" and "random")
			// - Custom Sky (sky/world[0-9]*/sky[0-9]*.properties, sky/world[0-9]*/sun.properties, sky/world[0-9]*/moon_phases.properties)
			//
			// Only the Dynamic Lights file is documented to be able to be put in a namespace
			// other than Minecraft. To futureproof ourselves and not break potentially valid
			// packs, accept other namespaces too.
			//
			// These properties files may refer to assets in other locations, usually textures.
			//
			// In old OptiFine versions (pre-1.13), OptiFine put its assets on the "mcpatcher" subfolder instead
			#[cfg(feature = "optifine-support")]
			Self::GenericProperties => compile_hardcoded_pack_file_glob_pattern(
				"assets/*/{mcpatcher,optifine}/{\
					gui/background.properties,\
					bettergrass.properties,\
					color.properties,\
					colormap/blocks/**/?*.properties,\
					block.properties,\
					cit/**/?*.properties,\
					ctm/**/?*.properties,\
					anim/**/?*.properties,\
					gui/container/**/?*.properties,\
					dynamic_lights.properties,\
					emissive.properties,\
					font/ascii.properties,\
					font/ascii_sga.properties,\
					gui/loading/loading.properties,\
					natural.properties,\
					mob/**/?*.properties,\
					random/**/?*.properties,\
					sky/world[0-9]*/sky[0-9]*.properties,\
					sky/world[0-9]*/sun.properties,\
					sky/world[0-9]*/moon_phases.properties}"
			),

			// Current Minecraft versions are only able to read shaders from the Minecraft
			// namespace. However, this is likely to change in the future, and there might
			// be mods that add shaders in other namespaces. To support such mods as long
			// as they put their shaders in the core and program subdirectories, and future
			// proof our patterns, accept any namespace. It is also worth noting that, to
			// compute the resource location the shader, render program and pipeline
			// definition files are read from, Minecraft performs a simple string
			// concatenation of a prefix path, the file name and the extension. Therefore,
			// subdirectories are possible
			Self::VertexShader => compile_hardcoded_pack_file_glob_pattern("assets/*/shaders/{core,program}/**/?*.vsh"),
			Self::FragmentShader => compile_hardcoded_pack_file_glob_pattern("assets/*/shaders/{core,program}/**/?*.fsh"),
			Self::TranslationUnitSegment => {
				// Even though such possibility is not used in the vanilla resource pack,
				// core shaders can use C-style relative imports which are resolved from the
				// location of their shader files. Therefore, it is legal to find them in
				// the same folder as vertex and fragment shaders, or in a subfolder, due to
				// how resource locations are computed.
				//
				// Another obscure tidbit of trivia is that effects (also known as
				// "post-processing shaders") can't import translation unit segments as of
				// 1.17.1: it's simply just not implemented. We're not sure why Mojang did
				// not just reuse the same code for both types of shaders, as the importing
				// preprocessor code looks generic enough to handle both types just fine
				compile_hardcoded_pack_file_glob_pattern(
					"assets/*/shaders/{core,program,include}/**/?*.glsl"
				)
			},

			Self::TrueTypeFont => compile_hardcoded_pack_file_glob_pattern("assets/*/font/**/?*.ttf"),
			Self::FontCharacterSizes => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/font/**/?*.bin")
			}
			Self::Text => {
				// Be restrictive in the text files pattern as plain text files are somewhat
				// deprecated for this purpose: it's not likely that Minecraft adds more of
				// them in the future, and in fact the credits text file was replaced by a
				// JSON file. After all, we live in a post "XML fever" world, where JSON is
				// the new plain text data exchange format adequate for any purpose ;)
				compile_hardcoded_pack_file_glob_pattern(
					"assets/minecraft/texts/{end,splashes,credits}.txt"
				)
			},
			Self::NbtStructure => {
				compile_hardcoded_pack_file_glob_pattern("data/*/structures/**/?*.nbt")
			}
			Self::CommandsFunction => {
				compile_hardcoded_pack_file_glob_pattern("data/*/functions/**/?*.mcfunction")
			}
		}
	}

	/// Returns the canonical extension for the pack file that contains this asset type.
	///
	/// The canonical extension of a pack file is the extension that Minecraft expects when
	/// dealing with the asset contained in the pack file.
	const fn canonical_extension(self) -> &'static str {
		match self {
			Self::MinecraftModel => "json",
			Self::MinecraftMetadata => "mcmeta",
			#[cfg(feature = "optifine-support")]
			Self::OptifineCustomEntityModel => "jem",
			#[cfg(feature = "optifine-support")]
			Self::OptifineCustomEntityModelPart => "jpm",
			#[cfg(feature = "mtr3-support")]
			Self::Mtr3CustomTrainModel => "bbmodel",
			Self::GenericJson => "json",
			Self::GenericOggVorbisAudio => "ogg",
			#[cfg(feature = "audio-transcoding")]
			Self::GenericAudio => "ogg",
			Self::PackIcon | Self::BannerLayer | Self::EyeLayer => "png",
			#[cfg(feature = "optifine-support")]
			Self::OptifineTexture => "png",
			Self::GenericTexture => "png",
			#[cfg(feature = "optifine-support")]
			Self::GenericProperties => "properties",
			Self::VertexShader => "vsh",
			Self::FragmentShader => "fsh",
			Self::TranslationUnitSegment => "glsl",
			Self::TrueTypeFont => "ttf",
			Self::FontCharacterSizes => "bin",
			Self::Text => "txt",
			Self::NbtStructure => "nbt",
			Self::CommandsFunction => "mcfunction"
		}
	}
}

/// A matcher that can be used to determine the asset type of a pack file, given its [`RelativePath`].
/// In turn, the asset type determines how that pack file should be optimized according to some
/// settings.
pub struct PackFileAssetTypeMatcher {
	asset_type_globset: GlobSet
}

impl PackFileAssetTypeMatcher {
	/// Returns a new matcher that can be used to determine the asset type of a pack file, given its
	/// [`RelativePath`].
	pub fn new() -> Self {
		let mut globset_builder = GlobSetBuilder::new();

		for asset_type in PackFileAssetType::into_enum_iter() {
			globset_builder.add(asset_type.to_glob_pattern());
		}

		Self {
			asset_type_globset: globset_builder.build().unwrap()
		}
	}

	/// Matches the corresponding asset types for the specified [`RelativePath`]. This operation
	/// potentially involves regular expressions and heap allocations, so users of this method
	/// are encouraged to not do gratuitous matches.
	pub fn matches_for(&self, path: &RelativePath<'_>) -> PackFileAssetTypeMatches {
		PackFileAssetTypeMatches {
			matches: self
				.asset_type_globset
				.matches(path) // Calls matches_candidate_into, which returns indices in ascending order
				.into_iter()
				.map(|asset_type_index| PackFileAssetType::try_from(asset_type_index).unwrap())
				.collect()
		}
	}
}

/// A set of asset type matches for a pack file, given its [`RelativePath`]. This struct is
/// constructed by the [`PackFileAssetTypeMatcher::matches_for`] method.
pub struct PackFileAssetTypeMatches {
	matches: Vec<PackFileAssetType>
}

impl PackFileAssetTypeMatches {
	/// Checks whether there are no matches in this set, so that the `process_data` would always
	/// return `None`.
	pub fn is_empty(&self) -> bool {
		self.matches.is_empty()
	}

	/// Returns the data needed to process this pack file. The concrete pack file optimization
	/// strategy is selected according to the asset types that matched this file and the specified
	/// file options.
	///
	/// The return value will be `Some` if and only if the provided file options correspond to some
	/// matched asset type, and the read producer closure returns `Some`. Equivalently, `None` will
	/// be returned if no asset types matched, the file options are not appropriate for any of the
	/// asset type matches, or the file read producer returns `None`. So a return value of `None`
	/// means that the file could not be recognized as a Minecraft asset, that the file options are
	/// not appropriate, or that the file read producer did not yield a file to read.
	pub fn process_data<R: AsyncRead + Send + Unpin + 'static>(
		&self,
		file_options: Option<FileOptions>,
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<R>>
	) -> Option<PackFileProcessData> {
		for asset_type in &self.matches {
			macro_rules! return_pack_file_to_process_data {
				($file_type:ident, $optimization_settings:expr) => {
					return pack_file_to_process_data(
						*asset_type,
						$file_type::new(file_read_producer, *asset_type, $optimization_settings)
					)
				};
			}

			// This match expression is uglier than it needs to be due to this Rust bug:
			// https://github.com/rust-lang/rust/issues/88015
			// However, some of the additional redundancy is mitigated thanks to the macro, and the fact that
			// there are almost as many arms as asset types can make future additions easier, so it's not that bad
			match asset_type {
				PackFileAssetType::MinecraftMetadata if let Some(FileOptions::JsonFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(JsonFile, optimization_settings),
				PackFileAssetType::MinecraftModel if let Some(FileOptions::JsonFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(JsonFile, optimization_settings),
				#[cfg(feature = "optifine-support")]
				PackFileAssetType::OptifineCustomEntityModel if let Some(FileOptions::JsonFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(JsonFile, optimization_settings),
				#[cfg(feature = "optifine-support")]
				PackFileAssetType::OptifineCustomEntityModelPart if let Some(FileOptions::JsonFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(JsonFile, optimization_settings),
				#[cfg(feature = "mtr3-support")]
				PackFileAssetType::Mtr3CustomTrainModel if let Some(FileOptions::JsonFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(JsonFile, optimization_settings),
				PackFileAssetType::GenericJson if let Some(FileOptions::JsonFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(JsonFile, optimization_settings),
				#[cfg(feature = "audio-transcoding")]
				PackFileAssetType::GenericOggVorbisAudio if let Some(FileOptions::AudioFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(AudioFile, optimization_settings),
				#[cfg(not(feature = "audio-transcoding"))]
				PackFileAssetType::GenericOggVorbisAudio if file_options.is_none() =>
					return_pack_file_to_process_data!(PassthroughFile, ()),
				#[cfg(feature = "audio-transcoding")]
				PackFileAssetType::GenericAudio if let Some(FileOptions::AudioFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(AudioFile, optimization_settings),
				PackFileAssetType::PackIcon if let Some(FileOptions::PngFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(PngFile, optimization_settings),
				PackFileAssetType::BannerLayer if let Some(FileOptions::PngFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(PngFile, optimization_settings),
				PackFileAssetType::EyeLayer if let Some(FileOptions::PngFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(PngFile, optimization_settings),
				#[cfg(feature = "optifine-support")]
				PackFileAssetType::OptifineTexture if let Some(FileOptions::PngFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(PngFile, optimization_settings),
				PackFileAssetType::GenericTexture if let Some(FileOptions::PngFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(PngFile, optimization_settings),
				#[cfg(feature = "optifine-support")]
				PackFileAssetType::GenericProperties if let Some(FileOptions::PropertiesFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(PropertiesFile, optimization_settings),
				PackFileAssetType::VertexShader if let Some(FileOptions::ShaderFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(ShaderFile, optimization_settings),
				PackFileAssetType::FragmentShader if let Some(FileOptions::ShaderFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(ShaderFile, optimization_settings),
				PackFileAssetType::TranslationUnitSegment if let Some(FileOptions::ShaderFileOptions(optimization_settings)) = file_options =>
					return_pack_file_to_process_data!(ShaderFile, optimization_settings),
				PackFileAssetType::TrueTypeFont
				| PackFileAssetType::FontCharacterSizes
				| PackFileAssetType::Text
				| PackFileAssetType::NbtStructure
				| PackFileAssetType::CommandsFunction if file_options.is_none() =>
					return_pack_file_to_process_data!(PassthroughFile, ()),
				_ => {
					// The file options do not match the asset type, but maybe we have more asset types to try
					continue
				}
			}
		}

		// The file options do not match with any matched asset type
		None
	}
}

/// Converts a maybe [`PackFile`] of some statically-known concrete type, of some asset type, to the
/// more generic [`PackFileProcessData`] struct that abstracts client away code from the implementation
/// types of the [`PackFile`].
fn pack_file_to_process_data(
	asset_type: PackFileAssetType,
	pack_file: Option<impl PackFile>
) -> Option<PackFileProcessData> {
	// In practice, what we box here is allocated on the heap anyway, so boxing only adds another
	// level of indirection (e.g. we're boxing a struct that holds a pointer to a buffer, getting
	// a pointer to a pointer). This is relatively cheap compared to cloning the actual data, and
	// does not bloat client code due to monomorphization. Sadly, on-stack dynamic dispatch is not
	// feasible because it requires binding variables for every PackFile implementation that live
	// for more than a stack frame
	pack_file.map(|pack_file| PackFileProcessData {
		is_compressed: pack_file.is_compressed(),
		canonical_extension: asset_type.canonical_extension(),
		optimized_byte_chunks_stream: Box::new(pack_file.process().map(|byte_chunk_result| {
			match byte_chunk_result {
				Ok((optimization_strategy, optimized_bytes)) => Ok((
					optimization_strategy,
					Box::new(optimized_bytes) as Box<dyn AsRef<[u8]> + Send>
				)),
				Err(error) => Err(error.into())
			}
		}))
	})
}

/// Compiles the specified pack file glob pattern, assuming it was hardcoded in the application binary.
/// Any validity error is discarded and turned into a panic, as modification of hardcoded data is not
/// to be handled as an error.
///
/// Please note that, even though this function requires a static string slice in an effort to prevent
/// accidental misuse, it is possible to get string slices that live indefinitely by leaking a heap
/// allocation.
fn compile_hardcoded_pack_file_glob_pattern(glob_pattern: &'static str) -> Glob {
	compile_pack_file_glob_pattern(glob_pattern).unwrap()
}
