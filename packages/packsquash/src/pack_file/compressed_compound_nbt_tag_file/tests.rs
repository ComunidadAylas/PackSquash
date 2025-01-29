use super::*;

use futures::StreamExt;
use pretty_assertions::assert_eq;
use tokio_test::io::Builder;

/// A real-world structure NBT file whose compressed stream is known to not be able to be
/// optimized any further with the default options, and thus falls back to the result of
/// `reencapsulate_gzip_members`.
static OPTIMIZED_REAL_STRUCTURE_FILE: &[u8] = include_bytes!("verdalia32.nbt");
/// The same file as `OPTIMIZED_REAL_STRUCTURE_FILE`, but split into two gzip members,
/// which guarantees reduced storage efficiency that can be further optimized, while also
/// testing the ability to handle multiple gzip members.
static OPTIMIZED_REAL_STRUCTURE_FILE_SPLITTED_IN_TWO_GZIP_MEMBERS: &[u8] =
	include_bytes!("verdalia32_splitted.nbt");

/// Processes the given input data as a [CompressedCompoundNbtTagFile], expecting a successful result
/// that equals the input data while potentially being smaller.
async fn successful_process_test(input: &[u8], expect_smaller: bool) {
	let input_nbt = parse_compressed_nbt(input);

	let optimized_data_stream = CompressedCompoundNbtTagFile {
		read: Builder::new().read(input).build(),
		file_length_hint: input.len(),
		optimization_settings: CompressedCompoundNbtTagFileOptions::default()
	}
	.process();

	let optimization_output = optimized_data_stream
		.map(|result| tokio_stream::iter(result.expect("No error should happen while processing").1))
		.flatten()
		.collect::<Vec<u8>>()
		.await;

	assert!(
		!optimization_output.is_empty(),
		"No valid output was produced"
	);

	println!("Original size: {} bytes", input.len());
	println!("Optimized size: {} bytes", optimization_output.len());

	let output_nbt = parse_compressed_nbt(&optimization_output);

	assert_eq!(input_nbt, output_nbt);
	assert!(
		optimization_output.len() <= input.len(),
		"Optimizing this file should never yield a bigger result"
	);
	if expect_smaller {
		assert!(
			optimization_output.len() < input.len(),
			"Optimizing this file should yield a smaller result"
		);
	}
}

fn parse_compressed_nbt(compressed_nbt: &[u8]) -> fastnbt::Value {
	let mut decompressed_nbt = Vec::with_capacity(compressed_nbt.len());

	MultiGzDecoder::new(compressed_nbt)
		.read_to_end(&mut decompressed_nbt)
		.expect("Failed to decompress NBT file");

	fastnbt::from_bytes::<fastnbt::Value>(&decompressed_nbt).expect("Failed to parse NBT file")
}

#[tokio::test(flavor = "multi_thread")]
async fn minifying_works() {
	successful_process_test(OPTIMIZED_REAL_STRUCTURE_FILE, false).await
}

#[tokio::test(flavor = "multi_thread")]
async fn minifying_several_gzip_members_works() {
	successful_process_test(
		OPTIMIZED_REAL_STRUCTURE_FILE_SPLITTED_IN_TWO_GZIP_MEMBERS,
		true
	)
	.await
}
