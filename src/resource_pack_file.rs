use std::convert::TryInto;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Once, RwLock};

use super::EMPTY_OS_STR;

use enumset::{EnumSet, EnumSetType};
use serde::{Deserialize, Serialize};

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
use gstreamer::{caps::Caps, ElementFactory};

use java_properties::{LineEnding, PropertiesIter, PropertiesWriter};

use glsl::parser::Parse;
use glsl::syntax::TranslationUnit;
use glsl::transpiler;

use lazy_static::lazy_static;

lazy_static! {
	static ref COMPACTED: String = String::from("Compacted");
	static ref AUDIO_TRANSCODED: String = String::from("Transcoded");
	static ref OGG_COPIED: String = String::from("Copied due to file settings");
	static ref LOSSLESS: String = String::from("lossless");
	static ref DEFAULT_AUDIO_TRANSCODING_SETTINGS: AudioTranscodingSettings =
		AudioTranscodingSettings::default();
	static ref DEFAULT_PNG_OPTIMIZATION_SETTINGS: PngOptimizationSettings =
		PngOptimizationSettings::default();
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

#[derive(EnumSetType, Serialize, Deserialize)]
#[enumset(serialize_deny_unknown, serialize_as_list)]
pub enum Mod {
	#[serde(rename = "OptiFine")]
	Optifine
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FileSettings {
	/// The settings for transcoding audio files to a more space-efficient format that
	/// is accepted by Minecraft.
	AudioSettings(AudioTranscodingSettings),
	/// The settings for optimizing PNG files.
	PngSettings(PngOptimizationSettings)
}

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AudioTranscodingSettings {
	transcode_ogg: bool,
	channels: i32,
	sampling_frequency: i32,
	target_pitch: f32,
	minimum_bitrate: i32,
	maximum_bitrate: i32
}

impl Default for AudioTranscodingSettings {
	fn default() -> Self {
		Self {
			transcode_ogg: true,
			channels: 1,
			sampling_frequency: 32000,
			target_pitch: 1.0,
			minimum_bitrate: 40000,
			maximum_bitrate: 96000
		}
	}
}

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PngOptimizationSettings {
	quantize_image: bool
}

impl Default for PngOptimizationSettings {
	fn default() -> Self {
		Self {
			quantize_image: true
		}
	}
}

/// Converts a path to a resource pack file. If the conversion is unsuccessful, because the
/// provided file settings are invalid, an error is returned. If the path is not a resource
/// pack file, no resource pack file is returned successfully. No settings are valid for
/// every resource pack file type.
pub fn path_to_resource_pack_file<'a>(
	path: &PathBuf,
	path_in_root: bool,
	skip_pack_icon: bool,
	allowed_mods: &EnumSet<Mod>,
	file_settings: Option<&'a FileSettings>
) -> Result<Option<Box<dyn ResourcePackFile + 'a>>, Box<dyn Error>> {
	let extension = path
		.extension()
		.unwrap_or(&EMPTY_OS_STR)
		.to_str()
		.unwrap()
		.to_lowercase();

	if extension == "json" || extension == "jsonc" || extension == "mcmeta" {
		Ok(Some(Box::new(JsonFile {
			path: path.to_path_buf()
		})))
	} else if extension == "png" {
		if path_in_root && skip_pack_icon && path.file_name().unwrap_or(&EMPTY_OS_STR) == "pack.png" {
			// Ignore pack.png if desired, as it is not visible for server resource packs
			Ok(None)
		} else if PngFile::are_file_settings_valid(&file_settings) {
			Ok(Some(Box::new(PngFile {
				path: path.to_path_buf(),
				settings: file_settings
			})))
		} else {
			Err(Box::new(SimpleError::new(
				"The provided settings are not appropriate for PNG files"
			)))
		}
	} else if extension == "ogg"
		|| extension == "oga"
		|| extension == "mp3"
		|| extension == "flac"
		|| extension == "wav"
	{
		if AudioFile::are_file_settings_valid(&file_settings) {
			Ok(Some(Box::new(AudioFile {
				path: path.to_path_buf(),
				is_ogg: extension == "ogg" || extension == "oga",
				settings: file_settings
			})))
		} else {
			Err(Box::new(SimpleError::new(
				"The provided settings are not appropriate for audio files"
			)))
		}
	} else if extension == "fsh" || extension == "vsh" {
		Ok(Some(Box::new(ShaderFile {
			path: path.to_path_buf()
		})))
	} else if extension == "ttf" {
		Ok(Some(Box::new(PassthroughFile {
			path: path.to_path_buf(),
			message: "Copied, but might be optimized manually (more information: https://stackoverflow.com/questions/2635423/way-to-reduce-size-of-ttf-fonts)",
			is_compressed: false
		})))
	} else if extension == "bin" {
		Ok(Some(Box::new(PassthroughFile {
			path: path.to_path_buf(),
			message: "Copied",
			is_compressed: false
		})))
	} else if extension == "properties" && allowed_mods.contains(Mod::Optifine) {
		Ok(Some(Box::new(PropertiesFile {
			path: path.to_path_buf()
		})))
	} else {
		// Unknown file type
		Ok(None)
	}
}

