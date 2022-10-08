use std::{ffi::OsStr, fs::FileType, io, iter::Empty, path::Path};

use tokio_test::io::{Builder, Mock};

use crate::vfs::{
	IteratorTraversalOptions, VfsFile, VfsPackFileIterEntry, VfsPackFileMetadata, VirtualFileSystem
};

use super::PackMeta;

struct MockVfs(&'static str);

impl VirtualFileSystem for MockVfs {
	type FileRead = Mock;

	type FileIter = Empty<Result<VfsPackFileIterEntry, io::Error>>;

	fn file_set(&self, _: &Path, _: IteratorTraversalOptions) -> Self::FileIter {
		unimplemented!()
	}

	fn open<P: AsRef<Path>>(&self, path: P) -> Result<VfsFile<Self::FileRead>, io::Error> {
		let path = path.as_ref().as_os_str();
		if path == OsStr::new("pack.mcmeta") || path == OsStr::new("pack.mcmetac") {
			Ok(VfsFile {
				reader: Builder::new().read(self.0.as_bytes()).build(),
				file_size_hint: self.0.len().try_into().unwrap_or(u64::MAX),
				metadata: VfsPackFileMetadata {
					modification_time: None
				}
			})
		} else {
			unreachable!()
		}
	}

	fn file_type<P: AsRef<Path>>(&self, _: P) -> Result<FileType, io::Error> {
		unimplemented!()
	}
}

#[tokio::test]
async fn well_formed_pack_mcmeta_works() {
	PackMeta::new(
		&MockVfs(
			r#"
				{
					"pack": {
						"pack_format": 7,
						"description": "My pack"
					}
				}"#
		),
		""
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_extra_objects_works() {
	PackMeta::new(
		&MockVfs(
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
				}"#
		),
		""
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_description_object() {
	PackMeta::new(
		&MockVfs(
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
				}"#
		),
		""
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn well_formed_pack_mcmeta_with_description_array() {
	PackMeta::new(
		&MockVfs(
			r#"
				{
					"pack": {
						"pack_format": 7,
						"description": ["My pack", "is awesome"]
					}
				}"#
		),
		""
	)
	.await
	.expect("Unexpected failure reading pack metadata");
}

#[tokio::test]
async fn pack_mcmeta_with_missing_description() {
	assert!(
		PackMeta::new(
			&MockVfs(
				r#"
					{
						"pack": {
							"pack_format": 7
						}
					}"#
			),
			""
		)
		.await
		.is_err(),
		"Expected failure reading pack metadata"
	);
}

#[tokio::test]
async fn pack_mcmeta_with_bad_pack_format() {
	assert!(
		PackMeta::new(
			&MockVfs(
				r#"
					{
						"pack": {
							"pack_format": -0.5,
							"description": "My bad pack"
						}
					}"#
			),
			""
		)
		.await
		.is_err(),
		"Expected failure reading pack metadata"
	);
}

#[tokio::test]
async fn pack_mcmeta_without_expected_structure() {
	assert!(
		PackMeta::new(&MockVfs("42"), "").await.is_err(),
		"Expected failure reading pack metadata"
	);
}
