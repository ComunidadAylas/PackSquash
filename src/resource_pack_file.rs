use std::convert::TryInto;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Once, RwLock};

use simple_error::SimpleError;

use imagequant::Attributes;
use imagequant::Image;

use oxipng::{optimize_from_memory, AlphaOptim, Deflaters, Headers, IndexSet, Options};

use lodepng::ffi::ColorType;
use lodepng::Encoder;

use json_comments::StripComments;
use serde_json::value::Value;

use gstreamer::prelude::*;
use gstreamer::MessageView;
use gstreamer::{caps::Caps, Element, ElementFactory};

use java_properties::{LineEnding, PropertiesIter, PropertiesWriter};

use lazy_static::lazy_static;

lazy_static! {
	static ref JSON_COMPACTED: String = String::from("JSON compacted");
	static ref OGG_COMPRESSED: String =
		String::from("OGG compressed. Consider removing tags for extra savings");
	static ref PROPERTIES_COMPACTED: String = String::from("Properties compacted");
}

static GSTREAMER_INIT: Once = Once::new();

pub trait ResourcePackFile {
	/// Processes this resource pack file, returning its processed byte contents.
	/// A descriptive string containing the performed action with the file is also
	/// returned in the tuple.
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>>;

	/// Returns the canonical extension for this resource pack file, to use for
	/// the resulting ZIP file.
	fn canonical_extension(&self) -> &str;

	/// Returns whether this resource pack file contents are already compressed,
	/// and therefore compressing them further seems wasteful in time.
	fn is_compressed(&self) -> bool;
}

pub struct OggEncodingSettings {
	/// The number of audio channels that the resulting file will have.
	/// Mono files take 2 times less space that stereo ones, but for music
	/// stereo channels can be noticeable and desired.
	/// If None, the channels present in the source file are kept
	pub channels: Option<i32>,
	/// The sampling frequency of the resulting file, which determines the
	/// number of audio samples per second that it contains. As per Nyquist-Shannon
	/// theorem, for a given sampling frequency of x Hz only frequencies up to
	/// x / 2 Hz can be recreated without aliasing artifacts. Human speech typically
	/// employs frequencies up to 6 kHz, so a sampling frequency of 12 kHz saves space
	/// with acceptable audio quality. However, other sound types (e.g. music) use
	/// more frequencies in the human audio spectrum, which goes up to 20 kHz.
	/// Therefore, in any case, a frequency greater than 40 kHz (or 44.1 kHz, due to
	/// technical reasons) is wasteful for encoding audio that is going to be heard
	/// by humans and not meant to be edited further.
	/// If none, the source audio is not resampled, and the frequency is kept as-is
	pub sampling_frequency: Option<i32>,
	/// The minimum bps that the OGG encoder will try to use to store audio
	pub minimum_bitrate: i32,
	/// The maximum bps that the OGG encoder will try to use to store audio
	pub maximum_bitrate: i32
}

/// Converts a path to a resource pack file object.
pub fn path_to_resource_pack_file(
	path: &PathBuf,
	skip_pack_icon: bool,
	path_in_root: bool,
	process_mod_files: bool,
	ogg_encoding_settings: OggEncodingSettings
) -> Option<Box<dyn ResourcePackFile>> {
	let empty_os_str = OsStr::new("");
	let extension = path.extension().unwrap_or(empty_os_str);

	if extension == "json" || extension == "mcmeta" {
		Some(Box::new(JsonFile {
			path: path.to_path_buf()
		}))
	} else if extension == "png" {
		if path_in_root && skip_pack_icon && path.file_name().unwrap_or(empty_os_str) == "pack.png" {
			// Ignore pack.png if desired, as it is not visible for server resource packs
			None
		} else {
			Some(Box::new(PngFile {
				path: path.to_path_buf()
			}))
		}
	} else if extension == "ogg" || extension == "oga" || extension == "flac" || extension == "wav" {
		Some(Box::new(OggFile {
			path: path.to_path_buf(),
			encoding_settings: ogg_encoding_settings
		}))
	} else if extension == "ttf" {
		Some(Box::new(PassthroughFile {
			path: path.to_path_buf(),
			message: "Copied, but might be optimized manually. See: https://stackoverflow.com/questions/2635423/way-to-reduce-size-of-ttf-fonts",
			is_compressed: false
		}))
	} else if extension == "fsh" || extension == "vsh" || extension == "bin" {
		Some(Box::new(PassthroughFile {
			path: path.to_path_buf(),
			message: "Copied",
			is_compressed: false
		}))
	} else if extension == "properties" && process_mod_files {
		// These files are used in OptiFine resource packs
		Some(Box::new(PropertiesFile {
			path: path.to_path_buf()
		}))
	} else {
		// Unknown file type
		None
	}
}

