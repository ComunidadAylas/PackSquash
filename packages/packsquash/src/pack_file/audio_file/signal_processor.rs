//! Implements the digital signal processing routines required by the audio file optimization
//! code.

use super::OptimizationError;
use crate::config::ChannelCount;
use dasp_frame::{Frame, Mono, Stereo};
use dasp_interpolate::sinc::Sinc;
use dasp_signal::Signal;
use rubato::{FftFixedIn, Resampler};
use std::io::{ErrorKind, Read};
use std::num::{NonZeroU32, NonZeroU8};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::{MediaSourceStream, MediaSourceStreamOptions, ReadOnlySource};
use symphonia::core::meta::{Limit, MetadataOptions};
use symphonia::core::probe::Hint;
use symphonia::core::units::Duration;
use vorbis::{VorbisDecoder, VorbisError, OV_HOLE};

/// The count of frames (i.e. an audio sample for each channel) that will be accumulated
/// in a block before being handed off to resamplers and consumers. This controls the
/// trade-off between efficiency (reducing call overhead by operating on many frames at
/// once) and memory usage. The current value has been experimentally selected to offer
/// a good trade-off for general usage.
const FRAME_BLOCK_SIZE: usize = 512;

/// Constructs and executes a dasp pipeline to process the audio frames returned by a dasp
/// signal, appyling resampling, channel mixing and pitch shifting as specified. The raw,
/// processed samples are then yielded in blocks to the specified consumer.
// FIXME this and the Signal implementation macros could be replaced with const generics,
// but dasp doesn't support them yet
macro_rules! dasp_pipeline {
	(
		$input_signal:expr,
		target_pitch = $target_pitch:expr,
		do_channel_mixing = $do_channel_mixing:expr,
		resampler_state = $resampler_state:expr,
		processed_sample_block_consumer = $processed_sample_block_consumer:expr,
		is_silent = $is_silent:ident,
		input_channel_count = $input_channel_count:expr,
		channel_mixing_target_channel_count = $channel_mixing_target_channel_count:expr,
		channel_mixing_strategy = $channel_mixing_strategy:expr,
		downmix_to_resample_strategy = $downmix_to_resample_strategy:expr,
		upmix_after_resample_strategy = $upmix_after_resample_strategy:expr,
		upmix_after_resample_cleanup = $upmix_after_resample_cleanup:expr
	) => {{
		let input_signal = $input_signal;

		// It might be faster and/or yield better results to do pitch shifting before or
		// after channel mixing and other operations, depending on whether the shifting
		// would expand or reduce the sample list. However, this is very likely to be a
		// seldom used PackSquash feature, so the greatly increased code complexity is
		// not worth the effort. This yields OK results with good performance anyway
		let pitch_shifted_signal = if $target_pitch != 1.0 {
			// Minecraft lets OpenAL "Frequency Shift by Pitch" implement pitch shifting
			// (see https://www.openal.org/documentation/openal-1.1-specification.pdf).
			// OpenAL achieves the pitch shift by resampling the audio, i.e. changing
			// both tempo and pitch, which introduces the "chipmunk effect". As we want
			// the sound to be played back at its original speed at target_pitch, we
			// shift the pitch to the inverse value. For instance, if the target pitch
			// is 0.5 (half less pitch), we shift the pitch here to 1 / 0.5 = 2 (double
			// the pitch), so the pitch shifts cancel each other out
			let interpolator = Sinc::new(dasp_ring_buffer::Fixed::from(
				[<[f32; $input_channel_count]>::EQUILIBRIUM; 32]
			));
			EitherSignal::A(input_signal.scale_hz(interpolator, 1.0 / $target_pitch as f64))
		} else {
			EitherSignal::B(input_signal)
		};

		// Any channel mixing here has to be a mono -> stereo conversion or stereo -> mono
		// conversion, as we only support those two
		match ($do_channel_mixing, $resampler_state) {
			(false, None) => {
				// No channel mixing, no resampling

				let mut frame_buf = [const { vec![] }; $input_channel_count];
				for vec in frame_buf.iter_mut() {
					vec.reserve_exact(FRAME_BLOCK_SIZE);
				}

				// Accumulate frames (one sample for each channel) in blocks for processing
				let mut signal_iter = pitch_shifted_signal.until_exhausted().peekable();
				while let Some(frame) = signal_iter.next() {
					// Our signal implementations return an extra silence frame when EOF
					// is reached. This is because when they know the decoder can no longer
					// provide any samples they still have to return some frame. Ignore
					// that
					if signal_iter.peek().is_none() {
						continue;
					}

					// The compiler can optimize the iterator out
					$is_silent &= frame.iter().all(|sample| *sample == f32::EQUILIBRIUM);

					// The compiler can optimize the loop out
					for channel in 0..$input_channel_count {
						frame_buf[channel].push(frame[channel]);
					}

					if frame_buf[0].len() >= FRAME_BLOCK_SIZE {
						$processed_sample_block_consumer(&frame_buf)?;

						// Discard this block of samples
						for channel in 0..$input_channel_count {
							frame_buf[channel].clear();
						}
					}
				}

				// Consume the frames that didn't make it to a block
				if !frame_buf[0].is_empty() {
					$processed_sample_block_consumer(&frame_buf)?;
				}
			}
			(false, Some((mut resampler, mut resampled_samples_buf))) => {
				// No channel mixing, resampling

				let mut frame_buf = [const { vec![] }; $input_channel_count];
				for vec in frame_buf.iter_mut() {
					vec.reserve_exact(FRAME_BLOCK_SIZE);
				}

				// Accumulate frames (one sample for each channel) in blocks for processing
				let mut signal_iter = pitch_shifted_signal.until_exhausted().peekable();
				while let Some(frame) = signal_iter.next() {
					// Ignore the spurious last frame (see above)
					if signal_iter.peek().is_none() {
						continue;
					}

					// The compiler can optimize the iterator out
					$is_silent &= frame.iter().all(|sample| *sample == f32::EQUILIBRIUM);

					// The compiler can optimize the loop out
					for channel in 0..$input_channel_count {
						frame_buf[channel].push(frame[channel]);
					}

					if frame_buf[0].len() >= FRAME_BLOCK_SIZE {
						resampler.process_into_buffer(
							&frame_buf,
							&mut resampled_samples_buf,
							None
						)?;

						$processed_sample_block_consumer(&resampled_samples_buf)?;

						// Discard this block of samples
						for channel in 0..$input_channel_count {
							frame_buf[channel].clear();
						}
					}
				}

				// Resample and consume the frames that didn't make it to a block
				if !frame_buf[0].is_empty() {
					// The resampler requires input blocks of fixed size, so pad
					// the buffer. Note that, as Rubato's resamplers work on fixed
					// size input or output blocks, we can only move the padding
					// around, not get rid of it. Luckily, this doesn't matter for
					// practical purposes
					for channel in 0..$input_channel_count {
						frame_buf[channel].resize(FRAME_BLOCK_SIZE, f32::EQUILIBRIUM);
					}
					resampler.process_into_buffer(&frame_buf, &mut resampled_samples_buf, None)?;

					$processed_sample_block_consumer(&resampled_samples_buf)?;
				}
			}
			(true, None) => {
				// Channel mixing, no resampling

				let mut frame_buf = [const { vec![] }; $channel_mixing_target_channel_count];
				for vec in frame_buf.iter_mut() {
					vec.reserve_exact(FRAME_BLOCK_SIZE);
				}

				// Accumulate frames (one sample for each channel) in blocks for processing
				let mut signal_iter = pitch_shifted_signal.until_exhausted().peekable();
				while let Some(frame) = signal_iter.next() {
					// Ignore the spurious last frame (see above)
					if signal_iter.peek().is_none() {
						continue;
					}

					// The compiler can optimize the iterator out
					$is_silent &= frame.iter().all(|sample| *sample == f32::EQUILIBRIUM);

					// Do the channel mixing to the frame buffer
					$channel_mixing_strategy(&mut frame_buf, frame);

					if frame_buf[0].len() >= FRAME_BLOCK_SIZE {
						$processed_sample_block_consumer(&frame_buf)?;

						// Discard this block of samples
						for channel in 0..$channel_mixing_target_channel_count {
							frame_buf[channel].clear();
						}
					}
				}

				// Consume the frames that didn't make it to a block
				if !frame_buf[0].is_empty() {
					$processed_sample_block_consumer(&frame_buf)?;
				}
			}
			(true, Some((mut resampler, mut resampled_samples_buf))) => {
				// Channel mixing, resampling

				let mut frame_buf = [Vec::with_capacity(FRAME_BLOCK_SIZE)];

				// Accumulate frames (one sample for each channel) in blocks for processing
				let mut signal_iter = pitch_shifted_signal.until_exhausted().peekable();
				while let Some(frame) = signal_iter.next() {
					// Ignore the spurious last frame (see above)
					if signal_iter.peek().is_none() {
						continue;
					}

					// The compiler can optimize the iterator out
					$is_silent &= frame.iter().all(|sample| *sample == f32::EQUILIBRIUM);

					// Downmix to mono if necessary before resampling
					frame_buf[0].push($downmix_to_resample_strategy(frame));

					if frame_buf[0].len() >= FRAME_BLOCK_SIZE {
						resampler.process_into_buffer(
							&frame_buf,
							&mut resampled_samples_buf,
							None
						)?;

						// Upmix to stereo if necessary before handing off the block
						$upmix_after_resample_strategy(&mut resampled_samples_buf);

						$processed_sample_block_consumer(&resampled_samples_buf)?;

						// Run any necessary cleanup on the sample buffer after
						// the consumer is done with it
						$upmix_after_resample_cleanup(&mut resampled_samples_buf);

						// Discard this block of samples
						frame_buf[0].clear();
					}
				}

				// Resample and consume the frames that didn't make it to a block
				if !frame_buf[0].is_empty() {
					frame_buf[0].resize(FRAME_BLOCK_SIZE, f32::EQUILIBRIUM);
					resampler.process_into_buffer(&frame_buf, &mut resampled_samples_buf, None)?;

					$upmix_after_resample_strategy(&mut resampled_samples_buf);

					$processed_sample_block_consumer(&resampled_samples_buf)?;

					$upmix_after_resample_cleanup(&mut resampled_samples_buf);
				}
			}
		}
	}};
}

