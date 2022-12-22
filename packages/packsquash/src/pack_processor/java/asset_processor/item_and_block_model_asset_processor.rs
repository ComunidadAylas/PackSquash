use std::cell::Cell;
use crate::pack_processor::java::asset_processor::compile_hardcoded_pack_file_glob_pattern;
use crate::pack_processor::java::asset_processor::data::vanilla_model_list;
use crate::pack_processor::java::asset_processor::helper::{self, json_helper};
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
use crate::util::patricia_set_util::{PatriciaSetContainsRelativePathExt, PatriciaSetRelativePathIterExt};
use crate::util::range_bounds_intersect::RangeBoundsIntersectExt;
use crate::vfs::VirtualFileSystem;
use crate::PackSquashAssetProcessingStrategy;
use ahash::AHashSet;
use once_cell::sync::Lazy;
use packsquash_options::{minecraft_version, FileOptionsMap, GlobalOptions, JsonFileOptions, MissingReferenceAction};
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
	//#[error("The texture variable {0} was already referenced by a texture variable. Please remove the reference loop")]
	//TextureVariableReferenceLoop(String),
	//#[error("The texture variable {0} refers to an undefined texture variable")]
	//UndefinedTransitiveTextureVariableReference(String),
	#[error("The model defines item display transforms in single-hand format, but a Minecraft version range that expects the dual-hand format is targeted")]
	LegacyItemDisplayTransform,
	#[error("The model defines item display transforms in the dual-hand format, but a Minecraft version range that expects the single-hand format is targeted")]
	FutureItemDisplayTransform,
	#[error("The model attribute {0} is not implemented for the target Minecraft version range")]
	FutureModelAttribute(&'static str),
	#[error("The model overrides elements from its parent, but this is not supported by the target Minecraft version range")]
	UnsupportedParentElementsOverride,
	//#[error("The texture variable {0} was referenced by an element face, but it is undefined")]
	//UndefinedElementFaceTextureVariable(String),
	#[error("An element face is missing texture UV coordinates, but this is not supported by the target Minecraft version range")]
	MissingElementFaceTextureCoordinates,
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
	game_version_supports_models: bool,
	game_version_supports_item_overrides_attribute: bool,
	game_version_supports_parent_elements_override: bool,
	game_version_expects_item_transforms_for_each_hand: bool,
	game_version_computes_missing_uvs: bool,
	game_version_supports_gui_light_attribute: bool
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
		game_version_supports_models,
		game_version_supports_item_overrides_attribute,
		game_version_supports_parent_elements_override,
		game_version_expects_item_transforms_for_each_hand,
		game_version_computes_missing_uvs,
		game_version_supports_gui_light_attribute
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
		if !self.game_version_supports_models {
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
		let check_missing_references = !matches!(
			self.global_options.missing_reference_action,
			MissingReferenceAction::Ignore
		);
		let missing_references_are_warnings = matches!(
			self.global_options.missing_reference_action,
			MissingReferenceAction::Warn
		);

		let mut asset_warnings = tiny_vec!();

		json_helper::deserialize::<ItemOrBlockModelRepresentative, _, _, _>(
			asset_path,
			self.vfs,
			self.squashed_pack_state,
			false,
			self.global_options.always_allow_json_comments,
			"json",
			|outcome: JsonAssetDeserializeOutcome<'_, ItemOrBlockModel<'_>>| match outcome {
				JsonAssetDeserializeOutcome::Value {
					value: mut model,
					canonical_path: canonical_asset_path,
					size_hint,
					..
				} => {
					// Several dependency handler closures can't mutably borrow the same variable at the
					// same time. To work around that, use interior mutability
					let warnings_cell = Cell::new(Some(&mut asset_warnings));

					self.process_model(
						&mut model,
						&file_options,
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
									Ok(self.pack_files.has_relative_path(model_path)
										|| self.pack_files.has_relative_path(
											&model_path.with_comment_extension_suffix()
										) || is_builtin_model_path(model_path))
								},
								|model_path| {
									if let Some(vanilla_models) = &*self.vanilla_models {
										Ok(vanilla_models.contains(model_path))
									} else {
										// We don't know the vanilla model set. Err on the side of caution
										Ok(false)
									}
								},
								|model_path, filtered_out_text| {
									InnerItemOrBlockModelAssetError::MissingModel {
										path: model_path,
										__filtered_out_text: filtered_out_text
									}
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
								|texture_path| Ok(self.pack_files.contains(texture_path.as_str())),
								|_texture_path| {
									// TODO validate using a vanilla texture list?
									Ok(false)
								},
								|model_path, filtered_out_text| {
									InnerItemOrBlockModelAssetError::MissingTexture {
										path: model_path,
										__filtered_out_text: filtered_out_text
									}
								}
							);

							warnings_cell.set(Some(warnings));

							validation_result
						}
					)?;

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
							warnings: asset_warnings
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
							warnings: asset_warnings
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
		mut model_dependency_handler: impl FnMut(
			&ResourceLocation
		) -> Result<(), InnerItemOrBlockModelAssetError>,
		mut texture_dependency_handler: impl FnMut(
			&ResourceLocation
		) -> Result<(), InnerItemOrBlockModelAssetError>
	) -> Result<(), InnerItemOrBlockModelAssetError> {
		let model_lacks_parent;
		if let Some(parent_model) = &model.parent {
			// Check that the parent model reference is satisfied
			model_dependency_handler(parent_model)?;

			// The ambient occlusion setting is always ignored for models with parents.
			// Set it to the default value to skip serializing it
			model.ambient_occlusion = true;

			model_lacks_parent = false;
		} else {
			model_lacks_parent = true;
		}

		// A model texture map may contain declarations for:
		//
		// - Texture variables used by element faces and the texture map on this model.
		// - Texture variables used by element faces and texture maps on descendant models.
		// - Texture variables used by element faces and texture maps on ancestor and
		//   descendant models.
		// - Texture variables unused by element faces and texture maps on this, ancestor or
		//   descendant models.
		//
		// For all of these cases, we want to validate that the texture variables
		// resolve to a valid texture (material), because even if they are unused,
		// it is a mistake for them to hold invalid references. Minecraft is too
		// lenient and always lets people shoot themselves in the foot, not warning
		// a thing unless they are used in element faces.
		//
		// For unused texture variables in particular, removing them from the map
		// is an optimization we want to do, because they won't contribute to the
		// final model look.
		//
		// The code below does not consider the model parent hierarchy at all, because
		// that does not play nice with parallel model processing. The ugly approaches
		// to parents include:
		//
		// - Following the parent chain for each model, potentially processing common,
		//   complex ancestor models much more times than necessary.
		// - Memoize the visited deserialized model data to a cache. This requires
		//   cloning or moving the data (the source buffer does not live for a static
		//   lifetime) and thread synchronization, so it may not be more performant as
		//   it seems in the first place when compared to the naive approach.
		//
		// TODO implement unused variable removal (optimization for the third case above)
		//      and parent hierarchy (see comments below for details where this would be useful)

		if let Some(texture_map) = &model.texture_map {
			for texture_location_or_reference in texture_map.values() {
				match texture_location_or_reference {
					TextureLocationOrReference::Location(texture_location) => {
						texture_dependency_handler(texture_location)?
					}
					TextureLocationOrReference::Reference(_texture_reference) => {
						// TODO we can't validate anything here because the reference may be
						//      defined in a child or parent model
						/*if let Some(texture_location) = resolve_texture_variable_to_resource_location(
							&**texture_reference,
							texture_map
						)? {
							texture_dependency_handler(texture_location)?
						} else if model_lacks_parent {
							// If this model has no parents, there is no way that this reference can
							// be resolved. Otherwise, the parent may resolve it, so it is plausible
							return Err(InnerItemOrBlockModelAssetError::UndefinedTransitiveTextureVariableReference(texture_reference.clone().into_owned()));
						}*/
					}
				}
			}
		}

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
						// Due to the texture_map field validation code, we can assume that,
						// if a texture map defines a variable, that variable definition is
						// valid, so we don't need to repeat the validation process again.
						//
						// Child models can inherit texture variables defines by their parents,
						// and parents can use variables defined in all of their children.
						// However, we can't properly validate that due to a lack of parent
						// hierarchy traversal support.
						//
						// TODO implement the above, or think about compromising by detecting
						//      Blockbench "#missing" placeholder variable

						/*let texture_map_defines_texture_variable =
							model.texture_map.as_ref().map_or_else(
								|| false,
								|texture_map| {
									texture_map.contains_key(
										face.texture.strip_prefix('#').unwrap_or(&*face.texture)
									)
								}
							);

						if model_lacks_parent && !texture_map_defines_texture_variable {
							return Err(
								InnerItemOrBlockModelAssetError::UndefinedElementFaceTextureVariable(
									face.texture.clone().into_owned()
								)
							);
						}*/

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

		if file_options.delete_bloat {
			model.bloat_fields.clear();
		}

		Ok(())
	}
}

// TODO remove or refactor this as appropriate
/*fn resolve_texture_variable_to_resource_location<'map, 'data, V: Into<Cow<'map, str>>>(
	texture_variable: V,
	texture_map: &'map AHashMap<Cow<'data, str>, TextureLocationOrReference<'data>>
) -> Result<Option<&'map ResourceLocation<'data>>, InnerItemOrBlockModelAssetError> {
	let mut texture_variable = texture_variable.into();

	// The fact that texture variables can refer to other texture variables is not
	// documented in the Minecraft Wiki, so it's very likely that few packs in the
	// wild use it
	let mut visited_variables = tiny_vec!([Cow<'_, str>; 1]);

	loop {
		// A texture variable reference loop may happen if we visit the same variable twice,
		// e.g. a -> b -> a. The game rejects such infinite loops, and so should we.
		// Because this vector is expected to be very small, the constant factors in this
		// O(n) lookup should offset the asymptotic O(1) cost of a set
		if visited_variables.contains(&texture_variable) {
			return Err(
				InnerItemOrBlockModelAssetError::TextureVariableReferenceLoop(
					texture_variable.into_owned()
				)
			);
		}

		// Cloning here is efficient for borrowed strings
		visited_variables.push(texture_variable.clone());

		match texture_map.get(&texture_variable) {
			Some(TextureLocationOrReference::Location(texture_location)) => {
				// This texture variable resolves to a texture resource location. This is
				// the final information we're looking for: variable resolution is complete
				return Ok(Some(texture_location));
			}
			Some(TextureLocationOrReference::Reference(texture_reference)) => {
				// This texture variable resolves to another texture variable. Follow the
				// dependency chain, like the game does
				texture_variable = Cow::Borrowed(&**texture_reference);
			}
			None => {
				// No dependency chain to follow. The variable can't be resolved, but this
				// may be okay if the model has a parent we are not taking into account in
				// the texture map
				return Ok(None);
			}
		}
	}
}*/

fn remove_item_transform_bloat(item_transform: Option<&mut ItemTransform>) {
	if let Some(item_transform) = item_transform {
		item_transform.bloat_fields.clear();
	}
}

fn is_builtin_model_path(relative_path: &RelativePath) -> bool {
	// See the ItemOrBlockModel struct documentation for more information about these
	// built-in models
	matches!(
		relative_path.as_str(),
		"assets/minecraft/models/builtin/generated.json"
			| "assets/minecraft/models/builtin/missing.json"
			| "assets/minecraft/models/builtin/entity.json"
	)
}
