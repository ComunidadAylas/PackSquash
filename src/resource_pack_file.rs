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

use serde_json::value::Value;

use gstreamer::caps::Caps;
use gstreamer::prelude::*;
use gstreamer::MessageView;

use lazy_static::lazy_static;

lazy_static! {
	static ref JSON_COMPACTED: String = String::from("JSON compacted");
	static ref OGG_COMPRESSED: String =
		String::from("OGG compressed. Consider removing tags for extra savings");
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

/// Converts a path to a resource pack file object.
pub fn path_to_resource_pack_file(
	path: &PathBuf,
	skip_pack_icon: bool,
	path_in_root: bool
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
			path: path.to_path_buf()
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
		// Parse the JSON so we know how to serialize it again in a compact manner
		let json_value: Value = serde_json::from_reader(BufReader::new(File::open(&self.path)?))?;

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
	path: PathBuf
}

impl ResourcePackFile for OggFile {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		GSTREAMER_INIT.call_once(|| {
			gstreamer::init().unwrap();
		});

		let result_ogg_lock_ptr = Arc::new(RwLock::new(Vec::with_capacity(16 * 1024)));

		let filesrc_element = gstreamer::ElementFactory::make("filesrc", Some("filesrc"))?;
		filesrc_element.set_property("location", &self.path.as_os_str().to_str().unwrap())?;
		// decodebin automatically plugs a demuxer + decoder in the pipeline
		let dec_element = gstreamer::ElementFactory::make("decodebin", None)?;
		let convert_element = gstreamer::ElementFactory::make("audioconvert", None)?;
		let downmix_filter_element = gstreamer::ElementFactory::make("capsfilter", None)?;
		downmix_filter_element.set_property(
			"caps",
			&Caps::new_simple("audio/x-raw", &[("channels", &1)])
		)?;
		let resample_element = gstreamer::ElementFactory::make("audioresample", None)?;
		resample_element.set_property("quality", &10)?;
		let downsample_filter_element = gstreamer::ElementFactory::make("capsfilter", None)?;
		downsample_filter_element.set_property(
			"caps",
			&Caps::new_simple("audio/x-raw", &[("rate", &32000)])
		)?;
		let enc_element = gstreamer::ElementFactory::make("vorbisenc", None)?;
		enc_element.set_property("min-bitrate", &40000)?;
		enc_element.set_property("max-bitrate", &96000)?;
		let mux_element = gstreamer::ElementFactory::make("oggmux", None)?;
		let sink_element = gstreamer::ElementFactory::make("appsink", None)?;
		sink_element.set_property("sync", &false)?; // Output at max speed, not realtime

		let gstreamer_pipeline = gstreamer::Pipeline::new(None);

		gstreamer_pipeline.add_many(&[
			&filesrc_element,
			&dec_element,
			&convert_element,
			&downmix_filter_element,
			&resample_element,
			&downsample_filter_element,
			&enc_element,
			&mux_element,
			&sink_element
		])?;

		filesrc_element.link(&dec_element)?;
		// The demuxer + decoder (decodebin) needs to be linked later with the converter, because in the beginning
		// it doesn't have a source pad: it acquires it on the fly after probing the input
		convert_element.link(&downmix_filter_element)?;
		downmix_filter_element.link(&resample_element)?;
		resample_element.link(&downsample_filter_element)?;
		downsample_filter_element.link(&enc_element)?;
		enc_element.link(&mux_element)?;
		mux_element.link(&sink_element)?;

		// Handle the demuxer receiving a source pad
		let dec_element_weak = dec_element.downgrade();
		dec_element.connect_pad_added(move |_, src_pad| {
			let dec_element = match dec_element_weak.upgrade() {
				Some(element) => element,
				_ => return
			};

			let convert_sink = match convert_element.get_static_pad("sink") {
				Some(sink) => sink,
				None => return
			};

			// Ignore event if the audio converter is already receiving data
			if !convert_sink.is_linked() {
				let src_caps = match src_pad.get_current_caps() {
					Some(caps) => caps,
					None => return
				};

				let src_caps_struct = match src_caps.get_structure(0) {
					Some(caps_struct) => caps_struct,
					None => return
				};

				// Ignore non-audio data
				if src_caps_struct.get_name().starts_with("audio/") {
					// The demuxer received a audio source. Link the recently
					// created demuxer source pad with the converter sink pad to
					// complete the pipeline
					if src_pad.link(&convert_sink).is_err() {
						return;
					}

					if dec_element.link(&convert_element).is_err() {
						return;
					}
				}
			}
		});

		let weak_result_ogg_lock_ptr = Arc::downgrade(&result_ogg_lock_ptr);
		sink_element
			.dynamic_cast::<gstreamer_app::AppSink>()
			.unwrap()
			.set_callbacks(
				gstreamer_app::AppSinkCallbacks::new()
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