/// Decodes audio samples from the specified source, and applies digital signal
/// processing algorithms to optionally resample, channel mix and pitch shift
/// them. After that, the maybe processed samples are yielded in blocks to the
/// provided consumer in planar format.
///
/// This audio decoding and processing code used here is specialized for mono
/// and stereo signals, and doesn't work for surround signals. Attempting to
/// decode a surround signal will promptly return an error.
pub fn decode_and_process_sample_blocks(
	source: impl Read + Send + Sync + 'static,
	is_ogg: bool,
	target_channels: Option<ChannelCount>,
	target_sampling_frequency_producer: impl FnOnce(
		NonZeroU32,
		NonZeroU8
	) -> Result<NonZeroU32, OptimizationError>,
	target_pitch: f32,
	mut processed_sample_block_consumer: impl FnMut(&[Vec<f32>]) -> Result<(), OptimizationError>
) -> Result<bool, OptimizationError> {
	// For Ogg Vorbis files, it's best to use our version of the reference implementation
	// patched with aoTuV and Lancer, because it's faster and more space efficient. We need
	// its encoder anyway, so using the Symphonia Vorbis decoder will increase the executable
	// size and negate some of the benefits of the patches we're using for the only benefit of
	// development convenience. Needless to say, we didn't come this far with this meticulously
	// crafted audio processing code and library selection for "development convenience"
	let mut decoder = if is_ogg {
		SignalDecoder::Vorbis(VorbisDecoder::new(source)?)
	} else {
		SignalDecoder::Symphonia(SymphoniaDecoder::new(source)?)
	};
	let input_channels = decoder.channels();
	let input_sampling_frequency = decoder.sampling_frequency();

	// Minecraft only supports mono and stereo sounds. Sound files with more channels
	// are refused with errors, so there's no point in us supporting them either
	if input_channels.get() > 2 {
		return Err(OptimizationError::UnsupportedChannelCount);
	}

	let do_channel_mixing = target_channels.map_or(false, |target_channels| {
		input_channels != target_channels.into()
	});
	let output_channels = if do_channel_mixing {
		// Even input channels -> 1 channel, odd input channels -> 2 channels
		NonZeroU8::new(1 + input_channels.get() % 2).unwrap()
	} else {
		input_channels
	};
	let target_sampling_frequency =
		target_sampling_frequency_producer(input_sampling_frequency, output_channels)?;

	let resampler_state = if target_sampling_frequency != input_sampling_frequency {
		let resampler = FftFixedIn::new(
			input_sampling_frequency.get().try_into().map_err(|_| {
				OptimizationError::InvalidSourceSamplingFrequency {
					sampling_frequency: input_sampling_frequency
				}
			})?,
			target_sampling_frequency.get().try_into().map_err(|_| {
				OptimizationError::InvalidTargetSamplingFrequency {
					sampling_frequency: target_sampling_frequency
				}
			})?,
			FRAME_BLOCK_SIZE,
			2,
			match (input_channels.get(), do_channel_mixing) {
				(_, true) => {
					// When channel mixing, for better performance:
					// - For mono signals, we resample first and then upmix to stereo.
					// - For stereo signals, we downmix to mono first and then resample.
					// Therefore, the resampler always sees one input channel
					1
				}
				(input_channels, false) => {
					// When channel mixing is not involved, we have to resample
					// all the input channels we have
					input_channels as usize
				}
			}
		)?;
		let resampled_samples_buf = resampler.output_buffer_allocate();

		Some((resampler, resampled_samples_buf))
	} else {
		None
	};

	let mut is_silent = true;

	let mut last_vorbis_error = None;
	let mut last_symphonia_error = None;
	let mut _hole_in_data_found = false;

	// FIXME we only really need to use dasp when pitch shifting, but we always go through
	// its pipeline. A more specialized code path would avoid copying around frames one by one
	match input_channels.get() {
		1 => dasp_pipeline!(
			match decoder {
				SignalDecoder::Vorbis(decoder) => EitherSignal::A(MonoVorbisSignal::new(
					decoder,
					&mut last_vorbis_error,
					&mut _hole_in_data_found
				)),
				SignalDecoder::Symphonia(decoder) =>
					EitherSignal::B(MonoSymphoniaSignal::new(decoder, &mut last_symphonia_error)),
			},
			target_pitch = target_pitch,
			do_channel_mixing = do_channel_mixing,
			resampler_state = resampler_state,
			processed_sample_block_consumer = processed_sample_block_consumer,
			is_silent = is_silent,
			input_channel_count = 1,
			channel_mixing_target_channel_count = 2,
			channel_mixing_strategy = |frame_buf: &mut [Vec<f32>; 2], frame: Mono<f32>| {
				// Upmix by duplicating the sample for each channel
				frame_buf[0].push(frame[0]);
				frame_buf[1].push(frame[0]);
			},
			downmix_to_resample_strategy = |frame: Mono<f32>| {
				// No-op: already mono
				frame[0]
			},
			upmix_after_resample_strategy = |resampled_samples_buf: &mut Vec<Vec<f32>>| {
				// Upmix mono to stereo by duplicating the samples for each channel.
				// Cloning the resampled samples is faster than resampling cloned samples
				resampled_samples_buf.push(resampled_samples_buf[0].clone());
			},
			upmix_after_resample_cleanup = |resampled_samples_buf: &mut Vec<Vec<f32>>| {
				// Rubato checks the length of this vector is equal to the number
				// of input channels, so it's necessary to undo the push
				resampled_samples_buf.pop();
			}
		),
		2 => dasp_pipeline!(
			match decoder {
				SignalDecoder::Vorbis(decoder) => EitherSignal::A(StereoVorbisSignal::new(
					decoder,
					&mut last_vorbis_error,
					&mut _hole_in_data_found
				)),
				SignalDecoder::Symphonia(decoder) => EitherSignal::B(StereoSymphoniaSignal::new(
					decoder,
					&mut last_symphonia_error
				))
			},
			target_pitch = target_pitch,
			do_channel_mixing = do_channel_mixing,
			resampler_state = resampler_state,
			processed_sample_block_consumer = processed_sample_block_consumer,
			is_silent = is_silent,
			input_channel_count = 2,
			channel_mixing_target_channel_count = 1,
			channel_mixing_strategy = |frame_buf: &mut [Vec<f32>; 1], frame: Stereo<f32>| {
				// Downmix by averaging the channel samples. This offers good results for
				// most stereo sounds, but the headroom (each channel can at most contribute
				// half to the total volume) may be undesirable in some rare circumstances.
				// Interesting read: https://dsp.stackexchange.com/a/3603
				frame_buf[0].push((frame[0] + frame[1]) / 2.0);
			},
			downmix_to_resample_strategy = |frame: Stereo<f32>| {
				// Downmix as above
				(frame[0] + frame[1]) / 2.0
			},
			upmix_after_resample_strategy = |_: &mut Vec<Vec<f32>>| {
				// Do nothing, channel mixing a stereo signal is converting it to mono
			},
			upmix_after_resample_cleanup = |_: &mut Vec<Vec<f32>>| {
				// Nothing to clean up
			}
		),
		_ => unreachable!("Unexpected channel count: {}", input_channels)
	}

	if let Some(vorbis_error) = last_vorbis_error {
		return Err(vorbis_error.into());
	}

	if let Some(symphonia_error) = last_symphonia_error {
		return Err(symphonia_error.into());
	}

	// FIXME read hole_in_data_found and output warning once the needed refactors are complete

	Ok(is_silent)
}

