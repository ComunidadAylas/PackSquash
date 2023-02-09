//! Implements the digital signal processing routines required by the audio file optimization
//! code.

use super::OptimizationError;
use crate::config::ChannelCount;
use dasp_frame::Frame;
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
use vorbis_rs::{VorbisDecoder, VorbisError, VorbisLibraryErrorKind};

/// The count of frames (i.e. an audio sample for each channel) that will be accumulated
/// in a block before being handed off to resamplers and consumers. This controls the
/// trade-off between efficiency (reducing call overhead by operating on many frames at
/// once) and memory usage. The current value has been experimentally selected to offer
/// a good trade-off for general usage.
const FRAME_BLOCK_SIZE: usize = 512;

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
		NonZeroU8,
		NonZeroU8
	) -> Result<NonZeroU32, OptimizationError>,
	target_pitch: f32,
	processed_sample_block_consumer: impl FnMut(&[Vec<f32>]) -> Result<(), OptimizationError>
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
	// are rejected with errors, so there's no point in us supporting them either
	if input_channels.get() > 2 {
		return Err(OptimizationError::UnsupportedChannelCount);
	}

	let output_channels =
		target_channels.map_or(input_channels, |target_channels| target_channels.into());
	let do_channel_mixing = input_channels != output_channels;
	let target_sampling_frequency = target_sampling_frequency_producer(
		input_sampling_frequency,
		input_channels,
		output_channels
	)?;

	let resampler = if target_sampling_frequency != input_sampling_frequency {
		Some(FftFixedIn::new(
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
		)?)
	} else {
		None
	};

	let mut last_vorbis_error = None;
	let mut last_symphonia_error = None;
	let mut _hole_in_data_found = false;

	macro_rules! input_signal {
		() => {
			match decoder {
				SignalDecoder::Vorbis(decoder) => EitherSignal::A(VorbisSignal::new(
					decoder,
					&mut last_vorbis_error,
					&mut _hole_in_data_found
				)),
				SignalDecoder::Symphonia(decoder) => {
					EitherSignal::B(SymphoniaSignal::new(decoder, &mut last_symphonia_error))
				}
			}
		};
	}

	let is_silent = match (input_channels.get(), do_channel_mixing) {
		(1, false) => execute_dasp_pipeline::<1, 1, _>(
			input_signal!(),
			target_pitch,
			resampler,
			processed_sample_block_consumer
		)?,
		(1, true) => execute_dasp_pipeline::<1, 2, _>(
			input_signal!(),
			target_pitch,
			resampler,
			processed_sample_block_consumer
		)?,
		(2, false) => execute_dasp_pipeline::<2, 2, _>(
			input_signal!(),
			target_pitch,
			resampler,
			processed_sample_block_consumer
		)?,
		(2, true) => execute_dasp_pipeline::<2, 1, _>(
			input_signal!(),
			target_pitch,
			resampler,
			processed_sample_block_consumer
		)?,
		_ => unreachable!("Unexpected channel count: {}", input_channels)
	};

	if let Some(vorbis_error) = last_vorbis_error {
		return Err(vorbis_error.into());
	}

	if let Some(symphonia_error) = last_symphonia_error {
		return Err(symphonia_error.into());
	}

	// TODO read hole_in_data_found and output warning once the needed refactors are complete

	Ok(is_silent)
}

/// Constructs and executes a dasp pipeline to process the audio frames returned by a dasp
/// signal, applying resampling, channel mixing and pitch shifting as specified. The raw,
/// processed samples are then yielded in blocks to the specified consumer.
fn execute_dasp_pipeline<
	const INPUT_CHANNELS: usize,
	const OUTPUT_CHANNELS: usize,
	S: Signal<Frame = [f32; INPUT_CHANNELS]>
