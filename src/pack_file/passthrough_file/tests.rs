use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static BYTES: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

/// Processes the given input data as a [PassthroughFile],
/// expecting a successful result that equals the input data.
async fn successful_process_test(input_data: &[u8]) {
	let data_stream = PassthroughFile {
		read: Builder::new().read(input_data).build(),
		is_compressed: false,
		optimization_strategy_message: "Copied"
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, BytesMut)> = data_stream
		.map(|result| result.expect("No error should happen while decoding"))
		.collect()
		.await;

	let mut data = Vec::with_capacity(input_data.len());
	for (_, partial_data) in process_result {
		data.extend_from_slice(&partial_data);
	}

	assert_eq!(data, input_data);
}

#[tokio::test]
async fn works() {
	successful_process_test(&BYTES).await;
}

#[tokio::test]
async fn works_for_empty_input() {
	successful_process_test(&[]).await;
}
