use std::{borrow::Cow, fmt::Display};

use packsquash_options::GlobalOptions;
use tinyvec::{Array, TinyVec};

use crate::{
	pack_processor::java::{pack_meta::PackMeta, resource_location::ResourceLocation},
	relative_path::{InvalidPathError, RelativePath}
};

pub(super) mod json_helper;

macro_rules! get_file_specific_options {
	($file_options_map:expr , $asset_path:expr , $file_specific_options_type:ident) => {
		::once_cell::unsync::Lazy::new(|| {
			$file_options_map
				.get($asset_path)
				.find_map(|file_options| {
					if let ::packsquash_options::FileOptions::$file_specific_options_type(
						file_options
					) = file_options
					{
						Some(file_options)
					} else {
						None
					}
				})
				.unwrap_or_default()
		})
	};
}

macro_rules! define_path_wrapper_err {
	($inner:ident : $wrapper:ident) => {
		#[derive(thiserror::Error, Debug)]
		#[error("{file_path}: {inner}")]
		pub struct $wrapper {
			inner: $inner,
			file_path: $crate::relative_path::RelativePath<'static>
		}

		paste::paste! {
			trait [<$inner ResultExt>]<T> {
				fn into_err_with_path(
					self,
					file_path: impl FnOnce() -> $crate::relative_path::RelativePath<'static>
				) -> Result<T, $wrapper>;
			}

			impl<T, E: Into<$inner>> [<$inner ResultExt>]<T> for Result<T, E> {
				fn into_err_with_path(
					self,
					file_path: impl FnOnce() -> $crate::relative_path::RelativePath<'static>
				) -> Result<T, $wrapper> {
					self.map_err(|inner| $wrapper {
						inner: inner.into(),
						file_path: file_path()
					})
				}
			}
		}
	};
}

pub(super) fn validate_asset_dependency<
	A: Array<Item = Cow<'static, str>>,
	E: From<InvalidPathError> + Display
>(
	asset_location: &ResourceLocation,
	pack_meta: &PackMeta,
	global_options: &GlobalOptions,
	warning_list: Option<&mut TinyVec<A>>,
	asset_existence_predicate: impl FnOnce(&RelativePath<'_>) -> Result<bool, E>,
	is_vanilla_asset_predicate: impl FnOnce(&RelativePath<'_>) -> Result<bool, E>,
	missing_reference_error_factory: impl FnOnce(RelativePath<'static>, &'static str) -> E
) -> Result<(), E> {
	let asset_path = asset_location.as_relative_path()?;

	let is_asset_filtered_out;
	let dependency_satisfied = if asset_existence_predicate(&asset_path)? {
		is_asset_filtered_out = false;

		// Easy case: the pack satisfies its own dependency
		true
	} else {
		// The pack does not contain a file that satisfies its own dependency.
		// In general, however, we can't be sure that the dependency is not satisfied:
		// another pack applied on the client may provide it, including the default
		// resource pack(s), which provides vanilla and modded assets.
		//
		// The only exception to the above is when the asset is explicitly filtered out
		// from being loaded from other packs: we then know for sure that the dependency
		// can't be satisfied.
		//
		// Therefore, consider the dependency satisfied only when:
		// - The assets is not filtered out, and
		// - It is a vanilla asset, or references to unknown packs and/or mods are
		//   accepted.
		is_asset_filtered_out = pack_meta.is_resource_location_filtered_out(asset_location);

		!is_asset_filtered_out
			&& (global_options.accept_references_to_unknown_packs_and_mods
				|| is_vanilla_asset_predicate(&asset_path)?)
	};

	if dependency_satisfied {
		Ok(())
	} else {
		let missing_reference_error = missing_reference_error_factory(
			asset_path,
			if is_asset_filtered_out {
				". In addition, it can't be loaded from other packs due to the pack.mcmeta filter section"
			} else {
				""
			}
		);

		if let Some(warning_list) = warning_list {
			warning_list.push(missing_reference_error.to_string().into());

			Ok(())
		} else {
			Err(missing_reference_error)
		}
	}
}