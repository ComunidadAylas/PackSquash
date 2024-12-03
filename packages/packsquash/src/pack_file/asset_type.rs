//! Contains code to identify a pack file's asset type, which is used to define and enhance the
//! optimizations that can be done to a file.

use std::{borrow::Cow, fmt::Debug};

use enumset::{EnumSet, EnumSetType};
use futures::StreamExt;
use globset::{Glob, GlobSet, GlobSetBuilder};
use tokio::io::AsyncRead;

use crate::config::GlobalOptions;
use crate::pack_file::audio_file::AudioFile;
use crate::pack_file::command_function_file::CommandFunctionFile;
use crate::pack_file::json_file::JsonFile;
use crate::pack_file::legacy_lang_file::LegacyLanguageFile;
use crate::pack_file::passthrough_file::PassthroughFile;
use crate::pack_file::png_file::PngFile;
#[cfg(feature = "optifine")]
use crate::pack_file::properties_file::PropertiesFile;
use crate::pack_file::shader_file::ShaderFile;
use crate::squash_zip::FileListingCircumstances;
use crate::{
	RelativePath,
	config::{CustomFileOptions, FileOptions, compile_pack_file_glob_pattern}
};

use super::{AsyncReadAndSizeHint, PackFile, PackFileConstructor, PackFileProcessData};

/// Represents a relevant pack file asset type, stored in a pack file. A [`PackFile`] can
/// represent assets of several types. An asset type adds constraints on the data format
/// of a [`PackFile`], or otherwise has characteristics that are relevant for optimization.
// When adding or removing variants from this enumeration, make sure to update the PackFileAssetTypeMatches
// and tweak_asset_types_mask_from_global_options implementations too. Also check out file-type
// specific code that may do different things depending on the asset type
#[derive(Debug, EnumSetType)]
#[repr(usize)]
pub enum PackFileAssetType {
	/// A Minecraft texture metadata asset, with `.mcmeta` extension. These files describe
	/// properties of textures.
	MinecraftTextureMetadata,
	/// A Minecraft texture metadata asset, maybe with comments and `.mcmetac` extension.
	MinecraftTextureMetadataWithComments,
	/// A Minecraft metadata asset, with `.mcmeta` extension. These files describe properties
	/// of textures and the pack itself.
	MinecraftMetadata,
	/// A Minecraft metadata asset, maybe with comments and `.mcmetac` extension.
	MinecraftMetadataWithComments,
	/// A Minecraft block or entity model in vanilla format, with `.json` extension.
	MinecraftModel,
	/// A Minecraft block or entity model in vanilla format, maybe with comments and
	/// `.jsonc` extension.
	MinecraftModelWithComments,
	/// An OptiFine custom entity model, with `.jem` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineCustomEntityModel,
	/// An OptiFine custom entity model, maybe with comments and `.jemc` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineCustomEntityModelWithComments,
	/// An OptiFine custom entity model part, with `.jpm` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineCustomEntityModelPart,
	/// An OptiFine custom entity model part, maybe with comments and `.jpmc` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineCustomEntityModelPartWithComments,
	/// A vanilla item model file used by the OptiFine custom items feature, with `.json`
	/// extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineVanillaItemModel,
	/// A vanilla item model file used by the OptiFine custom items feature, maybe with
	/// comments and `.jsonc` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineVanillaItemModelWithComments,
	/// An OptiFine texture metadata asset, in the same format as vanilla texture metadata
	/// files, with `.mcmeta` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineVanillaTextureMetadata,
	/// An OptiFine texture metadata asset, maybe with comments and the `.mcmetac` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineVanillaTextureMetadataWithComments,
	/// A Blockbench modded entity model project which contains custom train models for the
	/// Minecraft Transit Railway 3 mod, with `.bbmodel` extension.
	#[cfg(feature = "mtr3")]
	#[doc(cfg(feature = "mtr3"))]
	Mtr3CustomTrainModel,
	/// A Blockbench modded entity model project which contains custom train models for the
	/// Minecraft Transit Railway 3 mod, maybe with comments and `.bbmodelc` extension.
	#[cfg(feature = "mtr3")]
	#[doc(cfg(feature = "mtr3"))]
	Mtr3CustomTrainModelWithComments,
	/// Any asset in JSON format, with `.json` extension. Because this is a generic asset type,
	/// no optimizations specific to a particular JSON structure will be done.
	GenericJson,
	/// Any asset in JSON format, maybe with comments and `.jsonc` extension.
	GenericJsonWithComments,

