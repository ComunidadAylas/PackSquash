//! Implements a ZIP friendly, UTF-8 relative path object with several useful operations
//! and properties.

use std::{
	borrow::Cow,
	io,
	ops::Deref,
	path::{Path, MAIN_SEPARATOR}
};

use thiserror::Error;

/// Represents a relative UTF-8 filesystem path, that doesn't begin with
/// prefix or root directory components, only contains normal components,
/// and whose length is limited to 65535 bytes.
///
/// This relative path can be referenced as a string slice that contains its
/// representation, normalized to always use the forward slash as a component
/// separator. Therefore, any instance of this struct is appropriate for
/// direct consumption by ZIP file data structures.
///
/// The struct is efficient, because it tries to use smart pointers to avoid
/// allocating new buffers to represent the relative path, borrowing data when
/// possible.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct RelativePath<'a>(Cow<'a, str>);

/// Represents an error that may happen while converting a path to a relative
/// path.
#[derive(Error, Debug)]
pub enum InvalidPathError<'a> {
	#[error("The path contains non UTF-8 characters: {0}")]
	NonUnicode(Cow<'a, str>),
	#[error("The path exceeds the 65535 bytes size limit")]
	TooBig
}

impl From<InvalidPathError<'_>> for io::Error {
	fn from(error: InvalidPathError<'_>) -> Self {
		io::Error::new(
			io::ErrorKind::Other,
			match error {
				InvalidPathError::NonUnicode(error_message) => {
					InvalidPathError::NonUnicode(Cow::Owned(error_message.into_owned()))
				}
				InvalidPathError::TooBig => InvalidPathError::TooBig
			}
		)
	}
}

impl<'a> RelativePath<'a> {
	/// Creates a new relative path from the given ancestor and descendant paths,
	/// so that the returned path only has components that are in the descendant
	/// path, but not in the ancestor path.
	///
	/// # Assumptions
	/// This method assumes that the descendant path is indeed a descendant of
	/// the ancestor path, without any checking. Therefore, the returned path
	/// will only make sense if this assumption is withheld. No canonicalization
	/// or symlink resolving is performed, as those may expose the physical
	/// structure, which may be different than the logical, expected directory
	/// structure.
	pub fn new<P1: AsRef<Path> + ?Sized, P2: AsRef<Path> + ?Sized>(
		ancestor_path: &P1,
		descendant_path: &'a P2
	) -> Result<Self, InvalidPathError<'a>> {
		let mut descendant_path_components = descendant_path.as_ref().components();

		// Discard the first common components
		for _ in ancestor_path.as_ref().components() {
			descendant_path_components.next();
		}

		// Interpret the remaining components as a relative path
		let relative_path = descendant_path_components.as_path();

		// Now join all the remaining components together, separated by a
		// forward slash (/). Be smart and avoid any allocations if the
		// component separator character for the current platform is already
		// a forward slash
		let relative_path_string = if MAIN_SEPARATOR == '/' {
			Cow::Borrowed(
				relative_path
					.to_str()
					.ok_or_else(|| InvalidPathError::NonUnicode(relative_path.to_string_lossy()))?
			)
		} else {
			Cow::Owned(
				descendant_path_components
					.map(|component| {
						component.as_os_str().to_str().ok_or_else(|| {
							InvalidPathError::NonUnicode(relative_path.to_string_lossy())
						})
					})
					.collect::<Result<Vec<&str>, InvalidPathError>>()?
					.join("/")
			)
		};

		if relative_path_string.len() > u16::MAX as usize {
			return Err(InvalidPathError::TooBig);
		}

		Ok(Self(relative_path_string))
	}

	/// Returns a borrowed view to the UTF-8 string representation of this relative path.
	/// Although a relative path can be converted to a Rust string by other means, this
	/// method is more ergonomic to use, and avoids any runtime checks.
	pub fn as_str(&self) -> &str {
		self.0.as_ref()
	}

	/// Returns another relative path that owns equivalent path data to this path, so
	/// that its lifetime bounds can now be indefinitely long.
	///
	/// Because this method borrows `self`, it always allocates a new buffer, even if
	/// this relative path already owned the data. It is also not the same as cloning
	/// the relative path, as cloning may just copy a reference to already borrowed
	/// path data, without extending its lifetime.
	pub fn as_owned(&self) -> RelativePath<'static> {
		RelativePath(Cow::Owned(self.0.clone().into_owned()))
	}

	/// Consumes this relative path to get another that owns all the path data it refers
	/// to, so that its lifetime bounds can now be indefinitely long.
	///
	/// This only allocates memory if the relative path didn't own all of the data it
	/// referred to. Otherwise, this operation is effectively a no-op.
	pub fn into_owned(self) -> RelativePath<'static> {
		RelativePath(Cow::Owned(self.0.into_owned()))
	}

	/// Unwraps this relative path to get the internal copy-on-write smart pointer to
	/// the string that holds it.
	pub fn into_inner(self) -> Cow<'a, str> {
		self.0
	}

	/// Creates a new relative path directly from its raw representation, without
	/// any checks or processing. This is a low-level constructor.
	///
	/// # Assumptions
	/// The caller is responsible of providing a string that upholds the expectations
	/// of this struct; namely, that the string is a normalized, relative path that
	/// always uses the forward slash (/) as a component separator, and does not
	/// exceed 65535 bytes.
	pub(crate) fn from_inner<T: Into<Cow<'a, str>>>(string: T) -> Self {
		Self(string.into())
	}
}

impl AsRef<Path> for RelativePath<'_> {
	fn as_ref(&self) -> &Path {
		Path::new(&*self.0)
	}
}

impl Deref for RelativePath<'_> {
	type Target = Path;

	fn deref(&self) -> &Self::Target {
		Path::new(&*self.0)
	}
}
