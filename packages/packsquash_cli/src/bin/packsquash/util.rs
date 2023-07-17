use std::fmt::{Arguments, Display, Write as FmtWrite};
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

/// Newtype wrapper struct to prettify how serde errors with path information
/// are displayed.
#[repr(transparent)]
pub struct PrettyPathDeserializeErrorDisplay<E>(serde_path_to_error::Error<E>);

impl<E: Display> Display for PrettyPathDeserializeErrorDisplay<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let path = self.0.path();
		let inner = self.0.inner();

		// When the path is empty, its Display implementation writes a dot ("."),
		// which looks ugly and is somewhat confusing. Show the inner error
		// directly instead
		if path.iter().next().is_some() {
			write!(f, "{}: {}", path, inner)
		} else {
			inner.fmt(f)
		}
	}
}

impl<E> From<serde_path_to_error::Error<E>> for PrettyPathDeserializeErrorDisplay<E> {
	fn from(value: serde_path_to_error::Error<E>) -> Self {
		Self(value)
	}
}
