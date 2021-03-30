use futures_sink::Sink;
use gstreamer::{
	bus::BusStream, glib::BoolError, prelude::*, Buffer, Caps, Element, ElementFactory, FlowError,
	MessageView, Pipeline, Sample, State, StateChangeError, TagMergeMode, TagSetter
};

use std::{
	borrow::Cow,
	cell::Cell,
	convert::TryInto,
	pin::Pin,
	task::{Context, Poll}
};

use bytes::BytesMut;
use gstreamer::{buffer::Readable, MappedBuffer};
use gstreamer_app::{app_sink::AppSinkStream, app_src::AppSrcSink, AppSink, AppSrc};
use thiserror::Error;
use tokio::{io::AsyncRead, task};
use tokio_stream::Stream;
use tokio_util::{codec::FramedRead, io::poll_read_buf};

use super::{passthrough_file::PassthroughDecoder, OptimizedBytes, ResourcePackFile};

#[cfg(test)]
mod tests;

/// Represents an audio file, that can be optimized and/or transcoded to OGG.
///
/// Vanilla Minecraft uses OGG Vorbis files for both music and sound effects. Resource
/// packs may replace and add new sound events to Minecraft.
struct AudioFile<T: AsyncRead + Unpin + 'static> {
	read: T,
	file_length: usize,
	is_ogg: bool,
	optimization_settings: OptimizationSettings
}

/// Parameters that influence how a [AudioFile] is optimized.
struct OptimizationSettings {
	copy_if_ogg: bool,
	channels: i32,
	sampling_frequency: i32,
	target_pitch: f32,
	minimum_bitrate: i32,
	maximum_bitrate: i32
}

impl Default for OptimizationSettings {
	fn default() -> Self {
		Self {
			copy_if_ogg: false,
			channels: 0,
			sampling_frequency: 32_000,
			target_pitch: 1.0,
			minimum_bitrate: 40_000,
			maximum_bitrate: 96_000
		}
	}
}

/// The size of the temporary audio data buffers that are handed-off and received from
/// GStreamer, in bytes.
const AUDIO_DATA_BUFFER_SIZE: usize = 128 * 1024; // This fits in u32, the smallest reasonable usize
const AUDIO_DATA_BUFFER_SIZE_U32: u32 = AUDIO_DATA_BUFFER_SIZE as u32;
const AUDIO_DATA_BUFFER_SIZE_U64: u64 = AUDIO_DATA_BUFFER_SIZE_U32 as u64;

/// Represents an error that may happen while optimizing audio files.
#[derive(Error, Debug)]
enum OptimizationError {
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

/// Converts an [AsyncRead] to a stream of audio data processed by GStreamer by
/// sending chunks of data to an [AppSrcSink] and polling the output of [AppSinkStream].
struct ProcessedAudioDataStream<T: AsyncRead + Unpin + 'static> {
	read: T,
	input_buf: Cell<Vec<u8>>,
	pending_input_buf: bool,
	reached_eof: bool,
	signalled_eos: bool,
	gstreamer_pipeline: Pipeline,
	bus_message_stream: BusStream,
	input_byte_sink: AppSrcSink,
	output_byte_stream: AppSinkStream
}

impl<T: AsyncRead + Unpin + 'static> ProcessedAudioDataStream<T> {
	fn new(
		read: T,
		gstreamer_pipeline: Pipeline,
		input_byte_sink: AppSrcSink,
		output_byte_stream: AppSinkStream
	) -> Self {
		Self {
			read,
			input_buf: Cell::new(Vec::with_capacity(AUDIO_DATA_BUFFER_SIZE)),
			pending_input_buf: false,
			reached_eof: false,
			signalled_eos: false,
			bus_message_stream: gstreamer_pipeline.get_bus().unwrap().stream(),
			gstreamer_pipeline,
			input_byte_sink,
			output_byte_stream
		}
	}
}

