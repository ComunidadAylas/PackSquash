//! Contains the data types that support a virtual filesystem implementation
//! that operates with files from the operating system filesystems.

use std::borrow::Cow;
use std::{
	fs::{self, File, FileType},
	io::{self, ErrorKind},
	path::Path
};

use tokio::io::BufReader;
use walkdir::{DirEntry, WalkDir};

use crate::RelativePath;

use super::{
	IteratorTraversalOptions, VfsFile, VfsPackFileIterEntry, VfsPackFileMetadata, VirtualFileSystem
};

/// A virtual filesystem implementation that operates with files in the mounted
/// operating system filesystems. In other words, it is a facade for `std::fs`.
pub struct OsFilesystem;

impl VirtualFileSystem for OsFilesystem {
	type FileRead = BufReader<tokio::fs::File>;
	type FileIter = impl Iterator<Item = Result<VfsPackFileIterEntry, io::Error>>;

	fn file_iterator(
		&self,
		root_path: &Path,
		iterator_traversal_options: IteratorTraversalOptions
	) -> Self::FileIter {
		let entry_iter = WalkDir::new(root_path)
			.min_depth(1) // Do not yield the root path itself, but all of its children
			.follow_links(true)
			.max_open(10)
			.into_iter()
			.filter_entry(move |entry| {
				!iterator_traversal_options.ignore_system_and_hidden_files
					|| !is_system_or_hidden_file(entry)
			});

		entry_iter.filter_map(|entry_result| {
			entry_result.map_or_else(
				|err| Some(Err(io::Error::new(ErrorKind::Other, err))),
				|entry| {
					// Do not yield directories. We are interested in the directory tree leaves only
					(!entry.file_type().is_dir()).then(|| {
						// Use the entry depth to efficiently get a root path without doing
						// additional allocations or moving ownership of the method parameter
						let entry_depth = entry.depth();
						let file_path = entry.into_path();
						let root_path = file_path.ancestors().take(1 + entry_depth).last().unwrap();

						Ok(VfsPackFileIterEntry {
							relative_path: RelativePath::new(root_path, &file_path)?.into_owned(),
							file_path
						})
					})
				}
			)
		})
	}

	fn open<P: AsRef<Path>>(&self, path: P) -> Result<VfsFile<Self::FileRead>, io::Error> {
		// This matches what walkdir would do on a DirEntry, because we follow symlinks
		let metadata = fs::metadata(&path)?;

		Ok(VfsFile {
			file_read: BufReader::new(tokio::fs::File::from_std(File::open(path)?)),
			metadata: VfsPackFileMetadata {
				modification_time: metadata.modified().ok()
			},
			file_size_hint: metadata.len()
		})
	}

	fn file_type<P: AsRef<Path>>(&self, path: P) -> Result<FileType, io::Error> {
		fs::metadata(path).map(|metadata| metadata.file_type())
	}
}

/// Checks whether a [DirEntry] is a system or hidden file. This operation does no syscalls.
fn is_system_or_hidden_file(entry: &DirEntry) -> bool {
	let (_file_name_str, file_name): (Cow<'_, str>, &[u8]);

	#[cfg(unix)]
	{
		// Fast path
		file_name = std::os::unix::ffi::OsStrExt::as_bytes(entry.file_name());
	}
	#[cfg(not(unix))]
	{
		_file_name_str = entry.file_name().to_string_lossy();
		file_name = _file_name_str.as_bytes()
	}

	// List based on https://www.toptal.com/developers/gitignore/api/git,windows,linux,macos
	file_name.starts_with(b".")
		|| if entry.file_type().is_file() {
			file_name == b"desktop.ini"
				|| file_name == b"Desktop.ini"
				|| file_name == b"Thumbs.db"
				|| file_name == b"ehthumbs.db"
				|| file_name == b"ehthumbs_vista.db"
				|| file_name.ends_with(b".lnk")
				|| file_name.ends_with(b".orig")
				|| file_name.ends_with(b".bak")
				|| file_name.ends_with(b".tmp")
		} else {
			file_name == b"Network Trash Folder"
				|| file_name == b"Temporary Items"
				|| file_name == b"$RECYCLE.BIN"
		}
}

#[cfg(test)]
mod tests {
	use super::*;

	use std::fs::File;

	use pretty_assertions::assert_eq;
	use tempfile::Builder;

	#[test]
	fn os_filesystem_vfs_works() {
		let root_dir = Builder::new()
			.prefix("ps-osfs-test")
			.tempdir()
			.expect("I/O operations are assumed not to fail during tests");

		let mut file1_path = root_dir.path().join("hello");
		file1_path.push("world.txt");
		{
			fs::create_dir_all(file1_path.parent().unwrap())
				.expect("I/O operations are assumed not to fail during tests");

			File::create(file1_path).expect("I/O operations are assumed not to fail during tests");
		}

		let mut file2_path = root_dir.path().join("bye");
		file2_path.push("bye");
		file2_path.push("now.txt");
		{
			fs::create_dir_all(file2_path.parent().unwrap())
				.expect("I/O operations are assumed not to fail during tests");

			File::create(file2_path).expect("I/O operations are assumed not to fail during tests");
		}

		let mut file3_path = root_dir.path().join("bye");
		file3_path.push(".dont_come_back.txt");
		{
			File::create(file3_path).expect("I/O operations are assumed not to fail during tests");
		}

		let file_iter = OsFilesystem.file_iterator(
			root_dir.path(),
			IteratorTraversalOptions {
				ignore_system_and_hidden_files: true
			}
		);

		const RELATIVE_PATHS: &[&str] = &["hello/world.txt", "bye/bye/now.txt"];

		let mut file_count = 0;
		for file in file_iter {
			assert!(RELATIVE_PATHS.contains(
				&file
					.expect("I/O operations are assumed not to fail during tests")
					.relative_path
					.as_str()
			));

			file_count += 1;
		}

		assert_eq!(
			file_count,
			RELATIVE_PATHS.len(),
			"Unexpected number of files yielded"
		);
	}

	#[test]
	fn single_component_dot_relative_path_works() {
		let mut file_iter = OsFilesystem.file_iterator(
			Path::new("."),
			IteratorTraversalOptions {
				ignore_system_and_hidden_files: true
			}
		);

		assert!(
			file_iter.next().is_some(),
			"The current working directory, where the source files are located, is expected to have files"
		);
	}

	#[test]
	fn single_component_double_dot_relative_path_works() {
		let mut file_iter = OsFilesystem.file_iterator(
			Path::new(".."),
			IteratorTraversalOptions {
				ignore_system_and_hidden_files: true
			}
		);

		assert!(
			file_iter.next().is_some(),
			"The parent of the current working directory, where the source files are located, is expected to have files"
		);
	}

	#[test]
	fn hidden_top_directory_is_descended_into() {
		let root_dir = Builder::new()
			.prefix(".ps-osfs-test")
			.tempdir()
			.expect("I/O operations are assumed not to fail during tests");

		let file_path = root_dir.path().join("file.bin");
		File::create(file_path).expect("I/O operations are assumed not to fail during tests");

		let file_iter = OsFilesystem.file_iterator(
			root_dir.path(),
			IteratorTraversalOptions {
				ignore_system_and_hidden_files: true
			}
		);

		assert_eq!(
			file_iter.count(),
			1,
			"An initial hidden directory should be descended into and yield exactly one file in this test"
		);
	}
}