/// Helper enum to treat two different signal decoder types as if they were of
/// a single type for the purposes of this module.
enum SignalDecoder {
	Vorbis(VorbisDecoder),
	Symphonia(SymphoniaDecoder)
}

impl SignalDecoder {
	/// Returns the number of channels of the audio signal decoded by this decoder.
	fn channels(&mut self) -> NonZeroU8 {
		match self {
			Self::Vorbis(decoder) => decoder.channels(),
			Self::Symphonia(decoder) => decoder.channels()
		}
	}

	/// Returns the sampling frequency of the audio signal decoded by this decoder.
	fn sampling_frequency(&mut self) -> NonZeroU32 {
		match self {
			Self::Vorbis(decoder) => decoder.sampling_frequency(),
			Self::Symphonia(decoder) => decoder.sampling_frequency()
		}
	}
}

/// Helper enum to treat two different signals with a common associated frame
/// type as if they were a single signal type. In other words, this helper enum
/// provides a limited form of polymorphism without dynamic dispatch.
enum EitherSignal<F: Frame, S: Signal<Frame = F>, T: Signal<Frame = F>> {
	A(S),
	B(T)
}

impl<F: Frame, S: Signal<Frame = F>, T: Signal<Frame = F>> Signal for EitherSignal<F, S, T> {
	type Frame = F;

