//! Contains code to optimize audio files.

use bytes::{Buf, Bytes, BytesMut};
use optivorbis::remuxer::ogg_to_ogg;
use optivorbis::{
	Remuxer, VorbisCommentFieldsAction, VorbisOptimizerSettings, VorbisVendorStringAction
};
use rubato::{ResampleError, ResamplerConstructionError};
use std::borrow::Cow;
use std::cell::Cell;
use std::cmp;
use std::io::{Cursor, Read, Seek};
use std::num::NonZeroU32;
use thiserror::Error;
use tokio::io::AsyncRead;
use tokio_util::codec::{Decoder, FramedRead};
use vorbis_rs::{VorbisBitrateManagementStrategy, VorbisEncoder};

use crate::config::{AudioBitrateControlMode, AudioFileOptions, ChannelMixingOption};
use crate::pack_file::asset_type::PackFileAssetType;
use crate::pack_file::AsyncReadAndSizeHint;
use signal_processor::decode_and_process_sample_blocks;
use vorbis_stream_mangler::ValidatingAndObfuscatingOggVorbisStreamMangler;

use super::{PackFile, PackFileConstructor};

#[cfg(test)]
mod tests;

mod signal_processor;
mod vorbis_stream_mangler;

/// Represents an audio file, that can be optimized and/or transcoded to Ogg.
///
/// Vanilla Minecraft uses Ogg Vorbis files for both music and sound effects. Resource
/// packs may replace and add new sound events to Minecraft.
pub struct AudioFile<T: AsyncRead + Send + Unpin + 'static> {
	read: T,
	file_length_hint: usize,
	is_ogg: bool,
	optimization_settings: AudioFileOptions
}

/// Optimizer decoder that transforms audio files to an optimized representation.
pub struct OptimizerDecoder {
	optimization_settings: AudioFileOptions,
	is_ogg: bool,
	reached_eof: bool
}

