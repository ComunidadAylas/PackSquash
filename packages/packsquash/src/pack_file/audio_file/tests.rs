use pretty_assertions::assert_eq;
use std::num::NonZeroU8;
use std::time::Duration;
use std::{env, fs};

use tokio_stream::StreamExt;
use tokio_test::io::Builder;

use super::*;

static FLAC_AUDIO_DATA: &[u8] = include_bytes!("dtmf_tone.flac");
static FLAC_AUDIO_DATA_8KHZ: &[u8] = include_bytes!("dtmf_tone_8khz.flac");
static OGG_AUDIO_DATA: &[u8] = include_bytes!("dtmf_tone.ogg");
static EMPTY_OGG_AUDIO_DATA: &[u8] = include_bytes!("empty.ogg");
static OGG_AUDIO_DATA_UNUSUAL_SAMPLE_RATE: &[u8] = include_bytes!("araquanid_ambient.ogg");

/// Processes the given input data as a [AudioFile], using the provided settings,
/// expecting a successful result.
async fn successful_process_test(
	input_data: &[u8],
	is_ogg: bool,
	settings: AudioFileOptions,
	expect_same_file_size: bool,
	expected_channels: u8,
	expected_sample_rate: NonZeroU32
) {
	let data_stream = AudioFile {
		read: Builder::new().read(input_data).build(),
		file_length_hint: input_data.len(),
		is_ogg,
		optimization_settings: settings
	}
	.process();

	let process_result: Vec<(Cow<'static, str>, ByteBuffer)> = data_stream
		.map(|result| result.expect("No error should happen while processing"))
		.collect()
		.await;

	assert!(
		!process_result.is_empty(),
		"Some data was expected for this input"
	);

	let mut data = Vec::with_capacity(input_data.len());
	for (_, partial_data) in process_result {
		data.extend_from_slice(partial_data.as_ref());
	}

	assert!(
		!expect_same_file_size || data.len() == input_data.len(),
		"The processed audio file should be the same size as its unprocessed version"
	);

	// Do cheap checks to see if the result data looks like Ogg Vorbis I data, according to
	// its specification. All Ogg Vorbis logical bitstreams start with an Ogg BOS page that only
	// contains the 30 byte-long Vorbis identification header, for a total of 58 bytes

	assert!(
		data.len() > 58,
		"The processed audio file is too small to contain Ogg Vorbis data"
	);

	assert!(
		// Ogg capture pattern magic
		&data[0..=3] == b"OggS" &&
		// Ogg version (0 for the current format specification)
		data[4] == 0 &&
		// Header type flag (2 = only BOS flag is set)
		data[5] == 2 &&
		// A single segment for this packet, with 30 as its corresponding lacing value
		data[26] == 1 && data[27] == 30 &&
		// Vorbis header packet type 1: identification header
		data[28] == 1 &&
		// Vorbis header packet magic
		&data[29..=34] == b"vorbis" &&
		// Vorbis version
		data[35..=38] == [0; 4],
		"The processed audio file is not valid Ogg Vorbis data"
	);

	assert_eq!(
		expected_channels, data[39],
		"The processed audio file has an unexpected channel count"
	);

	assert_eq!(
		expected_sample_rate.get().to_le_bytes(),
		data[40..=43],
		"The processed audio file has an unexpected sampling frequency: {}",
		u32::from_le_bytes(data[40..=43].try_into().unwrap())
	);

	if env::var("WRITE_AUDIO_TEST_RESULTS").as_deref().ok() == Some("1") {
		fs::write("../../target/audio_test_result.ogg", &data)
			.expect("No error should happen while writing a test result to disk")
	}
}

/// Processes the given input data as a [AudioFile], using the provided settings,
/// expecting an error on the first stream result.
async fn error_process_test(
	read: impl AsyncRead + Unpin + Send + 'static,
	is_ogg: bool,
	settings: AudioFileOptions
) {
	let mut data_stream = AudioFile {
		read,
		file_length_hint: 0,
		is_ogg,
		optimization_settings: settings
	}
	.process();

	data_stream
		.next()
		.await
		.expect("Expected some result for this input")
		.expect_err("Expected an error for this input");
}

// These tests need to run on the multi-thread runtime due to tokio::task::block_in_place

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn transcoding_works() {
	successful_process_test(
		FLAC_AUDIO_DATA,
		false, // Is not Ogg
		Default::default(),
		false,                               // Smaller file size
		1,                                   // One channel (mono)
		POSITIONAL_AUDIO_SAMPLING_FREQUENCY  // Default sampling frequency
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn transcoding_unusual_sample_rate_works() {
	successful_process_test(
		OGG_AUDIO_DATA_UNUSUAL_SAMPLE_RATE,
		true, // Is Ogg
		Default::default(),
		false,                               // Smaller file size
		1,                                   // One channel (mono)
		POSITIONAL_AUDIO_SAMPLING_FREQUENCY  // Default sampling frequency
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn passthrough_works() {
	successful_process_test(
		OGG_AUDIO_DATA,
		true, // Is Ogg
		AudioFileOptions {
			transcode_ogg: false,
			empty_audio_optimization: false,
			two_pass_vorbis_optimization_and_validation: false,
			..Default::default()
		},
		true,                             // Same file size
		1,                                // One channel (mono)
		NonZeroU32::new(44_100).unwrap()  // Input sampling frequency
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn pitch_shifting_works() {
	successful_process_test(
		OGG_AUDIO_DATA,
		true, // Is Ogg
		AudioFileOptions {
			target_pitch: 1.25,
			..Default::default()
		},
		false,                               // Smaller file size
		1,                                   // One channel (mono)
		POSITIONAL_AUDIO_SAMPLING_FREQUENCY  // Default sampling frequency
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn channel_mixing_works() {
	successful_process_test(
		OGG_AUDIO_DATA,
		true, // Is Ogg
		AudioFileOptions {
			channels: ChannelMixingOption::ToChannels(NonZeroU8::new(2).unwrap().try_into().unwrap()),
			..Default::default()
		},
		false,                                   // Bigger file size
		2,                                       // Two channels (stereo)
		NON_POSITIONAL_AUDIO_SAMPLING_FREQUENCY  // Default sampling frequency
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn channel_mixing_and_pitch_shifting_work() {
	successful_process_test(
		OGG_AUDIO_DATA,
		true, // Is Ogg
		AudioFileOptions {
			target_pitch: 1.25,
			channels: ChannelMixingOption::ToChannels(NonZeroU8::new(2).unwrap().try_into().unwrap()),
			..Default::default()
		},
		false,                                   // Smaller file size
		2,                                       // Two channels (stereo)
		NON_POSITIONAL_AUDIO_SAMPLING_FREQUENCY  // Default sampling frequency
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn transcoded_audio_is_not_upsampled() {
	successful_process_test(
		FLAC_AUDIO_DATA_8KHZ,
		false, // Is Ogg
		Default::default(),
		false,                           // Smaller file size
		1,                               // One channel (mono)
		NonZeroU32::new(8_000).unwrap()  // Sampling frequency of the original audio data
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn valid_empty_input_works() {
	successful_process_test(
		EMPTY_OGG_AUDIO_DATA,
		true, // Is Ogg
		AudioFileOptions {
			empty_audio_optimization: true,
			..Default::default()
		},
		true,                        // Same size (should recognize the empty sound as such)
		1,                           // One channel (mono)
		NonZeroU32::new(1).unwrap()  // Sampling frequency of the crafted empty file
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn invalid_empty_input_is_handled() {
	error_process_test(
		Builder::new().read(&[]).build(),
		false, // Is not Ogg
		Default::default()
	)
	.await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn invalid_non_empty_input_is_handled() {
	error_process_test(
		Builder::new()
			.read(&[1, 2])
			.wait(Duration::from_millis(20))
			.read(&[3, 4])
			.build(),
		false, // Is not Ogg
		Default::default()
	)
	.await
}
