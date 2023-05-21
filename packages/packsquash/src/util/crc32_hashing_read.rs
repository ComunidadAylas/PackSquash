use std::{io, io::Read};

pub struct Crc32HashingRead<'hasher, R: Read> {
	inner: R,
	hasher: &'hasher mut crc32fast::Hasher
}

impl<'hasher, R: Read> Crc32HashingRead<'hasher, R> {
	pub fn new(inner: R, hasher: &'hasher mut crc32fast::Hasher) -> Self {
		Self { inner, hasher }
	}
}

impl<R: Read> Read for Crc32HashingRead<'_, R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.inner.read(buf).map(|bytes_read| {
			self.hasher.update(&buf[..bytes_read]);
			bytes_read
		})
	}
}
