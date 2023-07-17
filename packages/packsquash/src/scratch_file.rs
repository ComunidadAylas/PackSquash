use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use tempfile::tempfile;

#[cfg(test)]
mod tests;

pub struct ScratchFilesBudget {
	remaining_memory_budget: AtomicUsize
}

impl ScratchFilesBudget {
	pub fn new(memory_budget: usize) -> Self {
		Self {
			remaining_memory_budget: AtomicUsize::new(memory_budget)
		}
	}

	pub fn remaining_memory_budget(&self) -> usize {
		self.remaining_memory_budget.load(Ordering::Acquire)
	}

	pub fn spend(&self, bytes: usize) -> bool {
		self.remaining_memory_budget
			.fetch_update(Ordering::AcqRel, Ordering::Acquire, |budget| {
				budget.checked_sub(bytes)
			})
			.is_ok()
	}
}

pub struct ScratchFile<'budget> {
	remaining_memory_budget: &'budget AtomicUsize,
	inner: ScratchFileInner
}

impl<'budget> ScratchFile<'budget> {
	pub fn new(budget: &'budget ScratchFilesBudget) -> io::Result<Self> {
		Self::with_capacity(budget, 0)
	}

	pub fn with_capacity(budget: &'budget ScratchFilesBudget, capacity: usize) -> io::Result<Self> {
		let remaining_memory_budget = &budget.remaining_memory_budget;

		let mut buf = vec![];
		let have_enough_budget_for_buf = if capacity == 0 {
			// Fast path to avoid atomic operations
			true
		} else {
			remaining_memory_budget
				.fetch_update(Ordering::AcqRel, Ordering::Acquire, |budget| {
					// Avoid allocating memory if it's known to be too much beforehand
					if budget < capacity {
						return None;
					}

					buf = Vec::with_capacity(capacity);
					budget.checked_sub(buf.capacity())
				})
				.is_ok()
		};

		Ok(Self {
			remaining_memory_budget,
			inner: if have_enough_budget_for_buf {
				ScratchFileInner::InMemory(Cursor::new(buf))
			} else {
				let rolled_file = tempfile()?;
				ScratchFileInner::OnDisk(
					BufReader::new(rolled_file.try_clone()?),
					BufWriter::new(rolled_file)
				)
			}
		})
	}

	// TODO document that the purpose of this method is to leverage io::copy specialization for files on Linux
	pub fn copy_to(&mut self, mut writer: impl Write) -> io::Result<u64> {
		match &mut self.inner {
			ScratchFileInner::InMemory(cursor) => io::copy(cursor, &mut writer),
			ScratchFileInner::OnDisk(file_reader, _) => io::copy(file_reader, &mut writer)
		}
	}
}

enum ScratchFileInner {
	InMemory(Cursor<Vec<u8>>),
	OnDisk(BufReader<File>, BufWriter<File>)
}

impl Read for ScratchFile<'_> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match &mut self.inner {
			ScratchFileInner::InMemory(cursor) => cursor.read(buf),
			ScratchFileInner::OnDisk(file, _) => file.read(buf)
		}
	}
}

impl Write for ScratchFile<'_> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		match &mut self.inner {
			ScratchFileInner::InMemory(cursor) => {
				let buffer_position = cursor.position() as usize;
				let buffer_size = cursor.get_ref().len();
				let buffer_capacity = cursor.get_ref().capacity();

				let needed_capacity = buf.len().saturating_sub(buffer_size - buffer_position);

				cursor.get_mut().reserve(needed_capacity);

				let new_buffer_capacity = cursor.get_ref().capacity();

				let remaining_byte_budget = self
					.remaining_memory_budget
					.fetch_update(Ordering::AcqRel, Ordering::Acquire, |budget| {
						Some(budget.saturating_sub(new_buffer_capacity - buffer_capacity))
					})
					.unwrap();

				// We can only know the new buffer capacity after we reserve it,
				// so we use it anyway
				let bytes_written = cursor.write(buf)?;

				// If we run out of budget to store this scratch file, roll it over
				// to disk. While it is being rolled over and until the buffer
				// memory is freed, we're over budget for a small time window
				// (hysteresis). This should not matter in practice
				if remaining_byte_budget == 0 {
					let mut rolled_file = tempfile()?;
					let rolled_file_clone = rolled_file.try_clone()?;

					let buffer_position = cursor.position();
					let buffer = cursor.get_mut();

					rolled_file.write_all(buffer)?;
					rolled_file.seek(SeekFrom::Start(buffer_position))?;

					// Memory pressure is expected to be high here, so free the
					// buffer allocated memory as much as possible right now
					buffer.clear();
					buffer.shrink_to_fit();

					self.inner = ScratchFileInner::OnDisk(
						BufReader::new(rolled_file_clone),
						BufWriter::new(rolled_file)
					);

					// We manually freed the bulk of the memory consumed by the
					// in-memory buffer, so it's OK for other scratch files to
					// use its capacity
					self.remaining_memory_budget
						.fetch_add(new_buffer_capacity, Ordering::AcqRel);
				}

				Ok(bytes_written)
			}
			ScratchFileInner::OnDisk(_, file_writer) => file_writer.write(buf)
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		// Flushing an in-memory buffer is a no-op
		if let ScratchFileInner::OnDisk(_, file_writer) = &mut self.inner {
			file_writer.flush()?;
		}

		Ok(())
	}
}

impl Seek for ScratchFile<'_> {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		match &mut self.inner {
			ScratchFileInner::InMemory(cursor) => cursor.seek(pos),
			ScratchFileInner::OnDisk(file_reader, file_writer) => {
				// Empty the writer internal buffer to make sure it doesn't hold
				// stale data from the previous position
				file_writer.flush()?;

				// This will seek the underlying file to the specified position and
				// flush the reader (i.e., empty its internal buffer) when needed.
				// Both reader and writer are a view to the same file
				file_reader.seek(pos)
			}
		}
	}
}

impl BufRead for ScratchFile<'_> {
	fn fill_buf(&mut self) -> io::Result<&[u8]> {
		match &mut self.inner {
			ScratchFileInner::InMemory(cursor) => cursor.fill_buf(),
			ScratchFileInner::OnDisk(file_reader, _) => file_reader.fill_buf()
		}
	}

	fn consume(&mut self, amt: usize) {
		match &mut self.inner {
			ScratchFileInner::InMemory(cursor) => cursor.consume(amt),
			ScratchFileInner::OnDisk(file_reader, _) => file_reader.consume(amt)
		}
	}
}

impl Drop for ScratchFile<'_> {
	fn drop(&mut self) {
		if let ScratchFileInner::InMemory(cursor) = &mut self.inner {
			// There's no particular reason to believe that there is high memory pressure,
			// so err on the performance side and increment the budget slightly before the
			// actual buffer heap allocation is freed
			self.remaining_memory_budget
				.fetch_add(cursor.get_ref().capacity(), Ordering::AcqRel);
		}
	}
}
