use self::blockstate::{
	BlockState, CompositeBooleanCondition, Condition, Property, PropertyPredicate, VariantList
};
use self::vanilla_blockstate_property_list::BlockStatePropertyType;
use super::compile_hardcoded_pack_file_glob_pattern;
use crate::config::{
	FileOptions, FileOptionsMap, GlobalOptions, JsonFileOptions, MissingReferenceAction
};
use crate::pack_processor::java::{
	pack_meta::PackMeta, resource_location::ResourceLocation,
	resource_location::ResourceLocationError
};
use crate::relative_path::InvalidPathError;
use crate::scratch_file::ScratchFile;
use crate::squash_zip::SquashZipError;
use crate::squashed_pack_state::SquashedPackState;
use crate::util::{
	lazy_or_eager::LazyOrEager, patricia_set_relative_path_iter::PatriciaSetRelativePathIterExt,
	range_bounds_intersect::RangeBoundsIntersectExt, strip_utf8_bom::StripUtf8BomExt
};
use crate::vfs::VirtualFileSystem;
use crate::{PackSquashAssetProcessingStrategy, RelativePath};
use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use json_comments::StripComments;
use once_cell::sync::Lazy;
use once_cell::unsync::Lazy as UnsyncLazy;
use patricia_tree::PatriciaSet;
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::io::{Read, Seek};
use std::ops::Deref;
use std::{io, mem};
use thiserror::Error;
use tinyvec::{tiny_vec, TinyVec};