	/// Any audio asset in Ogg Vorbis format. Minecraft expects the `.ogg` extension for them.
	GenericOggVorbisAudio,
	/// Any audio asset in a supported format, other than Ogg Vorbis. Currently, the other
	/// supported audio formats and extensions are `.mp3`, `.flac`, `.wav` and `.m4a`. As
	/// Minecraft does not support these formats, they will be converted to Ogg Vorbis,
	/// with `.ogg` extension.
	GenericAudio,

	/// The `pack.png` pack icon file, located at the root directory of the pack.
	PackIcon,
	/// A banner or shield layer texture, with `.png` extension. This asset type is relevant
	/// due to a quirk on how older Minecraft versions (<= 1.12.2) processed them.
	BannerLayer,
	/// An Enderman, Ender Dragon, Phantom or spider eye layer texture, with `.png` extension.
	/// These textures are rendered by the eyes render type, which by default may have some
	/// problems with alpha blending that are exacerbated with the optimizations PackSquash does.
	/// As of November 20th, 2024, only Minecraft version 24w39a (Minecraft 1.21.2) and newer
	/// are not affected by these problems.
	EyeLayer,
	/// A texture that may be used as an input render target in a shader program via a sampler
	/// uniform.
	AuxiliaryShaderTargetTexture,
	/// An OptiFine-specific texture, with `.png` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	OptifineTexture,
	/// A texture used for train models and maybe other assets in the Minecraft Transit Railway
	/// 3 mod. The mod can deal with textures located in any place within the `assets/namespace`
	/// directory.
	#[cfg(feature = "mtr3")]
	#[doc(cfg(feature = "mtr3"))]
	Mtr3CustomGenericTexture,
	/// Any texture in PNG format, with `.png` extension.
	GenericTexture,

	/// A generic properties file added by OptiFine, with `.properties` extension.
	#[cfg(feature = "optifine")]
	#[doc(cfg(feature = "optifine"))]
	GenericProperties,

	/// A GLSL vertex shader, with `.vsh` extension.
	VertexShader,
	/// A GLSL fragment shader, with `.fsh` extension.
	FragmentShader,
	/// A segment of GLSL code that is included verbatim in another shader, with `.glsl`
	/// extension. Also known as "include shader", even though these assets are not
	/// standalone shaders by themselves.
	TranslationUnitSegment,

	/// A legacy language strings file, used in Minecraft version 1.12.2 and below, with
	/// `.lang` extension.
	LegacyLanguageFile,

	/// A font in OpenType or TrueType format, with `.ttf`, `.otf`, `.otc`, or `.ttc` extension.
	/// Minecraft is able to parse the OpenType extensions to the TrueType format since snapshot
	/// 17w43a (Minecraft 1.13), when LWJGL 3.1.2, which updated its bundled `stb_truetype` library
	/// to version 1.13, was made a requirement in its `client.json` file. Later Minecraft versions
	/// switched to using FreeType for font rendering.
	TrueTypeOrOpenTypeFont,
	/// A font in TrueType format, with `.ttf` extension. This is the only accepted font format before
	/// snapshot 17w43a (Minecraft 1.13).
	TrueTypeFont,
	/// A font in Unifont's `.hex` format, wrapped in a ZIP file container, of which the game only reads
	/// file entries with a `.hex` extension. This asset type was added as a part of the `unihex` font
	/// provider in snapshot 23w17a (Minecraft 1.20).
	ZippedUnifontHex,
	/// A binary file that describes the start and end positions of individual characters in
	/// legacy Unicode fonts, with `.bin` extension. Removed in snapshot 23w17a (Minecraft 1.20) in
	/// favor of standard Unifont `.hex` files in a new `unihex` provider.
	LegacyUnicodeFontCharacterSizes,
	/// A UTF-8 plain text file that is shown in-game in some form, with `.txt` extension.
	/// These texts are currently used for the End Poem and splash texts.
	Text,
	/// A UTF-8 plain text file containing the text shown in the end of the game credits when
	/// the Poem is shown, with `.txt` extension. This file was introduced in Minecraft 1.18-pre2.
	ClosingCreditsText,
	/// A UTF-8 plain text file with the game credits, and `.txt` extension. This file was used
	/// in Minecraft versions before 1.17.
	LegacyTextCredits,
	/// A structure file in NBT format, compressed with gzip. Used by data packs. Its extension
	/// is `.nbt`.
	LegacyNbtStructure,
	/// Like `LegacyNbtStructure`, but located under directories that match the registry name
	/// of such structure definitions. This placement change was implemented in snapshot 24w21a
	/// (Minecraft 1.21).
	NbtStructure,
	/// A vanilla Minecraft data pack command function, which contains a list of commands that
	/// can be executed and referred to as a whole. Its extension is `.mcfunction`.
	LegacyCommandFunction,
	/// Like `LegacyCommandFunction`, but located under directories that match the registry name
	/// of such function definitions. This placement change was implemented in snapshot 24w21a
	/// (Minecraft 1.21).
	CommandFunction,

