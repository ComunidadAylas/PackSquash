use futures::{
	future,
	stream::{poll_fn, select},
	FutureExt, SinkExt, StreamExt
};
use gstreamer::{
	glib::BoolError, prelude::*, Buffer, Caps, Element, ElementFactory, FlowError, MessageView,
	Pipeline, Sample, State, StateChangeError, TagMergeMode, TagSetter
};

use std::{
	borrow::Cow,
	pin::Pin,
	task::{Context, Poll}
};

use bytes::BytesMut;
use gstreamer::{buffer::Readable, MappedBuffer};
use gstreamer_app::{AppSink, AppSrc};
use thiserror::Error;
use tokio::{io::AsyncRead, task};
use tokio_stream::Stream;
use tokio_util::{codec::FramedRead, io::ReaderStream};

use crate::config::{AudioFileOptions, ChannelMixingOption};

use super::{
	passthrough_file::PassthroughDecoder, util::to_ascii_lowercase_extension, OptimizedBytes,
	PackFile, PackFileConstructor, PackFileConstructorArgs
};

#[cfg(test)]
mod tests;

/// Represents an audio file, that can be optimized and/or transcoded to Ogg.
///
/// Vanilla Minecraft uses Ogg Vorbis files for both music and sound effects. Resource
/// packs may replace and add new sound events to Minecraft.
pub struct AudioFile<T: AsyncRead + Unpin + 'static> {
	read: T,
	is_ogg: bool,
	optimization_settings: AudioFileOptions
}

/// The size of the temporary audio data buffers that are handed-off and received from
/// GStreamer, in bytes.
const AUDIO_DATA_BUFFER_SIZE: usize = 32 * 1024;
const AUDIO_DATA_BUFFER_SIZE_U32: u32 = AUDIO_DATA_BUFFER_SIZE as u32;
const AUDIO_DATA_BUFFER_SIZE_U64: u64 = AUDIO_DATA_BUFFER_SIZE_U32 as u64;

/// Represents an error that may happen while optimizing audio files.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum OptimizationError {
	#[error("GStreamer element state change error (more info with GST_DEBUG): {0}")]
	GstreamerStateChange(#[from] StateChangeError),
	#[error("GStreamer error (more info with GST_DEBUG): {0}")]
	GstreamerBool(#[from] BoolError),
	#[error("GStreamer flow error (more info with GST_DEBUG): {0}")]
	GstreamerFlow(#[from] FlowError),
	#[error("GStreamer error message (more info with GST_DEBUG): {0}")]
	GstreamerMessage(#[from] gstreamer::glib::Error),
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error)
}

/// Alias for the opaque stream type that provides the processed (optimized) audio
/// file data, when it was transcoded.
pub type ProcessedAudioDataStream<T> =
	impl Stream<Item = Result<MappedBuffer<Readable>, OptimizationError>> + Unpin;

/// Creates a new processed audio file data stream, that will use the specified
/// GStreamer pipeline, which contains the provided appsrc and appsink elements,
/// to process audio file bytes read from an [AsyncRead] to an optimized form,
/// via transcoding.
fn new_processed_audio_data_stream<T: AsyncRead + Unpin + 'static>(
	read: T,
	gstreamer_pipeline: Pipeline,
	appsrc: &AppSrc,
	appsink: &AppSink
) -> ProcessedAudioDataStream<T> {
	let read_stream = ReaderStream::new(read);
	let input_sample_sink = appsrc.sink().sink_err_into();
	let bus_message_stream = gstreamer_pipeline.bus().unwrap().stream();

	let mut input_forward_future = read_stream
		.map(|read_result| {
			read_result.map_or_else(
				|error| Err(error.into()),
				|buffer| {
					Ok(Sample::builder()
						.buffer(&Buffer::from_slice(buffer))
						.build())
				}
			)
		})
		.forward(input_sample_sink);

	// This stream just polls the input forward future to completion,
	// returning either end of stream and no items or an error when the
	// future resolves. While it is in progress, it always returns Pending
	let input_forward_stream = poll_fn(move |cx| -> Poll<Option<_>> {
		input_forward_future
			.poll_unpin(cx)
			.map(|result| result.err().map(Err))
	});

	let error_message_stream = bus_message_stream
		.filter_map(|msg| {
			future::ready(match msg.view() {
				// Convert error messages and pass them through
				MessageView::Error(error) => Some(Some(Err(error.error().into()))),
				// Let the first EOS message pass, so we can end this stream. This is
				// needed because after EOS there will be no more messages for our
				// purposes, and we should end the stream here
				MessageView::Eos(_) => Some(None),
				// Do not ley any other messages through
				_ => None
			})
		})
		// Passthrough any error, but end the stream if we find EOS
		.scan((), |_, error_or_eos| future::ready(error_or_eos))
		.take(1);

	select(
		// Return either errors that happen while forwarding data from the read or another
		// data, by polling from two streams.
		// The result stream will only end when both of these end
		input_forward_stream,
		select(
			// Return either actual output data or error conditions signalled in the bus
			error_message_stream,
			appsink.stream().map(|sample| {
				Ok(sample
					.buffer_owned()
					.unwrap()
					.into_mapped_buffer_readable()
					.unwrap())
			})
		)
	)
	.chain(poll_fn(move |_| -> Poll<Option<_>> {
		// After an error or EOS message was received in the bus, the appsink
		// stream signalled EOS, and we reached EOF on the read, we know that
		// the file was completely processed.
		// This is a good moment to tear down the pipeline, and let GStreamer
		// free memory
		gstreamer_pipeline.set_state(State::Null).ok();

		Poll::Ready(None)
	}))
}

/// Adapts internal stream structs used by [AudioFile] to a single [Stream]
/// that matches client code expectations.
pub enum AudioDataStream<T: AsyncRead + Unpin + 'static> {
	Transcoded(ProcessedAudioDataStream<T>),
	PassedThrough(FramedRead<T, PassthroughDecoder>),
	InitError(tokio_stream::Once<<Self as Stream>::Item>)
}