impl<T: AsyncRead + Unpin + 'static> Stream for ProcessedAudioDataStream<T> {
	type Item = Result<MappedBuffer<Readable>, OptimizationError>;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		if !self.reached_eof {
			eprintln!("Not EOF");

			// Get the Vec with the pending read buffer in the cell,
			// temporarily setting the cell to a dummy value
			let mut input_buf = self.input_buf.take();

			// If we don't have a buffer to hand-off to GStreamer, we should read one
			if !self.pending_input_buf {
				eprintln!("Polling read");
				if let Poll::Ready(read_result) =
					poll_read_buf(Pin::new(&mut self.read), cx, &mut input_buf)
				{
					eprintln!("Polled read");
					match read_result {
						Ok(_) => self.pending_input_buf = true,
						Err(err) => return Poll::Ready(Some(Err(err.into())))
					};
				}
			}

			// If we have a buffer of data ready to send to GStreamer, send it
			if self.pending_input_buf {
				eprintln!("Pending buffer");
				if !input_buf.is_empty() {
					if let Poll::Ready(sink_ready_result) =
						Pin::new(&mut self.input_byte_sink).poll_ready(cx)
					{
						eprintln!("Sink ready");
						match sink_ready_result {
							Ok(_) => match Pin::new(&mut self.input_byte_sink).start_send(
								Sample::builder()
									.buffer(&Buffer::from_slice(input_buf))
									.build()
							) {
								Ok(_) => {
									// The buffer was just sent to GStreamer, moving its
									// ownership. Prepare a new buffer for future reads
									input_buf = Vec::with_capacity(AUDIO_DATA_BUFFER_SIZE);
									self.pending_input_buf = false;
								}
								Err(err) => return Poll::Ready(Some(Err(err.into())))
							},
							Err(err) => return Poll::Ready(Some(Err(err.into())))
						};
					}
				} else {
					eprintln!("EOF reached");
					// AsyncRead only reads no bytes when EOF is reached. Remember this
					self.reached_eof = true;
				}
			}

			// Now restore the input buffer cell contents. It doesn't matter we don't
			// do so in case we bail with an error earlier because the caller should
			// not keep polling for items, and in such case the stream would be broken
			// anyway
			self.input_buf.set(input_buf);
		}

		// If we reached EOF, signal EOS once by using poll_close. If an error happens while
		// doing so, propagate it
		if self.reached_eof && !self.signalled_eos {
			eprintln!("EOF, not EOS");

			if let Poll::Ready(result) = Pin::new(&mut self.input_byte_sink).poll_close(cx) {
				eprintln!("EOS signalled");
				self.signalled_eos = true;

				if let Err(err) = result {
					return Poll::Ready(Some(Err(err.into())));
				}
			}
		}

		eprintln!("Polling appsink");
		// We have just handled sending data to GStreamer. Now handle the part
		// client code is more interested in: the output of the pipeline
		match Pin::new(&mut self.output_byte_stream).poll_next(cx) {
			Poll::Ready(Some(sample)) => Poll::Ready(Some(Ok(sample
				.get_buffer_owned()
				.unwrap()
				.into_mapped_buffer_readable()
				.unwrap()))),
			Poll::Ready(None) => {
				eprintln!("appsink ended");

				// The app sink is in EOS state, so it won't ever generate more output.
				// Tear the pipeline down
				self.gstreamer_pipeline.set_state(State::Null).ok();

				Poll::Ready(None)
			}
			Poll::Pending => {
				eprintln!("appsink pending");

				// Look at the pending bus messages for relevant error conditions.
				// This loop is necessary because we need to make sure that the task
				// that this stream belongs to is woken up whenever a new, potentially
				// important message arrives, too. If we just poll the bus once, it may
				// return a message inmediately, and that does not schedule the current
				// task to wake up when any other messages arrive. Such scenario leads to a
				// deadlock if nothing else schedules a wake up either, which may happen
				// when the generation of samples stops because of an error and EOF was reached.
				// Therefore, we poll it until it returns Poll::Pending, because when such
				// statuses are returned the method already scheduled this task to wake up for
				// new messages as needed. This also keeps the number of messages in the internal
				// unbounded channel queue small
				while let Poll::Ready(msg) = Pin::new(&mut self.bus_message_stream).poll_next(cx) {
					eprintln!("Polled message");
					match msg {
						Some(msg) => {
							if let MessageView::Error(err) = msg.view() {
								return Poll::Ready(Some(Err(err.get_error().into())));
							}
						}
						None => {
							eprintln!("Polled no more messages");

							// No more messages will ever be generated. Tear the pipeline down
							self.gstreamer_pipeline.set_state(State::Null).ok();

							return Poll::Ready(None);
						}
					}
				}

				Poll::Pending
			}
		}
	}
}