	fn next(&mut self) -> Self::Frame {
		match self {
			Self::A(signal) => signal.next(),
			Self::B(signal) => signal.next()
		}
	}

	fn is_exhausted(&self) -> bool {
		match self {
			Self::A(signal) => signal.is_exhausted(),
			Self::B(signal) => signal.is_exhausted()
		}
	}
}

/// A convenience wrapper for Symphonia decoder data types that probes an
/// input source and makes it readily usable for constructing a dasp audio
/// signal from it.
struct SymphoniaDecoder {
	format_reader: Box<dyn FormatReader>,
	decoder: Box<dyn Decoder>,
	main_track_id: u32,
	channels: NonZeroU8,
	sampling_frequency: NonZeroU32
}

impl SymphoniaDecoder {
	/// Constructs a new [SymphoniaDecoder] from the specified data source.
	fn new(source: impl Read + Send + Sync + 'static) -> Result<Self, OptimizationError> {
		// Probe the audio container format
		let format_reader = symphonia::default::get_probe()
			.format(
				&Hint::new(),
				MediaSourceStream::new(
					Box::new(ReadOnlySource::new(source)),
					MediaSourceStreamOptions::default()
				),
				&FormatOptions::default(),
				&MetadataOptions {
					limit_metadata_bytes: Limit::Maximum(0),
					limit_visual_bytes: Limit::Maximum(0)
				}
			)?
			.format;

		let main_track = format_reader
			.tracks()
			.iter()
			// Find the first track with a codec Symphonia can decode
			.find(|track| track.codec_params.codec != CODEC_TYPE_NULL)
			.ok_or(OptimizationError::NoAudioTrack)?;
		let main_track_id = main_track.id;

		let decoder = symphonia::default::get_codecs()
			.make(&main_track.codec_params, &DecoderOptions::default())?;

		let channels = match main_track
			.codec_params
			.channels
			.map(|channels| channels.count())
		{
			None | Some(0) => return Err(OptimizationError::UnsupportedChannelCount),
			Some(channels) => NonZeroU8::new(channels as u8).unwrap()
		};

		let sampling_frequency = NonZeroU32::new(
			main_track
				.codec_params
				.sample_rate
				.ok_or(OptimizationError::UnknownSamplingFrequency)?
		)
		// It does not make sense for the sampling frequency to be zero, but defensively
		// bail out if that happens for some reason
		.ok_or(OptimizationError::UnknownSamplingFrequency)?;

		Ok(Self {
			format_reader,
			decoder,
			main_track_id,
			channels,
			sampling_frequency
		})
	}