impl<T: AsyncRead + Unpin + 'static> Stream for AudioDataStream<T> {
	type Item = Result<(Cow<'static, str>, OptimizedBytes<ByteBuffer>), OptimizationError>;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		// Pass through to the underlying stream, mapping types accordingly
		match Pin::into_inner(self) {
			AudioDataStream::Transcoded(stream) => stream.poll_next_unpin(cx).map(|item| {
				item.map(|result| {
					result.map(|buf| {
						(
							Cow::Borrowed("Transcoded"),
							OptimizedBytes(ByteBuffer::MappedBuffer(buf))
						)
					})
				})
			}),
			AudioDataStream::PassedThrough(stream) => stream.poll_next_unpin(cx).map(|item| {
				item.map(|result| {
					result.map_or_else(
						|err| Err(err.into()),
						|(optimization_strategy_message, buf)| {
							Ok((
								optimization_strategy_message,
								OptimizedBytes(ByteBuffer::BytesMut(buf.0))
							))
						}
					)
				})
			}),
			AudioDataStream::InitError(stream) => stream.poll_next_unpin(cx)
		}
	}
}

/// Helper enum to allow clients of [AudioFile] consume bytes from different
/// owned representations, which skips costly conversions.
#[derive(Debug)]
pub enum ByteBuffer {
	MappedBuffer(MappedBuffer<Readable>),
	BytesMut(BytesMut)
}