struct JsonFile {
	path: PathBuf
}

impl ResourcePackFile for JsonFile {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		// Parse the JSON so we know how to serialize it again in a compact manner.
		// Also, pass it through a comment stripper so we ignore comments
		let json_value: Value =
			serde_json::from_reader(StripComments::new(BufReader::new(File::open(&self.path)?)))?;

		Ok((json_value.to_string().into_bytes(), COMPACTED.to_string()))
	}

	fn canonical_extension(&self) -> &str {
		let original_extension = self.path.extension().unwrap().to_str().unwrap();

		if original_extension.to_ascii_lowercase() == "jsonc" {
			// .jsonc extension is converted to .json, because we strip comments
			"json"
		} else {
			// Other extensions (e.g. mcmeta) are passed through
			original_extension
		}
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

struct PngFile<'a> {
	path: PathBuf,
	settings: Option<&'a FileSettings>
}

impl<'a> ResourcePackFile for PngFile<'a> {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		let optimization_settings = match &self.settings {
			Some(FileSettings::PngSettings(optimization_settings)) => optimization_settings,
			_ => &DEFAULT_PNG_OPTIMIZATION_SETTINGS
		};

		let input_png;
		let quality_description;
		if optimization_settings.quantize_image {
			// Set up the quantization attributes
			let mut quantization_attributes = Attributes::new();
			quantization_attributes.set_max_colors(256);
			quantization_attributes.set_speed(2);
			quantization_attributes.set_quality(0, 100);

			// Read the image to memory
			let file_bytes = fs::read(&self.path)?;
			let image = lodepng::decode32(&file_bytes)?;

			// Perform quantization only if there are more than 256 pixels
			if image.width > 16 || image.height > 16 {
				let mut image = Image::new(
					&quantization_attributes,
					&image.buffer,
					image.width,
					image.height,
					0.0 // sRGB
				)?;

				// Quantize the image and remap it, so it uses the computed palette
				let mut quantization_result = quantization_attributes.quantize(&image)?;
				quantization_result.set_dithering_level(1.0);
				let (palette, image_bytes) = quantization_result.remapped(&mut image)?;
				let mut encoder = Encoder::new();
				let color_mode = encoder.info_raw_mut();

				// Set the color mode to palette and store the palette
				color_mode.colortype = ColorType::PALETTE;
				color_mode.set_bitdepth(8);
				for color in palette {
					color_mode.palette_add(color)?;
				}

				input_png = encoder.encode(&image_bytes, image.width(), image.height())?;

				let mut quality_string = quantization_result.quantization_quality().to_string();
				quality_string.push('%');
				quality_description = quality_string;
			} else {
				// Quantization is not needed
				input_png = file_bytes;
				quality_description = LOSSLESS.to_string();
			}
		} else {
			// When not performing quantization, we just want to pass bytes through
			input_png = fs::read(&self.path)?;
			quality_description = LOSSLESS.to_string();
		}

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