/// Represents an error that may happen while optimizing audio files.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OptimizationError {
	#[error("Symphonia decoding error: {0}")]
	Symphonia(#[from] symphonia::core::errors::Error),
	#[error("Vorbis error: {0}")]
	Vorbis(#[from] vorbis_rs::VorbisError),
	#[error("Could not find a decodable audio track. Is this file in a supported format, and its extension correct?")]
	NoAudioTrack,
	#[error("Unknown or invalid channel count. Minecraft only supports mono and stereo sounds")]
	UnsupportedChannelCount,
	#[error("Unknown sampling frequency. Is this file corrupt?")]
	UnknownSamplingFrequency,
	#[error("Could not set up resampling: {0}")]
	ResamplerConstructionFailure(#[from] ResamplerConstructionError),
	#[error("Tried to resample from {sampling_frequency} Hz, but that frequency is too high. Please lower it")]
	InvalidSourceSamplingFrequency { sampling_frequency: NonZeroU32 },
	#[error("Tried to resample to {sampling_frequency} Hz, but that frequency is too high. Please lower it")]
	InvalidTargetSamplingFrequency { sampling_frequency: NonZeroU32 },
	#[error("An invalid target bitrate of zero was specified")]
	InvalidTargetBitrate,
	#[error("Resample error: {0}")]
	ResamplingFailure(#[from] ResampleError),
	#[error("{0}")]
	TwoPassOptimization(#[from] ogg_to_ogg::RemuxError),
	#[error("The Minecraft sample count limit for audio files was exceeded. Please reduce the sampling frequency or duration")]
	TooLongForMinecraft,
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error)
}

/// Helper enum to allow clients of [AudioFile] to consume bytes from different
/// owned representations, which skips costly conversions.
#[derive(Debug)]
pub enum ByteBuffer {
	CowSlice(Cow<'static, [u8]>),
	Bytes(Bytes)
}

impl AsRef<[u8]> for ByteBuffer {
	fn as_ref(&self) -> &[u8] {
		match self {
			ByteBuffer::CowSlice(buf) => buf,
			ByteBuffer::Bytes(buf) => buf
		}
	}
}

// FIXME: actual framing?
// (i.e. do not hold the entire file in memory before decoding, so that frame != file)
impl Decoder for OptimizerDecoder {
	type Item = (Cow<'static, str>, ByteBuffer);
	type Error = OptimizationError;

	fn decode(&mut self, _: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		Ok(None)
	}

	fn decode_eof(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
		// This method will be called when EOF is reached until it returns None. Because we
		// will only ever output a single item in the stream, always return None if we have
		// executed once already
		if self.reached_eof {
			return Ok(None);
		}
		self.reached_eof = true;

		let input_file = src.split_off(0).freeze();

		let skip_transcoding = self.is_ogg && !self.optimization_settings.transcode_ogg;
		let do_two_pass_optimization_and_validation = self
			.optimization_settings
			.two_pass_vorbis_optimization_and_validation;
		let do_ogg_obfuscation = self
			.optimization_settings
			.minecraft_version_supports_ogg_obfuscation
			&& self.optimization_settings.ogg_obfuscation;

		// First pass: transcode the input audio file to an efficient Ogg Vorbis representation.
		// This is necessary if the input audio file is not Ogg Vorbis, or if some modification
		// to the audio data is done (currently, channel mixing, resampling and pitch shifting)
		let (transcoded_file, channel_mixing_done, pitch_shifting_done) = if skip_transcoding {
			(ByteBuffer::Bytes(input_file.clone()), false, false)
		} else {
			let (transcoded_file, channel_mixing_done) = process_and_transcode(
				input_file.clone().reader(),
				self.is_ogg,
				&self.optimization_settings
			)?;

			(
				ByteBuffer::CowSlice(transcoded_file),
				channel_mixing_done,
				self.optimization_settings.target_pitch != 1.0
			)
		};

		// Second pass: run OptiVorbis on the input file, which may be transcoded by now. This
		// is a lossless, two-pass lossless optimization step that completes pretty quickly
		// (think on OxiPNG, but much, much faster and less quirkier)
		let transcoded_and_optimized_file = if do_two_pass_optimization_and_validation {
			ByteBuffer::CowSlice(
				validate_and_optimize(Cursor::new(transcoded_file.as_ref()), do_ogg_obfuscation)?
					.into()
			)
		} else {
			transcoded_file
		};

		// Third pass: check whether the transcoded and two-pass optimized file is actually smaller.
		// If not, quickly run OptiVorbis over the original file, which is practically guaranteed to
		// never return a file bigger than its input, and return that
		let optimized_file_is_input_file;
		let can_use_input_as_output = self.is_ogg && !pitch_shifting_done && !channel_mixing_done;

		let optimized_file = if do_two_pass_optimization_and_validation
			&& input_file.len() < transcoded_and_optimized_file.as_ref().len()
			&& can_use_input_as_output
		{
			optimized_file_is_input_file = true;
			ByteBuffer::CowSlice(
				validate_and_optimize(Cursor::new(input_file.as_ref()), do_ogg_obfuscation)?.into()
			)
		} else {
			optimized_file_is_input_file = false;
			transcoded_and_optimized_file
		};

		let optimization_strategy = match (skip_transcoding, do_two_pass_optimization_and_validation, optimized_file_is_input_file) {
			(false, false, false) => "Transcoded",
			(false, true, false) => "Transcoded, validated and optimized",
			(true, false, false) => "Copied",
			(true, true, false) => "Validated and optimized",
			(_, _, true) => "Validated and optimized, but transcoding yielded a bigger file. Try tweaking options for extra savings"
		}.into();

		Ok(Some((optimization_strategy, optimized_file)))
	}
}

/// Processes the input audio file and transcodes it to Ogg Vorbis, according to the
/// provided optimization settings. The audio signal processing done may include resampling,
/// pitch shifting and channel mixing. Empty sound files (e.g., without audio samples, or only
/// containing audio samples which are complete silence) may be special-cased for optimization,
/// yielding a minimal empty Ogg Vorbis file.
fn process_and_transcode(
	input_file: impl Read + Send + Sync + 'static,
	is_ogg: bool,
	optimization_settings: &AudioFileOptions
) -> Result<(Cow<'static, [u8]>, bool), OptimizationError> {
	// FIXME write to a SpooledTempFile whose maximum memory buffer size
	// is controlled by a global budget, once that refactor is complete
	let mut transcoded_file = vec![];
	let encoder = Cell::new(None);

	let mut channel_mixing_done = false;
	let is_silence = decode_and_process_sample_blocks(
		input_file,
		is_ogg,
		match optimization_settings.channels {
			ChannelMixingOption::ToChannels(count) => Some(count),
			ChannelMixingOption::Skip => None
		},
		|input_sampling_frequency, input_channel_count, output_channel_count| {
			let is_positional_audio = optimization_settings
				.is_positional_audio
				.unwrap_or(output_channel_count.get() == 1);

			channel_mixing_done = input_channel_count != output_channel_count;

			// Resampling to a frequency higher than the input one is a bad idea at
			// this point: it doesn't add meaningful audio information or helps using
			// different signal processing filters, but it definitely increases space
			// costs. Let's not do that
			let output_sampling_frequency = cmp::min(
				optimization_settings
					.sampling_frequency_override
					.unwrap_or(if is_positional_audio {
						optimization_settings.positional_audio_sampling_frequency
					} else {
						optimization_settings.non_positional_audio_sampling_frequency
					}),
				input_sampling_frequency
			);

			encoder.set(Some(VorbisEncoder::new(
				// Use a fixed serial for better compressibility when not using OptiVorbis,
				// which is non-zero to avoid some warnings
				1,
				// No comments
				[("", ""); 0],
				output_sampling_frequency,
				output_channel_count,
				match optimization_settings
					.audio_bitrate_control_mode_override
					.unwrap_or(optimization_settings.audio_bitrate_control_mode)
				{
					AudioBitrateControlMode::Cqf => VorbisBitrateManagementStrategy::QualityVbr {
						target_quality: target_bitrate_control_metric_to_quality(
							optimization_settings,
							is_positional_audio
						)
					},
					AudioBitrateControlMode::Vbr => VorbisBitrateManagementStrategy::Vbr {
						target_bitrate: target_bitrate_control_metric_to_bitrate(
							optimization_settings,
							is_positional_audio
						)?
					},
					AudioBitrateControlMode::Abr => VorbisBitrateManagementStrategy::Abr {
						average_bitrate: target_bitrate_control_metric_to_bitrate(
							optimization_settings,
							is_positional_audio
						)?
					},
					AudioBitrateControlMode::ConstrainedAbr => {
						VorbisBitrateManagementStrategy::ConstrainedAbr {
							maximum_bitrate: target_bitrate_control_metric_to_bitrate(
								optimization_settings,
								is_positional_audio
							)?
						}
					}
				},
				// Use jumbo Ogg pages for the least encapsulation overhead
				Some(u16::MAX),
				&mut transcoded_file
			)?));

			Ok(output_sampling_frequency)
		},
		optimization_settings.target_pitch,
		|block| {
			if let Some(mut vorbis_encoder) = encoder.take() {
				vorbis_encoder.encode_audio_block(block)?;
				encoder.set(Some(vorbis_encoder));
			}

			Ok(())
		}
	)?;

	// Explicitly finish the Ogg Vorbis stream, so we don't ignore any errors there
	if let Some(vorbis_encoder) = encoder.take() {
		vorbis_encoder.finish()?;
	}
	drop(encoder);

	Ok(
		if is_silence && optimization_settings.empty_audio_optimization {
			// Use a specially crafted minimal Ogg Vorbis file to represent
			// no audio data. This can save 1-2 KiB in Vorbis header information
			// per file: every known encoder assumes that audio samples will
			// follow, and thus they always add the complete codec setup
			// information to the headers, but we can do better and stub all
			// that out when not needed. Audio files full of silence are a
			// fairly common idiom among resource pack creators to disable
			// sounds.
			//
			// This file is a bit special in the sense that, even though it
			// follows the Vorbis format specification, it contains absolutely
			// no audio data. Minecraft handles this fine, but programs that
			// insist on decoding at least a sample may treat this as an error
			// condition (e.g., GStreamer)
			(
				Cow::Borrowed(include_bytes!("audio_file/empty.ogg")),
				channel_mixing_done
			)
		} else {
			(Cow::Owned(transcoded_file), channel_mixing_done)
		}
	)
}

/// Validates and optimizes the specified Ogg Vorbis file in two passes, using OptiVorbis.
fn validate_and_optimize<T: Read + Seek>(
	input_file: T,
	obfuscate: bool
) -> Result<Vec<u8>, OptimizationError> {
	let mut too_long_for_minecraft = false;
	// FIXME write to a SpooledTempFile whose maximum memory buffer size
	// is controlled by a global budget, once that refactor is complete
	let mut optimized_file = vec![];

	optivorbis::OggToOgg::new(
		ogg_to_ogg::Settings {
			randomize_stream_serials: true,
			first_stream_serial_offset: 0,
			// Beginning sample truncation is not supported by the Minecraft Vorbis decoder
			ignore_start_sample_offset: true,
			error_on_no_vorbis_streams: true,
			vorbis_stream_mangler: ValidatingAndObfuscatingOggVorbisStreamMangler::new(
				obfuscate,
				&mut too_long_for_minecraft
			)
		},
		{
			let mut optimizer_settings = VorbisOptimizerSettings::default();
			optimizer_settings.comment_fields_action = VorbisCommentFieldsAction::Delete;
			optimizer_settings.vendor_string_action = VorbisVendorStringAction::Empty;
			optimizer_settings
		}
	)
	.remux(input_file, &mut optimized_file)?;

	if too_long_for_minecraft {
		return Err(OptimizationError::TooLongForMinecraft);
	}

	Ok(optimized_file)
}

/// Converts the applicable target bitrate control metric, as defined in the specified
/// optimization settings, to a quality factor ready to pass on to a Vorbis encoder.
fn target_bitrate_control_metric_to_quality(
	optimization_settings: &AudioFileOptions,
	is_positional_audio: bool
) -> f32 {
	let target_bitrate_control_metric = optimization_settings
		.target_bitrate_control_metric_override
		.unwrap_or(if is_positional_audio {
			optimization_settings.positional_audio_target_bitrate_control_metric
		} else {
			optimization_settings.non_positional_audio_target_bitrate_control_metric
		});

	// Convert the more user-friendly range of [-1, 10] to the
	// [-0.1, 1] range expected by aoTuV
	target_bitrate_control_metric / 10.0
}

/// Converts the applicable target bitrate control metric, as defined in the specified
/// optimization settings, to a bitrate ready to pass on to a Vorbis encoder.
fn target_bitrate_control_metric_to_bitrate(
	optimization_settings: &AudioFileOptions,
	is_positional_audio: bool
) -> Result<NonZeroU32, OptimizationError> {
	let target_bitrate_control_metric = optimization_settings
		.target_bitrate_control_metric_override
		.unwrap_or(if is_positional_audio {
			optimization_settings.positional_audio_target_bitrate_control_metric
		} else {
			optimization_settings.non_positional_audio_target_bitrate_control_metric
		});

	// Convert the more user-friendly unit of kbits/s to bits/s, as
	// expected by Vorbis
	((target_bitrate_control_metric * 1000.0) as u32)
		.try_into()
		.map_err(|_| OptimizationError::InvalidTargetBitrate)
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFile for AudioFile<T> {
	type ByteChunkType = ByteBuffer;
	type OptimizationError = OptimizationError;
	type OptimizedByteChunksStream = FramedRead<T, OptimizerDecoder>;

	fn process(self) -> FramedRead<T, OptimizerDecoder> {
		FramedRead::with_capacity(
			self.read,
			OptimizerDecoder {
				optimization_settings: self.optimization_settings,
				is_ogg: self.is_ogg,
				reached_eof: false
			},
			self.file_length_hint
		)
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

impl<T: AsyncRead + Send + Unpin + 'static> PackFileConstructor<T> for AudioFile<T> {
	type OptimizationSettings = AudioFileOptions;

	fn new(
		file_read_producer: impl FnOnce() -> Option<AsyncReadAndSizeHint<T>>,
		asset_type: PackFileAssetType,
		optimization_settings: Self::OptimizationSettings
	) -> Option<Self> {
		file_read_producer().map(|(read, file_length_hint)| Self {
			read,
			// The file is too big to fit in memory if this conversion fails anyway
			file_length_hint: file_length_hint.try_into().unwrap_or(usize::MAX),
			is_ogg: matches!(asset_type, PackFileAssetType::GenericOggVorbisAudio),
			optimization_settings
		})
	}
}
