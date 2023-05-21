use std::io::{Read, Seek};

use ahash::AHashSet;
use enum_map::{enum_map, Enum, EnumMap};
use globset::Glob;
use packsquash_options::{FileOptionsMap, GlobalOptions};
use patricia_tree::PatriciaSet;
use strum::Display;

use self::{
	blockstate_asset_processor::BlockStateAssetProcessor,
	item_and_block_model_asset_processor::ItemAndBlockModelAssetProcessor
};
use crate::{
	pack_processor::java::pack_meta::PackMeta, relative_path::RelativePath,
	squashed_pack_state::SquashedPackState, vfs::VirtualFileSystem
};

#[macro_use]
mod helper;
pub(super) mod blockstate_asset_processor;
mod data;
pub(super) mod item_and_block_model_asset_processor;

#[derive(Enum)]
pub(super) enum AssetProcessorType {
	BlockStateAssetProcessor,
	ItemAndBlockModelAssetProcessor
}

#[derive(Display)]
pub(super) enum AssetProcessorWrapper<
	'params,
	'state,
	V: VirtualFileSystem + ?Sized,
	F: Read + Seek + Send,
	C: FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send
> {
	BlockStateAssetProcessor(BlockStateAssetProcessor<'params, 'state, V, F>),
	ItemAndBlockModelAssetProcessor(ItemAndBlockModelAssetProcessor<'params, 'state, V, F, C>)
}

pub(super) fn create_asset_processors<
	'params,
	'state,
	V: VirtualFileSystem + ?Sized,
	F: Read + Seek + Send
>(
	vfs: &'params V,
	pack_meta: &'params PackMeta,
	pack_files: &'params PatriciaSet,
	global_options: &'params GlobalOptions,
	file_options: &'params FileOptionsMap,
	squashed_pack_state: &'params SquashedPackState<'state, 'state, F>
) -> EnumMap<
	AssetProcessorType,
	AssetProcessorWrapper<
		'params,
		'state,
		V,
		F,
		impl FnOnce() -> Option<AHashSet<RelativePath<'static>>> + Send + 'params
	>
> {
	enum_map! {
		AssetProcessorType::BlockStateAssetProcessor => AssetProcessorWrapper::BlockStateAssetProcessor(
			BlockStateAssetProcessor::new(vfs, pack_meta, pack_files, global_options, file_options, squashed_pack_state)
		),
		AssetProcessorType::ItemAndBlockModelAssetProcessor => AssetProcessorWrapper::ItemAndBlockModelAssetProcessor(
			item_and_block_model_asset_processor::new(vfs, pack_meta, pack_files, global_options, file_options, squashed_pack_state)
		)
	}
}

macro_rules! get_asset_processor {
	($asset_processors:expr , $ty:ident) => {
		// This pattern is irrefutable by construction, but there are not better ways to unwrap the variant
		#[allow(irrefutable_let_patterns)]
		if let $crate::pack_processor::java::asset_processor::AssetProcessorWrapper::$ty(
			ref asset_processor
		) = $asset_processors[$crate::pack_processor::java::asset_processor::AssetProcessorType::$ty]
		{
			asset_processor
		} else {
			unreachable!()
		}
	};
}

/// Compiles the specified pack file glob pattern, assuming it was hardcoded in the application binary.
/// Any validity error is discarded and turned into a panic, as modification of hardcoded data is not
/// to be handled as an error.
///
/// Please note that, even though this function requires a static string slice in an effort to prevent
/// accidental misuse, it is possible to get string slices that live indefinitely by leaking a heap
/// allocation.
fn compile_hardcoded_pack_file_glob_pattern(glob_pattern: &'static str) -> Glob {
	packsquash_options::compile_pack_file_glob_pattern(glob_pattern).unwrap()
}
