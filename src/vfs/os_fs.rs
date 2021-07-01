//! TODO

use std::{
	fs::{self, File, FileType},
	io::{self, ErrorKind},
	path::Path
};

use tokio::io::BufReader;
use walkdir::{DirEntry, WalkDir};

use crate::{PackFileVfsMetadata, RelativePath};

use super::{DirectoryTraversalOptions, VirtualFileSystem};

/// TODO: tests and docs
pub struct OsFilesystem;

impl VirtualFileSystem for OsFilesystem {
	type FileRead = BufReader<tokio::fs::File>;
	type FileIter = impl Iterator<Item = Result<PackFileVfsMetadata<Self::FileRead>, io::Error>>;

	fn file_iterator(
		&self,
		root_path: &Path,
		directory_traversal_options: DirectoryTraversalOptions
	) -> Self::FileIter {
		let entry_iter = WalkDir::new(&root_path)
			.follow_links(true)
			.into_iter()
			.filter_entry(move |entry| {
				!directory_traversal_options.ignore_system_and_hidden_files
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
						let file_metadata = entry.metadata()?;
						let file_path = entry.into_path();
						let root_path = file_path.ancestors().take(1 + entry_depth).last().unwrap();

						Ok(PackFileVfsMetadata {
							relative_path: RelativePath::new(root_path, &file_path)?.into_owned(),
							file_read: BufReader::new(tokio::fs::File::from_std(File::open(
								&file_path
							)?)),
							creation_time: file_metadata.created()?,
							modification_time: file_metadata.modified()?,
							file_size: file_metadata.len()
						})
					})
				}
			)
		})
	}

	fn file_type<P: AsRef<Path>>(&self, path: P) -> Result<FileType, io::Error> {
		fs::metadata(path).map(|metadata| metadata.file_type())
	}
}

/// Checks whether a [DirEntry] is a system or hidden file. This operation does no syscalls.
fn is_system_or_hidden_file(entry: &DirEntry) -> bool {
	let file_name = entry.file_name().to_string_lossy();

	// List based on https://www.toptal.com/developers/gitignore/api/git,windows,linux,macos
	file_name.starts_with('.')
		|| if entry.file_type().is_file() {
			file_name == "desktop.ini"
				|| file_name == "Desktop.ini"
				|| file_name == "Thumbs.db"
				|| file_name == "ehthumbs.db"
				|| file_name == "ehthumbs_vista.db"
				|| file_name.ends_with(".lnk")
				|| file_name.ends_with(".orig")
				|| file_name.ends_with(".bak")
		} else {
			file_name == "Network Trash Folder"
				|| file_name == "Temporary Items"
				|| file_name == "$RECYCLE.BIN"
		}
}
