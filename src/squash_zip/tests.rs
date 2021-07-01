use std::{env, path::PathBuf};

use tempfile::Builder;

use super::*;

static INSTANTIATION_FAILURE: &str = "No error should happen while creating the SquashZip instance";
static RELATIVE_PATH_INSTANTIATION_FAILURE: &str = "Relative path creation was not expected to fail";
static UNEXPECTED_OPERATION_FAILURE: &str = "This SquashZip operation should not fail";
static UNEXPECTED_IO_FAILURE: &str = "I/O operations are assumed not to fail";

const SPOOL_BUFFER_SIZE: usize = 64 * 1024 * 1024;

/// Creates a temporary output file for testing, and returns its path. Depending on the value of
/// the `WRITE_SQUASHZIP_TEST_RESULTS` environment variable, the named of the created file may
/// be printed out.
///
/// Note that, due to API limitations in the tempfile crate, all files created for tests are not
/// deleted automatically when the tests end.
fn create_temporary_output_file(test_name: &'static str) -> PathBuf {
	let file_and_path = Builder::new()
		.prefix("squashzip-test")
		.suffix(".zip")
		.tempfile()
		.expect("Temporary file creation is assumed not to fail for tests")
		.keep()
		.expect("Keeping the temporary file is assumed not to fail for tests");

	let file_path = file_and_path.1;

	if env::var("WRITE_SQUASHZIP_TEST_RESULTS").unwrap_or(String::from("0")) == "1" {
		eprintln!(
			"Creating temporary output file {:?} for test {}",
			file_path, test_name
		);
	}

	file_path
}

