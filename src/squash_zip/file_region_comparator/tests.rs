use std::io::Cursor;

use super::*;

/// Tests the file region comparator functionality. It is assumed that the
/// input data length is even and that each region takes a half of the input
/// data.
async fn file_region_comparator_test(data: &[u8], expected_result: bool) {
	let mut comparator = FileRegionComparator::new(Cursor::new(data));
	assert_eq!(
		comparator
			.eq(0, (data.len() / 2) as u64, (data.len() / 2) as u32)
			.expect("No error should happen while comparing the regions"),
		expected_result,
		"Unexpected comparison result"
	);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn same_regions_work() {
	file_region_comparator_test(&[0, 1, 2, 0, 1, 2], true).await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn different_regions_work() {
	file_region_comparator_test(&[0, 1, 2, 0, 0, 0], false).await
}
