use std::fmt::{Arguments, Write as FmtWrite};
use std::io::{IoSlice, Write as IoWrite};

/// Newtype adapter struct to bridge `io::Write` implementations
/// to the `fmt::Write` trait.
#[repr(transparent)]
pub struct IoWriteToFmtWriteAdapter<T: IoWrite>(pub T);

impl<T: IoWrite> IoWrite for IoWriteToFmtWriteAdapter<T> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.0.write(buf)
	}

	fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
		self.0.write_vectored(bufs)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.0.flush()
	}

	fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
		self.0.write_all(buf)
	}
}

impl<T: IoWrite> FmtWrite for IoWriteToFmtWriteAdapter<T> {
	fn write_str(&mut self, s: &str) -> std::fmt::Result {
		self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
	}

	fn write_fmt(&mut self, args: Arguments<'_>) -> std::fmt::Result {
		self.0.write_fmt(args).map_err(|_| std::fmt::Error)
	}
}