mod blockstate;
mod vanilla_blockstate_list;
mod vanilla_blockstate_property_list;

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
	InvalidRelativePath(#[from] InvalidPathError<'static>),
	#[error("Unrecognized block state property: {0}")]
	UnrecognizedPropertyName(Cow<'static, str>),
	#[error("Invalid value {value} for block state property {name}: Expected {expected_types}")]
	InvalidPropertyValue {
		name: String,
		value: String,
		expected_types: String
	},
	#[error("A variant referenced a model at {path}, but it was not found in the pack{__filtered_out_text}")]
	MissingModel {
		path: RelativePath<'static>,
		#[doc(hidden)]
		__filtered_out_text: &'static str
	},
	#[error("JSON error at {0}")]
	JsonSerdeWithPath(#[from] serde_path_to_error::Error<serde_json::Error>),
	#[error("JSON error: {0}")]
	JsonSerde(#[from] serde_json::Error),
	/// Thrown when some error occurs in a ZIP file operation.
	#[error("Error while performing a ZIP file operation: {0}")]
	SquashZip(#[from] SquashZipError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

#[derive(Error, Debug)]
#[error("{file_path}: {inner}")]
pub struct BlockStateAssetError {
	inner: InnerBlockStateAssetError,
	file_path: RelativePath<'static>
}

// TODO docs
pub struct BlockStateAssetProcessor<
	'state,
	'settings,
	'budget,
	V: VirtualFileSystem + ?Sized,
	F: Read + Seek + Send
> {
	vfs: &'state V,
	pack_meta: &'state PackMeta,
	pack_files: &'state PatriciaSet,
	global_options: &'state GlobalOptions<'state>,
	file_options: &'state FileOptionsMap,
	squashed_pack_state: &'state SquashedPackState<'settings, 'budget, F>
}

impl<'state, 'settings, 'budget, V: VirtualFileSystem + ?Sized, F: Read + Seek + Send>
	BlockStateAssetProcessor<'state, 'settings, 'budget, V, F>
{
	pub fn new(
		vfs: &'state V,
		pack_meta: &'state PackMeta,
		pack_files: &'state PatriciaSet,
		global_options: &'state GlobalOptions,
		file_options: &'state FileOptionsMap,
		squashed_pack_state: &'state SquashedPackState<'settings, 'budget, F>
	) -> Self {
		Self {
			vfs,
			pack_meta,
			pack_files,
			global_options,
			file_options,
			squashed_pack_state
		}
	}

	pub fn process(&self) -> Result<(), BlockStateAssetError> {
		let game_version_range = self.pack_meta.game_version_range();

		// Block states were introduced in Minecraft 1.8
		if !(minecraft_version!(1, 8)..).intersects(game_version_range) {
			return Ok(());
		}

		let is_game_version_before_the_flattening =
			(..minecraft_version!(1, 13)).intersects(game_version_range);

		let game_version_has_multipart_model_strategy =
			(minecraft_version!(1, 9)..).intersects(game_version_range);

		let valid_vanilla_block_properties = Lazy::new(|| {
			vanilla_blockstate_property_list::matching_for_version_range(game_version_range)
		});

		let vanilla_blockstates =
			vanilla_blockstate_list::matching_for_version_range(game_version_range);

		let process_blockstate =
			|(candidate_asset_path, is_vanilla)| -> Result<(), BlockStateAssetError> {
				let file_options: UnsyncLazy<&JsonFileOptions, _> = UnsyncLazy::new(|| {
					self.file_options
						.get(&candidate_asset_path)
						.find_map(|file_options| {
							if let FileOptions::JsonFileOptions(file_options) = file_options {
								Some(file_options)
							} else {
								None
							}
						})
						.unwrap_or_default()
				});

				self.process_blockstate_file(
					&candidate_asset_path,
					&file_options,
					is_game_version_before_the_flattening,
					game_version_has_multipart_model_strategy,
					// For now, we only want to validate block properties of vanilla blockstates.
					// Make sure that non-vanilla blockstates do not get any properties, to avoid
					// potential clashes with modded blockstates
					if is_vanilla {
						LazyOrEager::Lazy(&valid_vanilla_block_properties)
					} else {
						LazyOrEager::Eager(None)
					}
				)
				.map_err(|err| BlockStateAssetError {
					inner: err,
					file_path: candidate_asset_path.into_owned()
				})
			};

		match (
			self.global_options
				.allow_non_vanilla_assets_of_vanilla_types,
			vanilla_blockstates.is_some()
		) {
			(true, _) | (_, false) => {
				// Exhaustive asset enumeration strategy: we either want to include non-vanilla assets
				// or we don't know what vanilla assets there are, so iterate over all the plausible
				// assets

				// Vanilla-format blockstates can be found in other namespaces in modded environments
				// (Forge and Fabric), but under the same paths on those namespaces. The unwritten agreement
				// is to rely on folders within namespaces to distinguish between asset types, while the
				// namespaces themselves are only used as mod IDs. Therefore, in practice it does not happen
				// that a mod-specific namespace reads non-blockstate assets from the blockstates path
				let any_blockstate_path_matcher =
					compile_hardcoded_pack_file_glob_pattern("assets/?*/blockstates/**/?*.jso{n,nc}")
						.compile_matcher();

				self.pack_files
					.relative_path_iter()
					.par_bridge()
					.filter_map(|relative_path| {
						// Minecraft expects blockstate files to be located at a valid resource
						// location (see net.minecraft.client.resources.model.ModelBakery and
						// net.minecraft.client.resources.model.ModelResourceLocation). Every
						// known mod for the Minecraft versions that support blockstates expands
						// on this vanilla logic, so filtering by that should be fine
						if any_blockstate_path_matcher.is_match(&relative_path)
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
					.try_for_each(process_blockstate)
			}
			(false, true) => {
				// Focused asset enumeration strategy: we only want vanilla assets and we know what vanilla
				// assets there are, so process them, and only them, right away

				// The Rust compiler needs some help to figure out that we want to unwrap the owned HashMap
				// from the AHashMap deref facade, which is necessary to be able to move it to the
				// into_par_iter() call
				<AHashSet<RelativePath<'_>> as Into<HashSet<_, _>>>::into(
					vanilla_blockstates.unwrap()
				)
				.into_par_iter()
				.map(|relative_path| (relative_path, true))
				.try_for_each(process_blockstate)
			}
		}
	}

	fn process_blockstate_file<'options>(
		&self,
		asset_path: &RelativePath,
		file_options: &impl Deref<Target = &'options JsonFileOptions>,
		is_game_version_before_the_flattening: bool,
		game_version_has_multipart_model_strategy: bool,
		valid_block_properties: impl Deref<
			Target = Option<
				AHashMap<&'static str, AHashMap<&'static str, TinyVec<[BlockStatePropertyType; 1]>>>
			>
		>
	) -> Result<(), InnerBlockStateAssetError> {
		macro_rules! open_or_mmap {
			($operation:ident) => {
				match self.vfs.$operation(asset_path) {
					Ok(asset_file) => asset_file,
					Err(err) if err.kind() == ::std::io::ErrorKind::NotFound => return Ok(()),
					Err(err) => return Err(err.into())
				}
			};
		}

		let canonical_asset_path = asset_path.canonicalize_extension("json");

		let allow_json_comments = asset_path.extension() == Some(OsStr::new("jsonc"))
			|| self.global_options.always_allow_json_comments;

		let mut models_to_be_checked_for_presence = AHashSet::new();

		// Deserialize the block state data to process it, and gather file metadata
		let asset_mmap;
		let (blockstate, file_size_hint) = if allow_json_comments {
			let asset_file = open_or_mmap!(open);

			// Mark as processed and return early it was processed or is being processed by
			// another thread (the same asset may be available at several non-canonical paths).
			// If we error out, it doesn't matter that the file was not actually processed.
			// We have to mark it after trying to open the file to ignore non-existing alternative
			// candidate paths
			if !self
				.squashed_pack_state
				.mark_file_as_processed(&canonical_asset_path)
			{
				return Ok(());
			}

			let is_previous_zip_file_current = self
				.squashed_pack_state
				.is_previous_zip_file_current(&canonical_asset_path, asset_file.modification_time);

			(
				if is_previous_zip_file_current {
					None
				} else {
					Some(serde_path_to_error::deserialize::<_, BlockState>(
						&mut serde_json::Deserializer::from_reader(StripComments::new(
							asset_file.reader.strip_utf8_bom()?
						))
					)?)
				},
				asset_file
					.file_size
					.map_or_else(|| 0, |file_size| file_size.try_into().unwrap_or(0))
			)
		} else {
			asset_mmap = open_or_mmap!(mmap);

			// Mark as processed and return early it was processed or is being processed by
			// another thread (the same asset may be available at several non-canonical paths).
			// If we error out, it doesn't matter that the file was not actually processed.
			// We have to mark it after trying to open the file to ignore non-existing alternative
			// candidate paths
			if !self
				.squashed_pack_state
				.mark_file_as_processed(&canonical_asset_path)
			{
				return Ok(());
			}

			let is_previous_zip_file_current = self
				.squashed_pack_state
				.is_previous_zip_file_current(&canonical_asset_path, asset_mmap.modification_time());

			(
				if is_previous_zip_file_current {
					None
				} else {
					Some(serde_path_to_error::deserialize::<_, BlockState>(
						&mut serde_json::Deserializer::from_slice(
							asset_mmap.strip_utf8_bom().unwrap()
						)
					)?)
				},
				asset_mmap.len()
			)
		};

		if let Some(blockstate) = blockstate {
			// The file name, without extension, matches the block name
			let block_name = asset_path
				.file_name()
				.unwrap()
				.to_str()
				.unwrap()
				.rsplit_once('.')
				.unwrap()
				.0;

			let processed_file = self.process_deserialized_blockstate(
				blockstate,
				block_name,
				file_options,
				valid_block_properties,
				is_game_version_before_the_flattening,
				game_version_has_multipart_model_strategy,
				matches!(
					self.global_options.missing_reference_action,
					MissingReferenceAction::ErrorOut | MissingReferenceAction::Warn
				)
				.then_some(&mut models_to_be_checked_for_presence),
				file_size_hint
			)?;

			self.squashed_pack_state
				.add_file_to_zip(&canonical_asset_path, processed_file, false)?;
		} else {
			self.squashed_pack_state
				.add_previous_zip_file(&canonical_asset_path)?;
		}

		// If some reference model is to be checked for presence, do it
		let mut missing_model_warnings = tiny_vec!();
		let missing_model_warnings_are_errors = matches!(
			self.global_options.missing_reference_action,
			MissingReferenceAction::ErrorOut
		);

		for referenced_model_path in models_to_be_checked_for_presence {
			// TODO ignore if it is a known, non-filtered vanilla model
			if self.pack_files.contains(referenced_model_path.as_str()) {
				// The dependency is satisfied
				continue;
			}

			// Short-circuit evaluation if any warning is an error anyway: consider that it
			// was not filtered out
			let is_model_filtered_out = !missing_model_warnings_are_errors
				&& self.pack_meta.is_resource_location_filtered_out(
					&ResourceLocation::try_from(&referenced_model_path).unwrap()
				);

			if missing_model_warnings_are_errors || is_model_filtered_out {
				return Err(InnerBlockStateAssetError::MissingModel {
					path: referenced_model_path,
					// Let whoever sees this error displayed know whether we elevated a
					// warning to an error due to resource filters
					__filtered_out_text: if is_model_filtered_out {
						" and it was filtered out by the pack.mcmeta filter section"
					} else {
						""
					}
				});
			} else {
				missing_model_warnings.push(
					format!(
						"A variant referenced a model at {}, but it was not found in the pack",
						referenced_model_path.as_str()
					)
					.into()
				);
			}
		}

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
				warnings: missing_model_warnings
			},
			asset_path = asset_path.as_str()
		);

		Ok(())
	}

	#[inline]
	fn process_deserialized_blockstate<'options>(
		&self,
		mut blockstate: BlockState,
		block_name: &str,
		file_options: &impl Deref<Target = &'options JsonFileOptions>,
		valid_block_properties: impl Deref<
			Target = Option<
				AHashMap<&'static str, AHashMap<&'static str, TinyVec<[BlockStatePropertyType; 1]>>>
			>
		>,
		is_game_version_before_the_flattening: bool,
		game_version_has_multipart_model_strategy: bool,
		mut models_to_be_checked_for_presence: Option<&mut AHashSet<RelativePath<'static>>>,
		file_size_hint: usize
	) -> Result<ScratchFile, InnerBlockStateAssetError> {
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
					if variant_predicate.is_legacy_invariant && !is_game_version_before_the_flattening
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
						is_game_version_before_the_flattening,
						models_to_be_checked_for_presence.as_mut()
					)?;
				}
			}
			(None, Some(multipart_variants)) => {
				// Check compatibility with the game first
				if !game_version_has_multipart_model_strategy {
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
						is_game_version_before_the_flattening,
						models_to_be_checked_for_presence.as_mut()
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

		// Setup a scratch file to hold the result
		let mut scratch_file = ScratchFile::with_capacity(
			self.squashed_pack_state.scratch_files_budget(),
			file_size_hint / 2
		)?;

		// Serialize the resulting JSON to the scratch file
		if file_options.minify {
			serde_json::to_writer(&mut scratch_file, &blockstate)?;
		} else {
			serde_json::to_writer_pretty(&mut scratch_file, &blockstate)?;
		}

		Ok(scratch_file)
	}

	fn optimize_variant_list<'options, 'data>(
		&self,
		variant_list: &mut VariantList,
		file_options: &impl Deref<Target = &'options JsonFileOptions>,
		is_game_version_before_the_flattening: bool,
		mut referenced_models: Option<&mut &mut AHashSet<RelativePath<'static>>>
	) -> Result<(), InnerBlockStateAssetError> {
		// Optimize list of variants to a single variant if it contains a single element
		if variant_list.len() == 1 && matches!(variant_list, VariantList::SeveralVariants(_)) {
			*variant_list = VariantList::SingleVariant(mem::take(&mut variant_list[0]));
		}

		// Debloat each variant if applicable, and note the model it uses
		for variant in variant_list.iter_mut() {
			let model_location = ResourceLocation::new(
				self.pack_meta.pack_type(),
				&variant.model,
				Some(if is_game_version_before_the_flattening {
					"models/block"
				} else {
					"models"
				}),
				Some("json")
			)?;

			if let Some(referenced_models) = &mut referenced_models {
				referenced_models.insert(model_location.as_relative_path()?);
			}

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