>(
	input_signal: S,
	target_pitch: f32,
	resampler: Option<FftFixedIn<f32>>,
	mut processed_sample_block_consumer: impl FnMut(&[Vec<f32>]) -> Result<(), OptimizationError>
) -> Result<bool, OptimizationError>
where
	[f32; INPUT_CHANNELS]: Frame,
	<[f32; INPUT_CHANNELS] as Frame>::Sample: dasp_sample::FromSample<f64>,
	f64: dasp_sample::FromSample<<[f32; INPUT_CHANNELS] as Frame>::Sample>
{
	let mut is_silent = true;

	// It might be faster and/or yield better results to do pitch shifting before or
	// after channel mixing and other operations, depending on whether the shifting
	// would expand or reduce the sample list. However, this is very likely to be a
	// seldom used PackSquash feature, so the greatly increased code complexity is
	// not worth the effort. This yields OK results with good performance anyway
	// FIXME we only really need to use dasp when pitch shifting, but we always go through
	// its pipeline. A more specialized code path would avoid copying around frames one by one
	let pitch_shifted_signal = if target_pitch != 1.0 {
		// Minecraft lets OpenAL "Frequency Shift by Pitch" implement pitch shifting
		// (see https://www.openal.org/documentation/openal-1.1-specification.pdf).
		// OpenAL achieves the pitch shift by resampling the audio, i.e. changing
		// both tempo and pitch, which introduces the "chipmunk effect". As we want
		// the sound to be played back at its original speed at target_pitch, we
		// shift the pitch to the inverse value. For instance, if the target pitch
		// is 0.5 (half less pitch), we shift the pitch here to 1 / 0.5 = 2 (double
		// the pitch), so the pitch shifts cancel each other out
		let interpolator = Sinc::new(dasp_ring_buffer::Fixed::from(
			[<[f32; INPUT_CHANNELS]>::EQUILIBRIUM; 32]
		));
		EitherSignal::A(input_signal.scale_hz(interpolator, 1.0 / target_pitch as f64))
	} else {
		EitherSignal::B(input_signal)
	};

	let mut signal_iter = pitch_shifted_signal.until_exhausted().peekable();

	match (INPUT_CHANNELS != OUTPUT_CHANNELS, resampler) {
		(_, None) => {
			// Maybe channel mixing, no resampling

			let mut output_frame_buf = [const { vec![] }; OUTPUT_CHANNELS];
			for vec in &mut output_frame_buf {
				vec.reserve_exact(FRAME_BLOCK_SIZE);
			}

			// Accumulate frames (one sample for each channel) in blocks for processing
			while let Some(frame) = signal_iter.next() {
				// Our signal implementations return an extra silence frame when EOF
				// is reached. This is because when they know the decoder can no longer
				// provide any samples they still have to return some frame. Ignore
				// that
				if signal_iter.peek().is_none() {
					continue;
				}

				// The compiler can optimize the iterator out
				is_silent &= frame.iter().all(|sample| *sample == f32::EQUILIBRIUM);

				// Do the channel mixing to the frame buffer, if necessary. The compiler
				// can optimize the match out because it is done on generic const parameters
				match (INPUT_CHANNELS, OUTPUT_CHANNELS) {
					(input_channels, output_channels) if input_channels == output_channels => {
						// No channel mixing necessary. Pass the samples through
						for channel in 0..OUTPUT_CHANNELS {
							output_frame_buf[channel].push(frame[channel]);
						}
					}
					(1, _) => {
						// Upmix by repeating the input sample on each output channel
						for channel_samples in &mut output_frame_buf {
							channel_samples.push(frame[0]);
						}
					}
					(input_channels, 1) => {
						// Downmix by averaging the channel samples. This offers good results for
						// most stereo sounds, but the headroom (each channel can at most contribute
						// half to the total volume) may be undesirable in some rare circumstances.
						// Interesting read: https://dsp.stackexchange.com/a/3603
						output_frame_buf[0].push(frame.iter().sum::<f32>() / input_channels as f32);
					}
					_ => unimplemented!()
				}

				if output_frame_buf[0].len() >= FRAME_BLOCK_SIZE {
					processed_sample_block_consumer(&output_frame_buf)?;

					// Discard this block of samples
					for channel_samples in &mut output_frame_buf {
						channel_samples.clear();
					}
				}
			}

			// Consume the frames that didn't make it to a block
			if !output_frame_buf[0].is_empty() {
				processed_sample_block_consumer(&output_frame_buf)?;
			}
		}
		(false, Some(mut resampler)) => {
			// No channel mixing, resampling

			let mut output_frame_buf = [const { vec![] }; OUTPUT_CHANNELS];
			for vec in &mut output_frame_buf {
				vec.reserve_exact(FRAME_BLOCK_SIZE);
			}

			let mut resampled_samples_buf = resampler.output_buffer_allocate();

			// Accumulate frames (one sample for each channel) in blocks for processing
			while let Some(frame) = signal_iter.next() {
				// Ignore the spurious last frame (see above)
				if signal_iter.peek().is_none() {
					continue;
				}

				// The compiler can optimize the iterator out
				is_silent &= frame.iter().all(|sample| *sample == f32::EQUILIBRIUM);

				// The compiler can optimize the loop out
				for channel in 0..OUTPUT_CHANNELS {
					output_frame_buf[channel].push(frame[channel]);
				}

				if output_frame_buf[0].len() >= FRAME_BLOCK_SIZE {
					resampler.process_into_buffer(
						&output_frame_buf,
						&mut resampled_samples_buf,
						None
					)?;

					processed_sample_block_consumer(&resampled_samples_buf)?;

					// Discard this block of samples
					for channel_samples in &mut output_frame_buf {
						channel_samples.clear();
					}
				}
			}

			// Resample and consume the frames that didn't make it to a block
			if !output_frame_buf[0].is_empty() {
				// The resampler requires input blocks of fixed size, so pad
				// the buffer. Note that, as Rubato's resamplers work on fixed
				// size input or output blocks, we can only move the padding
				// around, not get rid of it. Luckily, this doesn't matter for
				// practical purposes
				for channel_samples in &mut output_frame_buf {
					channel_samples.resize(FRAME_BLOCK_SIZE, f32::EQUILIBRIUM);
				}
				resampler.process_into_buffer(&output_frame_buf, &mut resampled_samples_buf, None)?;

				processed_sample_block_consumer(&resampled_samples_buf)?;
			}
		}
		(true, Some(mut resampler)) => {
			// Channel mixing, resampling.
			// Any channel mixing here has to be a mono -> stereo conversion or stereo -> mono
			// conversion, as we only support those two. Exploit that to always downmix to
			// achieve better performance by always downmixing to mono before resampling

			let mut output_frame_buf = [Vec::with_capacity(FRAME_BLOCK_SIZE)];

			let mut resampled_samples_buf = resampler.output_buffer_allocate();

			// Accumulate frames (one sample for each channel) in blocks for processing
			while let Some(frame) = signal_iter.next() {
				// Ignore the spurious last frame (see above)
				if signal_iter.peek().is_none() {
					continue;
				}

				// The compiler can optimize the iterator out
				is_silent &= frame.iter().all(|sample| *sample == f32::EQUILIBRIUM);

				// Downmix to mono as above before resampling. This is a no-op if we have
				// a single input channel, and should be optimized out by the compiler
				output_frame_buf[0].push(frame.iter().sum::<f32>() / INPUT_CHANNELS as f32);

				if output_frame_buf[0].len() >= FRAME_BLOCK_SIZE {
					resampler.process_into_buffer(
						&output_frame_buf,
						&mut resampled_samples_buf,
						None
					)?;

					// Upmix before handing off the block as above. Cloning a single channel of
					// resampled samples is faster than resampling samples of several channels
					for _ in 0..OUTPUT_CHANNELS - 1 {
						resampled_samples_buf.push(resampled_samples_buf[0].clone());
					}

					processed_sample_block_consumer(&resampled_samples_buf)?;

					// Undo the upmixing to reuse this buffer
					for _ in 0..OUTPUT_CHANNELS - 1 {
						resampled_samples_buf.pop();
					}

					// Discard this block of samples
					output_frame_buf[0].clear();
				}
			}

			// Resample and consume the frames that didn't make it to a block
			if !output_frame_buf[0].is_empty() {
				output_frame_buf[0].resize(FRAME_BLOCK_SIZE, f32::EQUILIBRIUM);
				resampler.process_into_buffer(&output_frame_buf, &mut resampled_samples_buf, None)?;

				for _ in 0..OUTPUT_CHANNELS - 1 {
					resampled_samples_buf.push(resampled_samples_buf[0].clone());
				}

				processed_sample_block_consumer(&resampled_samples_buf)?;

				for _ in 0..OUTPUT_CHANNELS - 1 {
					resampled_samples_buf.pop();
				}
			}
		}
	}

	Ok(is_silent)
}

