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

/// The maximum number of deserialized models that will be stored in the memory cache.
/// Caching 64 models on the 1.19.2 vanilla resource pack with
/// `process_not_self_referenced_and_non_vanilla_assets = false` gives a hit ratio of ~22.5% (the
/// theoretical maximum would be 60.38%), which provides a ~4% speedup on a certain test
/// environment. Caching more models (e.g., 128) doubles memory usage for a meager ~2% increase
/// in hit ratio and negligible speedups, while caching less models (e.g., 32) decreases the hit
/// ratio ~2% but increases runtime a ~1.3%, so 64 should be a good sweet spot for realistic usage.
///
/// We fix the cache size because that provides for simpler eviction algorithms and overall more
/// efficient cache operation. The downside is that the cache won't take into account the memory
/// usage of models when eviction is necessary, so memory usage might be a bit higher than if it
/// did.
///
/// Observe that, on systems with enough available RAM and proper OS filesystem caches, this cache
/// only helps reducing syscall and deserialization costs. Its impact is greater when deserialization
/// costs are high (e.g., complex models) and if the OS filesystem cache is not caching models. For
/// the aforementioned pack, a 64 entry cache takes ~1 MiB of RAM (measured with `accounting-allocator`),
/// which is within noise margin nowadays.
const CACHE_SIZE: usize = 64;

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