	/// A custom asset type, defined by the end user, whose contents are opaque to PackSquash and
	/// processed without any specific optimizations. Custom assets can never be matched by using
	/// [`PackFileAssetTypeMatcher`].
	// For better style, keep this variant last (i.e. only add new ones above)
	Custom
}

impl PackFileAssetType {
	/// Compiles a glob pattern that matches [`RelativePath`]s and can be used to identify the
	/// [`PackFileConstructor`] belonging to this pack file asset type.
	fn to_glob_pattern(self) -> Glob {
		match self {
			Self::MinecraftTextureMetadata => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/textures/**/?*.mcmeta")
			}
			Self::MinecraftTextureMetadataWithComments => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/textures/**/?*.mcmetac")
			}
			Self::MinecraftMetadata => compile_hardcoded_pack_file_glob_pattern("pack.mcmeta"),
			Self::MinecraftMetadataWithComments => {
				compile_hardcoded_pack_file_glob_pattern("pack.mcmetac")
			}
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
				compile_hardcoded_pack_file_glob_pattern("assets/*/models/{block,item}/**/?*.json")
			}
			Self::MinecraftModelWithComments => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/models/{block,item}/**/?*.jsonc")
			}
			#[cfg(feature = "optifine")]
			Self::OptifineCustomEntityModel => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/{mcpatcher,optifine}/cem/?*.jem"
			),
			#[cfg(feature = "optifine")]
			Self::OptifineCustomEntityModelWithComments => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/{mcpatcher,optifine}/cem/?*.jemc"
			),
			#[cfg(feature = "optifine")]
			Self::OptifineCustomEntityModelPart => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/{mcpatcher,optifine}/cem/?*.jpm"
			),
			#[cfg(feature = "optifine")]
			Self::OptifineCustomEntityModelPartWithComments => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/{mcpatcher,optifine}/cem/?*.jpmc"
			),
			#[cfg(feature = "optifine")]
			Self::OptifineVanillaItemModel => {
				// These models may be put in vanilla paths, or within the cit subdirectory
				// of the OptiFine folder: the documentation states that relative paths from
				// the corresponding properties file are accepted
				compile_hardcoded_pack_file_glob_pattern(
					"assets/*/{mcpatcher,optifine}/cit/**/?*.json"
				)
			}
			#[cfg(feature = "optifine")]
			Self::OptifineVanillaItemModelWithComments => compile_hardcoded_pack_file_glob_pattern(
				"assets/*/{mcpatcher,optifine}/cit/**/?*.jsonc"
			),
			#[cfg(feature = "optifine")]
			Self::OptifineVanillaTextureMetadata => {
				// Textures matched by the OptifineTexture asset type may have animation data
				// alongside them in vanilla format (the custom item and connected textures
				// features expressly document support for this)
				compile_hardcoded_pack_file_glob_pattern(
					"assets/*/{mcpatcher,optifine}/**/?*.png.mcmeta"
				)
			}
			#[cfg(feature = "optifine")]
			Self::OptifineVanillaTextureMetadataWithComments => compile_hardcoded_pack_file_glob_pattern(
				"assets/*/{mcpatcher,optifine}/**/?*.png.mcmetac"
			),
			#[cfg(feature = "mtr3")]
			Self::Mtr3CustomTrainModel => {
				// MTR can read train models from any resource location, but to keep things tidy and
				// ensure that no conflicts with other mods can happen, confine them to the MTR
				// namespace
				compile_hardcoded_pack_file_glob_pattern("assets/mtr/**/?*.bbmodel")
			}
			#[cfg(feature = "mtr3")]
			Self::Mtr3CustomTrainModelWithComments => {
				compile_hardcoded_pack_file_glob_pattern("assets/mtr/**/?*.bbmodelc")
			}
			Self::GenericJson => {
				// This is really generic on purpose, as exhaustively matching all the JSON
				// files a Minecraft resource pack can contain, even if we limit ourselves
				// to vanilla, is a recipe for extreme maintenance effort
				compile_hardcoded_pack_file_glob_pattern("{assets,data}/*/**/?*.json")
			}
			Self::GenericJsonWithComments => {
				compile_hardcoded_pack_file_glob_pattern("{assets,data}/*/**/?*.jsonc")
			}

			Self::GenericOggVorbisAudio => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/sounds/**/?*.{ogg,oga}")
			}
			Self::GenericAudio => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/sounds/**/?*.{mp3,flac,wav,m4a}")
			}

			Self::PackIcon => compile_hardcoded_pack_file_glob_pattern("pack.png"),
			Self::BannerLayer => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/textures/entity/{\
					banner/?*.png,\
					banner_base.png,\
					shield/?*.png,\
					shield_base.png,\
					shield_base_nopattern.png}"
			),
			Self::EyeLayer => compile_hardcoded_pack_file_glob_pattern(
				"assets/minecraft/textures/entity/{\
					enderman/enderman_eyes.png,\
					enderdragon/dragon_eyes.png,\
					spider_eyes.png,\
					phantom_eyes.png}"
			),
			Self::AuxiliaryShaderTargetTexture => {
				compile_hardcoded_pack_file_glob_pattern("assets/minecraft/textures/effect/**/?*.png")
			}
			#[cfg(feature = "optifine")]
			Self::OptifineTexture => {
				// OptiFine looks for PNGs in specific locations within its folder, but users can
				// define paths to files in different folders in OptiFine files. As a compromise,
				// let any PNG file within OptiFine go through
				compile_hardcoded_pack_file_glob_pattern("assets/*/{mcpatcher,optifine}/**/?*.png")
			}
			#[cfg(feature = "mtr3")]
			Self::Mtr3CustomGenericTexture => compile_hardcoded_pack_file_glob_pattern("assets/?*/**/?*.png"),
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
			#[cfg(feature = "optifine")]
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

			// Older Minecraft versions were only able to read shaders from the Minecraft namespace,
			// but versions since 24w34a (1.21.2) can read shaders from any namespace. That version
			// also allowed shaders to be within any subdirectory of the "shaders" directory. While
			// older Minecraft versions didn't have this flexibility, we're not enforcing that here
			// to extend compatibility with hypothetical mods that might have added this feature before
			// it was officially supported
			Self::VertexShader => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/shaders/**/?*.vsh")
			}
			Self::FragmentShader => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/shaders/**/?*.fsh")
			}
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
				// preprocessor code looks generic enough to handle both types just fine.
				// This stopped being a quirk in 24w34a at the latest, the snapshot when the
				// same GLSL preprocessor code for resolving imports was used for all shaders
				compile_hardcoded_pack_file_glob_pattern("assets/*/shaders/**/?*.glsl")
			}

			Self::LegacyLanguageFile => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/lang/**/?*.lang")
			}

			Self::TrueTypeOrOpenTypeFont => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/font/**/?*.{ttf,otf,otc,ttc}")
			}
			Self::TrueTypeFont => compile_hardcoded_pack_file_glob_pattern("assets/*/font/**/?*.ttf"),
			Self::ZippedUnifontHex => compile_hardcoded_pack_file_glob_pattern("assets/*/**/?*.zip"),
			Self::LegacyUnicodeFontCharacterSizes => {
				compile_hardcoded_pack_file_glob_pattern("assets/*/**/?*.bin")
			}
			Self::Text => {
				// Be restrictive in the text files pattern as plain text files are somewhat
				// deprecated for this purpose: it's not likely that Minecraft adds more of
				// them in the future, and in fact the credits text file was replaced by a
				// JSON file. After all, we live in a post "XML fever" world, where JSON is
				// the new plain text data exchange format adequate for any purpose ;)
				compile_hardcoded_pack_file_glob_pattern("assets/minecraft/texts/{end,splashes}.txt")
			}
			Self::ClosingCreditsText => {
				compile_hardcoded_pack_file_glob_pattern("assets/minecraft/texts/postcredits.txt")
			}
			Self::LegacyTextCredits => {
				compile_hardcoded_pack_file_glob_pattern("assets/minecraft/texts/credits.txt")
			}
			Self::LegacyNbtStructure => {
				compile_hardcoded_pack_file_glob_pattern("data/*/structures/**/?*.nbt")
			}
			Self::NbtStructure => {
				compile_hardcoded_pack_file_glob_pattern("data/*/structure/**/?*.nbt")
			}
			Self::LegacyCommandFunction => {
				compile_hardcoded_pack_file_glob_pattern("data/*/functions/**/?*.mcfunction")
			}
			Self::CommandFunction => {
				compile_hardcoded_pack_file_glob_pattern("data/*/function/**/?*.mcfunction")
			}

			Self::Custom => unreachable!()
		}
	}

	/// Returns the canonical extension for the pack file that contains this asset type. The
	/// canonical extension of a pack file is the extension that Minecraft expects when dealing
	/// with the asset contained in the pack file.
	///
	/// The canonical extension may be `None` if the asset type is known to already have a
	/// canonical extension by definition, and thus the extension does not need to be
	/// canonicalized.
	const fn canonical_extension(self) -> Option<&'static str> {
		match self {
			Self::MinecraftTextureMetadata => None,
			Self::MinecraftTextureMetadataWithComments => Some("mcmeta"),
			Self::MinecraftMetadata => None,
			Self::MinecraftMetadataWithComments => Some("mcmeta"),
			Self::MinecraftModel => None,
			Self::MinecraftModelWithComments => Some("json"),
			#[cfg(feature = "optifine")]
			Self::OptifineCustomEntityModel => None,
			#[cfg(feature = "optifine")]
			Self::OptifineCustomEntityModelWithComments => Some("jem"),
			#[cfg(feature = "optifine")]
			Self::OptifineCustomEntityModelPart => None,
			#[cfg(feature = "optifine")]
			Self::OptifineCustomEntityModelPartWithComments => Some("jpm"),
			#[cfg(feature = "optifine")]
			Self::OptifineVanillaItemModel => None,
			#[cfg(feature = "optifine")]
			Self::OptifineVanillaItemModelWithComments => Some("json"),
			#[cfg(feature = "optifine")]
			Self::OptifineVanillaTextureMetadata => None,
			#[cfg(feature = "optifine")]
			Self::OptifineVanillaTextureMetadataWithComments => Some("mcmeta"),
			#[cfg(feature = "mtr3")]
			Self::Mtr3CustomTrainModel => None,
			#[cfg(feature = "mtr3")]
			Self::Mtr3CustomTrainModelWithComments => Some("bbmodel"),
			Self::GenericJson => None,
			Self::GenericJsonWithComments => Some("json"),
			Self::GenericOggVorbisAudio => Some("ogg"),
			Self::GenericAudio => Some("ogg"),
			Self::PackIcon
			| Self::BannerLayer
			| Self::EyeLayer
			| Self::AuxiliaryShaderTargetTexture => None,
			#[cfg(feature = "optifine")]
			Self::OptifineTexture => None,
			#[cfg(feature = "mtr3")]
			Self::Mtr3CustomGenericTexture => None,
			Self::GenericTexture => None,
			#[cfg(feature = "optifine")]
			Self::GenericProperties => None,
			Self::VertexShader => None,
			Self::FragmentShader => None,
			Self::TranslationUnitSegment => None,
			Self::LegacyLanguageFile => None,
			Self::TrueTypeOrOpenTypeFont | Self::TrueTypeFont => None,
			Self::ZippedUnifontHex => None,
			Self::LegacyUnicodeFontCharacterSizes => None,
			Self::Text | Self::ClosingCreditsText | Self::LegacyTextCredits => None,
			Self::LegacyNbtStructure | Self::NbtStructure => None,
			Self::LegacyCommandFunction | Self::CommandFunction => None,
			Self::Custom => None
		}
	}
}