	/// Returns the number of channels of the audio signal that this decoder would decode.
	fn channels(&self) -> NonZeroU8 {
		self.channels
	}

	/// Returns the sampling frequency of the audio signal that this decoder would decode.
	fn sampling_frequency(&self) -> NonZeroU32 {
		self.sampling_frequency
	}
}

/// Creates a type that wraps a [VorbisDecoder] for a signal of a statically-known number of
/// channels into a dasp [Signal], allowing signal processing operations on the audio samples
/// contained in the decoded Vorbis file.
macro_rules! vorbis_signal_impl {
	{ $( $type_name:ident : $channels:expr ),+ } => {
		$(
			struct $type_name<'out> {
				vorbis_decoder: VorbisDecoder,
				last_error: &'out mut Option<VorbisError>,
				hole_in_data_found: &'out mut bool,
				sample_buffer: Option<Box<[Box<[f32]>]>>,
				sample_buffer_position: usize,
				end_of_stream: bool
			}

			impl<'out> $type_name<'out> {
				fn new(
					vorbis_decoder: VorbisDecoder,
					last_error: &'out mut Option<VorbisError>,
					hole_in_data_found: &'out mut bool
				) -> Self {
					Self {
						vorbis_decoder,
						last_error,
						hole_in_data_found,
						sample_buffer: None,
						sample_buffer_position: 0,
						end_of_stream: false
					}
				}
			}

			impl Signal for $type_name<'_> {
				type Frame = [f32; $channels];

				fn next(&mut self) -> Self::Frame {
					loop {
						match &self.sample_buffer {
							Some(sample_buffer) if self.sample_buffer_position < sample_buffer[0].len() => {
								// We read an audio packet before and have pending frames to yield

								let mut frame = Self::Frame::EQUILIBRIUM;
								for (channel, sample) in frame.iter_mut().enumerate() {
									*sample = sample_buffer[channel][self.sample_buffer_position];
								}
								self.sample_buffer_position += 1;

								break frame;
							}
							None | Some(_) => {
								// We have never read an audio packet, or we have consumed all of its samples

								match self.vorbis_decoder.decode_audio_block() {
									Ok(Some(audio_samples)) => {
										// FIXME Sadly, the sample buffer outlives this function, so it can't
										// borrow data from the decoder within safe Rust. Therefore, we have to
										// copy it to our own buffer. Maybe this can be optimized somehow?
										let mut audio_samples_vec =
											Vec::with_capacity(audio_samples.samples().len());

										// The channel order is defined in the Vorbis I specification, and here:
										// https://xiph.org/vorbis/doc/vorbisfile/ov_read.html
										for channel_samples in audio_samples.samples() {
											audio_samples_vec.push((*channel_samples).into());
										}

										self.sample_buffer = Some(audio_samples_vec.into_boxed_slice());
										self.sample_buffer_position = 0;

										// Grab the first frame from the sample buffer we just read
										continue;
									}
									Ok(None) => {
										// The stream ended successfully, and we should not return any
										// sample, but due to the Signal trait contract we have to. As
										// a compromise to avoid the complexity and additional memory
										// consumption of decoding two blocks at a time, pretend that
										// the input audio contained one more sample of complete silence
										self.end_of_stream = true;
									}
									Err(VorbisError::LibraryError {
										error_code: OV_HOLE,
										..
									}) => {
										// The stream is (a bit) corrupt, but oggdec would just ignore
										// the hole and keep decoding, so users will expect us to be
										// lenient (how are they supposed to fix this, anyway?). Try
										// again
										*self.hole_in_data_found = true;

										continue;
									}
									Err(err) => {
										// Some unrecoverable error happened. Halt decode
										self.end_of_stream = true;
										*self.last_error = Some(err);
									}
								}
							}
						}

						// In case of errors or EOF, fall back to a equilibrium-valued frame.
						// As per Signal trait docs, "calling next on an exhausted signal
						// should always yield Self::Frame::EQUILIBRIUM."
						break Self::Frame::EQUILIBRIUM;
					}
				}

				fn is_exhausted(&self) -> bool {
					self.end_of_stream
				}
			}
		)+
	};
}

