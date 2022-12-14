use crate::pack_processor::java::asset_processor::compile_hardcoded_pack_file_glob_pattern;
use crate::pack_processor::java::asset_processor::data::vanilla_model_list;
use crate::pack_processor::java::asset_processor::helper::json_helper;
use crate::pack_processor::java::asset_processor::helper::json_helper::JsonAssetDeserializeOutcome;
use crate::pack_processor::java::asset_processor::item_and_block_model_asset_processor::item_or_block_model::{
	ElementFaceDirection, ItemGuiLight, ItemOrBlockModel, ItemOrBlockModelRepresentative,
	ItemTransform, TextureLocationOrReference
};
use crate::pack_processor::java::pack_meta::PackMeta;
use crate::pack_processor::java::resource_location::ResourceLocation;
use crate::relative_path::{InvalidPathError, RelativePath};
use crate::squash_zip::SquashZipError;
use crate::squashed_pack_state::SquashedPackState;
use crate::util::patricia_set_relative_path_iter::PatriciaSetRelativePathIterExt;
use crate::util::range_bounds_intersect::RangeBoundsIntersectExt;
use crate::vfs::VirtualFileSystem;
use crate::PackSquashAssetProcessingStrategy;
use ahash::AHashSet;
use once_cell::sync::Lazy;
use packsquash_options::{minecraft_version, FileOptionsMap, GlobalOptions, JsonFileOptions};
use patricia_tree::PatriciaSet;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::io;
use std::io::{Read, Seek};
use std::ops::Deref;
use strum::IntoEnumIterator;
use thiserror::Error;
use tinyvec::tiny_vec;

mod item_or_block_model;

