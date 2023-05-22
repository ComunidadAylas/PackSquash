use std::{
	io::{Read, Seek},
	sync::{Arc, RwLock}
};

use packsquash_options::GlobalOptions;
use quick_cache::sync::Cache;

use super::{
	is_builtin_model_path,
	item_or_block_model::{ItemOrBlockModel, ItemOrBlockModelRepresentative},
	InnerItemOrBlockModelAssetError
};
use crate::{
	pack_processor::java::asset_processor::helper::json_helper::{self, JsonAssetDeserializeOutcome},
	relative_path::RelativePath,
	squashed_pack_state::SquashedPackState,
	vfs::VirtualFileSystem
};

// TODO gather stats to dynamically size this cache better
const CACHE_SIZE: usize = 128;

pub(super) struct ModelCache<'params, 'state, V: VirtualFileSystem + ?Sized, F: Read + Seek + Send> {
	cache: Cache<RelativePath<'static>, Option<Arc<RwLock<CachedItemOrBlockModel>>>>,
	vfs: &'params V,
	global_options: &'params GlobalOptions<'params>,
	squashed_pack_state: &'params SquashedPackState<'state, 'state, F>
}

pub(super) struct CachedItemOrBlockModel {
	pub(super) model: ItemOrBlockModel<'static>,
	pub(super) size_hint: usize
}

impl<'params, 'state, V: VirtualFileSystem + ?Sized, F: Read + Seek + Send>
	ModelCache<'params, 'state, V, F>
{
	pub(super) fn new(
		vfs: &'params V,
		global_options: &'params GlobalOptions<'params>,
		squashed_pack_state: &'params SquashedPackState<'state, 'state, F>
	) -> Self {
		Self {
			cache: Cache::new(CACHE_SIZE),
			vfs,
			global_options,
			squashed_pack_state
		}
	}

	pub(super) fn get(
		&self,
		asset_path: &RelativePath<'static>
	) -> Result<Option<Arc<RwLock<CachedItemOrBlockModel>>>, InnerItemOrBlockModelAssetError> {
		// Built-in models should not appear in resource packs, but if they do the game
		// ignores their version on the pack. Therefore, also do that ourselves to avoid
		// cache pollution
		if is_builtin_model_path(asset_path) {
			return Ok(None);
		}

		Ok(self.cache.get_or_insert_with(asset_path, || {
			json_helper::deserialize::<ItemOrBlockModelRepresentative, _, _, _>(
				asset_path,
				self.vfs,
				self.squashed_pack_state,
				true,
				false,
				self.global_options.always_allow_json_comments,
				|outcome: JsonAssetDeserializeOutcome<ItemOrBlockModel<'_>>| match outcome {
					JsonAssetDeserializeOutcome::Value {
						value: model,
						size_hint,
						..
					} => Some(Arc::new(RwLock::new(CachedItemOrBlockModel {
						model: model.into_owned(),
						size_hint
					}))),
					JsonAssetDeserializeOutcome::NotFound => None,
					JsonAssetDeserializeOutcome::AlreadyProcessed
					| JsonAssetDeserializeOutcome::FreshInPreviousZip => unreachable!()
				}
			)
		})?)
	}
}