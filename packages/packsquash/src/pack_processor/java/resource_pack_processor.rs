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

		// TODO run asset processors without dependencies in parallel (Rayon join(), scope())
		get_asset_processor!(asset_processors, BlockstateAssetProcessor).process()?;

		Ok(())
	}
}
