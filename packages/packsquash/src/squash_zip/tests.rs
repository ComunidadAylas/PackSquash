use std::{
	env,
	fs::File,
	io::Cursor,
	path::{Path, PathBuf}
};

use tempfile::{Builder, TempPath};

use pretty_assertions::assert_eq;

use super::*;

static INSTANTIATION_FAILURE: &str = "No error should happen while creating the SquashZip instance";
static RELATIVE_PATH_INSTANTIATION_FAILURE: &str = "Relative path creation was not expected to fail";
static UNEXPECTED_OPERATION_FAILURE: &str = "This SquashZip operation should not fail";
static UNEXPECTED_IO_FAILURE: &str = "I/O operations are assumed not to fail";

/// The budget for scratch files that SquashZip will use internally.
static SCRATCH_FILES_BUDGET: ScratchFilesBudget = ScratchFilesBudget::new(64 * 1024 * 1024);

/// The uncompressed size of the files that will be added to ZIP files during tests.
const FILE_SIZE: usize = 2048;

/// Helper struct that wraps either a `PathBuf` or `TempPath`, providing an `AsRef<Path>`
/// implementation.
enum NamedTempFilePath {
	PathBuf(PathBuf),
	TempPath(TempPath)
}

impl AsRef<Path> for NamedTempFilePath {
	fn as_ref(&self) -> &Path {
		match self {
			Self::PathBuf(path) => path,
			Self::TempPath(path) => path
		}
	}
}

/// Returns a [`SquashZipSettings`] struct suitable for tests. Deduplication is enabled or not
/// according to the value of the `enable_deduplication` parameter.
fn squash_zip_settings(enable_deduplication: bool) -> SquashZipSettings {
	SquashZipSettings {
		zopfli_iterations: 20,
		store_squash_time: true,
		enable_obfuscation: false,
		enable_deduplication,
		enable_size_increasing_obfuscation: false,
		percentage_of_records_tuned_for_obfuscation_discretion: 0.try_into().unwrap(),
		workaround_old_java_obfuscation_quirks: false
	}
}

/// Creates a temporary output file for testing, and returns its path. Depending on the value of
/// the `WRITE_SQUASHZIP_TEST_RESULTS` environment variable, the named of the created file may
/// be printed out.
fn create_temporary_output_file(test_name: &'static str) -> (File, NamedTempFilePath) {
	let keep = env::var("WRITE_SQUASHZIP_TEST_RESULTS").unwrap_or_else(|_| String::from("0")) == "1";

	let temp_file = Builder::new()
		.prefix("squashzip-test")
		.suffix(".zip")
		.tempfile()
		.expect("Temporary file creation is assumed not to fail for tests");

	if keep {
		let (file, path) = temp_file
			.keep()
			.expect("Keeping the temporary file is assumed not to fail for tests");

		eprintln!(
			"Creating temporary output file {:?} for test {}",
			path, test_name
		);

		(file, NamedTempFilePath::PathBuf(path))
	} else {
		let (file, path) = temp_file.into_parts();

		(file, NamedTempFilePath::TempPath(path))
	}
}

/// Generic helper function that adds the specified number of files to a new temporary
/// output ZIP file, finishes the ZIP file, and then reads it back, asserting that
/// PackSquash is able to read back relevant data from the files it generates.
#[allow(clippy::too_many_arguments)] // Alternatives are not really much more readable
fn add_files_finish_and_read_back_test(
	squash_zip: Option<SquashZip<'_, '_, File>>,
	file_count: u8,
	enable_deduplication: bool,
	file_name_number: impl Fn(u8) -> u8,
	file_byte: impl Fn(u8) -> u8,
	file_size: usize,
	skip_compression: impl Fn(u8) -> bool,
	test_name: &'static str,
	files_reused_from_previous_run: usize
) -> NamedTempFilePath {
	let squash_zip_settings = squash_zip_settings(enable_deduplication);
	let squash_zip = squash_zip.unwrap_or_else(|| {
		SquashZip::new(None, &squash_zip_settings, &SCRATCH_FILES_BUDGET)
			.expect(INSTANTIATION_FAILURE)
	});

	let (file, file_path) = create_temporary_output_file(test_name);

	for i in 0..file_count {
		squash_zip
			.add_file(
				&RelativePath::new(
					Path::new("./gimme/gimme"),
					Path::new(&format!(
						"./gimme/gimme/virtual/visions{}.bin",
						file_name_number(i)
					))
				)
				.expect(RELATIVE_PATH_INSTANTIATION_FAILURE),
				Cursor::new(vec![file_byte(i); file_size]),
				skip_compression(i)
			)
			.expect(UNEXPECTED_OPERATION_FAILURE);
	}

	squash_zip
		.finish(|| Ok(file))
		.expect(UNEXPECTED_OPERATION_FAILURE);

	let squash_zip = SquashZip::new(
		Some(File::open(&file_path).expect(UNEXPECTED_IO_FAILURE)),
		&squash_zip_settings,
		&SCRATCH_FILES_BUDGET
	)
	.expect(INSTANTIATION_FAILURE);

	assert_eq!(
		squash_zip.previous_zip_contents.len(),
		file_count as usize + files_reused_from_previous_run,
		"Unexpected number of ZIP files read back"
	);

	file_path
}

#[test]
fn add_single_finish_and_read_back_works() {
	add_files_finish_and_read_back_test(
		None,
		1,
		false,
		|i| i,
		|_| b'a',
		FILE_SIZE,
		|_| false,
		"add_single_finish_and_read_back_works",
		0
	);
}

