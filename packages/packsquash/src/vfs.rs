//! Contains the virtual file system trait and implementations.

use std::{
	borrow::Cow,
	io::{self, BufRead, Seek},
	ops::Deref,
	time::SystemTime
};

use memmap2::Mmap;

use crate::{relative_path::RelativePathPatriciaSet, RelativePath};

pub(crate) mod os_fs;

pub enum VirtualFileSystemType {
	OsFilesystem
}

/// Defines the contract that any virtual file system must implement.
pub trait VirtualFileSystem: Send + Sync {
	/// The type of the byte source that this virtual file system yields
	/// when successfully opening files.
	type FileRead: BufRead + Seek;

	/// Returns a set containing all the files that belong to this virtual filesystem.
	/// Files that don't hold any readable user data (i.e., directories) are not
	/// included in the set.
	///
	/// Any I/O error that may happen is propagated to the caller.
	// TODO document TOCTOU possibility
	fn file_set(&self) -> io::Result<RelativePathPatriciaSet<'static>>;

	/// Opens the file at the specified path for read-only access.
	fn open(&self, path: &RelativePath<'_>) -> io::Result<VfsFile<Self::FileRead>>;

	fn mmap(&self, path: &RelativePath<'_>) -> io::Result<VfsMmap>;

	// TODO docs: do not return error if it does not exist
	fn is_dir(&self, path: &RelativePath<'_>) -> io::Result<bool>;
}

/// An open file in a virtual filesystem, from which data can be read and
/// metadata is available.
pub struct VfsFile<R: BufRead + Seek> {
	/// A seekable reader from which the file contents can be retrieved.
	pub reader: R,
	/// The time when this file was last modified. Creating a file usually sets its modification
	/// time to the creation time.
	pub modification_time: Option<SystemTime>,
	pub file_size: Option<u64>
}

pub enum VfsMmap {
	PlatformMmap {
		mmap: Mmap,
		modification_time: Option<SystemTime>
	},
	UserspaceBuffer {
		buf: Cow<'static, [u8]>,
		modification_time: Option<SystemTime>
	}
}

impl VfsMmap {
	pub fn modification_time(&self) -> Option<SystemTime> {
		match self {
			Self::PlatformMmap {
				modification_time, ..
			} => *modification_time,
			Self::UserspaceBuffer {
				modification_time, ..
			} => *modification_time
		}
	}
}

impl AsRef<[u8]> for VfsMmap {
	fn as_ref(&self) -> &[u8] {
		match self {
			Self::PlatformMmap { mmap, .. } => mmap,
			Self::UserspaceBuffer { buf, .. } => buf
		}
	}
}

impl Deref for VfsMmap {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		match self {
			Self::PlatformMmap { mmap, .. } => mmap,
			Self::UserspaceBuffer { buf, .. } => buf
		}
	}
}