struct JsonFile {
	path: PathBuf
}

impl ResourcePackFile for JsonFile {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		// Parse the JSON so we know how to serialize it again in a compact manner.
		// Also, pass it through a comment stripper so we ignore comments
		let json_value: Value = serde_json::from_reader(StripComments::new(
			BufReader::new(File::open(&self.path)?)
		))?;

		Ok((
			json_value.to_string().into_bytes(),
			JSON_COMPACTED.to_string()
		))
	}

	fn canonical_extension(&self) -> &str {
		// Passthrough
		self.path.extension().unwrap().to_str().unwrap()
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

struct PngFile {
	path: PathBuf
}

impl ResourcePackFile for PngFile {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		// Read the image to a memory struct
		let image = lodepng::decode32_file(&self.path)?;
		let image_bytes = image.buffer.as_ref();

		let mut compression_attributes = Attributes::new();
		compression_attributes.set_max_colors(256);
		compression_attributes.set_speed(2);
		compression_attributes.set_quality(0, 100);

		let mut image = Image::new(
			&compression_attributes,
			image_bytes,
			image.width,
			image.height,
			0.0
		)?;

		// Quantize the image and remap it, so it uses the computed palette
		let mut quantization_result = compression_attributes.quantize(&image)?;
		quantization_result.set_dithering_level(1.0);
		let (palette, image_bytes) = quantization_result.remapped(&mut image)?;
		let mut encoder = Encoder::new();
		let color_mode = encoder.info_raw_mut();

		// Store used palette information
		color_mode.colortype = ColorType::PALETTE;
		color_mode.set_bitdepth(8);
		for color in palette {
			color_mode.palette_add(color)?;
		}

		let encoded_png = encoder.encode(&image_bytes, image.width(), image.height())?;

		// Init OxiPNG optimization settings
		let mut alpha_optimizations = IndexSet::with_capacity(6);
		alpha_optimizations.insert(AlphaOptim::Black);
		alpha_optimizations.insert(AlphaOptim::Down);
		alpha_optimizations.insert(AlphaOptim::Left);
		alpha_optimizations.insert(AlphaOptim::Right);
		alpha_optimizations.insert(AlphaOptim::Up);
		alpha_optimizations.insert(AlphaOptim::White);

		let mut optimization_filters = IndexSet::with_capacity(6);
		optimization_filters.insert(0);
		optimization_filters.insert(1);
		optimization_filters.insert(2);
		optimization_filters.insert(3);
		optimization_filters.insert(4);
		optimization_filters.insert(5);

		// Optimize the palette reduced PNG with Zopfli
		// compression and more things that LodePNG and imagequant
		// don't do
		let optimized_png = optimize_from_memory(
			&encoded_png,
			&Options {
				alphas: alpha_optimizations,
				backup: false,
				bit_depth_reduction: true,
				color_type_reduction: true,
				deflate: Deflaters::Zopfli,
				filter: optimization_filters,
				fix_errors: false,
				force: true,
				idat_recoding: true,
				interlace: Some(0), // No interlacing. 1 for Adam7 interlace
				palette_reduction: true,
				preserve_attrs: false,
				pretend: false,
				strip: Headers::All,
				timeout: None,
				use_heuristics: false
			}
		)?;

		Ok((
			optimized_png,
			format!(
				"PNG optimized with {}% quality",
				quantization_result.quantization_quality()
			)
		))
	}

	fn canonical_extension(&self) -> &str {
		// Passthrough
		self.path.extension().unwrap().to_str().unwrap()
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

struct OggFile {
	path: PathBuf,
	encoding_settings: OggEncodingSettings
}

impl ResourcePackFile for OggFile {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		GSTREAMER_INIT.call_once(|| {
			gstreamer::init().unwrap();
		});

		let result_ogg_lock_ptr = Arc::new(RwLock::new(Vec::with_capacity(16 * 1024)));

		let gstreamer_pipeline = gstreamer::Pipeline::new(None);

		// Create the always used parts of the pipeline
		let filesrc = ElementFactory::make("filesrc", None)?;
		let decoder = ElementFactory::make("decodebin", None)?; // Contains a demuxer + decoder
		let encoder = ElementFactory::make("vorbisenc", None)?;
		let muxer = ElementFactory::make("oggmux", None)?;
		let app_sink = ElementFactory::make("appsink", None)?;

		filesrc.set_property("location", &self.path.to_str().unwrap())?;
		decoder.set_property("expose-all-streams", &false)?;
		decoder.set_property(
			"caps",
			&Caps::new_simple("audio/x-raw", &[]) // Only decode audio streams
		)?;
		encoder.set_property("min-bitrate", &self.encoding_settings.minimum_bitrate)?;
		encoder.set_property("max-bitrate", &self.encoding_settings.maximum_bitrate)?;
		app_sink.set_property("sync", &false)?; // Output at max speed, not realtime

		// decodebin (demuxer + decoder) needs to be linked later with the next step, because in the
		// beginning it doesn't have a source pad: it acquires it on the fly after probing the input

		// Add and link the common parts of the pipeline
		gstreamer_pipeline.add_many(&[&filesrc, &decoder, &encoder, &muxer, &app_sink])?;

		filesrc.link(&decoder)?;
		encoder.link(&muxer)?;
		muxer.link(&app_sink)?;

		/// Creates and configures the audio resampler and its corresponding filter element, so they
		/// can be added to the pipeline afterwards.
		fn create_resampler_and_filter<T: ToSendValue>(
			sampling_frequency: &T
		) -> Result<(Element, Element), Box<dyn Error>> {
			let resampler = ElementFactory::make("audioresample", Some("resample"))?;
			let resampler_filter = ElementFactory::make("capsfilter", Some("resample_filter"))?;

			resampler.set_property("quality", &10)?; // Good quality resampling
			resampler_filter.set_property(
				"caps",
				&Caps::new_simple("audio/x-raw", &[("rate", sampling_frequency)])
			)?;

			Ok((resampler, resampler_filter))
		}

		// Now add and link together the variable parts of pipeline
		if let Some(channels) = self.encoding_settings.channels {
			// Create the channel mixing elements
			let converter = ElementFactory::make("audioconvert", Some("channel_mix_convert"))?;
			let channel_mix_filter =
				ElementFactory::make("capsfilter", Some("channel_mix_filter"))?;

			channel_mix_filter.set_property(
				"caps",
				&Caps::new_simple("audio/x-raw", &[("channels", &channels)])
			)?;

			if let Some(sampling_frequency) = self.encoding_settings.sampling_frequency {
				// Now add and link the channel mixing elements with the resampling elements
				let (resampler, resampler_filter) =
					create_resampler_and_filter(&sampling_frequency)?;

				gstreamer_pipeline.add_many(&[
					&converter,
					&channel_mix_filter,
					&resampler,
					&resampler_filter
				])?;

				converter.link(&channel_mix_filter)?;
				channel_mix_filter.link(&resampler)?;
				resampler.link(&resampler_filter)?;
				resampler_filter.link(&encoder)?;
			} else {
				// Just add and link the channel mixing elements
				gstreamer_pipeline.add_many(&[&converter, &channel_mix_filter])?;

				converter.link(&channel_mix_filter)?;
				channel_mix_filter.link(&encoder)?;
			}
		} else if let Some(sampling_frequency) = self.encoding_settings.sampling_frequency {
			// Just add and link the resampling part
			let (resampler, resampler_filter) = create_resampler_and_filter(&sampling_frequency)?;

			resampler.set_property("quality", &10)?; // Resample with the best quality
			resampler_filter.set_property(
				"caps",
				&Caps::new_simple("audio/x-raw", &[("rate", &sampling_frequency)])
			)?;

			gstreamer_pipeline.add_many(&[&resampler, &resampler_filter])?;

			resampler.link(&resampler_filter)?;
			resampler_filter.link(&encoder)?;
		}

		// Handle the demuxer receiving a source pad
		let dec_element_weak = decoder.downgrade();
		let mix_converter = gstreamer_pipeline.get_by_name("channel_mix_convert");
		let resampler = gstreamer_pipeline.get_by_name("resample");
		decoder.connect_pad_added(move |_, src_pad| {
			let dec_element = match dec_element_weak.upgrade() {
				Some(element) => element,
				_ => return
			};

			// Decide the element whose sink to connect to depending on resampling and channel mixing settings
			let element_sink = match &mix_converter {
				Some(element) => element,
				_ => match &resampler {
					Some(element) => element,
					_ => &encoder
				}
			};

			let sink = match element_sink.get_static_pad("sink") {
				Some(sink) => sink,
				None => return
			};

			// Ignore event if the link is already set up
			if !sink.is_linked() {
				// The demuxer received a audio source. Link the recently
				// created demuxer source pad with the converter sink pad to
				// complete the pipeline
				if src_pad.link(&sink).is_err() {
					return;
				}

				if dec_element.link(element_sink).is_err() {
					return;
				}
			}
		});

		let weak_result_ogg_lock_ptr = Arc::downgrade(&result_ogg_lock_ptr);
		app_sink
			.dynamic_cast::<gstreamer_app::AppSink>()
			.unwrap()
			.set_callbacks(
				gstreamer_app::AppSinkCallbacks::builder()
					.new_sample(move |sink| {
						// Get the incoming sample (container for audio data)
						let sample = sink.pull_sample().map_err(|_| gstreamer::FlowError::Eos)?;
						// Now get the buffer contained in the sample
						let buffer = sample
							.get_buffer()
							.ok_or_else(|| gstreamer::FlowError::Error)?;
						// Request the buffer with read access
						let mapped_buffer = buffer
							.map_readable()
							.map_err(|_| gstreamer::FlowError::Error)?;

						// Now try to upgrade the weak pointer, that shouldn't be freed yet
						// because that's done when EOS is reached (and this closure is called no more)
						let result_ogg_lock_ptr = match weak_result_ogg_lock_ptr.upgrade() {
							Some(ptr) => ptr,
							_ => return Err(gstreamer::FlowError::Error)
						};

						// Now get the write lock and add the bytes to the resulting OGG file
						match result_ogg_lock_ptr.write() {
							Ok(mut guard) => guard.extend_from_slice(mapped_buffer.as_slice()),
							_ => return Err(gstreamer::FlowError::Error)
						};

						Ok(gstreamer::FlowSuccess::Ok)
					})
					.build()
			);

		gstreamer_pipeline.set_state(gstreamer::State::Playing)?;

		// Handle errors and end of stream
		let bus = gstreamer_pipeline.get_bus().unwrap();
		for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
			let message_view = msg.view();

			if let MessageView::Eos(..) = message_view {
				break;
			} else if let MessageView::Error(err) = message_view {
				gstreamer_pipeline.set_state(gstreamer::State::Null)?;

				return Err(Box::new(SimpleError::new(
					err.get_debug().unwrap_or_else(|| String::from("unknown"))
				)));
			}
		}

		// Clean up state before returning
		gstreamer_pipeline.set_state(gstreamer::State::Null)?;

		// Unwrap ARC as it should have only one strong reference now (ours).
		// Also consume the RwLock
		let result_ogg = match Arc::try_unwrap(result_ogg_lock_ptr) {
			Ok(result_ogg_lock) => match result_ogg_lock.into_inner() {
				Ok(result) => result,
				_ => return Err(Box::new(SimpleError::new("Couldn't consume RW lock")))
			},
			_ => return Err(Box::new(SimpleError::new("Couldn't unwrap the ARC")))
		};
		Ok((result_ogg, OGG_COMPRESSED.to_string()))
	}

	fn canonical_extension(&self) -> &str {
		"ogg"
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

struct PropertiesFile {
	path: PathBuf
}

impl ResourcePackFile for PropertiesFile {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		let estimated_result_size = match fs::metadata(&self.path) {
			Ok(file_meta) => file_meta.len().try_into().unwrap_or(0),
			_ => 0
		};

		let mut compacted_properties = Vec::with_capacity(estimated_result_size);
		let mut compacted_properties_writer = PropertiesWriter::new(&mut compacted_properties);

		// Normalize line endings and separators
		compacted_properties_writer.set_line_ending(LineEnding::LF);
		compacted_properties_writer.set_kv_separator("=")?;

		// Read key-value pairs from the input file, and write them without comments
		// in the tersest way possible
		PropertiesIter::new(BufReader::new(File::open(&self.path)?)).read_into(|key, value| {
			compacted_properties_writer.write(&key, &value).unwrap();
		})?;

		Ok((compacted_properties, PROPERTIES_COMPACTED.to_string()))
	}

	fn canonical_extension(&self) -> &str {
		// Passthrough
		self.path.extension().unwrap().to_str().unwrap()
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

struct PassthroughFile<'a> {
	path: PathBuf,
	message: &'a str,
	is_compressed: bool
}

impl<'a> ResourcePackFile for PassthroughFile<'a> {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		// Just copy file contents to memory
		Ok((fs::read(&self.path)?, String::from(self.message)))
	}

	fn canonical_extension(&self) -> &str {
		// Passthrough
		self.path.extension().unwrap().to_str().unwrap()
	}

	fn is_compressed(&self) -> bool {
		self.is_compressed
	}
}
