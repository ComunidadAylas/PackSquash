//! Implements a ZIP friendly, UTF-8 relative path object with several useful
//! operations and properties.

use std::{
	borrow::Cow,
	fmt,
	fmt::{Display, Formatter},
	io,
	ops::Deref,
	path::MAIN_SEPARATOR
};

use camino::Utf8Path;
use itertools::Itertools;
use thiserror::Error;

/// Represents a relative UTF-8 filesystem path that doesn't begin with
/// prefix or root directory components, only contains normal components,
/// and whose length is limited to 65535 bytes.
///
/// This relative path can be referenced as a string slice that contains its
/// representation, normalized to always use the forward slash as a component
/// separator. Therefore, any instance of this struct is appropriate for pack
/// file paths and direct consumption by ZIP file data structures.
///
/// The struct is efficient, because it tries to use smart pointers to avoid
/// allocating new buffers to represent the relative path, borrowing data when
/// possible.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct RelativePath<'path>(Cow<'path, str>);

/// Represents an error that may happen while converting a path to a relative
/// path.
#[derive(Error, Debug)]
pub enum InvalidPathError {
	#[error("The path exceeds the 65535 bytes size limit")]
	TooBig
}

impl From<InvalidPathError> for io::Error {
	fn from(_: InvalidPathError) -> Self {
		io::Error::new(io::ErrorKind::Other, InvalidPathError::TooBig)
	}
}

impl<'path> RelativePath<'path> {
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
	pub(crate) fn new<P1: AsRef<Utf8Path> + ?Sized, P2: AsRef<Utf8Path> + ?Sized>(
		ancestor_path: &P1,
		descendant_path: &'path P2
	) -> Result<Self, InvalidPathError> {
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
			Cow::Borrowed(relative_path.as_str())
		} else {
			Cow::Owned(
				descendant_path_components
					.map(|component| component.as_str())
					.join("/")
			)
		};

		if relative_path_string.len() > u16::MAX as usize {
			return Err(InvalidPathError::TooBig);
		}

		Ok(Self(relative_path_string))
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

	pub fn reborrow(&self) -> RelativePath<'_> {
		RelativePath(Cow::Borrowed(&self.0))
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
	pub fn into_inner(self) -> Cow<'path, str> {
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
	pub(crate) fn from_inner<T: Into<Cow<'path, str>>>(string: T) -> Self {
		Self(string.into())
	}

	pub(crate) const fn from_inner_borrowed(string: &'path str) -> Self {
		Self(Cow::Borrowed(string))
	}
}

impl AsRef<Utf8Path> for RelativePath<'_> {
	fn as_ref(&self) -> &Utf8Path {
		Utf8Path::new(&*self.0)
	}
}

impl Deref for RelativePath<'_> {
	type Target = Utf8Path;

	fn deref(&self) -> &Self::Target {
		Utf8Path::new(&*self.0)
	}
}

impl Display for RelativePath<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
		f.write_str(self.as_str())
	}
}
