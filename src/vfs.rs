//! Contains virtual file systems implementations to use with `PackSquasher`.

use std::path::PathBuf;
use std::time::SystemTime;
use std::{fs::FileType, io, path::Path};

use crate::RelativePath;
use tokio::io::AsyncRead;

pub mod os_fs;

/// Defines the contract that any virtual file system must implement.
pub trait VirtualFileSystem: Send + Sync {
	/// The type of the byte source that this virtual file system yields
	/// when successfully opening files. It reading bytes from this source
	/// is costly, it is recommended to introduce some buffering for maximum
	/// performance.
	type FileRead: AsyncRead + Unpin + Send + 'static;
	/// The type of the iterator over the files within a path that this virtual
	/// file system yields.
	type FileIter: Iterator<Item = Result<VfsPackFileIterEntry, io::Error>>;

	/// Returns an iterator over the files that are in the filesystem subtree
	/// whose root is at `root_path`, which usually is a directory, according
	/// to the specified directory traversal options. This means that not only
	/// direct children of the `root_path` are yielded, but also grandchildren
	/// and so on. Files that don't hold any readable user data (i.e. directories)
	/// are not yielded.
	///
	/// Any I/O error that may happen is returned as an element in the iterator,
	/// so getting the iterator itself can't fail.
	fn file_iterator(
		&self,
		root_path: &Path,
		iterator_traversal_options: IteratorTraversalOptions
	) -> Self::FileIter;

	/// Opens the file at the specified virtual filesystem path for read-only access.
	fn open<P: AsRef<Path>>(&self, path: P) -> Result<VfsFile<Self::FileRead>, io::Error>;

	/// Returns the type of the file at the specified virtual filesystem path.
	fn file_type<P: AsRef<Path>>(&self, path: P) -> Result<FileType, io::Error>;
}

/// Contains options that tweak the operation of the [`VirtualFileSystem::file_iterator`]
/// method.
#[non_exhaustive]
#[derive(Default)]
pub struct IteratorTraversalOptions {
	/// Whether system (i.e. clearly not part of a pack file) and hidden files
	/// (usually, those whose name begins with a dot) are yielded or not.
	pub ignore_system_and_hidden_files: bool
}

/// A entry in a virtual filesystem directory that represents a possible pack file,
/// obtained via a virtual filesystem directory file iterator.
pub struct VfsPackFileIterEntry {
	/// The relative path of the pack file represented by this entry to the root of the pack.
	pub relative_path: RelativePath<'static>,
	/// The raw path buffer of the file represented by this entry. This path may or not may
	/// be relative, but it's apt for use in the [`VirtualFileSystem::open`] method.
	pub file_path: PathBuf
}

/// An open file in a virtual filesystem, from which data can be read and
/// metadata is available.
pub struct VfsFile<R: AsyncRead + Unpin + 'static> {
	/// An asynchronous stream that reads bytes from the file.
	pub file_read: R,
	/// The estimated size of the file, in bytes. This estimation need not be accurate, but
	/// it should prefer underestimating the size rather than overestimating it.
	pub file_size_hint: u64,
	/// The filesystem metadata for this file.
	pub metadata: VfsPackFileMetadata
}

/// Metadata of a virtual filesystem file.
pub struct VfsPackFileMetadata {
	/// The time when this file was last modified. Creating a file usually sets its modification
	/// time to the creation time.
	pub modification_time: Option<SystemTime>
}
