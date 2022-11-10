//! Contains the data types that support a virtual filesystem implementation
//! that operates with files from the operating system filesystems.

use memmap2::Mmap;
use patricia_tree::PatriciaSet;
use std::borrow::Cow;
use std::io::{BufReader, ErrorKind, Read};
use std::{
	fs::{self, File},
	io,
	path::Path
};

use walkdir::{DirEntry, WalkDir};

use crate::scratch_file::ScratchFilesBudget;
use crate::RelativePath;

use super::{VfsFile, VfsMmap, VirtualFileSystem};

/// A virtual filesystem implementation that operates with files in the mounted
/// operating system filesystems. In other words, it is a facade for `std::fs`.
pub struct OsFilesystem<'path, 'budget> {
	root_path: Cow<'path, Path>,
	memory_budget: &'budget ScratchFilesBudget
}

impl<'path, 'budget> OsFilesystem<'path, 'budget> {
	pub fn new(
		root_path: impl Into<Cow<'path, Path>>,
		memory_budget: &'budget ScratchFilesBudget
	) -> Self {
		Self {
			root_path: root_path.into(),
			memory_budget
		}
	}
}

impl VirtualFileSystem for OsFilesystem<'_, '_> {
	type FileRead = BufReader<File>;

	fn file_set(&self) -> io::Result<PatriciaSet> {
		WalkDir::new(&self.root_path)
			.min_depth(1) // Do not yield the root path itself, but all of its children
			.follow_links(true)
			.max_open(2)
			.into_iter()
			.filter_map(|entry_result| {
				entry_result.map_or_else(
					|err| Some(Err(err.into())),
					|entry| {
						// Do not yield directories. We are interested in the directory tree leaves only
						(!entry.file_type().is_dir()).then(|| {
							// Use the entry depth to efficiently get a root path without doing
							// additional allocations or moving ownership of the method parameter
							let entry_depth = entry.depth();
							let file_path = entry.into_path();
							let root_path =
								file_path.ancestors().take(1 + entry_depth).last().unwrap();

							Ok(RelativePath::new(root_path, &file_path)?
								.into_owned()
								.into_inner()
								.into_owned())
						})
					}
				)
			})
			.collect()
	}

	fn open(&self, path: &RelativePath) -> io::Result<VfsFile<Self::FileRead>> {
		let file = File::open(self.root_path.join(path))?;
		let file_metadata = file.metadata().ok();

		Ok(VfsFile {
			file_size: file_metadata.as_ref().map(|metadata| metadata.len()),
			modification_time: file_metadata.and_then(|metadata| metadata.modified().ok()),
			reader: BufReader::new(file)
		})
	}

	fn mmap(&self, path: &RelativePath) -> io::Result<VfsMmap> {
		let mut backing_file = File::open(self.root_path.join(path))?;
		let file_metadata = backing_file.metadata().ok();

		// Get the file size to decide which strategy to use to make the whole file
		// available as a memory buffer. If there is enough memory available, we
		// just naively read the entire file to a buffer. Otherwise, we memory map
		// it with OS-specific system calls. We only use memory mapping as a fallback
		// due to its potentially significant virtual memory mapping and page fault
		// costs, coupled with safety concerns and predictably insignificant
		// performance gains over a few syscalls to read everything to memory
		let file_size = file_metadata.as_ref().map_or_else(
			|| usize::MAX,
			|metadata| metadata.len().try_into().unwrap_or(usize::MAX)
		);

		Ok(if self.memory_budget.spend(file_size) {
			let mut buf = vec![];
			backing_file.read_to_end(&mut buf)?; // read_to_end reserves capacity before reading

			VfsMmap::UserspaceBuffer {
				buf: Cow::Owned(buf),
				modification_time: file_metadata.and_then(|metadata| metadata.modified().ok())
			}
		} else {
			VfsMmap::PlatformMmap {
				// SAFETY: memory map semantics cannot be ergonomically expressed in Rust. Any
				// modification to the underlying file is propagated to the map by the OS, but
				// as we deal with an immutable slice, the Rust compiler is free to assume that
				// it indeed is immutable, leading to undefined behavior if that turns out to
				// be wrong. In practice, however:
				// - It is unlikely for pack files to be changed while they are being processed.
				// - Accessing memory from memory map whose backing file was truncated or deleted
				//   may generate an unsolvable page fault, in which case the OS will kill us
				//   before we have any chance of working with garbage data (on Linux, with SIGBUS).
				// - On Linux, the ELF loader memory maps executables, but almost nobody ever
				//   worried about application code being modified while it is being executed. By
				//   that logic, even read-only Rust statics can be unsafe, which is unhelpful.
				// - As we resort to memory maps when memory pressure is high, the alternative
				//   would be running out of memory and be killed outright, especially on systems
				//   without memory overcommitment, such as Windows. Memory mapping hides that
				//   fact in a way that works in the vast majority of cases, turning a "will
				//   use too much memory and likely be killed" situation into a "it can work
				//   fine as long as files are not modified, and if they are the OS likely will
				//   prevent the worst problems", which is better.
				// Therefore, apply the ostrich algorithm to deal with these concerns
				#[allow(unsafe_code)]
				mmap: unsafe { Mmap::map(&backing_file)? },
				modification_time: file_metadata.and_then(|metadata| metadata.modified().ok())
			}
		})
	}

