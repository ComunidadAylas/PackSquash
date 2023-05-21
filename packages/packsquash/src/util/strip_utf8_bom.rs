//! Contains the implementation of the [`StripUtf8BomExt`] trait.

use std::{
	convert::Infallible,
	error::Error,
	io::{self, Read}
};

use tinyvec::ArrayVec;

/// The UTF-8 representation of the Unicode byte order mark character.
const BOM_UTF8: [u8; 3] = [0xEF, 0xBB, 0xBF];
/// The size of the UTF-8 representation of the Unicode byte order mark
/// character, in bytes.
const BOM_UTF8_SIZE: usize = BOM_UTF8.len();

/// Extension trait for byte sources that provides a method to ergonomically
/// strip an optional initial byte order mark character.
///
/// Byte order marks (BOMs) are not required or recommended for UTF-8, but
/// some programs, notably older Microsoft Notepad versions, store and expect
/// them.
pub trait StripUtf8BomExt<T: Read, E: Error> {
	/// Strips the initial UTF-8 BOM character from the bytes yielded by this
	/// reader. If there is no initial UTF-8 BOM character, no bytes are stripped.
	///
	/// Implementations of this method are guaranteed to be efficient, avoiding
	/// buffering the reader data as much as possible.
	fn strip_utf8_bom(self) -> Result<T, E>;
}

impl<R: Read> StripUtf8BomExt<StripUtf8Bom<R>, io::Error> for R {
	fn strip_utf8_bom(mut self) -> io::Result<StripUtf8Bom<R>> {
		let maybe_bom = self
			.by_ref()
			.bytes()
			.take(BOM_UTF8_SIZE)
			.collect::<Result<ArrayVec<[u8; BOM_UTF8_SIZE]>, _>>()?;

		Ok(if maybe_bom == &BOM_UTF8 {
			// We've just read and skipped the BOM, so pass through the reader
			StripUtf8Bom::RemovedBom(self)
		} else {
			// Our read was too short, or it was not a BOM. Make sure we yield
			// any non-BOM header bytes back to consumers
			StripUtf8Bom::RewindHeader {
				non_bom_header: maybe_bom,
				rest: Some(self)
			}
		})
	}
}

impl<'slice> StripUtf8BomExt<&'slice [u8], Infallible> for &'slice [u8] {
	fn strip_utf8_bom(self) -> Result<&'slice [u8], Infallible> {
		Ok(
			if self.len() >= BOM_UTF8_SIZE && self[..BOM_UTF8_SIZE] == BOM_UTF8 {
				&self[BOM_UTF8_SIZE..]
			} else {
				self
			}
		)
	}
}

/// An adapter that either passes through operations to an inner reader
/// or yields non-BOM header bytes until exhaustion and chains reads to
/// an inner reader.
pub enum StripUtf8Bom<R: Read> {
	RemovedBom(R),
	RewindHeader {
		non_bom_header: ArrayVec<[u8; BOM_UTF8_SIZE]>,
		rest: Option<R>
	}
}

impl<R: Read> Read for StripUtf8Bom<R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::RemovedBom(read) => read.read(buf),
			Self::RewindHeader {
				non_bom_header,
				rest
			} => {
				let mut bytes_read = 0;
				let mut buf_iter = buf.iter_mut();

				while let (Some(buffer_byte), false) = (buf_iter.next(), non_bom_header.is_empty()) {
					// non_bom_header is at most BOM_UTF8_SIZE elements long, so this O(n) remove is fast
					// (draining does rotate_left once and requires a comparison, which is not better for small arrays)
					*buffer_byte = non_bom_header.remove(0);
					bytes_read += 1;
				}

				if non_bom_header.is_empty() {
					*self = Self::RemovedBom(rest.take().unwrap());
				}

				Ok(bytes_read)
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::io::Cursor;

	use pretty_assertions::assert_eq;

	use super::*;

	fn strip_utf8_bom_and_expect<T: Read, E: Error>(
		read: impl StripUtf8BomExt<T, E>,
		expected: &[u8]
	) {
		let mut stripped_read = read.strip_utf8_bom().expect("Unexpected I/O failure");

		let mut stripped_data = Vec::with_capacity(expected.len());
		stripped_read
			.read_to_end(&mut stripped_data)
			.expect("Unexpected I/O failure");

		assert_eq!(stripped_data, expected);
	}

	#[test]
	fn strip_utf8_bom_works() {
		strip_utf8_bom_and_expect(Cursor::new([0xEF, 0xBB, 0xBF, 0xAA, 0xBB]), &[0xAA, 0xBB]);
	}

	#[test]
	fn strip_utf8_with_small_read_buf_works() {
		let read = Cursor::new([0xAA, 0xBB]);
		let mut read = read.strip_utf8_bom().expect("Unexpected I/O failure");
		let mut buf = [0];

		read.read_exact(&mut buf).expect("Unexpected I/O failure");
		assert_eq!(buf[0], 0xAA);

		read.read_exact(&mut buf).expect("Unexpected I/O failure");
		assert_eq!(buf[0], 0xBB);

		assert!(
			matches!(read.read_exact(&mut buf), Err(err) if err.kind() == io::ErrorKind::UnexpectedEof)
		);
	}

	#[test]
	fn slice_strip_utf8_bom_specialization_works() {
		strip_utf8_bom_and_expect::<&[u8], _>(&[0xEF, 0xBB, 0xBF, 0xAA, 0xBB][..], &[0xAA, 0xBB]);
	}

	#[test]
	fn empty_data_is_not_stripped() {
		strip_utf8_bom_and_expect(Cursor::new([]), &[]);
	}

	#[test]
	fn small_data_is_not_stripped() {
		strip_utf8_bom_and_expect(Cursor::new([0xAA, 0xBB]), &[0xAA, 0xBB]);
	}

	#[test]
	fn big_data_without_utf8_bom_is_not_stripped() {
		strip_utf8_bom_and_expect::<&[u8], _>(
			&[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF][..],
			&[0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]
		);
	}
}
