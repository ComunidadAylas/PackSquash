use std::io::{self, IoSlice};

use tokio::io::{AsyncWrite, AsyncWriteExt};

/// Extension trait for efficiently writing several entire buffers of data
/// to an asynchronous writer.
pub trait AsyncWriteAllVectoredExt {
	/// Like [`write_all`](`AsyncWriteExt::write_all`), except that it writes from a slice of buffers.
	async fn write_all_vectored(&mut self, bufs: &mut [IoSlice<'_>]) -> io::Result<()>;
}

impl<T: AsyncWrite + Unpin> AsyncWriteAllVectoredExt for T {
	async fn write_all_vectored(&mut self, mut buffers: &mut [IoSlice<'_>]) -> io::Result<()> {
		// Filter out initial empty buffers, if any
		IoSlice::advance_slices(&mut buffers, 0);

		while !buffers.is_empty() {
			match self.write_vectored(buffers).await {
				Ok(0) => return Err(io::ErrorKind::WriteZero.into()),
				Ok(n) => IoSlice::advance_slices(&mut buffers, n),
				Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
				Err(e) => return Err(e)
			}
		}

		Ok(())
	}
}
