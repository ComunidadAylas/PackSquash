//! Contains miscellaneous small helper functions that are used for processing pack files.

use std::{borrow::Cow, ffi::OsStr, path::Path};

use globset::{Glob, GlobSet};

use crate::config::compile_pack_file_glob_pattern;

use super::PackFileAssetType;

/// Returns a slice of the input buffer that skips the initial byte order
/// mark character that some programs (most notably, Microsoft software,
/// like Notepad) add to UTF-8 encoded files. If no BOM is present, the
/// buffer is returned as-is.
pub fn strip_utf8_bom(buf: &[u8]) -> &[u8] {
	// These bytes are the UTF-8 representation of
	// character 0xFEFF (BYTE ORDER MARK)
	if buf.len() > 2 && buf[..3] == [0xEF, 0xBB, 0xBF] {
		&buf[3..]
	} else {
		buf
	}
}

/// Returns the extension of the provided path, as if the `to_ascii_lowercase` method was called on
/// it, but avoiding allocating a new copy of the string if no changes were made. This is efficient
/// because extensions are usually short and already lowercase.
pub fn to_ascii_lowercase_extension<P: AsRef<Path> + ?Sized>(path: &P) -> Cow<'_, str> {
	let extension = Path::new(path.as_ref())
		.extension()
		.unwrap_or_else(|| OsStr::new(""))
		.to_string_lossy();

	if extension.chars().any(|c| c.is_ascii_uppercase()) {
		Cow::Owned(extension.to_ascii_lowercase())
	} else {
		extension
	}
}

/// Compiles the specified pack file glob pattern, assuming it was hardcoded in the application binary.
/// Any validity error is discarded and turned into a panic, as modification of hardcoded data is not
/// to be handled as an error.
///
/// Please note that, even though this function requires a static string slice in an effort to prevent
/// accidental misuse, it is possible to get string slices that live indefinitely by leaking a heap
/// allocation.
pub fn compile_hardcoded_pack_file_glob_pattern(glob_pattern: &'static str) -> Glob {
	compile_pack_file_glob_pattern(glob_pattern).unwrap()
}

/// Returns the asset type for a pack file given its path. If several asset types are suitable for this file,
/// the last in the enum declaration will be returned. A filter function can be used to conditionally ignore
/// some matched asset types, depending on the configuration. It is assumed that the `globset` was constructed
/// by the `pack_file_asset_type_globset` function.
pub(super) fn best_asset_type_match<P: AsRef<Path>, F: FnMut(&T) -> bool, T: PackFileAssetType>(
	globset: &GlobSet,
	path: P,
	type_filter: F
) -> Option<T> {
	globset
		.matches(path)
		.into_iter()
		.filter_map(|type_index| T::try_from(type_index).ok())
		.filter(type_filter)
		.max()
}
