use pretty_assertions::assert_eq;
use std::io::{Read, Seek, Write};

use super::*;

#[test]
fn works() {
	let scratch_files_budget = ScratchFilesBudget::new(1);
	let mut scratch_file = ScratchFile::new(&scratch_files_budget).expect("Unexpected I/O error");

	assert!(
		!is_rolled(&scratch_file),
		"Scratch files with enough initial budget start their lifecycle as not rolled"
	);

	assert_eq!(
		scratch_file
			.seek(SeekFrom::Current(0))
			.expect("No error should occur during this I/O operation"),
		0,
		"Unexpected initial seek position"
	);

	scratch_file
		.write_all(&[0, 1, 2, 3, 4])
		.expect("No error should occur during this I/O operation");

	assert_eq!(
		scratch_file
			.seek(SeekFrom::Current(-2))
			.expect("No error should occur during this I/O operation"),
		3,
		"Unexpected initial seek position"
	);

	let mut buf = [0; 2];
	scratch_file
		.read_exact(&mut buf)
		.expect("No error should occur during this I/O operation");

	assert_eq!(
		buf,
		[3, 4],
		"Unexpected bytes read back after first write and seek"
	);

	scratch_file
		.seek(SeekFrom::Start(4))
		.expect("No error should occur during this I/O operation");

	scratch_file
		.write_all(&[5; 6 + 1])
		.expect("No I/O errors are assumed during tests");

	// After all these writes, most sane reallocation strategies would have
	// increased vector capacity, and thus rolled over to disk
	assert!(
		is_rolled(&scratch_file),
		"Scratch files buffer budget should be honored"
	);

	scratch_file
		.seek(SeekFrom::Start(0))
		.expect("No I/O errors are assumed during tests");

	let mut buf = [0; 11];
	scratch_file
		.read_exact(&mut buf)
		.expect("No I/O errors are assumed during tests");

	assert_eq!(
		buf,
		[0, 1, 2, 3, 5, 5, 5, 5, 5, 5, 5],
		"Unexpected output after reading back rolled file contents"
	);

	scratch_file
		.write_all(&[7, 7, 7])
		.expect("No I/O errors are assumed during tests");

	scratch_file
		.seek(SeekFrom::Current(-3))
		.expect("No I/O errors are assumed during tests");

	let mut buf = [0; 3];

	scratch_file
		.read_exact(&mut buf)
		.expect("No I/O errors are assumed during tests");

	assert_eq!(
		buf,
		[7, 7, 7],
		"Unexpected bytes read back after rolled write"
	);
}

/// Returns whether a scratch file was written out to disk, which means that any I/O
/// operation would potentially interact with a disk. Due to the use of buffering,
/// however, not every I/O operation when rolled over will necessarily imply actual
/// I/O.
fn is_rolled(scratch_file: &ScratchFile) -> bool {
	matches!(scratch_file.inner, ScratchFileInner::OnDisk(_, _))
}