/// A matcher that can be used to determine pack file's asset type, given its [`RelativePath`].
/// In turn, the asset type determines how that pack file should be optimized according to some
/// settings.
pub struct PackFileAssetTypeMatcher {
	asset_type_globset: GlobSet,
	asset_types_mask: EnumSet<PackFileAssetType>
}

impl PackFileAssetTypeMatcher {
	/// Returns a new matcher that can be used to determine the pack file's asset type, given its
	/// [`RelativePath`]. A mask is used to limit what asset types can match. If the mask contains
	/// the custom asset type, [PackFileAssetType::Custom], it will be silently excluded from the mask.
	pub fn new(asset_types_mask: EnumSet<PackFileAssetType>) -> Self {
		let mut globset_builder = GlobSetBuilder::new();
		let asset_types_mask = asset_types_mask - PackFileAssetType::Custom;

		// The iteration is done in ascending discriminant order
		for asset_type in asset_types_mask.iter() {
			globset_builder.add(asset_type.to_glob_pattern());
		}

		Self {
			asset_type_globset: globset_builder.build().unwrap(),
			asset_types_mask
		}
	}

	/// Matches the corresponding asset types for the specified [`RelativePath`]. This operation
	/// potentially involves regular expressions and heap allocations, so users of this method
	/// are encouraged to not do gratuitous matches.
	pub fn matches_for(&self, path: &RelativePath<'_>) -> PackFileAssetTypeMatches {
		PackFileAssetTypeMatches {
			matches: Cow::Owned(
				self.asset_type_globset
					.matches(path) // Calls matches_candidate_into, which returns indices in ascending order
					.into_iter()
					.map(|asset_type_index| {
						self.asset_types_mask.iter().nth(asset_type_index).unwrap()
					})
					.collect()
			)
		}
	}
}