/// Generic helper function that adds the specified number of files to a new temporary
/// output ZIP file, finishes the ZIP file, and then reads it back, asserting that
/// PackSquash is able to read back relevant data from the files it generates.
async fn add_files_finish_and_read_back_test(
	squash_zip: Option<SquashZip<File>>,
	file_count: u8,
	enable_deduplication: bool,
	file_name_number: impl Fn(u8) -> u8,
	file_byte: impl Fn(u8) -> u8,
	skip_compression: impl Fn(u8) -> bool,
	test_name: &'static str
) -> PathBuf {
	let mut squash_zip = squash_zip.unwrap_or(
		SquashZip::new(
			None,
			SquashZipSettings {
				store_squash_time: true,
				enable_obfuscation: false,
				enable_deduplication,
				enable_size_increasing_obfuscation: false,
				percentage_of_structures_tuned_for_obfuscation_discretion: 0.try_into().unwrap(),
				spool_buffer_size: SPOOL_BUFFER_SIZE
			}
		)
		.await
		.expect(INSTANTIATION_FAILURE)
	);

	let file_path = create_temporary_output_file(test_name);

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
				&mut tokio_stream::iter(std::iter::repeat(&[file_byte(i)][..]).take(2048)),
				skip_compression(i)
			)
			.await
			.expect(UNEXPECTED_OPERATION_FAILURE);
	}

	squash_zip
		.finish(&file_path)
		.await
		.expect(UNEXPECTED_OPERATION_FAILURE);

	let squash_zip = SquashZip::new(
		Some(File::open(&file_path).await.expect(UNEXPECTED_IO_FAILURE)),
		SquashZipSettings {
			store_squash_time: true,
			enable_obfuscation: false,
			enable_deduplication,
			enable_size_increasing_obfuscation: false,
			percentage_of_structures_tuned_for_obfuscation_discretion: 0.try_into().unwrap(),
			spool_buffer_size: SPOOL_BUFFER_SIZE
		}
	)
	.await
	.expect(INSTANTIATION_FAILURE);

	assert!(
		squash_zip.previous_file_count() >= file_count as usize,
		"Not all previous ZIP contents were read back"
	);

	file_path
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn add_single_finish_and_read_back_works() {
	add_files_finish_and_read_back_test(
		None,
		1,
		false,
		|i| i,
		|_| b'a',
		|_| false,
		"add_single_finish_and_read_back_works"
	)
	.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn add_several_finish_and_read_back_works() {
	add_files_finish_and_read_back_test(
		None,
		3,
		false,
		|i| i,
		|_| b'a',
		|_| false,
		"add_several_finish_and_read_back_works"
	)
	.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn add_several_finish_and_read_back_with_deduplication_works() {
	add_files_finish_and_read_back_test(
		None,
		3,
		true,
		|i| i,
		|_| b'a',
		|_| true,
		"add_several_finish_and_read_back_with_deduplication_works"
	)
	.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn add_several_compressed_finish_and_read_back_with_deduplication_works() {
	let bigger_file = add_files_finish_and_read_back_test(
		None,
		3,
		true,
		|i| i,
		|_| b'a',
		|i| i < 2,
		"add_several_compressed_finish_and_read_back_with_deduplication_works (bigger file)"
	)
	.await;

	let smaller_file = add_files_finish_and_read_back_test(
		None,
		3,
		true,
		|i| i,
		|_| b'a',
		|_| false,
		"add_several_compressed_finish_and_read_back_with_deduplication_works (smaller file)"
	)
	.await;

	let bigger_file_size = File::open(bigger_file)
		.await
		.expect(UNEXPECTED_IO_FAILURE)
		.metadata()
		.await
		.expect(UNEXPECTED_IO_FAILURE)
		.len();

	let smaller_file_size = File::open(smaller_file)
		.await
		.expect(UNEXPECTED_IO_FAILURE)
		.metadata()
		.await
		.expect(UNEXPECTED_IO_FAILURE)
		.len();

	assert!(
		bigger_file_size > smaller_file_size,
		"Deduplication didn't have the expected effect on file size"
	);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn add_several_and_read_back_some_duplicates_works() {
	add_files_finish_and_read_back_test(
		None,
		3,
		true,
		|i| i,
		|i| b'a' + i % 2,
		|_| true,
		"add_several_and_read_back_some_duplicates_works"
	)
	.await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn add_several_finish_then_reuse_and_add_works() {
	// Two different files, visions0.bin and visions1.bin, with bytes 'a' and 'b' repeated
	let zip_path = add_files_finish_and_read_back_test(
		None,
		2,
		true,
		|i| i,
		|i| b'a' + i % 2,
		|_| true,
		"add_several_finish_then_reuse_and_add_works (first part)"
	)
	.await;

	let mut squash_zip = SquashZip::new(
		Some(File::open(&zip_path).await.expect(UNEXPECTED_IO_FAILURE)),
		SquashZipSettings {
			store_squash_time: true,
			enable_obfuscation: false,
			enable_deduplication: true,
			enable_size_increasing_obfuscation: false,
			percentage_of_structures_tuned_for_obfuscation_discretion: 0.try_into().unwrap(),
			spool_buffer_size: SPOOL_BUFFER_SIZE
		}
	)
	.await
	.expect(INSTANTIATION_FAILURE);

	// Add the previous visions0.bin file
	squash_zip
		.add_previous_file(&RelativePath::from_inner("virtual/visions0.bin"))
		.await
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
		|_| true,
		"add_several_finish_then_reuse_and_add_works (second part)"
	)
	.await;

	let squash_zip = SquashZip::new(
		Some(File::open(&zip_path).await.expect(UNEXPECTED_IO_FAILURE)),
		SquashZipSettings {
			store_squash_time: true,
			enable_obfuscation: false,
			enable_deduplication: true,
			enable_size_increasing_obfuscation: false,
			percentage_of_structures_tuned_for_obfuscation_discretion: 0.try_into().unwrap(),
			spool_buffer_size: SPOOL_BUFFER_SIZE
		}
	)
	.await
	.expect(INSTANTIATION_FAILURE);

	assert_eq!(
		squash_zip.previous_file_count(),
		3,
		"Three files were expected to have been read back"
	);

	for file in [
		"virtual/visions0.bin",
		"virtual/visions2.bin",
		"virtual/visions3.bin"
	] {
		squash_zip
			.file_process_time(&RelativePath::from_inner(file))
			.expect(&format!(
				"Expected file not read back from output ZIP: {}",
				file
			));
	}
}
