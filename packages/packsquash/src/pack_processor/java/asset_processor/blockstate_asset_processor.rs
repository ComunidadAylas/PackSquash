use std::{
	borrow::Cow,
	collections::HashSet,
	io,
	io::{Read, Seek},
	mem,
	ops::Deref
};

use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use once_cell::sync::Lazy;
use packsquash_options::{
	minecraft_version, FileOptionsMap, GlobalOptions, JsonFileOptions, MissingReferenceAction
};
use rayon::prelude::*;
use thiserror::Error;
use tinyvec::{tiny_vec, TinyVec};

use self::blockstate::{
	BlockState, BlockStateRepresentative, CompositeBooleanCondition, Condition, Property,
	PropertyPredicate, VariantList
};
use super::compile_hardcoded_pack_file_glob_pattern;
use crate::{
	pack_processor::java::{
		asset_processor::{
			data::{
				vanilla_blockstate_list, vanilla_blockstate_property_list,
				vanilla_blockstate_property_list::BlockStatePropertyType
			},
			helper,
			helper::json_helper::{self, JsonAssetDeserializeOutcome},
			item_and_block_model_asset_processor::{
				ItemAndBlockModelAssetProcessor, ItemOrBlockModelAssetError
			}
		},
		pack_meta::PackMeta,
		resource_location::{ResourceLocation, ResourceLocationError}
	},
	relative_path::{InvalidPathError, RelativePathPatriciaSet},
	squash_zip::SquashZipError,
	squashed_pack_state::SquashedPackState,
	util::{lazy_or_eager::LazyOrEager, range_bounds_intersect::RangeBoundsIntersectExt},
	vfs::VirtualFileSystem,
	PackSquashAssetProcessingStrategy, RelativePath
};

mod blockstate;