/// Adapts internal stream structs used by [AudioFile] to a single [Stream]
/// that matches client code expectations.
enum AudioDataStream<T: AsyncRead + Unpin + 'static> {
	Transcoded(ProcessedAudioDataStream<T>),
	PassedThrough(FramedRead<T, PassthroughDecoder>),
	InitError(tokio_stream::Once<<Self as Stream>::Item>)
}

impl<T: AsyncRead + Unpin + 'static> Stream for AudioDataStream<T> {
	type Item = Result<(Cow<'static, str>, OptimizedBytes<ByteBuffer>), OptimizationError>;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		// Pass through to the underlying stream, mapping types accordingly
		match Pin::into_inner(self) {
			AudioDataStream::Transcoded(stream) => Pin::new(stream).poll_next(cx).map(|item| {
				item.map(|result| {
					result.map(|buf| {
						(
							Cow::Borrowed("Transcoded"),
							OptimizedBytes(ByteBuffer::MappedBuffer(buf))
						)
					})
				})
			}),
			AudioDataStream::PassedThrough(stream) => Pin::new(stream).poll_next(cx).map(|item| {
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
			AudioDataStream::InitError(stream) => Pin::new(stream).poll_next(cx)
		}
	}
}

/// Helper enum to allow clients of [AudioFile] consume bytes from different
/// owned representations, which skips costly conversions.
#[derive(Debug)]
enum ByteBuffer {
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

impl<T: AsyncRead + Unpin + 'static>
	ResourcePackFile<ByteBuffer, OptimizationError, AudioDataStream<T>> for AudioFile<T>
{
	fn process(self) -> AudioDataStream<T> {
		// Bail with a passthrough stream if we want to do so
		if self.is_ogg && self.optimization_settings.copy_if_ogg {
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
				// that the method returns "true if GStreamer could be initialized".
				// See: https://github.com/GStreamer/gstreamer/blob/44bdad58f623e50a07476c0f40f8ff7543396f7c/gst/gst.c#L411
				gstreamer::init().unwrap();
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

			appsrc.set_property("size", &self.file_length.try_into().unwrap_or(-1i64))?;
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
					&[("rate", &self.optimization_settings.sampling_frequency)]
				)
			)?;

			encoder.set_property("min-bitrate", &self.optimization_settings.minimum_bitrate)?;
			encoder.set_property("max-bitrate", &self.optimization_settings.maximum_bitrate)?;

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
				let sink_pad = sink_element.get_static_pad("sink").unwrap();

				// Ignore event if the link is already set up
				if sink_pad.is_linked() {
					return;
				}

				if result_audio_channels > 0 {
					// We want to mix to some number of channels, so configure the
					// necessary caps filter (channel mix filter)
					let channel_mix_filter = ElementFactory::make("capsfilter", None).unwrap();
					channel_mix_filter
						.set_property(
							"caps",
							&Caps::new_simple("audio/x-raw", &[("channels", &result_audio_channels)])
						)
						.unwrap();

					// Get the pipeline, which is the parent of any element in the
					// pipeline, and add the new caps filter to it
					sink_element
						.get_parent()
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

			Ok(AudioDataStream::Transcoded(ProcessedAudioDataStream::new(
				self.read,
				gstreamer_pipeline,
				appsrc.downcast_ref::<AppSrc>().unwrap().sink(),
				appsink.downcast_ref::<AppSink>().unwrap().stream()
			)))
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
