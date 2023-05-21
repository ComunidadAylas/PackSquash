use std::io::{Read, Seek};

use itertools::Itertools;
use packsquash_options::{FileOptionsMap, GlobalOptions};
use patricia_tree::PatriciaSet;

use super::{pack_meta::PackMeta, PackError};
use crate::{
	pack_processor::java::asset_processor::create_asset_processors,
	squashed_pack_state::SquashedPackState, vfs::VirtualFileSystem
};

// TODO
pub(super) struct ResourcePackProcessor;

impl ResourcePackProcessor {
	pub(super) fn new() -> Self {
		Self
	}

	pub(super) fn process<'params, 'state, F: Read + Seek + Send>(
		self,
		vfs: &'params (impl VirtualFileSystem + ?Sized),
		pack_meta: &'params PackMeta,
		pack_files: &'params PatriciaSet,
		global_options: &'params GlobalOptions,
		file_options: &'params FileOptionsMap,
		squashed_pack_state: &'params SquashedPackState<'state, 'state, F>
	) -> Result<(), PackError> {
		let asset_processors = create_asset_processors(
			vfs,
			pack_meta,
			pack_files,
			global_options,
			file_options,
			squashed_pack_state
		);

		// Notify interested consumers about the asset processors we're going to use
		status_info!(AssetProcessorsToRun {
			asset_processors_list: asset_processors.values().join(", ")
		});

		macro_rules! try_join_several {
			( $closure_a:expr, $closure_b:expr ) => {
				match rayon::join($closure_a, $closure_b) {
					(Err(err), _) => return Err(PackError::from(err)),
					(_, Err(err)) => return Err(PackError::from(err)),
					(Ok(a), Ok(b)) => (a, b)
				}
			};
			( $closure_a:expr, $closure_b:expr, $( $remaining_closures:expr ),+ ) => {
				try_join_several!(|| Ok(try_join_several!($closure_a, $closure_b)), $( $remaining_closures ),+)
			}
		}

		// TODO this doesn't fail-fast when an error happens: rayon waits for each asset processor
		//      to complete, which is undesirable. Refactoring with spawn() and using channels
		//      could avoid this, at the (likely significant) performance cost of needing to wrap
		//      asset processor references in Arc's. scope() would be great if it didn't block,
		//      but that could be worked around with another thread. Are there better ways to deal
		//      with it?
		try_join_several!(
			|| get_asset_processor!(asset_processors, BlockStateAssetProcessor).process(
				get_asset_processor!(asset_processors, ItemAndBlockModelAssetProcessor)
			),
			|| get_asset_processor!(asset_processors, ItemAndBlockModelAssetProcessor).process()
		);

		Ok(())
	}
}