#[derive(Error, Debug)]
pub enum InnerBlockStateAssetError {
	#[error("Missing model definition strategy")]
	MissingModelDefinitionStrategy,
	#[error(
		"The model definition strategy {0} is not implemented for the target Minecraft version range"
	)]
	FutureModelDefinitionStrategy(&'static str),
	#[error(
		"Specifying both variants and multipart is redundant and leads to unexpected results. \
		Please use a single model definition strategy"
	)]
	RedundantModelDefinitionStrategies,
	#[error("Invalid resource location: {0}")]
	InvalidResourceLocation(#[from] ResourceLocationError),
	#[error("Invalid pack file path: {0}")]
	InvalidRelativePath(#[from] InvalidPathError),
	#[error("Unrecognized block state property: {0}")]
	UnrecognizedPropertyName(Cow<'static, str>),
	#[error("Invalid value {value} for block state property {name}: Expected {expected_types}")]
	InvalidPropertyValue {
		name: String,
		value: String,
		expected_types: String
	},
	#[error("A variant referenced a model at {path}, but it is not a known model and it was not found in the pack{__filtered_out_text}")]
	MissingModel {
		path: RelativePath<'static>,
		#[doc(hidden)]
		__filtered_out_text: &'static str
	},
	#[error("{0}")]
	InvalidModel(#[from] ItemOrBlockModelAssetError),
	#[error("{0}")]
	JsonDeserializationError(#[from] json_helper::JsonDeserializationError),
	/// Thrown when some error occurs in a ZIP file operation.
	#[error("Error while performing a ZIP file operation: {0}")]
	SquashZip(#[from] SquashZipError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

define_path_wrapper_err!(InnerBlockStateAssetError: BlockStateAssetError);

// TODO docs
pub struct BlockStateAssetProcessor<
	'params,
	'state,
	V: VirtualFileSystem + ?Sized,
	F: Read + Seek + Send
> {
	vfs: &'params V,
	pack_meta: &'params PackMeta,
	pack_files: &'params RelativePathPatriciaSet<'static>,
	global_options: &'params GlobalOptions<'params>,
	file_options: &'params FileOptionsMap<'params>,
	squashed_pack_state: &'params SquashedPackState<'state, 'state, F>,
	game_version_supports_blockstates: bool,
	is_game_version_before_the_flattening: bool,
	game_version_has_multipart_model_strategy: bool
}

impl<'params, 'state, V: VirtualFileSystem + ?Sized, F: Read + Seek + Send>
	BlockStateAssetProcessor<'params, 'state, V, F>
{
	pub fn new(
		vfs: &'params V,
		pack_meta: &'params PackMeta,
		pack_files: &'params RelativePathPatriciaSet<'static>,
		global_options: &'params GlobalOptions,
		file_options: &'params FileOptionsMap,
		squashed_pack_state: &'params SquashedPackState<'state, 'state, F>
	) -> Self {
		let game_version_range = pack_meta.game_version_range();

		let game_version_supports_blockstates =
			(minecraft_version!(1, 8)..).intersects(game_version_range);
		let is_game_version_before_the_flattening =
			(..minecraft_version!(1, 13)).intersects(game_version_range);

		let game_version_has_multipart_model_strategy =
			(minecraft_version!(1, 9)..).intersects(game_version_range);

		Self {
			vfs,
			pack_meta,
			pack_files,
			global_options,
			file_options,
			squashed_pack_state,
			game_version_supports_blockstates,
			is_game_version_before_the_flattening,
			game_version_has_multipart_model_strategy
		}
	}

	pub fn process(
		&self,
		model_asset_processor: &ItemAndBlockModelAssetProcessor<
			V,
			F,
			impl FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send
		>
	) -> Result<(), BlockStateAssetError> {
		if !self.game_version_supports_blockstates {
			return Ok(());
		}

		let game_version_range = self.pack_meta.game_version_range();

		let valid_vanilla_block_properties = Lazy::new(|| {
			vanilla_blockstate_property_list::matching_for_version_range(game_version_range)
		});

		let vanilla_blockstates =
			vanilla_blockstate_list::matching_for_version_range(game_version_range);

		let process_asset = |(candidate_asset_path, is_vanilla)| {
			self.process_blockstate_asset(
				&candidate_asset_path,
				get_file_specific_options!(self.file_options, &candidate_asset_path, JsonFileOptions),
				// For now, we only want to validate block properties of vanilla block states.
				// Make sure that non-vanilla block states do not get any properties, to avoid
				// potential clashes with modded block states
				if is_vanilla {
					LazyOrEager::Lazy(&valid_vanilla_block_properties)
				} else {
					LazyOrEager::Eager(None)
				},
				model_asset_processor
			)
			.into_err_with_path(|| candidate_asset_path.into_owned())
		};

		match (
			self.global_options
				.process_not_self_referenced_and_non_vanilla_assets,
			vanilla_blockstates
		) {
			(true, vanilla_blockstates @ Some(_)) | (_, vanilla_blockstates @ None) => {
				// Exhaustive asset enumeration strategy: we either want to include non-vanilla assets
				// or we don't know what vanilla assets there are, so iterate over all the plausible
				// assets

				// Vanilla-format block states can be found in other namespaces in modded environments
				// (Forge and Fabric), but under the same paths on those namespaces. The unwritten agreement
				// is to rely on folders within namespaces to distinguish between asset types, while the
				// namespaces themselves are only used as mod IDs. Therefore, in practice it does not happen
				// that a mod-specific namespace reads non-blockstate assets from the block states path
				let any_blockstate_path_matcher =
					compile_hardcoded_pack_file_glob_pattern("assets/?*/blockstates/**/?*.json")
						.compile_matcher();

				self.pack_files
					.iter()
					.par_bridge()
					.filter_map(|relative_path| {
						// Minecraft expects block state files to be located at a valid resource
						// location (see net.minecraft.client.resources.model.ModelBakery and
						// net.minecraft.client.resources.model.ModelResourceLocation). Every
						// known mod for the Minecraft versions that support block states expands
						// on this vanilla logic, so filtering by that should be fine
						if any_blockstate_path_matcher.is_match(&*relative_path)
							&& ResourceLocation::try_from(&relative_path).is_ok()
						{
							let is_vanilla = vanilla_blockstates.as_ref().map_or_else(
								|| false,
								|vanilla_blockstates| vanilla_blockstates.contains(&relative_path)
							);

							Some((relative_path, is_vanilla))
						} else {
							None
						}
					})
					.try_for_each(process_asset)
			}
			(false, Some(vanilla_blockstates)) => {
				// Focused asset enumeration strategy: we only want vanilla assets and we know what vanilla
				// assets there are, so process them, and only them, right away

				// The Rust compiler needs some help to figure out that we want to unwrap the owned HashMap
				// from the AHashMap deref facade, which is necessary to be able to move it to the
				// into_par_iter() call
				<AHashSet<RelativePath<'_>> as Into<HashSet<_, _>>>::into(vanilla_blockstates)
					.into_par_iter()
					.map(|relative_path| (relative_path, true))
					.try_for_each(process_asset)
			}
		}
	}

	#[inline]
	fn process_blockstate_asset<'options>(
		&self,
		asset_path: &RelativePath,
		file_options: impl Deref<Target = &'options JsonFileOptions>,
		valid_block_properties: impl Deref<
			Target = Option<
				AHashMap<&'static str, AHashMap<&'static str, TinyVec<[BlockStatePropertyType; 1]>>>
			>
		>,
		model_asset_processor: &ItemAndBlockModelAssetProcessor<
			V,
			F,
			impl FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send
		>
	) -> Result<(), InnerBlockStateAssetError> {
		let check_missing_references = !matches!(
			self.global_options.missing_reference_action,
			MissingReferenceAction::Ignore
		);
		let missing_references_are_warnings = matches!(
			self.global_options.missing_reference_action,
			MissingReferenceAction::Warn
		);

		let mut asset_warnings = tiny_vec!();

		json_helper::deserialize::<BlockStateRepresentative, _, _, _>(
			asset_path,
			self.vfs,
			self.squashed_pack_state,
			true,
			false,
			self.global_options.always_allow_json_comments,
			|outcome: JsonAssetDeserializeOutcome<BlockState<'_>>| match outcome {
				JsonAssetDeserializeOutcome::Value {
					value: mut blockstate,
					size_hint,
					..
				} => {
					// The file name without the extension matches the block name
					let block_name = asset_path.file_name().unwrap().rsplit_once('.').unwrap().0;

					// Validate and optimize the deserialized block state data
					self.process_blockstate(
						&mut blockstate,
						block_name,
						&file_options,
						valid_block_properties,
						|model_location| {
							helper::validate_asset_dependency(
								model_location,
								self.pack_meta,
								self.global_options,
								missing_references_are_warnings.then_some(&mut asset_warnings),
								|model_path| {
									let model_exists = |candidate_model_path| {
										let exists = self.pack_files.contains(candidate_model_path);

										// If the model exists, it should be able to be processed
										if exists {
											model_asset_processor
												.models_referenced_by_blockstates
												.lock()
												.unwrap()
												.insert(model_path.as_owned());
										}

										Ok::<_, InnerBlockStateAssetError>(exists)
									};

									// Always do the check, even if we're not interested in erroring
									// or warning, to get the side effect of calling process_model_asset
									// on existing models. Due to the logic OR operator short-circuiting
									// property, we'll avoid checking the second path if the first candidate
									// exists, which is desirable and saves operations
									let exists = model_exists(model_path)?;

									Ok(!check_missing_references || exists)
								},
								|model_path| {
									if let Some(vanilla_models) =
										&*model_asset_processor.vanilla_models
									{
										Ok(vanilla_models.contains(model_path))
									} else {
										// We don't know the vanilla model set. Err on the side of caution
										Ok(false)
									}
								},
								|model_path, filtered_out_text| {
									InnerBlockStateAssetError::MissingModel {
										path: model_path,
										__filtered_out_text: filtered_out_text
									}
								}
							)
						}
					)?;

					// Serialize it back to a scratch file
					let processed_file = json_helper::serialize(
						blockstate,
						self.squashed_pack_state.scratch_files_budget(),
						size_hint,
						file_options.minify
					)?;

					self.squashed_pack_state
						.add_file_to_zip(asset_path, processed_file, false)?;

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
				JsonAssetDeserializeOutcome::NotFound => {
					// Ignore files that do not exist (i.e., opening them returned a "not found" error):
					// they most likely are candidate paths the pack does not contain
					Ok(())
				}
				JsonAssetDeserializeOutcome::FreshInPreviousZip
				| JsonAssetDeserializeOutcome::AlreadyProcessed => unreachable!()
			}
		)?
	}

	#[inline]
	fn process_blockstate<'options>(
		&self,
		blockstate: &mut BlockState,
		block_name: &str,
		file_options: &impl Deref<Target = &'options JsonFileOptions>,
		valid_block_properties: impl Deref<
			Target = Option<
				AHashMap<&'static str, AHashMap<&'static str, TinyVec<[BlockStatePropertyType; 1]>>>
			>
		>,
		mut model_dependency_handler: impl FnMut(
			&ResourceLocation
		) -> Result<(), InnerBlockStateAssetError>
	) -> Result<(), InnerBlockStateAssetError> {
		// First, run more through validation and optimization
		match (&mut blockstate.variants, &mut blockstate.multipart_variants) {
			(None, None) => {
				// The game requires either "variants" or "multipart" to be present
				return Err(InnerBlockStateAssetError::MissingModelDefinitionStrategy);
			}
			(Some(variants), None) => {
				for (variant_predicate, variant_list) in variants {
					// In versions < 1.13, the special "normal" property was used to match
					// no variants. Check that special property is only used in the versions
					// it should be used (newer ones just use an empty string)
					if variant_predicate.is_legacy_invariant
						&& !self.is_game_version_before_the_flattening
					{
						return Err(InnerBlockStateAssetError::UnrecognizedPropertyName(
							Cow::Borrowed("normal")
						));
					}

					// Validate property-value pairs used for selecting lists of variants.
					// Do not error out if we find no property data for this block: it may
					// be non-vanilla, or we may lack data about a specific vanilla version
					if let Some(valid_block_properties) = valid_block_properties
						.as_ref()
						.and_then(|valid_block_properties| valid_block_properties.get(block_name))
					{
						for (property_name, property_value) in &variant_predicate.property_value_pairs
						{
							let valid_property_value_types = valid_block_properties
								.get(&**property_name)
								.ok_or_else(|| {
									InnerBlockStateAssetError::UnrecognizedPropertyName(
										property_name.clone().into()
									)
								})?;

							validate_property_value_for_types(
								property_name,
								property_value,
								valid_property_value_types
							)?;
						}
					}

					// Optimize the lists of variants themselves
					self.optimize_variant_list(
						variant_list,
						file_options,
						&mut model_dependency_handler
					)?;
				}
			}
			(None, Some(multipart_variants)) => {
				// Check compatibility with the game first
				if !self.game_version_has_multipart_model_strategy {
					return Err(InnerBlockStateAssetError::FutureModelDefinitionStrategy(
						"multipart"
					));
				}

				let valid_block_properties = valid_block_properties
					.as_ref()
					.and_then(|valid_block_properties| valid_block_properties.get(block_name));

				for multipart_variant_selector in multipart_variants {
					// Validate and optimize the selector condition, if any
					if let Some(condition) = &mut multipart_variant_selector.condition {
						if let Some(valid_block_properties) = valid_block_properties {
							validate_blockstate_multipart_condition(
								condition,
								valid_block_properties
							)?;
						}

						// This optimization can be done even if we don't know the properties
						// the block has
						optimize_and_composite(condition);
					}

					// Optimize the list of variants used by this selector
					self.optimize_variant_list(
						&mut multipart_variant_selector.variants,
						file_options,
						&mut model_dependency_handler
					)?;

					// Delete unknown keys that the game won't read
					if file_options.delete_bloat {
						multipart_variant_selector.bloat_fields.clear();
					}
				}
			}
			(Some(_), Some(_)) => {
				// The game technically accepts both "variants" and "multipart", but "variants"
				// takes precedence and predicates declared in only one of the strategies may
				// be ignored. Moreover, using one of them is enough to implement whatever
				// model selection logic that is desired. Thus, mixing them can be considered
				// an usage error: I can't come up with any case in which it is a good idea
				return Err(InnerBlockStateAssetError::RedundantModelDefinitionStrategies);
			}
		}

		// Delete unknown keys that the game won't read
		if file_options.delete_bloat {
			blockstate.bloat_fields.clear();
		}

		Ok(())
	}

	fn optimize_variant_list<'options, 'data>(
		&self,
		variant_list: &mut VariantList,
		file_options: &impl Deref<Target = &'options JsonFileOptions>,
		mut model_dependency_handler: impl FnMut(
			&ResourceLocation
		) -> Result<(), InnerBlockStateAssetError>
	) -> Result<(), InnerBlockStateAssetError> {
		// Optimize list of variants to a single variant if it contains a single element
		if variant_list.len() == 1 && matches!(variant_list, VariantList::SeveralVariants(_)) {
			*variant_list = VariantList::SingleVariant(mem::take(&mut variant_list[0]));
		}

		// Debloat each variant if applicable, and note the model it uses
		for variant in variant_list.iter_mut() {
			let model_location = ResourceLocation::new(
				self.pack_meta.pack_type(),
				&*variant.model,
				Some(if self.is_game_version_before_the_flattening {
					"models/block"
				} else {
					"models"
				}),
				Some("json")
			)?;

			model_dependency_handler(&model_location)?;

			if file_options.delete_bloat {
				variant.bloat_fields.clear();
			}
		}

		Ok(())
	}
}

fn validate_blockstate_multipart_condition(
	condition: &Condition,
	valid_block_properties: &AHashMap<&'static str, TinyVec<[BlockStatePropertyType; 1]>>
) -> Result<(), InnerBlockStateAssetError> {
	match condition {
		Condition::PropertyPredicates(property_predicates) => {
			// A non-composite condition is valid if every property-predicate pair references
			// valid properties and values for those properties
			for (property, property_predicate) in property_predicates {
				let allowed_value_types =
					valid_block_properties.get(&*property.0).ok_or_else(|| {
						InnerBlockStateAssetError::UnrecognizedPropertyName(
							property.0.to_string().into()
						)
					})?;

				for value in &property_predicate.values {
					validate_property_value_for_types(&property.0, value, allowed_value_types)?;
				}
			}
		}
		Condition::Composite(composite_condition) => {
			// A composite condition is valid when its non-composite descendant conditions are valid
			let conditions = match composite_condition {
				CompositeBooleanCondition::And(conditions) => conditions,
				CompositeBooleanCondition::Or(conditions) => conditions
			};

			for condition in conditions {
				validate_blockstate_multipart_condition(condition, valid_block_properties)?;
			}
		}
	}

	Ok(())
}

fn validate_property_value_for_types(
	name: &str,
	value: &str,
	valid_property_types: &[BlockStatePropertyType]
) -> Result<(), InnerBlockStateAssetError> {
	// Consider the property value invalid unless we prove otherwise.
	// This also errors out when specifying variants in blocks that do not have them
	let mut is_valid = false;
	for valid_property_value_type in valid_property_types {
		is_valid = match valid_property_value_type {
			BlockStatePropertyType::Enum { values } => values.contains(&value),
			BlockStatePropertyType::Boolean => matches!(value, "true" | "false"),
			BlockStatePropertyType::PositiveInteger { maximum_value } => value
				.parse::<u8>()
				.map_or_else(|_| false, |value| value <= maximum_value.get()),
			BlockStatePropertyType::StrictlyPositiveInteger { maximum_value } => value
				.parse::<u8>()
				.map_or_else(|_| false, |value| value > 0 && value <= maximum_value.get())
		};

		if is_valid {
			break;
		}
	}

	if is_valid {
		Ok(())
	} else {
		Err(InnerBlockStateAssetError::InvalidPropertyValue {
			name: name.into(),
			value: value.into(),
			expected_types: valid_property_types.iter().join("; ")
		})
	}
}

/// TODO docs:
/// Optimizes {"AND": [{"state1": "value"}, {"state2": "value"}, ...]} to {"state1": "value", "state2": "value", ...}
#[inline]
fn optimize_and_composite(condition: &mut Condition) {
	if let Condition::Composite(CompositeBooleanCondition::And(_)) = condition {
		let mut property_predicates = vec![];
		let mut all_conditions_are_and_composites_or_predicates = true;

		optimize_and_composite_inner(
			condition,
			&mut property_predicates,
			&mut all_conditions_are_and_composites_or_predicates
		);

		if all_conditions_are_and_composites_or_predicates {
			*condition = Condition::PropertyPredicates(AHashMap::from_iter(
				property_predicates
					.into_iter()
					.map(|(property, predicate)| (Property(property.0.clone()), predicate.clone()))
			));
		}
	}
}

fn optimize_and_composite_inner<'a, 'condition>(
	condition: &'a Condition<'condition>,
	property_predicates: &mut Vec<(&'a Property<'condition>, &'a PropertyPredicate)>,
	all_conditions_are_and_composites_or_predicates: &mut bool
) {
	match condition {
		Condition::Composite(CompositeBooleanCondition::And(child_conditions)) => {
			// AND composite condition. Examine its children to accumulate the predicates and check
			// whether they are either composite AND conditions or property predicates
			for child_condition in child_conditions {
				if !*all_conditions_are_and_composites_or_predicates {
					break;
				}

				optimize_and_composite_inner(
					child_condition,
					property_predicates,
					all_conditions_are_and_composites_or_predicates
				);
			}
		}
		Condition::Composite(_) => {
			// Found a non-AND composite condition. This signals that this optimization is not possible
			*all_conditions_are_and_composites_or_predicates = false;
		}
		Condition::PropertyPredicates(these_property_predicates) => {
			// Accumulate the candidate predicate in the chain
			property_predicates.extend(these_property_predicates)
		}
	}
}