/// A set of asset type matches for a pack file, given its [`RelativePath`]. This struct is
/// constructed by the [`PackFileAssetTypeMatcher::matches_for`] method.
pub struct PackFileAssetTypeMatches {
	matches: Cow<'static, [PackFileAssetType]>
}

impl PackFileAssetTypeMatches {
	/// Returns a [`PackFileAssetTypeMatches`] that represents a single match with the
	/// [`PackFileAssetType::Custom`] asset type.
	pub fn of_custom_asset_type() -> Self {
		Self {
			matches: Cow::Borrowed(&[PackFileAssetType::Custom])
		}
	}

	/// Checks whether there are no matches in this set, so that `process_data` would always
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
		for asset_type in &*self.matches {
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
				PackFileAssetType::MinecraftTextureMetadata
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				PackFileAssetType::MinecraftTextureMetadataWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				PackFileAssetType::MinecraftMetadata
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				PackFileAssetType::MinecraftMetadataWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				PackFileAssetType::MinecraftModel
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				PackFileAssetType::MinecraftModelWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineCustomEntityModel
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineCustomEntityModelWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineCustomEntityModelPart
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineCustomEntityModelPartWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineVanillaItemModel
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineVanillaItemModelWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineVanillaTextureMetadata
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineVanillaTextureMetadataWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "mtr3")]
				PackFileAssetType::Mtr3CustomTrainModel
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				#[cfg(feature = "mtr3")]
				PackFileAssetType::Mtr3CustomTrainModelWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				PackFileAssetType::GenericJson
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				PackFileAssetType::GenericJsonWithComments
					if let Some(FileOptions::JsonFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(JsonFile, optimization_settings)
				}
				PackFileAssetType::GenericOggVorbisAudio
					if let Some(FileOptions::AudioFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(AudioFile, optimization_settings)
				}
				PackFileAssetType::GenericAudio
					if let Some(FileOptions::AudioFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(AudioFile, optimization_settings)
				}
				PackFileAssetType::PackIcon
					if let Some(FileOptions::PngFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(PngFile, optimization_settings)
				}
				PackFileAssetType::BannerLayer
					if let Some(FileOptions::PngFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(PngFile, optimization_settings)
				}
				PackFileAssetType::EyeLayer
					if let Some(FileOptions::PngFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(PngFile, optimization_settings)
				}
				PackFileAssetType::AuxiliaryShaderTargetTexture
					if let Some(FileOptions::PngFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(PngFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::OptifineTexture
					if let Some(FileOptions::PngFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(PngFile, optimization_settings)
				}
				#[cfg(feature = "mtr3")]
				PackFileAssetType::Mtr3CustomGenericTexture
					if let Some(FileOptions::PngFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(PngFile, optimization_settings)
				}
				PackFileAssetType::GenericTexture
					if let Some(FileOptions::PngFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(PngFile, optimization_settings)
				}
				#[cfg(feature = "optifine")]
				PackFileAssetType::GenericProperties
					if let Some(FileOptions::PropertiesFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(PropertiesFile, optimization_settings)
				}
				PackFileAssetType::VertexShader
					if let Some(FileOptions::ShaderFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(ShaderFile, optimization_settings)
				}
				PackFileAssetType::FragmentShader
					if let Some(FileOptions::ShaderFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(ShaderFile, optimization_settings)
				}
				PackFileAssetType::TranslationUnitSegment
					if let Some(FileOptions::ShaderFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(ShaderFile, optimization_settings)
				}
				PackFileAssetType::LegacyLanguageFile
					if let Some(FileOptions::LegacyLanguageFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(LegacyLanguageFile, optimization_settings)
				}
				PackFileAssetType::LegacyCommandFunction | PackFileAssetType::CommandFunction
					if let Some(FileOptions::CommandFunctionFileOptions(optimization_settings)) =
						file_options =>
				{
					return_pack_file_to_process_data!(CommandFunctionFile, optimization_settings)
				}
				PackFileAssetType::TrueTypeOrOpenTypeFont
				| PackFileAssetType::TrueTypeFont
				| PackFileAssetType::ZippedUnifontHex
				| PackFileAssetType::LegacyUnicodeFontCharacterSizes
				| PackFileAssetType::Text
				| PackFileAssetType::ClosingCreditsText
				| PackFileAssetType::LegacyTextCredits
				| PackFileAssetType::LegacyNbtStructure
				| PackFileAssetType::NbtStructure
					if file_options.is_none() =>
				{
					return_pack_file_to_process_data!(PassthroughFile, ())
				}
				PackFileAssetType::Custom
					if let Some(FileOptions::CustomFileOptions(CustomFileOptions {
						force_include: true,
						..
					})) = file_options =>
				{
					return_pack_file_to_process_data!(PassthroughFile, ())
				}
				_ => {
					// The file options do not match the asset type, but maybe we have more asset types to try
					continue;
				}
			}
		}

		// The file options do not match with any matched asset type
		None
	}
}

