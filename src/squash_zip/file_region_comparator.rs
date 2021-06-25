use std::{
	cmp,
	convert::TryInto,
	io::{ErrorKind, Read, Result, Seek, SeekFrom}
};

#[cfg(test)]
mod tests;

/// Utility struct that allows comparing two binary regions in a single readable and seekable byte
/// source for bitwise equality.
pub(super) struct FileRegionComparator<T: Read + Seek> {
	inner: T
}

impl<T: Read + Seek> FileRegionComparator<T> {
	/// Creates a new [`FileRegionComparator`] from the specified stream.
	pub fn new(inner: T) -> Self {
		Self { inner }
	}

	/// Compares two binary regions of the stream that begin at `first_offset` and `second_offset`,
	/// both of which are of the same size, `region_size`. If EOF is encountered while reading
	/// the first section, it will mark the end of both regions, and the partial equality result
	/// up to that point will be returned. However, if EOF is reached while reading the second
	/// section for comparison with the first, the function will return successfully with a
	/// inequality result (because both regions can't possibly contain the same number of bytes,
	/// under the assumption that EOF is a final condition).
	///
	/// This function makes no assumptions about the position of the stream. Similarly, the resulting
	/// seek position after this method returns is not defined.
	pub fn eq(&mut self, first_offset: u64, second_offset: u64, region_size: u32) -> Result<bool> {
		// 8 KiB buffers. Relatively big because collisions should be rare and reading
		// as much data as possile in a single iteration speeds up the process of computing
		// the expected positive result
		let mut buf1 = [0; 8192];
		let mut buf2 = [0; 8192];

		let mut bytes_read = 0;
		let mut regions_are_equal = true;

		while regions_are_equal && bytes_read < region_size {
			let maximum_bytes_to_read = cmp::min(
				buf1.len(),
				(region_size - bytes_read).try_into().unwrap_or(usize::MAX)
			);

			self.inner
				.seek(SeekFrom::Start(first_offset + bytes_read as u64))?;

			let bytes_read_iter = self.inner.read(&mut buf1[..maximum_bytes_to_read])?;

			if bytes_read_iter > 0 {
				self.inner
					.seek(SeekFrom::Start(second_offset + bytes_read as u64))?;

				if let Err(err) = self.inner.read_exact(&mut buf2[..bytes_read_iter]) {
					match err.kind() {
						ErrorKind::UnexpectedEof => return Ok(false),
						_ => return Err(err)
					};
				}

				regions_are_equal = buf1[..bytes_read_iter] == buf2[..bytes_read_iter];
				bytes_read += bytes_read_iter as u32;
			} else {
				// Assume EOF as final condition (i.e. the file will not be appended to by another thread)
				break;
			}
		}

		Ok(regions_are_equal)
	}
}