	fn is_dir(&self, path: &RelativePath) -> io::Result<bool> {
		match fs::metadata(self.root_path.join(path)) {
			Ok(metadata) => Ok(metadata.is_dir()),
			Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
			Err(err) => Err(err)
		}
	}
}

// TODO decide what to do with this: parameter on constructor to toggle?
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

	use crate::util::patricia_set_relative_path_iter::PatriciaSetRelativePathIterExt;
	use pretty_assertions::assert_eq;
	use tempfile::Builder;

	static VIRTUALLY_UNLIMITED_MEMORY_BUDGET: ScratchFilesBudget =
		ScratchFilesBudget::new(usize::MAX);

	#[test]
	fn os_filesystem_vfs_works() {
		let root_dir = Builder::new()
			.prefix("ps-osfs-test")
			.tempdir()
			.expect("I/O operations are assumed not to fail during tests");

		// The following lines create this file tree:
		// ├── bye
		// │   ├── bye
		// │   │   └── now.txt
		// │   └── .dont_come_back.txt
		// └── hello
		//     └── world.txt

		let mut file1_path = root_dir.path().join("hello");
		file1_path.push("world.txt");
		{
			fs::create_dir_all(file1_path.parent().unwrap())
				.expect("I/O operations are assumed not to fail during tests");

			File::create(&file1_path).expect("I/O operations are assumed not to fail during tests");
		}

		let mut file2_path = root_dir.path().join("bye");
		file2_path.push("bye");
		file2_path.push("now.txt");
		{
			fs::create_dir_all(file2_path.parent().unwrap())
				.expect("I/O operations are assumed not to fail during tests");

			File::create(&file2_path).expect("I/O operations are assumed not to fail during tests");
		}

		let mut file3_path = root_dir.path().join("bye");
		file3_path.push(".dont_come_back.txt");
		{
			File::create(&file3_path).expect("I/O operations are assumed not to fail during tests");
		}

		let file_set = OsFilesystem::new(root_dir.path(), &VIRTUALLY_UNLIMITED_MEMORY_BUDGET)
			.file_set()
			.expect("I/O operations are assumed not to fail during tests");

		const RELATIVE_PATHS: &[&str] = &[
			"hello/world.txt",
			"bye/bye/now.txt",
			"bye/.dont_come_back.txt"
		];

		let mut file_count = 0;
		for file in file_set.relative_path_iter() {
			assert!(RELATIVE_PATHS.contains(&file.as_str()));

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
		let file_set = OsFilesystem::new(Path::new("."), &VIRTUALLY_UNLIMITED_MEMORY_BUDGET)
			.file_set()
			.expect("I/O operations are assumed not to fail during tests");

		assert!(
			file_set.into_iter().next().is_some(),
			"The current working directory, where the source files are located, is expected to have files"
		);
	}

	#[test]
	fn single_component_double_dot_relative_path_works() {
		let file_set = OsFilesystem::new(Path::new(".."), &VIRTUALLY_UNLIMITED_MEMORY_BUDGET)
			.file_set()
			.expect("I/O operations are assumed not to fail during tests");

		assert!(
			file_set.into_iter().next().is_some(),
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
		File::create(&file_path).expect("I/O operations are assumed not to fail during tests");

		let file_set = OsFilesystem::new(root_dir.path(), &VIRTUALLY_UNLIMITED_MEMORY_BUDGET)
			.file_set()
			.expect("I/O operations are assumed not to fail during tests");

		assert_eq!(
			file_set.into_iter().count(),
			1,
			"An initial hidden directory should be descended into and yield exactly one file in this test"
		);
	}
}