/// Removes asset types from the specified mask that are not appropriate for the given global options.
/// This takes into account options such as the set of allowed mods.
pub fn tweak_asset_types_mask_from_global_options(
	mut asset_types_mask: EnumSet<PackFileAssetType>,
	global_options: &GlobalOptions
) -> EnumSet<PackFileAssetType> {
	#[cfg(any(feature = "optifine", feature = "mtr3"))]
	use crate::config::MinecraftMod;

	if global_options.skip_pack_icon {
		asset_types_mask -= PackFileAssetType::PackIcon;
	}

	#[cfg(feature = "optifine")]
	if !global_options.allow_mods.contains(MinecraftMod::Optifine) {
		asset_types_mask -= PackFileAssetType::OptifineCustomEntityModel
			| PackFileAssetType::OptifineCustomEntityModelWithComments
			| PackFileAssetType::OptifineCustomEntityModelPart
			| PackFileAssetType::OptifineCustomEntityModelPartWithComments
			| PackFileAssetType::OptifineVanillaItemModel
			| PackFileAssetType::OptifineVanillaItemModelWithComments
			| PackFileAssetType::OptifineVanillaTextureMetadata
			| PackFileAssetType::OptifineVanillaTextureMetadataWithComments
			| PackFileAssetType::OptifineTexture
			| PackFileAssetType::GenericProperties;
	}

	#[cfg(feature = "mtr3")]
	if !global_options
		.allow_mods
		.contains(MinecraftMod::MinecraftTransitRailway3)
	{
		asset_types_mask -= PackFileAssetType::Mtr3CustomTrainModel
			| PackFileAssetType::Mtr3CustomTrainModelWithComments
			| PackFileAssetType::Mtr3CustomGenericTexture;
	}

	asset_types_mask
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
		listing_circumstances: FileListingCircumstances {
			is_directory_listed_atlas_texture_sprite: pack_file
				.may_be_directory_listed_atlas_texture_sprite()
		},
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
