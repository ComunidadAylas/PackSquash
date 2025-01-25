//! Contains the definition of the pack dataset struct.

use std::collections::{HashMap, hash_map::Entry};
use std::fs::File;
use std::io;
use std::path::Path;

use tar::Archive;
use tempfile::{TempDir, tempdir};
use xz2::read::XzDecoder;

/// Represents a pack dataset, which provides access to packs in the dataset.
pub struct PackDataset<'path> {
	extracted_packs: HashMap<&'path Path, TempDir>
}

impl<'path> PackDataset<'path> {
	/// Creates a new pack dataset.
	#[allow(clippy::new_without_default)]
	#[must_use]
	pub fn new() -> Self {
		Self {
			extracted_packs: HashMap::new()
		}
	}

	/// Extracts a pack, contained in a TAR archive compressed with XZ and specified by its relative
	/// path from the test packs directory, to a temporary directory. The temporary directory is
	/// automatically deleted when this struct is dropped. Packs with the same relative path will
	/// only be extracted once, so calling this method several times for the same relative path will
	/// return the same temporary directory.
	///
	/// The tarballs extracted by this function can be generated with GNU `tar` like this:
	/// ```
	/// $ tar -cJf packages/packsquash/benches/data/packs/pack.tar.xz --exclude-vcs -C /tmp/pack .
	/// ```
	pub fn get(
		&mut self,
		relative_path: &'path (impl AsRef<Path> + ?Sized)
	) -> Result<&TempDir, io::Error> {
		match self.extracted_packs.entry(relative_path.as_ref()) {
			Entry::Occupied(entry) => Ok(entry.into_mut()),
			Entry::Vacant(entry) => {
				let pack_tarball = File::open(
					Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/benches/data/packs"))
						.join(relative_path)
				)?;
				let extract_directory = tempdir()?;

				Archive::new(XzDecoder::new(pack_tarball)).unpack(extract_directory.path())?;

				Ok(entry.insert(extract_directory))
			}
		}
	}
}
