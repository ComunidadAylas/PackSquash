mod pack_meta;
mod resource_location;
#[macro_use]
mod asset_processor;
pub(crate) mod pack_processor;
mod resource_pack_processor;

use self::asset_processor::blockstate_asset_processor::BlockStateAssetError;
use self::pack_meta::PackMetaError;
use crate::pack_processor::java::asset_processor::item_and_block_model_asset_processor::ItemOrBlockModelAssetError;
use crate::squash_zip::SquashZipError;
use crate::RelativePath;
use rayon::ThreadPoolBuildError;
use std::fmt::{Display, Formatter};
use std::io;
use strum::EnumIter;
use thiserror::Error;

#[derive(Clone, Copy, EnumIter, Eq, PartialEq, Hash, Debug)]
pub enum PackType {
	DataPack,
	ResourcePack
}

impl PackType {
	pub fn root_directory(&self) -> RelativePath<'static> {
		match self {
			Self::DataPack => RelativePath::from_inner("data"),
			Self::ResourcePack => RelativePath::from_inner("assets")
		}
	}
}

impl Display for PackType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::DataPack => f.write_str("data pack"),
			Self::ResourcePack => f.write_str("resource pack")
		}
	}
}

#[derive(Error, Debug)]
pub enum PackError {
	/// Thrown when an invalid file glob pattern was found in the file options.
	#[error("Invalid file glob pattern: {0}")]
	InvalidFileGlobPattern(#[from] globset::Error),
	/// Thrown when an error happened while parsing the pack metadata file,
	/// which defines some basic characteristics of a pack.
	#[error("Pack metadata file error: {0}")]
	PackMetaError(#[from] PackMetaError),
	#[error("Asset error: {0}")]
	BlockStateAssetError(#[from] BlockStateAssetError),
	#[error("Asset error: {0}")]
	ItemOrBlockModelAssetError(#[from] ItemOrBlockModelAssetError),
	/// Thrown when some error occurs in a ZIP file operation.
	#[error("Error while performing a ZIP file operation: {0}")]
	SquashZip(#[from] SquashZipError),
	#[error("Thread error: {0}")]
	ThreadError(#[from] ThreadPoolBuildError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}