impl AsRef<[u8]> for ByteBuffer {
	fn as_ref(&self) -> &[u8] {
		match self {
			ByteBuffer::MappedBuffer(buf) => buf,
			ByteBuffer::BytesMut(buf) => buf
		}
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFile for AudioFile<T> {
	type ByteChunkType = ByteBuffer;
	type OptimizationError = OptimizationError;
	type OptimizedBytesChunksStream = AudioDataStream<T>;

	fn process(self) -> AudioDataStream<T> {
		// Bail with a passthrough stream if we want to do so
		if self.is_ogg && !self.optimization_settings.transcode_ogg {
			return AudioDataStream::PassedThrough(FramedRead::new(
				self.read,
				PassthroughDecoder {
					optimization_strategy_message: "Copied due to file settings"
				}
			));
		}

		// Perform GStreamer pipeline initialization operations and stream creation
		// in a closure that returns a result, to be able to use the more ergonomic ?
		// operator
		|| -> Result<AudioDataStream<T>, OptimizationError> {
			// With one task per resource file, this is optimal
			task::block_in_place(|| {
				// GStreamer always acquires a mutex lock to check if it was already
				// initialized, and if so returns successfully, so there's no point
				// for PackSquash to introduce extra synchronization to guarantee that
				// this is only called once. This behavior is unlikely to change, because
				// it is a reasonable expectation of the documentation, which states
				// that the method returns success "if GStreamer could be initialized"
				// (it doesn't specify now or before).
				// See: https://github.com/GStreamer/gstreamer/blob/44bdad58f623e50a07476c0f40f8ff7543396f7c/gst/gst.c#L411
				gstreamer::init().unwrap()
			});

			let gstreamer_pipeline = Pipeline::new(None);

			// Create the pipeline elements
			let appsrc = ElementFactory::make("appsrc", None).unwrap();
			let decoder = ElementFactory::make("decodebin", None).unwrap(); // Contains a demuxer + decoder
			let converter = ElementFactory::make("audioconvert", None).unwrap();
			let resampler = ElementFactory::make("audioresample", None).unwrap();
			let resampler_filter = ElementFactory::make("capsfilter", None).unwrap();
			let encoder = ElementFactory::make("vorbisenc", None).unwrap();
			let muxer = ElementFactory::make("oggmux", None).unwrap();
			let appsink = ElementFactory::make("appsink", None).unwrap();

			appsrc
				.set_property("max-bytes", &AUDIO_DATA_BUFFER_SIZE_U64)
				.unwrap();

			decoder.set_property("expose-all-streams", &false).unwrap();
			decoder
				.set_property(
					"caps",
					&Caps::new_simple("audio/x-raw", &[]) // Only decode audio streams
				)
				.unwrap();

			resampler.set_property("quality", &10).unwrap(); // Good quality resampling

			resampler_filter.set_property(
				"caps",
				&Caps::new_simple(
					"audio/x-raw",
					&[(
						"rate",
						&i32::from(self.optimization_settings.sampling_frequency)
					)]
				)
			)?;

			encoder.set_property(
				"min-bitrate",
				&i32::from(self.optimization_settings.minimum_bitrate)
			)?;
			encoder.set_property(
				"max-bitrate",
				&i32::from(self.optimization_settings.maximum_bitrate)
			)?;

			appsink.set_property("sync", &false).unwrap(); // Output at max speed, not realtime
			appsink
				.set_property("blocksize", &AUDIO_DATA_BUFFER_SIZE_U32)
				.unwrap();
			appsink.set_property("max-buffers", &8u32).unwrap();

			// decodebin (demuxer + decoder) needs to be linked later with the next step, because in the
			// beginning it doesn't have a source pad: it acquires it on the fly after probing the input

			// Add and link the constant parts of the pipeline together
			gstreamer_pipeline
				.add_many(&[
					&appsrc,
					&decoder,
					&converter,
					&resampler,
					&resampler_filter,
					&encoder,
					&muxer,
					&appsink
				])
				.unwrap();

			appsrc.link(&decoder)?;
			Element::link_many(&[&resampler, &resampler_filter, &encoder, &muxer, &appsink])?;

			// Discard all event-provided tags. As the encoder is
			// not simultaneously a tag reader, and we not explicitly
			// add any tags, the resulting file will have no tags
			encoder
				.downcast_ref::<TagSetter>()
				.unwrap()
				.set_tag_merge_mode(TagMergeMode::KeepAll);

			// If we need to change the target pitch, add the needed elements
			let mut pitch_shifter = None;

			#[allow(clippy::float_cmp)] // Edge case where comparing float equality is okay
			if self.optimization_settings.target_pitch != 1.0 {
				let pitch_shifter_element = ElementFactory::make("pitch", None).unwrap();

				// Minecraft lets OpenAL "Frequency Shift by Pitch" implement pitch shifting
				// (see https://www.openal.org/documentation/openal-1.1-specification.pdf).
				// OpenAL achieves the pitch shift by resampling the audio, i.e. changing both
				// tempo and pitch, which introduces the "chipmunk effect". As we want the sound
				// to be played back at its original speed at target_pitch, we shift the pitch
				// to the inverse value. For instance, if the target pitch is 0.5 (half less pitch),
				// we shift the pitch here to 1 / 0.5 = 2 (double the pitch), so the pitch shifts
				// cancel each other out
				pitch_shifter_element
					.set_property("pitch", &(1.0 / self.optimization_settings.target_pitch))?;
				pitch_shifter_element
					.set_property("tempo", &(1.0 / self.optimization_settings.target_pitch))?;

				gstreamer_pipeline.add(&pitch_shifter_element).unwrap();
				pitch_shifter_element.link(&resampler)?;

				pitch_shifter = Some(pitch_shifter_element);
			}

			// Handle the demuxer receiving a source pad
			let result_audio_channels = self.optimization_settings.channels;
			decoder.connect_pad_added(move |decoder, _| {
				// The decoder has just received a audio source.
				// Get the target element, and then its sink, to connect the
				// decoder source pad to it
				let sink_element = if let Some(pitch_shifter) = pitch_shifter.as_ref() {
					pitch_shifter
				} else {
					&resampler
				};
				let sink_pad = sink_element.static_pad("sink").unwrap();

				// Ignore event if the link is already set up
				if sink_pad.is_linked() {
					return;
				}

				if let ChannelMixingOption::ToChannels(num_channels) = result_audio_channels {
					// We want to mix to some number of channels, so configure the
					// necessary caps filter (channel mix filter)
					let channel_mix_filter = ElementFactory::make("capsfilter", None).unwrap();
					channel_mix_filter
						.set_property(
							"caps",
							&Caps::new_simple(
								"audio/x-raw",
								&[("channels", &i32::from(num_channels))]
							)
						)
						.unwrap();

					// Get the pipeline, which is the parent of any element in the
					// pipeline, and add the new caps filter to it
					sink_element
						.parent()
						.unwrap()
						.downcast_ref::<Pipeline>()
						.unwrap()
						.add(&channel_mix_filter)
						.unwrap();

					// Link the decoder with the converter, the converter to the channel mix filter, and
					// the channel mix filter to the sink element:
					// ... -> decoder -> converter -> channel_mix_filter -> sink_element -> ...
					Element::link_many(&[decoder, &converter, &channel_mix_filter, sink_element])
						.unwrap();

					// We also need to set the new element to play, or otherwise
					// the entire pipeline will pause
					channel_mix_filter.sync_state_with_parent().unwrap();
				} else {
					// No channel mixing is desired, so link the decoder with the
					// converter, and the converter directly with the sink element:
					// ... -> decoder -> converter -> sink_element -> ...
					Element::link_many(&[decoder, &converter, sink_element]).unwrap();
				}
			});

			// The pipeline is good to go. Let's start!
			gstreamer_pipeline.set_state(State::Playing)?;

			Ok(AudioDataStream::Transcoded(
				new_processed_audio_data_stream(
					self.read,
					gstreamer_pipeline,
					appsrc.downcast_ref::<AppSrc>().unwrap(),
					appsink.downcast_ref::<AppSink>().unwrap()
				)
			))
		}()
		.map_or_else(
			|err| AudioDataStream::InitError(tokio_stream::once(Err(err))),
			|stream| stream
		)
	}

	fn canonical_extension(&self) -> &str {
		"ogg"
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

impl<T: AsyncRead + Unpin + 'static> PackFileConstructor<T> for AudioFile<T> {
	type OptimizationSettings = AudioFileOptions;

	fn new<F: FnMut() -> Option<(T, u64)>>(
		mut file_read_producer: F,
		args: PackFileConstructorArgs<'_, AudioFileOptions>
	) -> Option<Self> {
		let extension = &*to_ascii_lowercase_extension(args.path);

		if matches!(extension, "ogg" | "oga" | "mp3" | "opus" | "flac" | "wav") {
			file_read_producer().map(|(read, _)| Self {
				read,
				is_ogg: extension == "ogg",
				optimization_settings: args.optimization_settings
			})
		} else {
			None
		}
	}
}
