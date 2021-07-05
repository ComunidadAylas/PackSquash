use std::{
	cmp,
	fs::File,
	io::{self, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write},
	pin::Pin,
	task::{Context, Poll}
};

use tempfile::tempfile;
use tokio::{
	io::{AsyncRead, AsyncSeek, AsyncWrite, ReadBuf},
	task
};

#[cfg(test)]
mod tests;

/// A buffered, asynchronous and spooled temporary file, which will keep its contents
/// in memory until a configurable size threshold is reached, at which point it is
/// written to disk, and all I/O operations continue there. When rolled over to the
/// disk, in order to keep I/O operations as fast as possible, especially smaller and
/// more frequent ones, reads and writes are buffered.
///
/// [`Read`], [`Write`] and [`Seek`] traits are implemented via synchronous I/O,
/// delegating to the underlying data store in use. However, to help using this struct
/// inside asynchronous tasks, [`AsyncRead`], [`AsyncWrite`] and [`AsyncSeek`]
/// implementations are provided. These implementations will always yield results
/// inmediately, but, to avoid starving other asynchronous tasks, if blocking would
/// occur, the current task is blocked in place, so it momentarily becomes a blocking
/// task in the Tokio runtime. Due to how Tokio is designed, these asynchronous traits
/// will panic when used in the context of a current thread runtime, but it is fine to
/// use them in multithreaded runtimes or outside of any runtime.
// FIXME: it'd be nice to make this struct async-first. However, async read, write and
// seek traits are pretty low level, and implementing those on top of similarly low
// level operations of other data sources (because Tokio ext traits which provide
// high-level features like "seek" or "read" return opaque futures, and boxing them is
// not a good way to go) is painful. This would also require rewriting the auxiliary
// structs that just accept the synchronous traits, taking into account that Zopfli
// can't accept async I/O. For now, this struct is an acceptable compromise which allows
// as much async-ready code as is reasonable, so at least it'll make migration easier
pub(super) enum BufferedAsyncSpooledTempFile {
	InMemory(usize, Cursor<Vec<u8>>),
	OnDisk(BufReader<File>, BufWriter<File>)
}

impl BufferedAsyncSpooledTempFile {
	/// Creates a new [`BufferedAsyncSpooledTempFile`] with the specified size threshold.
	pub fn new(size_threshold: usize) -> Self {
		Self::with_capacity(0, size_threshold)
	}

	/// Creates a new [`BufferedAsyncSpooledTempFile`] with the specified in-memory buffer
	/// capacity and size threshold. The in-memory buffer capacity will never exceed the
	/// size threshold.
	pub fn with_capacity(capacity: usize, size_threshold: usize) -> Self {
		Self::InMemory(
			size_threshold,
			Cursor::new(Vec::with_capacity(cmp::min(capacity, size_threshold)))
		)
	}
}

impl Read for BufferedAsyncSpooledTempFile {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::InMemory(_, cursor) => cursor.read(buf),
			Self::OnDisk(file_reader, _) => file_reader.read(buf)
		}
	}
}

impl Write for BufferedAsyncSpooledTempFile {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		match self {
			Self::InMemory(size_threshold, cursor) => {
				let bytes_written = cursor.write(
					&buf[..cmp::min(buf.len(), *size_threshold - cursor.position() as usize)]
				)?;
				let new_cursor_position = cursor.position();

				if new_cursor_position >= *size_threshold as u64 {
					let mut rolled_file = tempfile()?;

					rolled_file.write_all(cursor.get_ref())?;
					rolled_file.seek(SeekFrom::Start(new_cursor_position))?;

					*self = Self::OnDisk(
						BufReader::new(rolled_file.try_clone()?),
						BufWriter::new(rolled_file)
					);
				}

				Ok(bytes_written)
			}
			Self::OnDisk(_, file_writer) => file_writer.write(buf)
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		if let Self::OnDisk(_, file_writer) = self {
			file_writer.flush()?;
		}

		Ok(())
	}
}

impl Seek for BufferedAsyncSpooledTempFile {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		match self {
			Self::InMemory(_, cursor) => cursor.seek(pos),
			Self::OnDisk(file_reader, file_writer) => {
				// Flush the write buffer (i.e. empty it)
				file_writer.flush()?;

				// This will seek the underlying file to the specified position and
				// flush read buffers when needed. So no buffer will read or write
				// stall data from the previous position, and because both wrappers
				// refer to the same underlying file, we only need to do the seek once
				file_reader.seek(pos)
			}
		}
	}
}

impl AsyncRead for BufferedAsyncSpooledTempFile {
	fn poll_read(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
		buf: &mut ReadBuf<'_>
	) -> Poll<io::Result<()>> {
		match Pin::into_inner(self) {
			Self::InMemory(_, cursor) => Pin::new(cursor).poll_read(cx, buf),
			Self::OnDisk(file_reader, _) => {
				let read_buf = buf.initialize_unfilled();
				let remaining_bytes = file_reader.capacity() - file_reader.buffer().len();

				// Avoid blocking if the bytes already are in a memory buffer
				let bytes_read;
				if read_buf.len() < remaining_bytes {
					bytes_read = file_reader.read(read_buf)?;
				} else {
					bytes_read = task::block_in_place(|| file_reader.read(read_buf))?;
				}

				buf.advance(bytes_read);

				Poll::Ready(Ok(()))
			}
		}
	}
}

impl AsyncWrite for BufferedAsyncSpooledTempFile {
	fn poll_write(
		self: Pin<&mut Self>,
		_: &mut Context<'_>,
		buf: &[u8]
	) -> Poll<Result<usize, io::Error>> {
		let this = Pin::into_inner(self);
		match this {
			Self::InMemory(size_threshold, cursor) => {
				if cursor.position() as usize + buf.len() >= *size_threshold {
					// Write is very likely to roll to disk (and, in fact, it will,
					// because Write::write implementation for Cursor<Vec<u8>> always
					// writes the entire buf contents). Block in place
					Poll::Ready(task::block_in_place(|| this.write(buf)))
				} else {
					Poll::Ready(this.write(buf))
				}
			}
			Self::OnDisk(_, file_writer) => {
				let remaining_bytes = file_writer.capacity() - file_writer.buffer().len();

				// Avoid blocking if the bytes will go to a memory buffer
				if buf.len() < remaining_bytes {
					Poll::Ready(file_writer.write(buf))
				} else {
					Poll::Ready(task::block_in_place(|| file_writer.write(buf)))
				}
			}
		}
	}

	fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
		match Pin::into_inner(self) {
			Self::InMemory(_, _) => Poll::Ready(Ok(())),
			Self::OnDisk(_, file_writer) => Poll::Ready(task::block_in_place(|| file_writer.flush()))
		}
	}

	fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
		self.poll_flush(cx)
	}
}

impl AsyncSeek for BufferedAsyncSpooledTempFile {
	fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
		let this = Pin::into_inner(self);
		match this {
			Self::InMemory(_, cursor) => cursor.seek(position).map(|_| ()),
			Self::OnDisk(_, _) => task::block_in_place(|| this.seek(position)).map(|_| ())
		}
	}

	fn poll_complete(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<u64>> {
		let this = Pin::into_inner(self);
		match this {
			Self::InMemory(_, cursor) => Poll::Ready(Ok(cursor.position())),
			Self::OnDisk(_, _) => {
				Poll::Ready(task::block_in_place(|| this.seek(SeekFrom::Current(0))))
			}
		}
	}
}
