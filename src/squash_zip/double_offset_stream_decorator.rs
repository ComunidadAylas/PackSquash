use std::{
	cell::Cell,
	io::{Read, Result, Seek, SeekFrom, Write}
};

#[cfg(test)]
mod tests;

/// Wraps a struct that implements [`Read`], [`Write`] and [`Seek`] in order to extend its functionality.
/// In particular, it maintains two independent offsets for read and write I/O operations, so that read
/// operations begin at and increment the read operations offset, while write operations begin at and
/// increment the write operations offset.
///
/// As this decorator isolates regions that are being read from those that are being written to
/// in the same stream, it implements [`Read`] and [`Write`] for immutable references to itself,
/// so callers can borrow it immutably more than once to use it as a [`Read`] and a [`Write`] at the
/// same time. This isolation "breaks" if the read and write regions end up overlapping, as in such
/// case the read operations may return bytes that have just been written in surprising (albeit safe)
/// ways. In any case, it is always guaranteed that writes do not impact the offset of reads, and
/// viceversa.
pub(super) struct DoubleOffsetStreamDecorator<T: Read + Write + Seek> {
	read_offset: Cell<u64>,
	write_offset: Cell<u64>,
	last_op_was_read: Cell<bool>,
	inner: Cell<Option<T>>
}

impl<T: Read + Write + Seek> DoubleOffsetStreamDecorator<T> {
	/// Instantiates a new double offset decorator for a stream. The specified `read_offset` is the absolute
	/// offset to which the first read operation after a write operation seeks to before reading. Conversely,
	/// the `write_offset` is the absolute offset to seek to before the first write operation. Both offsets
	/// will be updated with each type of operations accordingly.
	///
	/// # Assumptions
	/// For performance reasons, it is assumed that the current seek position (offset) of the stream matches
	/// `read_offset`. Breaking this assumption will lead to unexpected results.
	pub fn new(inner: T, read_offset: u64, write_offset: u64) -> Self {
		Self {
			read_offset: Cell::new(read_offset),
			write_offset: Cell::new(write_offset),
			last_op_was_read: Cell::new(true),
			inner: Cell::new(Some(inner))
		}
	}

	/// Returns the current write position in the stream as an absolute offset.
	pub fn write_position(&self) -> u64 {
		self.write_offset.get()
	}
}

impl<T: Read + Write + Seek> Read for &DoubleOffsetStreamDecorator<T> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		let mut inner = self.inner.take().unwrap();

		let result = || -> Result<usize> {
			if !self.last_op_was_read.replace(true) {
				let read_offset = self.read_offset.get();
				inner.seek(SeekFrom::Start(read_offset))?;
			}

			let bytes_read = inner.read(buf)?;
			self.read_offset
				.set(self.read_offset.get() + bytes_read as u64);

			Ok(bytes_read)
		}();

		self.inner.set(Some(inner));

		result
	}
}

impl<T: Read + Write + Seek> Write for &DoubleOffsetStreamDecorator<T> {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		let mut inner = self.inner.take().unwrap();

		let result = || -> Result<usize> {
			if self.last_op_was_read.replace(false) {
				let write_offset = self.write_offset.get();
				inner.seek(SeekFrom::Start(write_offset))?;
			}

			let bytes_written = inner.write(buf)?;
			self.write_offset
				.set(self.write_offset.get() + bytes_written as u64);

			Ok(bytes_written)
		}();

		self.inner.set(Some(inner));

		result
	}

	fn flush(&mut self) -> Result<()> {
		let mut inner = self.inner.take().unwrap();

		let result = inner.flush();

		self.inner.set(Some(inner));

		result
	}
}
