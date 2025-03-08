use std::{borrow::Cow, ops::Deref};

use memchr::memmem;
use thiserror::Error;

use super::zip_file_record::END_OF_CENTRAL_DIRECTORY_SIGNATURE;

/// Represents a string that can be used as a ZIP file comment, which is limited
/// to 65535 bytes in size, doesn't contain the end of central directory
/// signature bytes, and only contains non-extended ASCII characters.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub struct ZipArchiveCommentString<'str>(Cow<'str, str>);

/// Represents an error that may happen while converting a string to a ZIP file
/// comment.
#[derive(Error, Debug)]
pub enum InvalidFileCommentStringError {
	#[error("The ZIP file comment contains non-ASCII characters at position {0}")]
	NonAscii(usize),
	#[error(
		"The ZIP file comment contains a reserved sequence of 4 characters starting at position {0}"
	)]
	ContainsEocdSignature(usize),
	#[error("The ZIP file comment exceeds the 65535 characters size limit")]
	TooBig
}

impl<'str> ZipArchiveCommentString<'str> {
	/// Creates a new ZIP file comment from the given string, which is limited
	/// to 65535 bytes in size, does not contain the end of central directory
	/// signature bytes, and only contain non-extended ASCII characters.
	pub(crate) fn new(
		comment: impl Into<Cow<'str, str>>
	) -> Result<Self, InvalidFileCommentStringError> {
		let comment = comment.into();

		if comment.len() > u16::MAX as usize {
			return Err(InvalidFileCommentStringError::TooBig);
		}

		// Because we know that the comment is 65535 characters long or less, and we expect
		// this constructor to be run once in the lifecycle of a pack optimization operation,
		// the naive "O(2n)" algorithm below is ~5x faster than multi-pattern searching with
		// Aho-Corasick, which has much higher overhead due to the initial automaton construction

		// The ZIP specification does not specify a character encoding for ZIP file comments,
		// so play it safe and only accept non-extended ASCII characters, which are interoperable
		// with extended ASCII codepages and UTF-8
		if let Some(non_ascii_char_position) = comment
			.char_indices()
			.find_map(|(i, c)| (!c.is_ascii()).then_some(i))
		{
			return Err(InvalidFileCommentStringError::NonAscii(
				non_ascii_char_position
			));
		}

		// The end of central directory signture bytes are technically valid ASCII, but they will
		// cause confusion on programs that attempt to locate the EOCD structure by searching for
		// ocurrences of its signature, which is the only practical general method for seek-enabled
		// ZIP file parsing with arbitrary padding and comments. Therefore, prevent failure modes
		// by ensuring the comment lacks such a signature. (Even though the comment string could
		// technically contain other ZIP header signatures, the enforced lack of a EOCD would render
		// those signatures inert for both streaming and seeking ZIP parsers.)
		if let Some(eocd_signature_position) =
			memmem::find(comment.as_bytes(), &END_OF_CENTRAL_DIRECTORY_SIGNATURE)
		{
			return Err(InvalidFileCommentStringError::ContainsEocdSignature(
				eocd_signature_position
			));
		}

		Ok(Self(comment))
	}
}

impl AsRef<str> for ZipArchiveCommentString<'_> {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl Deref for ZipArchiveCommentString<'_> {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