#[derive(Error, Debug)]
enum InnerItemOrBlockModelAssetError {
	#[error("{0}")]
	JsonDeserializationError(#[from] json_helper::JsonDeserializationError),
	#[error("Invalid pack file path: {0}")]
	InvalidRelativePath(#[from] InvalidPathError<'static>),
	/// Thrown when some error occurs in a ZIP file operation.
	#[error("Error while performing a ZIP file operation: {0}")]
	SquashZip(#[from] SquashZipError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

#[derive(Error, Debug)]
#[error("{file_path}: {inner}")]
pub struct ItemOrBlockModelAssetError {
	inner: InnerItemOrBlockModelAssetError,
	file_path: RelativePath<'static>
}

pub struct ItemAndBlockModelAssetProcessor<
	'state,
	'settings,
	'budget,
	V: VirtualFileSystem + ?Sized,
	F: Read + Seek + Send,
	C: FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send
> {
	vfs: &'state V,
	pack_meta: &'state PackMeta,
	pack_files: &'state PatriciaSet,
	global_options: &'state GlobalOptions<'state>,
	file_options: &'state FileOptionsMap,
	squashed_pack_state: &'state SquashedPackState<'settings, 'budget, F>,
	pub vanilla_models: Lazy<Option<AHashSet<RelativePath<'static>>>, C>,
	supports_item_overrides_attribute: bool,
	supports_parent_elements_override: bool,
	expects_item_transforms_for_each_hand: bool,
	computes_missing_uvs: bool,
	supports_gui_light_attribute: bool
}

pub fn new<'state, 'settings, 'budget, V: VirtualFileSystem + ?Sized, F: Read + Seek + Send>(
	vfs: &'state V,
	pack_meta: &'state PackMeta,
	pack_files: &'state PatriciaSet,
	global_options: &'state GlobalOptions,
	file_options: &'state FileOptionsMap,
	squashed_pack_state: &'state SquashedPackState<'settings, 'budget, F>
) -> ItemAndBlockModelAssetProcessor<
	'state,
	'settings,
	'budget,
	V,
	F,
	impl FnOnce() -> Option<AHashSet<RelativePath<'static>>> + 'state + Send
> {
	let vanilla_models =
		Lazy::new(|| vanilla_model_list::matching_for_version_range(pack_meta.game_version_range()));

	let game_version_range = pack_meta.game_version_range();

	let supports_item_overrides_attribute =
		(minecraft_version!(1, 9)..).intersects(game_version_range);
	let supports_parent_elements_override =
		(minecraft_version!(1, 9)..).intersects(game_version_range);
	let expects_item_transforms_for_each_hand =
		(minecraft_version!(1, 9)..).intersects(game_version_range);
	let computes_missing_uvs = (minecraft_version!(1, 9)..).intersects(game_version_range);
	let supports_gui_light_attribute =
		(minecraft_version!(1, 15, 2)..).intersects(game_version_range);

	ItemAndBlockModelAssetProcessor {
		vfs,
		pack_meta,
		pack_files,
		global_options,
		file_options,
		squashed_pack_state,
		vanilla_models,
		supports_item_overrides_attribute,
		supports_parent_elements_override,
		expects_item_transforms_for_each_hand,
		computes_missing_uvs,
		supports_gui_light_attribute
	}
}

impl<
		'state,
		'settings,
		'budget,
		V: VirtualFileSystem + ?Sized,
		F: Read + Seek + Send,
		C: FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send
	> ItemAndBlockModelAssetProcessor<'state, 'settings, 'budget, V, F, C>
{
	pub fn process(&self) -> Result<(), ItemOrBlockModelAssetError> {
		// Item and block models were introduced in Minecraft 1.8
		if !(minecraft_version!(1, 8)..).intersects(self.pack_meta.game_version_range()) {
			return Ok(());
		}

		let process_asset = |candidate_asset_path| {
			self.process_model_asset(
				&candidate_asset_path,
				get_file_specific_options!(self.file_options, &candidate_asset_path, JsonFileOptions)
			)
		};

		match (
			self.global_options
				.process_not_self_referenced_and_non_vanilla_assets,
			self.vanilla_models.is_some()
		) {
			(true, _) | (_, false) => {
				// Exhaustive asset enumeration strategy: other packs may depend on the models defined by
				// this pack or we don't know what vanilla models there are, so iterate over all the plausible
				// assets

				// Vanilla-format models can be found in other namespaces in modded environments (Forge and
				// Fabric), but under the same paths on those namespaces. The unwritten agreement is to rely
				// on folders within namespaces to distinguish between asset types, while the namespaces
				// themselves are only used as mod IDs. Therefore, in practice it does not happen that a
				// mod-specific namespace reads non-model assets from the models path.
				//
				// In addition, vanilla models technically can be stored at directories other than "block" or
				// "item". We limit ourselves to those two directories because:
				// - The vanilla resource pack does not use this feature, and neither does any other known
				//   resource pack.
				// - It enhances forward compatibility if Mojang ever decides to add other types of models
				//   (e.g., mob models).
				// - Mojang has phased out subdirectories other than "item" and "block" for textures on 1.19.3.
				//
				// This directories limitation doesn't apply to block models self-referenced by blockstates
				// in this pack, however. They will be processed as usual no matter their path, because
				// PackSquash knows for sure that they are block models
				let any_model_path_matcher = compile_hardcoded_pack_file_glob_pattern(
					"assets/?*/models/{block,item}/**/?*.jso{n,nc}"
				)
				.compile_matcher();

				self.pack_files
					.relative_path_iter()
					.par_bridge()
					.filter_map(|relative_path| {
						// Minecraft expects model files to be located at a valid resource
						// location (see net.minecraft.client.resources.model.ModelBakery and
						// net.minecraft.client.resources.model.ModelResourceLocation). Every
						// known mod for the Minecraft versions that support block states expands
						// on this vanilla logic, so filtering by that should be fine
						if any_model_path_matcher.is_match(&relative_path)
							&& ResourceLocation::try_from(&relative_path).is_ok()
						{
							Some(relative_path)
						} else {
							None
						}
					})
					.try_for_each(process_asset)
			}
			(false, true) => {
				// Focused asset enumeration strategy: we only want vanilla assets and we know what vanilla
				// assets there are, in addition to models self-referenced by this pack. Process unreferenced
				// vanilla models right away; the block state asset processor will take care of telling us
				// to process self-referenced models. If vanilla models are referenced by other parts of the
				// pack, they won't be processed several times

				self.vanilla_models
					.as_ref()
					.unwrap()
					.par_iter()
					// Cloning these elements is cheap because they are borrowed references to static data
					.cloned()
					.try_for_each(process_asset)
			}
		}
	}

	pub fn process_model_asset<'options>(
		&self,
		asset_path: &RelativePath,
		file_options: impl Deref<Target = &'options JsonFileOptions>
	) -> Result<(), ItemOrBlockModelAssetError> {
		self.process_model_asset_inner(asset_path, file_options)
			.map_err(|err| ItemOrBlockModelAssetError {
				inner: err,
				file_path: asset_path.as_owned()
			})
	}

	#[inline]
	fn process_model_asset_inner<'options>(
		&self,
		asset_path: &RelativePath,
		file_options: impl Deref<Target = &'options JsonFileOptions>
	) -> Result<(), InnerItemOrBlockModelAssetError> {
		json_helper::deserialize::<ItemOrBlockModelRepresentative, _, _, _>(
			asset_path,
			self.vfs,
			self.squashed_pack_state,
			false,
			self.global_options.always_allow_json_comments,
			"json",
			"jsonc",
			|outcome: JsonAssetDeserializeOutcome<'_, ItemOrBlockModel<'_>>| match outcome {
				JsonAssetDeserializeOutcome::Value {
					value: mut model,
					canonical_path: canonical_asset_path,
					size_hint,
					..
				} => {
					// Validate and optimize the deserialized block state data
					self.process_model(&mut model, &file_options, None, None)?;

					// Serialize it back to a scratch file
					let processed_file = json_helper::serialize(
						model,
						self.squashed_pack_state.scratch_files_budget(),
						size_hint,
						file_options.minify
					)?;

					self.squashed_pack_state.add_file_to_zip(
						&canonical_asset_path,
						processed_file,
						false
					)?;

					// Let interested parties know we've processed this file
					status_trace!(
						ProcessedAsset {
							strategy: match (file_options.delete_bloat, file_options.minify) {
								(false, false) =>
									PackSquashAssetProcessingStrategy::ValidatedAndPrettified,
								(false, true) =>
									PackSquashAssetProcessingStrategy::ValidatedAndMinified,
								(true, false) =>
									PackSquashAssetProcessingStrategy::ValidatedDebloatedAndPrettified,
								(true, true) =>
									PackSquashAssetProcessingStrategy::ValidatedDebloatedAndMinified,
							},
							warnings: tiny_vec!()
						},
						asset_path = asset_path.as_str()
					);

					Ok(())
				}
				JsonAssetDeserializeOutcome::FreshInPreviousZip {
					canonical_path: canonical_asset_path
				} => {
					self.squashed_pack_state
						.add_previous_zip_file(&canonical_asset_path)?;

					// Let interested parties know we've blindly copied this file
					status_trace!(
						ProcessedAsset {
							strategy: PackSquashAssetProcessingStrategy::CopiedFromPreviousZip,
							warnings: tiny_vec!()
						},
						asset_path = asset_path.as_str()
					);

					Ok(())
				}
				JsonAssetDeserializeOutcome::NotFound
				| JsonAssetDeserializeOutcome::CanonicalPathAlreadyProcessed => {
					// Ignore files that do not exist (i.e., opening them returned a "not found" error):
					// they most likely are candidate paths the pack does not contain. Also ignore other
					// candidates once one is processed
					Ok(())
				}
			}
		)?
	}

	fn process_model(
		&self,
		model: &mut ItemOrBlockModel,
		file_options: &JsonFileOptions,
		mut models_to_be_checked_for_presence: Option<&mut AHashSet<RelativePath<'static>>>,
		mut textures_to_be_checked_for_presence: Option<&mut AHashSet<RelativePath<'static>>>
	) -> Result<(), InnerItemOrBlockModelAssetError> {
		if let Some(parent_model) = &model.parent {
			// The parent model reference must be satisfied somehow, if we're interested in checking
			// that
			if let Some(models_to_be_checked_for_presence) = &mut models_to_be_checked_for_presence {
				let parent_model_path = parent_model.as_relative_path()?;

				if !is_vanilla_builtin_model_path(&parent_model_path) {
					models_to_be_checked_for_presence.insert(parent_model_path);
				}
			}

			// The ambient occlusion setting is always ignored for models with parents.
			// Set it to the default value to skip serializing it
			model.ambient_occlusion = true;

			// TODO do something else with the parent?
		}

		if let Some(texture_map) = &model.texture_map {
			// TODO use this data for validation somehow
			for (texture_variable, texture_location_or_reference) in texture_map {
				match texture_location_or_reference {
					TextureLocationOrReference::Location(texture_location) => {
						if let Some(textures_to_be_checked_for_presence) =
							&mut textures_to_be_checked_for_presence
						{
							textures_to_be_checked_for_presence
								.insert(texture_location.as_relative_path()?);
						}
					}
					TextureLocationOrReference::Reference(texture_reference) => ()
				}
			}
		}

		if let Some(item_display_transforms) = &mut model.item_display_transforms {
			if self.expects_item_transforms_for_each_hand {
				let has_legacy_transforms = item_display_transforms.third_person.is_some()
					|| item_display_transforms.first_person.is_some();
			// TODO error if previous bool is true
			} else {
				let has_future_transforms = item_display_transforms.third_person_left_hand.is_some()
					|| item_display_transforms.third_person_right_hand.is_some()
					|| item_display_transforms.first_person_left_hand.is_some()
					|| item_display_transforms.first_person_right_hand.is_some();
				// TODO error if previous bool is true
			}

			macro_rules! optimize_secondary_hand_transform {
				($main_hand_transform_name:ident, $secondary_hand_transform_name:ident) => {
					let main_hand_transform =
						item_display_transforms.$main_hand_transform_name.as_ref();
					let secondary_hand_transform = item_display_transforms
						.$secondary_hand_transform_name
						.as_ref()
						.or(main_hand_transform);

					// If the secondary hand transform has the same value as the main hand transform,
					// make sure to don't set it explicitly, because the game will default to the
					// main hand transform, and being explicit wastes space
					if secondary_hand_transform == main_hand_transform {
						item_display_transforms.$secondary_hand_transform_name = None;
					}
				};
			}

			optimize_secondary_hand_transform!(third_person_right_hand, third_person_left_hand);
			optimize_secondary_hand_transform!(first_person_right_hand, first_person_left_hand);

			if file_options.delete_bloat {
				remove_item_transform_bloat(item_display_transforms.third_person_left_hand.as_mut());
				remove_item_transform_bloat(item_display_transforms.third_person_right_hand.as_mut());
				remove_item_transform_bloat(item_display_transforms.third_person.as_mut());
				remove_item_transform_bloat(item_display_transforms.first_person_left_hand.as_mut());
				remove_item_transform_bloat(item_display_transforms.first_person_right_hand.as_mut());
				remove_item_transform_bloat(item_display_transforms.first_person.as_mut());
				remove_item_transform_bloat(item_display_transforms.head.as_mut());
				remove_item_transform_bloat(item_display_transforms.gui.as_mut());
				remove_item_transform_bloat(item_display_transforms.ground.as_mut());
				remove_item_transform_bloat(item_display_transforms.fixed.as_mut());

				item_display_transforms.bloat_fields.clear();
			}
		}

		if let Some(item_overrides) = &mut model.item_overrides {
			if !self.supports_item_overrides_attribute {
				// TODO return Err
			}

			for item_override in item_overrides {
				if let Some(models_to_be_checked_for_presence) =
					&mut models_to_be_checked_for_presence
				{
					models_to_be_checked_for_presence.insert(item_override.model.as_relative_path()?);
				}

				if file_options.delete_bloat {
					item_override.bloat_fields.clear();
				}
			}
		}

		if let Some(item_gui_light) = &mut model.item_gui_light {
			if !self.supports_gui_light_attribute {
				// TODO return Err
			}

			// If this model is a root model, "side" equals the value the game would use
			// if this field was omitted, so take advantage of that for optimization
			if model.parent.is_none() && matches!(item_gui_light, ItemGuiLight::Side) {
				model.item_gui_light = None;
			}
		}

		if let Some(elements) = &mut model.elements {
			for element in elements {
				for face_direction in ElementFaceDirection::iter() {
					let default_face_uv = element.default_face_uv(face_direction);

					if let Some(face) = &mut element.faces[face_direction] {
						// TODO validate according to texture map
						//face.texture;

						match face.uv {
							Some(uv) => {
								if default_face_uv == uv && self.computes_missing_uvs {
									element.faces[face_direction] = None;
								}
							}
							None => {
								if !self.computes_missing_uvs {
									todo!("Return Err")
								}
							}
						}
					}
				}

				if file_options.delete_bloat {
					element.bloat_fields.clear();
				}
			}
		}

		if file_options.delete_bloat {
			model.bloat_fields.clear();
		}

		Ok(())
	}
}

fn remove_item_transform_bloat(item_transform: Option<&mut ItemTransform>) {
	if let Some(item_transform) = item_transform {
		item_transform.bloat_fields.clear();
	}
}

fn is_vanilla_builtin_model_path(relative_path: &RelativePath) -> bool {
	// See the ItemOrBlockModel struct documentation for more information about these
	// built-in models
	matches!(
		relative_path.as_str(),
		"assets/minecraft/models/builtin/generated.json"
			| "assets/minecraft/models/builtin/missing.json"
			| "assets/minecraft/models/builtin/entity.json"
	)
}
