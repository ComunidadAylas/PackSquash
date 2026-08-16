use std::{ffi::OsStr, fs, io, iter::Empty, path::Path};

use itertools::Itertools;
use pretty_assertions::assert_eq;
use tempfile::{Builder as TempDirBuilder, TempDir};
use tokio_test::io::{Builder as MockFileBuilder, Mock};

use crate::vfs::{
	IteratorTraversalOptions, VfsFile, VfsPackFileIterEntry, VfsPackFileMetadata, VirtualFileSystem
};

use super::{PackMetadata, PackMetadataError, PackType};

/// A mock virtual file system that reads a fixed pack metadata JSON payload from `pack.mcmeta`, and
/// whose root directory contains marker directories for the given [`PackType`]s, so that
/// [`PackMetadata::read`]'s pack type detection logic can be exercised realistically.
///
/// Marker directories are backed by a real temporary directory, because
/// [`VirtualFileSystem::file_type`] must return a [`std::fs::FileType`] that cannot be instantiated
/// otherwise.
struct MockVfs {
	metadata_json: &'static str,
	root_dir: TempDir
}

impl MockVfs {
	fn new(metadata_json: &'static str, pack_types: impl IntoIterator<Item = PackType>) -> Self {
		let root_dir = TempDirBuilder::new()
			.prefix("ps-pack-metadata-test")
			.tempdir()
			.expect("I/O operations are assumed not to fail during tests");

		for pack_type in pack_types {
			fs::create_dir(root_dir.path().join(pack_type.prefix_directory()))
				.expect("I/O operations are assumed not to fail during tests");
		}

		Self {
			metadata_json,
			root_dir
		}
	}
}

impl VirtualFileSystem for MockVfs {
	type FileRead = Mock;

	type FileIter = Empty<Result<VfsPackFileIterEntry, io::Error>>;

	fn file_iterator(&self, _: &Path, _: IteratorTraversalOptions) -> Self::FileIter {
		unimplemented!()
	}

	fn open<P: AsRef<Path>>(&self, path: P) -> Result<VfsFile<Self::FileRead>, io::Error> {
		let path = path.as_ref().as_os_str();
		if path == OsStr::new("pack.mcmeta") || path == OsStr::new("pack.mcmetac") {
			Ok(VfsFile {
				file_read: MockFileBuilder::new()
					.read(self.metadata_json.as_bytes())
					.build(),
				file_size_hint: self.metadata_json.len().try_into().unwrap_or(u64::MAX),
				metadata: VfsPackFileMetadata {
					modification_time: None
				}
			})
		} else {
			unreachable!()
		}
	}

	fn file_type<P: AsRef<Path>>(&self, path: P) -> Result<fs::FileType, io::Error> {
		fs::metadata(self.root_dir.path().join(path.as_ref())).map(|metadata| metadata.file_type())
	}
}

/// Convenience façade for [`PackMetadata::read`] over a [`MockVfs`].
async fn read_metadata(
	metadata_json: &'static str,
	pack_types: impl IntoIterator<Item = PackType>
) -> Result<PackMetadata, PackMetadataError> {
	PackMetadata::read(&MockVfs::new(metadata_json, pack_types), "").await
}

#[tokio::test]
async fn well_formed_pack_mcmeta_works() {
	read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 7,
					"description": "My pack"
				}
			}"#,
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_extra_objects_works() {
	read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 7,
					"description": "My pack"
				},
				"language": {
					"custom": {
						"name": "My custom language",
						"region": "Westartica",
						"bidirectional": false
					}
				}
			}"#,
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_description_object() {
	read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 7,
					"description": {
						"text": "My pack - ",
						"extra": [{
							"text": "© Myself",
							"italic": true
						}]
					}
				}
			}"#,
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_description_array() {
	read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 7,
					"description": ["My pack", "is awesome"]
				}
			}"#,
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn pack_mcmeta_with_missing_description() {
	assert!(
		matches!(
			read_metadata(
				r#"
					{
						"pack": {
							"pack_format": 7
						}
					}"#,
				[PackType::ClientResources]
			)
			.await,
			Err(PackMetadataError::JsonSerde(_))
		),
		"Expected failure reading pack metadata"
	);
}

#[tokio::test]
async fn pack_mcmeta_with_bad_pack_format() {
	assert!(
		matches!(
			read_metadata(
				r#"
					{
						"pack": {
							"pack_format": -0.5,
							"description": "My bad pack"
						}
					}"#,
				[PackType::ClientResources]
			)
			.await,
			Err(PackMetadataError::JsonSerde(_))
		),
		"Expected failure reading pack metadata"
	);
}

#[tokio::test]
async fn pack_mcmeta_without_expected_structure() {
	assert!(
		matches!(
			read_metadata("42", [PackType::ClientResources]).await,
			Err(PackMetadataError::JsonSerde(_))
		),
		"Expected failure reading pack metadata"
	);
}

#[tokio::test]
async fn pack_mcmeta_with_missing_pack_format_information_is_rejected() {
	assert!(
		matches!(
			read_metadata(
				r#"
					{
						"pack": {
							"description": "My pack"
						}
					}"#,
				[PackType::ClientResources]
			)
			.await,
			Err(PackMetadataError::JsonSerde(_))
		),
		"Expected failure reading pack metadata that lacks any pack format version information"
	);
}