		// Optimize the PNG with Zopfli compression and lossless transformations
		// that LodePNG and imagequant don't do
		let optimized_png = optimize_from_memory(
			&input_png,
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
			format!("PNG optimized with {} quality", quality_description)
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

impl<'a> PngFile<'a> {
	/// Checks whether the specified settings are valid for this resource pack
	/// file.
	fn are_file_settings_valid(file_settings: &Option<&FileSettings>) -> bool {
		matches!(file_settings, Some(FileSettings::PngSettings(_)) | None)
	}
}

struct AudioFile<'a> {
	path: PathBuf,
	is_ogg: bool,
	settings: Option<&'a FileSettings>
}

impl<'a> ResourcePackFile for AudioFile<'a> {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		let transcoding_settings = match &self.settings {
			Some(FileSettings::AudioSettings(transcoding_settings)) => transcoding_settings,
			_ => &DEFAULT_AUDIO_TRANSCODING_SETTINGS
		};

		if !self.is_ogg || transcoding_settings.transcode_ogg {
			// It is not OGG, or we want to transcode OGG anyway. Let the party begin!
			GSTREAMER_INIT.call_once(|| gstreamer::init().unwrap());

			let result_ogg_lock_ptr = Arc::new(RwLock::new(Vec::with_capacity(16 * 1024)));

			let gstreamer_pipeline = gstreamer::Pipeline::new(None);

			// Create the pipeline
			let filesrc = ElementFactory::make("filesrc", None)?;
			let decoder = ElementFactory::make("decodebin", None)?; // Contains a demuxer + decoder
			let converter = ElementFactory::make("audioconvert", None)?;
			let channel_mix_filter = ElementFactory::make("capsfilter", None)?;
			let pitch_shifter = ElementFactory::make("pitch", None)?;
			let resampler = ElementFactory::make("audioresample", None)?;
			let resampler_filter = ElementFactory::make("capsfilter", None)?;
			let encoder = ElementFactory::make("vorbisenc", None)?;
			let muxer = ElementFactory::make("oggmux", None)?;
			let app_sink = ElementFactory::make("appsink", None)?;

			filesrc.set_property("location", &self.path.to_str().unwrap())?;
			decoder.set_property("expose-all-streams", &false)?;
			decoder.set_property(
				"caps",
				&Caps::new_simple("audio/x-raw", &[]) // Only decode audio streams
			)?;
			channel_mix_filter.set_property(
				"caps",
				&Caps::new_simple(
					"audio/x-raw",
					&[("channels", &transcoding_settings.channels)]
				)
			)?;
			// Minecraft lets OpenAL "Frequency Shift by Pitch" implement pitch shifting
			// (see https://www.openal.org/documentation/openal-1.1-specification.pdf).
			// OpenAL achieves the pitch shift by resampling the audio, i.e. changing both
			// tempo and pitch, which introduces the "chipmunk effect". As we want the sound
			// to be played back at its original speed at target_pitch, we shift the pitch
			// to the inverse value. For instance, if the target pitch is 0.5 (half less pitch),
			// we shift the pitch here to 1 / 0.5 = 2 (double the pitch), so the pitch shifts
			// cancel each other out
			pitch_shifter.set_property("pitch", &(1.0 / transcoding_settings.target_pitch))?;
			pitch_shifter.set_property("tempo", &(1.0 / transcoding_settings.target_pitch))?;
			resampler.set_property("quality", &10)?; // Good quality resampling
			resampler_filter.set_property(
				"caps",
				&Caps::new_simple(
					"audio/x-raw",
					&[("rate", &transcoding_settings.sampling_frequency)]
				)
			)?;
			encoder.set_property("min-bitrate", &transcoding_settings.minimum_bitrate)?;
			encoder.set_property("max-bitrate", &transcoding_settings.maximum_bitrate)?;
			app_sink.set_property("sync", &false)?; // Output at max speed, not realtime

			// decodebin (demuxer + decoder) needs to be linked later with the next step, because in the
			// beginning it doesn't have a source pad: it acquires it on the fly after probing the input

			// Add and link the pipeline together
			gstreamer_pipeline.add_many(&[
				&filesrc,
				&decoder,
				&converter,
				&channel_mix_filter,
				&pitch_shifter,
				&resampler,
				&resampler_filter,
				&encoder,
				&muxer,
				&app_sink
			])?;

			filesrc.link(&decoder)?;
			converter.link(&channel_mix_filter)?;
			channel_mix_filter.link(&pitch_shifter)?;
			pitch_shifter.link(&resampler)?;
			resampler.link(&resampler_filter)?;
			resampler_filter.link(&encoder)?;
			encoder.link(&muxer)?;
			muxer.link(&app_sink)?;

			// Discard all event-provided tags. As the encoder is
			// not simultaneously a tag reader, and we not explicitly
			// add any tags, the resulting file will have no tags
			encoder
				.dynamic_cast::<gstreamer::TagSetter>()
				.unwrap()
				.set_tag_merge_mode(gstreamer::TagMergeMode::KeepAll);

			// Handle the demuxer receiving a source pad
			let dec_element_weak = decoder.downgrade();
			decoder.connect_pad_added(move |_, src_pad| {
				let dec_element = match dec_element_weak.upgrade() {
					Some(element) => element,
					_ => return
				};

				// Get the sink pad of the converter, to connect the decoder to it
				let converter_sink = match converter.get_static_pad("sink") {
					Some(sink) => sink,
					None => return
				};

				// Ignore event if the link is already set up
				if !converter_sink.is_linked() {
					// The demuxer received a audio source. Link the recently
					// created demuxer source pad with the converter sink pad to
					// complete the pipeline
					if src_pad.link(&converter_sink).is_err() {
						return;
					}

					dec_element.link(&converter).unwrap_or(());
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
							let sample =
								sink.pull_sample().map_err(|_| gstreamer::FlowError::Eos)?;
							// Now get the buffer contained in the sample
							let buffer = sample.get_buffer().ok_or(gstreamer::FlowError::Error)?;
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
								Ok(mut guard) => guard.extend_from_slice(&mapped_buffer[..]),
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
			Ok((result_ogg, AUDIO_TRANSCODED.to_string()))
		} else {
			// The easy case: is OGG and the user does not want us to touch :)
			Ok((fs::read(&self.path)?, OGG_COPIED.to_string()))
		}
	}

	fn canonical_extension(&self) -> &str {
		"ogg"
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

impl<'a> AudioFile<'a> {
	/// Checks whether the specified settings are valid for this resource pack
	/// file.
	fn are_file_settings_valid(file_settings: &Option<&FileSettings>) -> bool {
		matches!(file_settings, Some(FileSettings::AudioSettings(_)) | None)
	}
}

struct ShaderFile {
	path: PathBuf
}

impl ResourcePackFile for ShaderFile {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		let estimated_result_size = match fs::metadata(&self.path) {
			Ok(file_meta) => file_meta.len().try_into().unwrap_or(usize::MAX),
			_ => 0
		};

		let mut compacted_shader = String::with_capacity(estimated_result_size);
		let translation_unit = TranslationUnit::parse(fs::read_to_string(&self.path)?)?;
		transpiler::glsl::show_translation_unit(&mut compacted_shader, &translation_unit);

		Ok((compacted_shader.into_bytes(), COMPACTED.to_string()))
	}

	fn canonical_extension(&self) -> &str {
		self.path.extension().unwrap().to_str().unwrap()
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

struct PropertiesFile {
	path: PathBuf
}

impl ResourcePackFile for PropertiesFile {
	fn process(&self) -> Result<(Vec<u8>, String), Box<dyn Error>> {
		let estimated_result_size = match fs::metadata(&self.path) {
			Ok(file_meta) => file_meta.len().try_into().unwrap_or(usize::MAX),
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

		Ok((compacted_properties, COMPACTED.to_string()))
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