/// Helper enum to treat two different signal decoder types as if they were of
/// a single type for the purposes of this module.
enum SignalDecoder<R: Read> {
	Vorbis(VorbisDecoder<R>),
	Symphonia(SymphoniaDecoder)
}

impl<R: Read> SignalDecoder<R> {
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

/// Wraps a [VorbisDecoder] of a signal of a statically-known number of channels into
/// a dasp [Signal], allowing signal processing operations on the audio samples
/// contained in the decoded Vorbis file.
struct VorbisSignal<'out, R: Read, const CHANNELS: usize> {
	vorbis_decoder: VorbisDecoder<R>,
	last_error: &'out mut Option<VorbisError>,
	hole_in_data_found: &'out mut bool,
	sample_buffer: Option<Box<[Box<[f32]>]>>,
	sample_buffer_position: usize,
	end_of_stream: bool
}

impl<'out, R: Read, const CHANNELS: usize> VorbisSignal<'out, R, CHANNELS> {
	fn new(
		vorbis_decoder: VorbisDecoder<R>,
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

impl<const CHANNELS: usize, R: Read> Signal for VorbisSignal<'_, R, CHANNELS>
where
	[f32; CHANNELS]: Frame
{
	type Frame = [f32; CHANNELS];

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

					return frame;
				}
				None | Some(_) => {
					// We have never read an audio packet, or we have consumed all of its samples

					match self.vorbis_decoder.decode_audio_block() {
						Ok(Some(audio_samples)) => {
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
						Err(VorbisError::LibraryError(library_error))
							if library_error.kind() == VorbisLibraryErrorKind::Hole =>
						{
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
			return Self::Frame::EQUILIBRIUM;
		}
	}

	fn is_exhausted(&self) -> bool {
		self.end_of_stream
	}
}

/// Wraps a [SymphoniaDecoder] of a signal of a statically-known number of channels
/// into a dasp [Signal], allowing signal processing operations on the audio samples
/// contained in the decoded file.
struct SymphoniaSignal<'error, const CHANNELS: usize> {
	symphonia_decoder: SymphoniaDecoder,
	last_error: &'error mut Option<symphonia::core::errors::Error>,
	current_sample_buffer: Option<SampleBuffer<f32>>,
	current_sample_buffer_position: usize,
	end_of_stream: bool
}

impl<'error, const CHANNELS: usize> SymphoniaSignal<'error, CHANNELS> {
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

impl<const CHANNELS: usize> Signal for SymphoniaSignal<'_, CHANNELS>
where
	[f32; CHANNELS]: Frame
{
	type Frame = [f32; CHANNELS];

	fn next(&mut self) -> Self::Frame {
		loop {
			match &self.current_sample_buffer {
				Some(sample_buffer) if self.current_sample_buffer_position < sample_buffer.len() => {
					// We read an audio packet before and have pending frames to yield

					let frame = sample_buffer.samples()[self.current_sample_buffer_position
						..self.current_sample_buffer_position + CHANNELS]
						.try_into()
						.unwrap();

					self.current_sample_buffer_position += CHANNELS;

					return frame;
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
			return Self::Frame::EQUILIBRIUM;
		}
	}

	fn is_exhausted(&self) -> bool {
		self.end_of_stream
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