vorbis_signal_impl! {
	MonoVorbisSignal: 1,
	StereoVorbisSignal: 2
}

/// Creates a type that wraps a [SymphoniaDecoder] for a signal of a statically-known number of
/// channels into a dasp [Signal], allowing signal processing operations on the audio samples
/// contained in the decoded audio file.
macro_rules! symphonia_signal_impl {
	{ $( $type_name:ident : $channels:expr ),+ } => {
		$(
			struct $type_name<'error> {
				symphonia_decoder: SymphoniaDecoder,
				last_error: &'error mut Option<symphonia::core::errors::Error>,
				current_sample_buffer: Option<SampleBuffer<f32>>,
				current_sample_buffer_position: usize,
				end_of_stream: bool
			}

			impl<'error> $type_name<'error> {
				fn new(
					symphonia_decoder: SymphoniaDecoder,
					last_error: &'error mut Option<symphonia::core::errors::Error>
				) -> Self {
					Self {
						symphonia_decoder,
						last_error,
						current_sample_buffer: None,
						current_sample_buffer_position: 0,
						end_of_stream: false
					}
				}
			}

			impl Signal for $type_name<'_> {
				type Frame = [f32; $channels];

				fn next(&mut self) -> Self::Frame {
					loop {
						match &self.current_sample_buffer {
							Some(sample_buffer) if self.current_sample_buffer_position < sample_buffer.len() => {
								// We read an audio packet before and have pending frames to yield

								let frame = sample_buffer.samples()[self.current_sample_buffer_position
									..self.current_sample_buffer_position + Self::Frame::CHANNELS]
									.try_into()
									.unwrap();

								self.current_sample_buffer_position += Self::Frame::CHANNELS;

								break frame;
							}
							None | Some(_) => {
								// We have never read an audio packet, or we have consumed all of its samples

								let mut current_packet = None;
								while let (None, false) = (&current_packet, self.end_of_stream) {
									match self.symphonia_decoder.format_reader.next_packet() {
										Ok(packet) => {
											// Ignore packets from secondary audio tracks
											if packet.track_id() == self.symphonia_decoder.main_track_id {
												current_packet = Some(packet);
											}
										}
										Err(symphonia::core::errors::Error::IoError(err))
											if err.kind() == ErrorKind::UnexpectedEof =>
										{
											// Symphonia signals normal, expected EOF/EOS conditions as
											// errors
											self.end_of_stream = true;
										}
										Err(symphonia::core::errors::Error::ResetRequired) => {
											// This means that the stream is composed of several audio
											// streams that must be decoded independently. This is rare,
											// and most programs deal with it by pretending that the
											// stream ended, so do that to keep things simple
											self.end_of_stream = true;
										}
										Err(err) => {
											// Unrecoverable error. Halt decode
											self.end_of_stream = true;
											*self.last_error = Some(err);
										}
									}
								}

								if let Some(current_packet) = current_packet {
									match self.symphonia_decoder.decoder.decode(&current_packet) {
										Ok(buffer) => {
											let sample_buffer =
												self.current_sample_buffer.get_or_insert_with(|| {
													SampleBuffer::<f32>::new(
														buffer.capacity() as Duration,
														*buffer.spec()
													)
												});

											// The channel order matches the Channels iterator order. Symphonia
											// is supposed to abstract us away from whatever order the codec is
											// actually using, which may vary in surround signals. See:
											// https://github.com/pdeljanov/Symphonia/commit/4e40f9dbccae5200939fdfc1f3dab750bd92d7a8
											// For stereo signals, this matches the conventional left-right layout
											sample_buffer.copy_interleaved_ref(buffer);
											self.current_sample_buffer_position = 0;

											// Yield the first frame from the replenished sample buffer
											continue;
										}
										Err(symphonia::core::errors::Error::ResetRequired) => {
											// Similar case as above when demuxing and requiring reset
											self.end_of_stream = true;
										}
										Err(err) => {
											// Unrecoverable error. Halt decode
											self.end_of_stream = true;
											*self.last_error = Some(err);
										}
									}
								}
							}
						}

						// In case of errors or EOF, fall back to a equilibrium-valued frame.
						// As per Signal trait docs, "calling next on an exhausted signal
						// should always yield Self::Frame::EQUILIBRIUM."
						break Self::Frame::EQUILIBRIUM;
					}
				}

				fn is_exhausted(&self) -> bool {
					self.end_of_stream
				}
			}
		)+
	};
}

symphonia_signal_impl! {
	MonoSymphoniaSignal: 1,
	StereoSymphoniaSignal: 2
}
