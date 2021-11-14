use std::convert::TryInto;
use std::io;
use std::{convert::TryFrom, path::Path};

use enumset::EnumSet;
use json_comments::StripComments;
use serde_json::Value;
use tokio::io::AsyncReadExt;

use thiserror::Error;

use crate::pack_file::strip_utf8_bom;
use crate::{config::MinecraftQuirk, vfs::VirtualFileSystem};

/// The pack format version used in Minecraft versions from 1.13 to 1.14.4.
pub const PACK_FORMAT_VERSION_1_13: i32 = 4;
/// The pack format version used in Minecraft versions from 1.17 to 1.17.1.
pub const PACK_FORMAT_VERSION_1_17: i32 = 7;

/// Metadata for a resource or data pack, contained in the `pack.mcmeta` file
/// in the root folder of a pack.
///
/// References:
/// - https://minecraft.fandom.com/wiki/Resource_Pack#Contents
/// - https://minecraft.fandom.com/wiki/Data_Pack#pack.mcmeta
/// - Minecraft class `net.minecraft.server.packs.metadata.pack.PackMetadataSectionSerializer`
pub struct PackMeta {
	pack_format_version: i32
}

/// Represents an error that may happen while parsing pack metadata files.
#[derive(Error, Debug)]
pub enum PackMetaError {
	#[error("JSON error: {0}")]
	JsonSerde(#[from] serde_json::Error),
	#[error("Syntax error: {0}")]
	MalformedMeta(&'static str),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

impl PackMeta {
	/// Creates a new pack metadata struct from a virtual filesystem and its root path.
	pub async fn new<F: VirtualFileSystem, P: AsRef<Path>>(
		vfs: &F,
		root_path: P
	) -> Result<Self, PackMetaError> {
		const PACK_FORMAT_VERSION_IS_NOT_INTEGER: &str =
			"\"pack_format_version\" is not a Java integer";

		let pack_format_version;

		let mut file = vfs.open(root_path.as_ref().join("pack.mcmeta"))?;
		let mut pack_meta_value =
			Vec::with_capacity(file.file_size_hint.try_into().unwrap_or(usize::MAX));

		file.file_read.read_to_end(&mut pack_meta_value).await?;

		// Parse the pack metadata to get its format version and do some basic validation.
		// We do this parsing manually, instead of using auxiliary structs that derive
		// deserialization traits, because it is faster, provides more relevant error
		// information, and we only need to parse a few things that are unlikely to change
		match serde_json::from_reader(StripComments::new(strip_utf8_bom(&*pack_meta_value)))? {
			Value::Object(root_object) => {
				match root_object.get("pack").ok_or(PackMetaError::MalformedMeta(
					"Missing \"pack\" key in root object"
				))? {
					Value::Object(pack_meta_object) => {
						match pack_meta_object.get("pack_format").ok_or(
							PackMetaError::MalformedMeta(
								"Missing \"pack_format\" key in pack metadata object"
							)
						)? {
							Value::Number(pack_format_version_number) => {
								// Minecraft always reads this field as a Java integer,
								// so a conversion to an i32 should be successful
								pack_format_version = i32::try_from(
									pack_format_version_number.as_i64().ok_or(
										PackMetaError::MalformedMeta(
											PACK_FORMAT_VERSION_IS_NOT_INTEGER
										)
									)?
								)
								.map_err(|_| {
									PackMetaError::MalformedMeta(PACK_FORMAT_VERSION_IS_NOT_INTEGER)
								})?;
							}
							_ => {
								return Err(PackMetaError::MalformedMeta(
									PACK_FORMAT_VERSION_IS_NOT_INTEGER
								))
							}
						};

						// Also validate the pack description, because it is required by Minecraft
						match pack_meta_object.get("description") {
							Some(Value::String(_))
							| Some(Value::Object(_))
							| Some(Value::Array(_)) => {
								// This can possibly be a Minecraft text component, parsed by the
								// static class Serializer at net.minecraft.network.chat.Component
							}
							Some(_) => {
								return Err(PackMetaError::MalformedMeta(
									"The \"description\" key value is not a text component"
								))
							}
							None => {
								return Err(PackMetaError::MalformedMeta(
									"Missing \"description\" key in pack metadata object"
								))
							}
						};
					}
					_ => {
						return Err(PackMetaError::MalformedMeta(
							"The \"pack\" key value is not a JSON object"
						))
					}
				}
			}
			_ => {
				return Err(PackMetaError::MalformedMeta(
					"The JSON value is not an object"
				))
			}
		};

		Ok(Self {
			pack_format_version
		})
	}

	/// Returns a maybe pessimistic set of Minecraft quirks that will need to be
	/// worked around to guarantee that the pack will work as expected.
	///
	/// This is done by looking at the `pack_format` version in the pack metadata,
	/// as that version specifies a range of Minecraft versions that the pack is
	/// meant to be compatible with. If only a subset of Minecraft versions that
	/// a `pack_format` version targets are affected by a quirk, that quirk will be
	/// returned in the set. Similarly, if the Minecraft versions a `pack_format`
	/// version targets may or may not be affected by some quirk, that quirk will
	/// be returned too.
	pub fn target_minecraft_versions_quirks(&self) -> EnumSet<MinecraftQuirk> {
		let mut quirks = EnumSet::empty();

		if self.pack_format_version < PACK_FORMAT_VERSION_1_13 {
			quirks |= MinecraftQuirk::GrayscaleImagesGammaMiscorrection;
		}

		if self.pack_format_version < PACK_FORMAT_VERSION_1_17 {
			quirks |= MinecraftQuirk::Java8ZipParsing;
		}

		quirks
	}
}

#[cfg(test)]
mod tests {
	use std::{convert::TryInto, ffi::OsStr, fs::FileType, io, iter::Empty, path::Path};

	use crate::{
		vfs::{IteratorTraversalOptions, VirtualFileSystem},
		VfsFile, VfsPackFileIterEntry, VfsPackFileMetadata
	};
	use tokio_test::io::{Builder, Mock};

	use super::PackMeta;

	struct MockVfs(&'static str);

	impl VirtualFileSystem for MockVfs {
		type FileRead = Mock;

		type FileIter = Empty<Result<VfsPackFileIterEntry, io::Error>>;

		fn file_iterator(&self, _: &Path, _: IteratorTraversalOptions) -> Self::FileIter {
			unimplemented!()
		}

		fn open<P: AsRef<Path>>(&self, path: P) -> Result<VfsFile<Self::FileRead>, io::Error> {
			if path.as_ref().as_os_str() == OsStr::new("pack.mcmeta") {
				Ok(VfsFile {
					file_read: Builder::new().read(self.0.as_bytes()).build(),
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
	async fn pack_mcmeta_without_expected_json_estructure() {
		assert!(
			PackMeta::new(&MockVfs("42"), "").await.is_err(),
			"Expected failure reading pack metadata"
		);
	}
}
