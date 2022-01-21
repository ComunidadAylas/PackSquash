use pretty_assertions::assert_eq;
use std::io::{Read, Seek, Write};

use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

use super::*;

#[test]
fn sync_io_works() {
	let mut spooled_file = BufferedAsyncSpooledTempFile::new(10);

	assert!(
		!is_rolled(&spooled_file),
		"Spooled temporary files start their lifecycle as not rolled"
	);

	assert_eq!(
		Seek::seek(&mut spooled_file, SeekFrom::Current(0))
			.expect("No error should occur during this I/O operation"),
		0,
		"Unexpected initial seek position"
	);

	Write::write_all(&mut spooled_file, &[0, 1, 2, 3, 4])
		.expect("No error should occur during this I/O operation");

	assert_eq!(
		Seek::seek(&mut spooled_file, SeekFrom::Current(-2))
			.expect("No error should occur during this I/O operation"),
		3,
		"Unexpected initial seek position"
	);

	let mut buf = [0; 2];
	Read::read_exact(&mut spooled_file, &mut buf)
		.expect("No error should occur during this I/O operation");

	assert_eq!(
		buf,
		[3, 4],
		"Unexpected bytes read back after first write and seek"
	);

	Seek::seek(&mut spooled_file, SeekFrom::Start(4))
		.expect("No error should occur during this I/O operation");

	// Write 1 byte more above threshold to test that it doesn't get lost
	Write::write_all(&mut spooled_file, &[5; 6 + 1]).expect("No I/O errors are assumed during tests");

	assert!(
		is_rolled(&spooled_file),
		"Expected file to roll to disk after reaching threshold"
	);

	Seek::seek(&mut spooled_file, SeekFrom::Start(0))
		.expect("No I/O errors are assumed during tests");

	let mut buf = [0; 11];
	Read::read_exact(&mut spooled_file, &mut buf).expect("No I/O errors are assumed during tests");

	assert_eq!(
		buf,
		[0, 1, 2, 3, 5, 5, 5, 5, 5, 5, 5],
		"Unexpected output after reading back rolled file contents"
	);

	Write::write_all(&mut spooled_file, &[7, 7, 7]).expect("No I/O errors are assumed during tests");

	Seek::seek(&mut spooled_file, SeekFrom::Current(-3))
		.expect("No I/O errors are assumed during tests");

	let mut buf = [0; 3];

	Read::read_exact(&mut spooled_file, &mut buf).expect("No I/O errors are assumed during tests");

	assert_eq!(
		buf,
		[7, 7, 7],
		"Unexpected bytes read back after rolled write"
	);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn async_io_works() {
	let mut spooled_file = BufferedAsyncSpooledTempFile::new(10);

	// Here we do more "light" tests because we already test the sync
	// primitives used by async code more throughly in other test

	AsyncWriteExt::write_all(&mut spooled_file, &[0, 1, 2, 3, 4])
		.await
		.expect("No error should occur during this I/O operation");

	AsyncSeekExt::seek(&mut spooled_file, SeekFrom::Current(-6))
		.await
		.expect_err("Seeking to a negative offset should be an error");

	AsyncSeekExt::seek(&mut spooled_file, SeekFrom::Start(0))
		.await
		.expect("No error should occur during this I/O operation");

	let mut buf = [0; 5];
	AsyncReadExt::read_exact(&mut spooled_file, &mut buf)
		.await
		.expect("No error should occur during this I/O operation");

	assert_eq!(
		buf,
		[0, 1, 2, 3, 4],
		"Unexpected bytes read back after first write and seek"
	);

	assert_eq!(
		AsyncSeekExt::stream_position(&mut spooled_file)
			.await
			.expect("No error should occur during this I/O operation"),
		5,
		"Unexpected stream position after first write and read"
	);

	AsyncWriteExt::write_all(&mut spooled_file, &[5, 6, 7, 8, 9])
		.await
		.expect("No error should occur during this I/O operation");

	assert!(
		is_rolled(&spooled_file),
		"The spooled file should roll after its threshold is reached"
	);

	AsyncSeekExt::seek(&mut spooled_file, SeekFrom::Current(-5))
		.await
		.expect("No error should occur during this I/O operation");

	let mut buf = [0; 5];
	AsyncReadExt::read_exact(&mut spooled_file, &mut buf)
		.await
		.expect("No error should occur during this I/O operation");

	assert_eq!(
		buf,
		[5, 6, 7, 8, 9],
		"Unexpected bytes read back after first write and seek"
	);
}

/// Returns whether a spooled file was written out to disk, which means that any I/O
/// operation would potentially interact with a disk. Due to the use of buffering,
/// however, not every I/O operation when rolled over will necessarily imply actual
/// I/O.
fn is_rolled(spooled_file: &BufferedAsyncSpooledTempFile) -> bool {
	matches!(spooled_file, BufferedAsyncSpooledTempFile::OnDisk(_, _))
}