#[tokio::test]
async fn unknown_pack_type_is_rejected() {
	assert!(
		matches!(
			read_metadata(
				r#"
					{
						"pack": {
							"pack_format": 7,
							"description": "My pack"
						}
					}"#,
				[]
			)
			.await,
			Err(PackMetadataError::UnknownPackType)
		),
		"Expected failure reading pack metadata for a root directory without pack type marker directories"
	);
}

#[tokio::test]
async fn ambiguous_pack_type_is_rejected() {
	assert!(
		matches!(
			read_metadata(
				r#"
					{
						"pack": {
							"pack_format": 7,
							"description": "My pack"
						}
					}"#,
				[PackType::ClientResources, PackType::ServerData]
			)
			.await,
			Err(PackMetadataError::AmbiguousPackType)
		),
		"Expected failure reading pack metadata for a root directory with marker directories for multiple pack types"
	);
}

#[tokio::test]
async fn server_data_pack_type_is_detected() {
	let pack_metadata = read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 7,
					"description": "My pack"
				}
			}"#,
		[PackType::ServerData]
	)
	.await
	.expect("Unexpected failure reading pack metadata");

	assert_eq!(
		pack_metadata.ty,
		PackType::ServerData,
		"Expected the data pack marker directory to result in a server data pack type"
	);
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_legacy_supported_formats_array_works() {
	read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 15,
					"supported_formats": [15, 20],
					"description": "My pack"
				}
			}"#,
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_legacy_supported_formats_object_works() {
	read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 15,
					"supported_formats": {
						"min_inclusive": 15,
						"max_inclusive": 20
					},
					"description": "My pack"
				}
			}"#,
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_min_max_format_works() {
	read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 66,
					"min_format": [65, 0],
					"max_format": [70, 1],
					"description": "My pack"
				}
			}"#,
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn pack_mcmeta_with_invalid_min_max_format_is_rejected() {
	assert!(
		matches!(
			read_metadata(
				r#"
					{
						"pack": {
							"pack_format": 66,
							"min_format": [70, 0],
							"max_format": [65, 1],
							"description": "My pack"
						}
					}"#,
				[PackType::ClientResources]
			)
			.await,
			Err(PackMetadataError::JsonSerde(_))
		),
		"Expected failure reading pack metadata whose max_format is lower than its min_format"
	);
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_overlay_works() {
	let pack_metadata = read_metadata(
		r#"
			{
				"pack": {
					"pack_format": 7,
					"description": "My pack"
				},
				"overlays": {
					"entries": [
						{
							"directory": "overlay_1",
							"formats": [8, 10]
						}
					]
				}
			}"#,
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");

	assert_eq!(
		pack_metadata.layers.len(),
		2,
		"Expected both the base pack layer and the overlay layer to be present"
	);
}

#[tokio::test]
async fn well_formed_real_and_complex_pack_mcmeta_works() {
	let pack_metadata = read_metadata(
		include_str!("tests/bare_bones_1_4_12.jsonc"),
		[PackType::ClientResources]
	)
	.await
	.expect("Unexpected failure reading pack metadata");

	assert_eq!(
		pack_metadata.ty,
		PackType::ClientResources,
		"Expected the assets marker directory to result in a client resources pack type"
	);

	let layers = pack_metadata
		.layers
		.iter()
		.map(|(directory, ranges)| {
			let ranges = ranges
				.iter()
				.map(|range| {
					(
						(range.start().major_version, range.start().minor_version),
						(range.end().major_version, range.end().minor_version)
					)
				})
				.sorted()
				.collect_vec();

			(directory, ranges)
		})
		.sorted()
		.collect_vec();

	assert_eq!(
		&layers,
		&[
			(&arcstr::literal!(""), vec![((34, 0), (75, 0))]),
			(
				&arcstr::literal!("overlay_1_21_11"),
				vec![((70, 0), (75, 0))]
			),
			(
				&arcstr::literal!("overlay_1_21_2"),
				vec![((42, 0), (75, 0))]
			),
			(
				&arcstr::literal!("overlay_1_21_4"),
				vec![((46, 0), (75, 0))]
			),
			(
				&arcstr::literal!("overlay_1_21_5"),
				vec![((55, 0), (75, 0))]
			),
			(
				&arcstr::literal!("overlay_1_21_6"),
				vec![((55, 0), (75, 0))]
			),
			(
				&arcstr::literal!("overlay_1_21_9"),
				vec![((65, 0), (75, 0))]
			)
		],
		"Unexpected pack layers and/or format version ranges"
	);

	let bounding_format_version_range = pack_metadata.bounding_format_version_range();

	assert_eq!(
		(
			(
				bounding_format_version_range.start().major_version,
				bounding_format_version_range.start().minor_version
			),
			(
				bounding_format_version_range.end().major_version,
				bounding_format_version_range.end().minor_version
			)
		),
		((34, 0), (75, 0)),
		"Unexpected bounding format version range"
	);
}
