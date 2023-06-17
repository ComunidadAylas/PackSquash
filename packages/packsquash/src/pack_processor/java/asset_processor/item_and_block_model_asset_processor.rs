use std::{
	borrow::Cow,
	cell::Cell,
	io,
	io::{Read, Seek},
	ops::Deref,
	sync::{Arc, Mutex, RwLock}
};

use ahash::{AHashMap, AHashSet};
use compact_str::CompactString;
use once_cell::sync::Lazy;
use packsquash_options::{
	minecraft_version, FileOptionsMap, GlobalOptions, JsonFileOptions, MissingReferenceAction
};
use rayon::prelude::*;
use strum::IntoEnumIterator;
use thiserror::Error;
use tinyvec::{tiny_vec, TinyVec};

use self::cache::{CachedItemOrBlockModel, ModelCache};
use crate::{
	pack_processor::java::{
		asset_processor::{
			compile_hardcoded_pack_file_glob_pattern,
			data::vanilla_model_list,
			helper::{self, json_helper},
			item_and_block_model_asset_processor::item_or_block_model::{
				ElementFaceDirection, ItemGuiLight, ItemOrBlockModel, ItemTransform,
				TextureLocationOrReference
			}
		},
		pack_meta::PackMeta,
		resource_location::ResourceLocation
	},
	relative_path::{InvalidPathError, RelativePath, RelativePathPatriciaSet},
	squash_zip::SquashZipError,
	squashed_pack_state::SquashedPackState,
	util::range_bounds_intersect::RangeBoundsIntersectExt,
	vfs::VirtualFileSystem,
	PackSquashAssetProcessingStrategy
};

mod cache;
mod item_or_block_model;

