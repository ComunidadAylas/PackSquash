use super::{pack_meta::PackMeta, PackError};
use crate::pack_processor::java::asset_processor::create_asset_processors;
use crate::squashed_pack_state::SquashedPackState;
use crate::vfs::VirtualFileSystem;
use itertools::Itertools;
use packsquash_options::{FileOptionsMap, GlobalOptions};
use patricia_tree::PatriciaSet;
use std::io::{Read, Seek};

// TODO
pub(super) struct ResourcePackProcessor;

impl ResourcePackProcessor {
	pub(super) fn new() -> Self {
		Self
	}

	pub(super) fn process<F: Read + Seek + Send>(
		self,
		vfs: &(impl VirtualFileSystem + ?Sized),
		pack_meta: PackMeta,
		pack_files: PatriciaSet,
		global_options: &GlobalOptions,
		file_options: &FileOptionsMap,
		squashed_pack_state: &SquashedPackState<'_, '_, F>
	) -> Result<(), PackError> {
		let asset_processors = create_asset_processors(
			vfs,
			&pack_meta,
			&pack_files,
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