#[test]
fn add_empty_finish_and_read_back_works() {
	add_files_finish_and_read_back_test(
		None,
		1,
		false,
		|i| i,
		|_| b'a',
		0,
		|_| false,
		"add_empty_finish_and_read_back_works",
		0
	);
}

#[test]
fn add_tiny_finish_and_read_back_works() {
	add_files_finish_and_read_back_test(
		None,
		1,
		false,
		|i| i,
		|_| b'a',
		1,
		|_| false,
		"add_tiny_finish_and_read_back_works",
		0
	);
}

#[test]
fn add_several_finish_and_read_back_works() {
	add_files_finish_and_read_back_test(
		None,
		3,
		false,
		|i| i,
		|_| b'a',
		FILE_SIZE,
		|_| false,
		"add_several_finish_and_read_back_works",
		0
	);
}

#[test]
fn add_several_finish_and_read_back_with_deduplication_works() {
	add_files_finish_and_read_back_test(
		None,
		3,
		true,
		|i| i,
		|_| b'a',
		FILE_SIZE,
		|_| true,
		"add_several_finish_and_read_back_with_deduplication_works",
		0
	);
}

#[test]
fn add_several_compressed_finish_and_read_back_with_deduplication_works() {
	let bigger_file = add_files_finish_and_read_back_test(
		None,
		3,
		true,
		|i| i,
		|_| b'a',
		FILE_SIZE,
		|i| i < 2,
		"add_several_compressed_finish_and_read_back_with_deduplication_works (bigger file)",
		0
	);

	let smaller_file = add_files_finish_and_read_back_test(
		None,
		3,
		true,
		|i| i,
		|_| b'a',
		FILE_SIZE,
		|_| false,
		"add_several_compressed_finish_and_read_back_with_deduplication_works (smaller file)",
		0
	);

	let bigger_file_size = File::open(bigger_file)
		.expect(UNEXPECTED_IO_FAILURE)
		.metadata()
		.expect(UNEXPECTED_IO_FAILURE)
		.len();

	let smaller_file_size = File::open(smaller_file)
		.expect(UNEXPECTED_IO_FAILURE)
		.metadata()
		.expect(UNEXPECTED_IO_FAILURE)
		.len();

	assert!(
		bigger_file_size > smaller_file_size,
		"Deduplication didn't have the expected effect on file size"
	);
}

#[test]
fn add_several_and_read_back_some_duplicates_works() {
	add_files_finish_and_read_back_test(
		None,
		3,
		true,
		|i| i,
		|i| b'a' + i % 2,
		FILE_SIZE,
		|_| true,
		"add_several_and_read_back_some_duplicates_works",
		0
	);
}

#[test]
fn add_several_finish_then_reuse_and_add_works() {
	// Two different files, visions0.bin and visions1.bin, with bytes 'a' and 'b' repeated
	let zip_path = add_files_finish_and_read_back_test(
		None,
		2,
		true,
		|i| i,
		|i| b'a' + i % 2,
		FILE_SIZE,
		|_| true,
		"add_several_finish_then_reuse_and_add_works (first part)",
		0
	);

	let squash_zip_settings = squash_zip_settings(true);
	let squash_zip = SquashZip::new(
		Some(File::open(&zip_path).expect(UNEXPECTED_IO_FAILURE)),
		&squash_zip_settings,
		&SCRATCH_FILES_BUDGET
	)
	.expect(INSTANTIATION_FAILURE);

	// Add the previous visions0.bin file
	squash_zip
		.add_previous_file(&RelativePath::from_inner("virtual/visions0.bin"))
		.expect(UNEXPECTED_OPERATION_FAILURE);

	// Add visions2.bin and visions3.bin. The resulting ZIP file should contain
	// these files:
	// - visions0.bin ('a' bytes), sharing local file header with visions2.bin ('a' bytes)
	// - visions3.bin ('c' bytes)
	// visions1.bin must not appear.
	let zip_path = add_files_finish_and_read_back_test(
		Some(squash_zip),
		2,
		true,
		|i| i + 2,
		|i| if i == 0 { b'a' } else { b'c' },
		FILE_SIZE,
		|_| true,
		"add_several_finish_then_reuse_and_add_works (second part)",
		1
	);

	let squash_zip = SquashZip::new(
		Some(File::open(&zip_path).expect(UNEXPECTED_IO_FAILURE)),
		&squash_zip_settings,
		&SCRATCH_FILES_BUDGET
	)
	.expect(INSTANTIATION_FAILURE);

	for file in [
		"virtual/visions0.bin",
		"virtual/visions2.bin",
		"virtual/visions3.bin"
	] {
		squash_zip
			.file_process_time(&RelativePath::from_inner(file))
			.unwrap_or_else(|| panic!("Expected file not read back from output ZIP: {}", file));
	}
}

#[test]
fn several_files_with_same_path_are_handled_properly() {
	let squash_zip_settings = squash_zip_settings(false);
	let squash_zip = SquashZip::new(None::<File>, &squash_zip_settings, &SCRATCH_FILES_BUDGET)
		.expect(INSTANTIATION_FAILURE);

	let file_path = &RelativePath::from_inner("virtual/visions0.bin");
	let add_file = || squash_zip.add_file(file_path, Cursor::new([0]), true);

	add_file().expect(UNEXPECTED_OPERATION_FAILURE);
	add_file().expect_err(UNEXPECTED_OPERATION_FAILURE);
}