#[derive(Error, Debug)]
enum InnerItemOrBlockModelAssetError {
	#[error("A loop was found in the model parent hierarchy. Please ensure that a parent is not referenced several times")]
	ModelInheritanceLoop,
	#[error("The texture variable {0} is undefined or it was caught in a references loop")]
	UndefinedTextureVariable(CompactString),
	#[error("The model defines item display transforms in single-hand format, but a Minecraft version range that expects the dual-hand format is targeted")]
	LegacyItemDisplayTransform,
	#[error("The model defines item display transforms in the dual-hand format, but a Minecraft version range that expects the single-hand format is targeted")]
	FutureItemDisplayTransform,
	#[error("The model attribute {0} is not implemented for the target Minecraft version range")]
	FutureModelAttribute(&'static str),
	#[error("The model overrides elements from its parent, but this is not supported by the target Minecraft version range")]
	UnsupportedParentElementsOverride,
	#[error("An element face is missing texture UV coordinates, but this is not supported by the target Minecraft version range")]
	MissingElementFaceTextureCoordinates,
	#[error("The texture variable {0} was referenced by an element face, but it is undefined or it was caught in a references loop")]
	UndefinedElementFaceTextureVariable(String),
	#[error("Another model at {path} was referenced, but it is not a known model and it was not found in the pack{__filtered_out_text}")]
	MissingModel {
		path: RelativePath<'static>,
		#[doc(hidden)]
		__filtered_out_text: &'static str
	},
	#[error("A texture at {path} was referenced, but it is not a known texture and it was not found in the pack{__filtered_out_text}")]
	MissingTexture {
		path: RelativePath<'static>,
		#[doc(hidden)]
		__filtered_out_text: &'static str
	},
	#[error("{0}")]
	JsonDeserializationError(#[from] json_helper::JsonDeserializationError),
	#[error("Invalid pack file path: {0}")]
	InvalidRelativePath(#[from] InvalidPathError),
	/// Thrown when some error occurs in a ZIP file operation.
	#[error("Error while performing a ZIP file operation: {0}")]
	SquashZip(#[from] SquashZipError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

define_path_wrapper_err!(InnerItemOrBlockModelAssetError: ItemOrBlockModelAssetError);

pub struct ItemAndBlockModelAssetProcessor<
	'params,
	'state,
	V: VirtualFileSystem + ?Sized,
	F: Read + Seek + Send,
	C: FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send
> {
	vfs: &'params V,
	pack_meta: &'params PackMeta,
	pack_files: &'params RelativePathPatriciaSet<'static>,
	global_options: &'params GlobalOptions<'params>,
	file_options: &'params FileOptionsMap<'params>,
	squashed_pack_state: &'params SquashedPackState<'state, 'state, F>,
	pub vanilla_models: Lazy<Option<AHashSet<RelativePath<'static>>>, C>,
	pub models_referenced_by_blockstates: Mutex<AHashSet<RelativePath<'static>>>,
	game_version_supports_models: bool,
	game_version_supports_item_overrides_attribute: bool,
	game_version_supports_parent_elements_override: bool,
	game_version_expects_item_transforms_for_each_hand: bool,
	game_version_computes_missing_uvs: bool,
	game_version_supports_gui_light_attribute: bool
}

pub fn new<'params, 'state, V: VirtualFileSystem + ?Sized, F: Read + Seek + Send>(
	vfs: &'params V,
	pack_meta: &'params PackMeta,
	pack_files: &'params RelativePathPatriciaSet<'static>,
	global_options: &'params GlobalOptions<'params>,
	file_options: &'params FileOptionsMap<'params>,
	squashed_pack_state: &'params SquashedPackState<'state, 'state, F>
) -> ItemAndBlockModelAssetProcessor<
	'params,
	'state,
	V,
	F,
	impl FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send + 'params
> {
	let vanilla_models =
		Lazy::new(|| vanilla_model_list::matching_for_version_range(pack_meta.game_version_range()));

	let game_version_range = pack_meta.game_version_range();

	let game_version_supports_models = (minecraft_version!(1, 8)..).intersects(game_version_range);
	let game_version_supports_item_overrides_attribute =
		(minecraft_version!(1, 9)..).intersects(game_version_range);
	let game_version_supports_parent_elements_override =
		(minecraft_version!(1, 9)..).intersects(game_version_range);
	let game_version_expects_item_transforms_for_each_hand =
		(minecraft_version!(1, 9)..).intersects(game_version_range);
	let game_version_computes_missing_uvs =
		(minecraft_version!(1, 9)..).intersects(game_version_range);
	let game_version_supports_gui_light_attribute =
		(minecraft_version!(1, 15, 2)..).intersects(game_version_range);

	ItemAndBlockModelAssetProcessor {
		vfs,
		pack_meta,
		pack_files,
		global_options,
		file_options,
		squashed_pack_state,
		vanilla_models,
		models_referenced_by_blockstates: Mutex::new(AHashSet::new()),
		game_version_supports_models,
		game_version_supports_item_overrides_attribute,
		game_version_supports_parent_elements_override,
		game_version_expects_item_transforms_for_each_hand,
		game_version_computes_missing_uvs,
		game_version_supports_gui_light_attribute
	}
}

impl<
		'params,
		'state,
		V: VirtualFileSystem + ?Sized,
		F: Read + Seek + Send,
		C: FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send
	> ItemAndBlockModelAssetProcessor<'params, 'state, V, F, C>
{
	pub fn process(&self) -> Result<(), ItemOrBlockModelAssetError> {
		if !self.game_version_supports_models {
			return Ok(());
		}

		let model_cache = ModelCache::new(self.vfs, self.global_options, self.squashed_pack_state);
		let candidate_asset_paths = self.enumerate_models();

		// First, populate a set with the models that are known to have children
		// TODO enumerate and process models_referenced_by_blockstates on finish()
		let mut non_leaf_models = AHashSet::new();
		for candidate_asset_path in &candidate_asset_paths {
			if let Some(cached_model) = model_cache
				.get(candidate_asset_path)
				.into_err_with_path(|| candidate_asset_path.as_owned())?
			{
				if let Some(parent) = &cached_model.read().unwrap().model.parent {
					non_leaf_models.insert(
						parent
							.as_relative_path()
							.into_err_with_path(|| candidate_asset_path.as_owned())?
					);
				}
			}
		}

		// Now process the candidate assets
		candidate_asset_paths
			.par_iter()
			.try_for_each(|candidate_asset_path| {
				if let (true, Some(cached_model_data)) = (
					self.squashed_pack_state
						.mark_file_as_processed(candidate_asset_path),
					model_cache
						.get(candidate_asset_path)
						.into_err_with_path(|| candidate_asset_path.as_owned())?
				) {
					self.process_model_asset(
						cached_model_data,
						candidate_asset_path,
						get_file_specific_options!(
							self.file_options,
							&candidate_asset_path,
							JsonFileOptions
						),
						&model_cache,
						non_leaf_models.contains(candidate_asset_path)
					)
					.into_err_with_path(|| candidate_asset_path.as_owned())?;
				}

				Ok(())
			})
	}

	fn enumerate_models(&self) -> AHashSet<RelativePath<'static>> {
		match (
			self.global_options
				.process_not_self_referenced_and_non_vanilla_assets,
			&*self.vanilla_models
		) {
			(true, _) | (_, None) => {
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
					"assets/?*/models/{block,item}/**/?*.json"
				)
				.compile_matcher();

				AHashSet::from_iter(self.pack_files.iter().filter(|relative_path| {
					// Minecraft expects model files to be located at a valid resource
					// location (see net.minecraft.client.resources.model.ModelBakery and
					// net.minecraft.client.resources.model.ModelResourceLocation). Every
					// known mod for the Minecraft versions that support block states expands
					// on this vanilla logic, so filtering by that should be fine
					any_model_path_matcher.is_match(&**relative_path)
						&& ResourceLocation::try_from(relative_path).is_ok()
				}))
			}
			(false, Some(vanilla_models)) => {
				// Focused asset enumeration strategy: we only want vanilla assets and we know what vanilla
				// assets there are, in addition to models self-referenced by this pack. Process unreferenced
				// vanilla models right away; the block state asset processor will take care of telling us
				// to process self-referenced models. If vanilla models are referenced by other parts of the
				// pack, they won't be processed several times

				AHashSet::from_iter(
					vanilla_models
						.iter()
						// Cloning these elements is cheap because they are references to static data
						.cloned()
				)
			}
		}
	}

	#[inline]
	fn process_model_asset<'options>(
		&self,
		cached_model_data: Arc<RwLock<CachedItemOrBlockModel>>,
		asset_path: &RelativePath<'_>,
		file_options: impl Deref<Target = &'options JsonFileOptions>,
		model_cache: &ModelCache<'params, 'state, V, F>,
		has_children: bool
	) -> Result<(), InnerItemOrBlockModelAssetError> {
		let check_missing_references = !matches!(
			self.global_options.missing_reference_action,
			MissingReferenceAction::Ignore
		);
		let missing_references_are_warnings = matches!(
			self.global_options.missing_reference_action,
			MissingReferenceAction::Warn
		);

		let mut asset_warnings = tiny_vec!();

		// Several dependency handler closures can't mutably borrow the same variable at the
		// same time. To work around that, use interior mutability
		let warnings_cell = Cell::new(Some(&mut asset_warnings));

		let mut cached_model_data = cached_model_data.write().unwrap();
		self.process_model(
			&mut cached_model_data.model,
			asset_path,
			&file_options,
			model_cache,
			has_children,
			&warnings_cell,
			|model_location| {
				if !check_missing_references {
					return Ok(());
				}

				let warnings = warnings_cell.take().unwrap();

				let validation_result = helper::validate_asset_dependency(
					model_location,
					self.pack_meta,
					self.global_options,
					missing_references_are_warnings.then_some(warnings),
					|model_path| {
						Ok(self.pack_files.contains(model_path) || is_builtin_model_path(model_path))
					},
					|model_path| {
						if let Some(vanilla_models) = &*self.vanilla_models {
							Ok(vanilla_models.contains(model_path))
						} else {
							// We don't know the vanilla model set. Err on the side of caution
							Ok(false)
						}
					},
					|model_path, filtered_out_text| InnerItemOrBlockModelAssetError::MissingModel {
						path: model_path,
						__filtered_out_text: filtered_out_text
					}
				);

				warnings_cell.set(Some(warnings));

				validation_result
			},
			|texture_location| {
				if !check_missing_references {
					return Ok(());
				}

				let warnings = warnings_cell.take().unwrap();

				let validation_result = helper::validate_asset_dependency(
					texture_location,
					self.pack_meta,
					self.global_options,
					missing_references_are_warnings.then_some(warnings),
					|texture_path| Ok(self.pack_files.contains(texture_path)),
					|_texture_path| {
						// TODO validate using a vanilla texture list?
						Ok(false)
					},
					|model_path, filtered_out_text| InnerItemOrBlockModelAssetError::MissingTexture {
						path: model_path,
						__filtered_out_text: filtered_out_text
					}
				);

				warnings_cell.set(Some(warnings));

				validation_result
			}
		)?;

		// Serialize it back to a scratch file
		let processed_file = json_helper::serialize(
			&cached_model_data.model,
			self.squashed_pack_state.scratch_files_budget(),
			cached_model_data.size_hint,
			file_options.minify
		)?;

		// The lock on the model is not necessary anymore. Release it now
		// to shorten the critical region
		drop(cached_model_data);

		self.squashed_pack_state
			.add_file_to_zip(asset_path, processed_file, false)?;

		// Let interested parties know we've processed this file
		status_trace!(
			ProcessedAsset {
				strategy: match (file_options.delete_bloat, file_options.minify) {
					(false, false) => PackSquashAssetProcessingStrategy::ValidatedAndPrettified,
					(false, true) => PackSquashAssetProcessingStrategy::ValidatedAndMinified,
					(true, false) =>
						PackSquashAssetProcessingStrategy::ValidatedDebloatedAndPrettified,
					(true, true) => PackSquashAssetProcessingStrategy::ValidatedDebloatedAndMinified
				},
				warnings: asset_warnings
			},
			asset_path = asset_path.as_str()
		);

		Ok(())
	}

	fn process_model(
		&self,
		model: &mut ItemOrBlockModel<'_>,
		model_path: &RelativePath<'_>,
		file_options: &JsonFileOptions,
		model_cache: &ModelCache<V, F>,
		has_children: bool,
		asset_warnings_cell: &Cell<Option<&mut TinyVec<[Cow<'static, str>; 2]>>>,
		mut model_dependency_handler: impl FnMut(
			&ResourceLocation
		) -> Result<(), InnerItemOrBlockModelAssetError>,
		mut texture_dependency_handler: impl FnMut(
			&ResourceLocation
		) -> Result<(), InnerItemOrBlockModelAssetError>
	) -> Result<(), InnerItemOrBlockModelAssetError> {
		let mut visited_models = AHashSet::from([model_path.reborrow()]);
		let mut parent_location = model.parent.as_ref().cloned();

		while let Some(model_location) = parent_location {
			let parent_path = model_location.as_relative_path()?;

			parent_location = model_cache
				.get(&parent_path)?
				.and_then(|parent_model| parent_model.read().unwrap().model.parent.as_ref().cloned());

			if !visited_models.insert(parent_path) {
				return Err(InnerItemOrBlockModelAssetError::ModelInheritanceLoop);
			}

			// Check that the parent model reference is satisfied
			model_dependency_handler(&model_location)?;
		}

		let model_lacks_parent = visited_models.len() < 2;

		if !model_lacks_parent {
			// The ambient occlusion setting is always ignored for models with parents.
			// Set it to the default value to skip serializing it
			model.ambient_occlusion = true;
		}

		let known_texture_variables = self.validate_texture_map(
			model,
			file_options,
			model_cache,
			has_children,
			&mut texture_dependency_handler
		)?;

		if let Some(item_display_transforms) = &mut model.item_display_transforms {
			// Check that item hand transforms are in the format expected by the target version range.
			// By definition of the pack format version numbers, the possible target version ranges
			// cannot be ambiguous: we always know that either one format or the other is expected
			if self.game_version_expects_item_transforms_for_each_hand {
				let has_legacy_transforms = item_display_transforms.third_person.is_some()
					|| item_display_transforms.first_person.is_some();

				if has_legacy_transforms {
					return Err(InnerItemOrBlockModelAssetError::LegacyItemDisplayTransform);
				}
			} else {
				let has_future_transforms = item_display_transforms.third_person_left_hand.is_some()
					|| item_display_transforms.third_person_right_hand.is_some()
					|| item_display_transforms.first_person_left_hand.is_some()
					|| item_display_transforms.first_person_right_hand.is_some();

				if has_future_transforms {
					return Err(InnerItemOrBlockModelAssetError::FutureItemDisplayTransform);
				}
			}

			macro_rules! optimize_secondary_hand_transform {
				($secondary_hand_transform_name:ident, $main_hand_transform_name:ident) => {
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

			optimize_secondary_hand_transform!(third_person_left_hand, third_person_right_hand);
			optimize_secondary_hand_transform!(first_person_left_hand, first_person_right_hand);

			if file_options.delete_bloat {
				fn remove_item_transform_bloat(item_transform: Option<&mut ItemTransform>) {
					if let Some(item_transform) = item_transform {
						item_transform.bloat_fields.clear();
					}
				}

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
			if !self.game_version_supports_item_overrides_attribute {
				return Err(InnerItemOrBlockModelAssetError::FutureModelAttribute(
					"overrides"
				));
			}

			for item_override in item_overrides {
				model_dependency_handler(&item_override.model)?;

				if file_options.delete_bloat {
					item_override.bloat_fields.clear();
				}
			}
		}

		if let Some(item_gui_light) = &mut model.item_gui_light {
			if !self.game_version_supports_gui_light_attribute {
				return Err(InnerItemOrBlockModelAssetError::FutureModelAttribute(
					"gui_light"
				));
			}

			// If this model is a root model, "side" equals the value the game would use
			// if this field was omitted, so take advantage of that for optimization
			if model_lacks_parent && matches!(item_gui_light, ItemGuiLight::Side) {
				model.item_gui_light = None;
			}
		}

		if let Some(elements) = &mut model.elements {
			if !self.game_version_supports_parent_elements_override && !model_lacks_parent {
				return Err(InnerItemOrBlockModelAssetError::UnsupportedParentElementsOverride);
			}

			for element in elements {
				for face_direction in ElementFaceDirection::iter() {
					let default_face_uv = element.default_face_uv(face_direction);

					if let Some(face) = &mut element.faces[face_direction] {
						// Model element faces map cuboid faces to their textures (materials).
						//
						// Texture variables were validated above, but all the quirks related to
						// non-leaf models apply here. Leverage the knowledge we have about
						// the valid texture variables when applicable

						if let Some((defined_texture_variables, _)) = &known_texture_variables {
							if !defined_texture_variables.contains(&*face.texture) {
								return Err(
									InnerItemOrBlockModelAssetError::UndefinedElementFaceTextureVariable(
										face.texture.clone().into_owned()
									)
								);
							}
						}

						// Deal with face UV coordinates (i.e., texture coordinates): validate
						// that UVs are not missing if the game version requires them, and
						// optimize them out if they are equal to the default coordinates a
						// supporting game version would use

						if let Some(uv) = face.uv {
							if self.game_version_computes_missing_uvs && uv == default_face_uv {
								face.uv = None;
							}
						} else if !self.game_version_computes_missing_uvs {
							return Err(
								InnerItemOrBlockModelAssetError::MissingElementFaceTextureCoordinates
							);
						}
					}
				}

				if file_options.delete_bloat {
					element.bloat_fields.clear();
				}
			}
		}

		// TODO if known_texture_variables is Some, throw warnings for known undefined texture
		//      variables (second element of the tuple)
		if let Some((_, undefined_texture_variables)) = known_texture_variables {
			let asset_warnings = asset_warnings_cell.take().unwrap();

			asset_warnings.extend(
				undefined_texture_variables.map(|undefined_texture_variable| {
					InnerItemOrBlockModelAssetError::UndefinedTextureVariable(
						undefined_texture_variable
					)
					.to_string()
					.into()
				})
			);

			asset_warnings_cell.set(Some(asset_warnings));
		}

		if file_options.delete_bloat {
			model.bloat_fields.clear();
		}

		Ok(())
	}

	fn validate_texture_map(
		&self,
		model: &mut ItemOrBlockModel<'_>,
		file_options: &JsonFileOptions,
		model_cache: &ModelCache<V, F>,
		has_children: bool,
		mut texture_dependency_handler: impl FnMut(
			&ResourceLocation
		) -> Result<(), InnerItemOrBlockModelAssetError>
	) -> Result<
		Option<(AHashSet<CompactString>, impl Iterator<Item = CompactString>)>,
		InnerItemOrBlockModelAssetError
	> {
		// A model texture map may contain declarations for:
		//
		// - Texture variables used by element faces and the texture map on this model.
		// - Texture variables used by element faces and texture maps on ancestor and
		//   descendant models.
		// - Texture variables unused by element faces and texture maps on this,
		//   ancestor and descendant models.
		//
		// For all of these cases, we want to validate that the texture variables
		// resolve to a valid texture (material), because even if they are unused,
		// it is a mistake for them to hold invalid references. Minecraft is too
		// lenient and always lets people shoot themselves in the foot, not warning
		// a thing unless they are used in element faces.
		//
		// The validation described above can be done safely only if blockstates
		// or hardcoded item registries reference this model directly (causing
		// Minecraft to resolve texture variables from this model, allowing for
		// parent models to reference variables defined in children models, and
		// children models to reference variables defined in some parent).
		// However, that can't be known when optimizing most real-world packs,
		// so as a proxy for the "Minecraft parses this model as if it were a
		// leaf in the tree" condition, we may apply it for models that 1) cannot
		// have unknown children (e.g., models that declare those as parents in other
		// packs) and 2) are leaf models (with no children). This proxy condition
		// is correct and useful in practice for packs that do not assume models
		// from other packs (including the vanilla pack) and that do not contain
		// any model that plays a dual role as "parsed as leaf" and as parent
		// of other models

		// TODO surround the following condition with parentheses, add
		//      || was_referenced_by_blockstate (|| was_referenced_by_item_registry?)
		let is_parsed_as_leaf_model = !self
			.global_options
			.accept_references_to_unknown_packs_and_mods
			&& file_options.assume_only_childless_models_are_baked
			&& !has_children;

		if !is_parsed_as_leaf_model {
			return Ok(None);
		}

		let mut defined_texture_variables = AHashSet::new();
		let mut texture_variables_dependencies = AHashMap::new();

		let mut current_texture_map = model.texture_map.as_ref().cloned();
		let mut next_parent = model.parent.as_ref().cloned();

		let mut has_unknown_parents = false;
		while let Some(texture_map) = current_texture_map.as_ref() {
			for (texture_variable, texture_reference) in texture_map.iter() {
				// If this texture variable was already defined or resolved to a
				// dependent texture variable, ignore redefinitions as the game does
				if defined_texture_variables.contains(texture_variable)
					|| texture_variables_dependencies.contains_key(texture_variable)
				{
					continue;
				}

				fn define_texture_variable(
					texture_variable: &CompactString,
					defined_texture_variables: &mut AHashSet<CompactString>,
					texture_variables_dependencies: &mut AHashMap<CompactString, CompactString>
				) {
					let mut resolved_texture_variables =
						tiny_vec!([_; 2] => texture_variable.clone());

					while let Some(resolved_texture_variable) = resolved_texture_variables.pop() {
						resolved_texture_variables.extend(
							texture_variables_dependencies
								.extract_if(|_, child_texture_variable| {
									*child_texture_variable == resolved_texture_variable
								})
								.map(|(parent_texture_variable, _)| parent_texture_variable)
						);
						defined_texture_variables.insert(resolved_texture_variable);
					}
				}

				match texture_reference {
					TextureLocationOrReference::Location(texture_location) => {
						texture_dependency_handler(texture_location)?;
						define_texture_variable(
							texture_variable,
							&mut defined_texture_variables,
							&mut texture_variables_dependencies
						);
					}
					TextureLocationOrReference::Reference(child_texture_variable) => {
						if defined_texture_variables.contains(child_texture_variable) {
							define_texture_variable(
								texture_variable,
								&mut defined_texture_variables,
								&mut texture_variables_dependencies
							);
						} else {
							texture_variables_dependencies
								.insert(texture_variable.clone(), child_texture_variable.clone());
						}
					}
				}
			}

			if let Some(parent_model) = next_parent
				.as_ref()
				.map(|parent_location| parent_location.as_relative_path())
				.transpose()?
				// Models in resource packs can't override builtin models, and builtin models don't
				// define texture variables or have parents, so we can assume that they are hierarchy
				// terminators
				.filter(|parent_path| !is_builtin_model_path(parent_path))
				.map(|parent_path| {
					let parent_model = model_cache.get(&parent_path);
					if let Ok(None) = &parent_model {
						has_unknown_parents = true;
					}
					parent_model
				})
				.transpose()?
				.flatten()
			{
				let parent_model = &parent_model.read().unwrap().model;
				current_texture_map = parent_model.texture_map.as_ref().cloned();
				next_parent = parent_model.parent.as_ref().cloned();
			} else {
				current_texture_map = None;
			}
		}

		Ok(if has_unknown_parents {
			None
		} else {
			Some((
				defined_texture_variables,
				texture_variables_dependencies.into_keys()
			))
		})
	}

	/*/// Resolves a texture variable to its corresponding texture resource location, visiting
	/// ancestor models as necessary. This method only yields accurate results when run on
	/// texture maps belonging to models that are directly parsed by Minecraft as item or block
	/// models, which is not the case for ancestor models that are only used as parents of other
	/// models.
	///
	/// The resolution procedure is guaranteed to yield either a texture variable resolved to
	/// a resource location or an error, in case the resolution cannot be done.
	fn resolve_texture_variable_to_resource_location<'data>(
		&self,
		mut texture_variable: Arc<TextureLocationOrReference<'data>>,
		texture_map: Arc<AHashMap<Cow<'data, str>, Arc<TextureLocationOrReference<'data>>>>,
		mut parent_model: Option<Arc<ResourceLocation<'data>>>,
		model_cache: &Mutex<Cache<RelativePath<'_>, Arc<RwLock<CachedItemOrBlockModel<'static>>>>>
	) -> Result<Arc<TextureLocationOrReference<'data>>, InnerItemOrBlockModelAssetError> {
		let mut texture_map = Some(texture_map);

		// The fact that texture variables can refer to other texture variables is not
		// documented in the Minecraft Wiki, so it's very likely that few packs in the
		// wild use it. We don't use TinyVec here because the Default trait implementation
		// for Arc does heap allocations, so using a Vec actually minimizes heap allocations
		// due to not Default-initializing any backing array
		let mut visited_variables = Vec::with_capacity(2);

		loop {
			match &*texture_variable {
				TextureLocationOrReference::Location(_) => {
					// This texture variable resolves to a texture resource location. This is
					// the final information we're looking for: variable resolution is complete
					return Ok(texture_variable);
				}
				TextureLocationOrReference::Reference(texture_var) => {
					// A texture variable reference loop may happen if we visit the same variable twice,
					// e.g. a -> b -> a. The game rejects such infinite loops, and so should we.
					// Because this vector is expected to be very small, the constant factors in this
					// O(n) lookup should offset the asymptotic O(1) cost offered by a hash table set
					if visited_variables.contains(&texture_variable) {
						return Err(
							InnerItemOrBlockModelAssetError::TextureVariableReferenceLoop(
								texture_var.to_string()
							)
						);
					}

					visited_variables.push(Arc::clone(&texture_variable));

					match (
						texture_map
							.as_ref()
							.and_then(|texture_map| texture_map.get(texture_var)),
						&parent_model
					) {
						(Some(resolved_texture_variable), _) => {
							// This texture variable resolves to another texture variable. Follow the
							// dependency chain, like the game does
							texture_variable = Arc::clone(resolved_texture_variable);
						}
						(None, Some(next_model)) => {
							// The texture variable is not defined in this model, but it may be
							// defined in some ancestor. Continue the search on the parent model
							let next_model_path = next_model.as_relative_path()?;
							if let Some(next_model) = self
								.deserialize_model_asset_from_cache(&next_model_path, model_cache)?
							{
								let next_model = next_model.read();
								texture_map = next_model.model.texture_map.as_ref().cloned();
								parent_model = next_model.model.parent.as_ref().cloned();
							} else {
								// The model ancestors are unknown or built-in. They may not exist, they
								// may exist but be of another pack and either define the texture variable
								// or not. The first situation is an error; in the second, we can't be sure of
								// the children either (other packs may define this model as a parent).
								// Built-in models don't define texture variables to use, so error out in
								// any case. Note that resolving texture variables may yield incorrect
								// results when there are children or parents we don't know about, so
								// we shouldn't attempt variable resolution in that case in the first
								// place
								return Err(
									InnerItemOrBlockModelAssetError::UndefinedTransitiveTextureVariableReference(
										texture_var.to_string()
									)
								);
							}
						}
						(None, None) => {
							// The texture variable is not defined in this model, and there are no
							// ancestors to look up, so yield a resolution failure
							return Err(
								InnerItemOrBlockModelAssetError::UndefinedTransitiveTextureVariableReference(
									texture_var.to_string()
								)
							);
						}
					}
				}
			}
		}
	}*/
}

fn is_builtin_model_path(relative_path: &RelativePath<'_>) -> bool {
	// See the ItemOrBlockModel struct documentation for more information about these
	// built-in models
	matches!(
		relative_path.as_str(),
		"assets/minecraft/models/builtin/generated.json"
			| "assets/minecraft/models/builtin/missing.json"
			| "assets/minecraft/models/builtin/entity.json"
	)
}
