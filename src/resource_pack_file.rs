use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Once};
use std::{
	borrow::Cow,
	cmp,
	error::Error,
	fmt::{Display, Formatter},
	io::Read
};
use std::{convert::TryInto, sync::Mutex};
use std::{fs::File, num::NonZeroI32};

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

use gstreamer::{
	caps::Caps,
	glib::clone::Downgrade,
	prelude::{Cast, ObjectExt},
	ElementExt, ElementExtManual, ElementFactory, GstBinExt, GstBinExtManual, MessageView, PadExt,
	PadExtManual, TagSetterExt
};

use java_properties::{LineEnding, PropertiesIter, PropertiesWriter};

use glsl::parser::Parse;
use glsl::syntax::TranslationUnit;
use glsl::transpiler;

use lazy_static::lazy_static;

lazy_static! {
	static ref DEFAULT_PNG_OPTIMIZATION_SETTINGS: PngOptimizationSettings =
		PngOptimizationSettings::default();
	static ref DEFAULT_AUDIO_TRANSCODING_SETTINGS: AudioTranscodingSettings =
		AudioTranscodingSettings::default();
}

static GSTREAMER_INIT: Once = Once::new();

type ProcessResult<'a> = (Option<Vec<u8>>, Cow<'a, str>);

pub trait ResourcePackFile {
	/// Processes this resource pack file, returning its processed byte contents.
	/// A descriptive string containing the performed action with the file is also
	/// returned in the tuple.
	fn process(&mut self) -> Result<ProcessResult, Box<dyn Error>>;

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
	sampling_frequency: NonZeroI32,
	target_pitch: f32,
	minimum_bitrate: NonZeroI32,
	maximum_bitrate: NonZeroI32
}

impl Default for AudioTranscodingSettings {
	fn default() -> Self {
		Self {
			transcode_ogg: true,
			channels: 0,
			sampling_frequency: NonZeroI32::new(32000).unwrap(),
			target_pitch: 1.0,
			minimum_bitrate: NonZeroI32::new(40000).unwrap(),
			maximum_bitrate: NonZeroI32::new(96000).unwrap()
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

#[derive(Debug)]
pub struct InvalidSettingsForResourcePackFile {}

impl Display for InvalidSettingsForResourcePackFile {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("The type of settings specified for the resource pack file is not appropriate")
	}
}

impl Error for InvalidSettingsForResourcePackFile {}

/// Converts a path to a resource pack file. If the conversion is unsuccessful, because the
/// provided file settings are invalid, an error is returned. If the path is not a resource
/// pack file, no resource pack file is returned successfully. No settings are valid for
/// every resource pack file type.
pub fn path_to_resource_pack_file<'a>(
	path: &Path,
	path_in_root: bool,
	skip_pack_icon: bool,
	allowed_mods: &EnumSet<Mod>,
	file_settings: Option<&'a FileSettings>
) -> Result<Box<dyn ResourcePackFile + 'a>, Box<dyn Error>> {
	let extension = path
		.extension()
		.unwrap_or(&EMPTY_OS_STR)
		.to_str()
		.unwrap()
		.to_lowercase();

	match &extension[..] {
		"json" | "jsonc" | "mcmeta" => Ok(Box::new(JsonFile::new(path, extension)?)),
		"jem" | "jpm" if allowed_mods.contains(Mod::Optifine) => Ok(Box::new(JsonFile::new(path, extension)?)),
		"png" if path_in_root && skip_pack_icon && path.file_name().unwrap_or(&EMPTY_OS_STR) == "pack.png" =>
			// Ignore pack.png if desired, as it is not visible for server resource packs
			Ok(Box::new(SkippedFile {})),
		"png" => match file_settings {
				Some(FileSettings::PngSettings(png_settings)) => Ok(Box::new(PngFile::new(path, extension, Some(png_settings))?)),
				None => Ok(Box::new(PngFile::new(path, extension, None)?)),
				_ => Err(Box::new(InvalidSettingsForResourcePackFile {}))
			},
		"ogg" | "oga" | "mp3" | "flac" | "wav" => match file_settings {
				Some(FileSettings::AudioSettings(audio_settings)) => Ok(Box::new(AudioFile::new(path, extension, Some(audio_settings))?)),
				None => Ok(Box::new(AudioFile::new(path, extension, None)?)),
				_ => Err(Box::new(InvalidSettingsForResourcePackFile {}))
			},
		"fsh" | "vsh" => Ok(Box::new(ShaderFile::new(path, extension)?)),
		"ttf" => Ok(Box::new(PassthroughFile::new(path, "ttf", "Copied, but might be optimized manually (more information: https://stackoverflow.com/questions/2635423/way-to-reduce-size-of-ttf-fonts)", false)?)),
		"bin" => Ok(Box::new(PassthroughFile::new(path, "bin", "Copied", false)?)),
		"properties" if allowed_mods.contains(Mod::Optifine) => Ok(Box::new(PropertiesFile::new(path, extension)?)),
		_ => Ok(Box::new(SkippedFile {}))
	}
}

struct JsonFile<T: Read> {
	data: T,
	extension: String
}

impl JsonFile<BufReader<File>> {
	fn new(file_path: &Path, extension: String) -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			data: BufReader::new(File::open(file_path)?),
			extension
		})
	}
}

impl<T: Read> ResourcePackFile for JsonFile<T> {
	fn process(&mut self) -> Result<ProcessResult, Box<dyn Error>> {
		// Parse the JSON so we know how to serialize it again in a compact manner.
		// Also, pass it through a comment stripper so we ignore comments
		let json_value: Value = serde_json::from_reader(StripComments::new(&mut self.data))?;

		Ok((
			Some(json_value.to_string().into_bytes()),
			Cow::Borrowed("Compacted")
		))
	}

	fn canonical_extension(&self) -> &str {
		match &self.extension[..] {
			// .jsonc extension is converted to .json, because we strip comments
			"jsonc" => "json",
			// Other extensions (e.g. mcmeta) are passed through
			_ => &self.extension
		}
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

struct PngFile<'a, T: Read> {
	data: T,
	data_length: usize,
	extension: String,
	settings: &'a PngOptimizationSettings
}

impl<'a> PngFile<'a, BufReader<File>> {
	fn new(
		file_path: &Path,
		extension: String,
		settings: Option<&'a PngOptimizationSettings>
	) -> Result<Self, Box<dyn Error>> {
		let file = File::open(file_path)?;
		Ok(Self {
			data_length: file
				.metadata()
				.map(|file_metadata| file_metadata.len().try_into().unwrap_or(usize::MAX))?,
			data: BufReader::new(file),
			extension,
			settings: settings.unwrap_or(&DEFAULT_PNG_OPTIMIZATION_SETTINGS)
		})
	}
}

impl<'a, T: Read> ResourcePackFile for PngFile<'a, T> {
	fn process(&mut self) -> Result<ProcessResult, Box<dyn Error>> {
		let mut input_png = {
			let mut buffer = Vec::with_capacity(self.data_length);
			self.data.read_to_end(&mut buffer)?;
			buffer
		};

		let quality_description;
		let mut quality_percent_string;
		if self.settings.quantize_image {
			// Set up the quantization attributes
			let mut quantization_attributes = Attributes::new();
			quantization_attributes.set_max_colors(256);
			quantization_attributes.set_speed(2);
			quantization_attributes.set_quality(0, 100);

			// Read the image to memory
			let image = lodepng::decode32(&input_png)?;

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

				quality_percent_string = quantization_result.quantization_quality().to_string();
				quality_percent_string.push('%');
				quality_description = &quality_percent_string[..];
			} else {
				// Quantization is not needed
				quality_description = "lossless";
			}
		} else {
			// When not performing quantization, we just want to pass bytes through
			quality_description = "lossless";
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
			Some(optimized_png),
			Cow::Owned(format!(
				"PNG optimized with {} quality",
				quality_description
			))
		))
	}

	fn canonical_extension(&self) -> &str {
		// Passthrough
		&self.extension
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

struct AudioFile<'a, T: Read + Send + Sync> {
	data: Arc<Mutex<T>>,
	data_length: usize,
	is_ogg: bool,
	settings: &'a AudioTranscodingSettings
}

impl<'a> AudioFile<'a, BufReader<File>> {
	fn new(
		file_path: &Path,
		extension: String,
		settings: Option<&'a AudioTranscodingSettings>
	) -> Result<Self, Box<dyn Error>> {
		let file = File::open(file_path)?;
		Ok(Self {
			data_length: file
				.metadata()
				.map(|file_metadata| file_metadata.len().try_into().unwrap_or(usize::MAX))?,
			data: Arc::new(Mutex::new(BufReader::new(file))),
			is_ogg: extension == "ogg" || extension == "oga",
			settings: settings.unwrap_or(&DEFAULT_AUDIO_TRANSCODING_SETTINGS)
		})
	}
}

impl<'a, T: Read + Send + Sync + 'static> ResourcePackFile for AudioFile<'a, T> {
	fn process(&mut self) -> Result<ProcessResult, Box<dyn Error>> {
		if !self.is_ogg || self.settings.transcode_ogg {
			// It is not OGG, or we want to transcode OGG anyway. Let the party begin!
			let result_ogg_lock = Arc::new(Mutex::new(Vec::with_capacity(self.data_length / 8)));

			GSTREAMER_INIT.call_once(|| gstreamer::init().unwrap());

			let gstreamer_pipeline = gstreamer::Pipeline::new(None);

			// Create the pipeline elements
			let appsrc = ElementFactory::make("appsrc", None)?;
			let decoder = ElementFactory::make("decodebin", None)?; // Contains a demuxer + decoder
			let converter = ElementFactory::make("audioconvert", None)?;
			let pitch_shifter = ElementFactory::make("pitch", None)?;
			let resampler = ElementFactory::make("audioresample", None)?;
			let resampler_filter = ElementFactory::make("capsfilter", None)?;
			let encoder = ElementFactory::make("vorbisenc", None)?;
			let muxer = ElementFactory::make("oggmux", None)?;
			let appsink = ElementFactory::make("appsink", None)?;

			appsrc.set_property("size", &self.data_length.try_into().unwrap_or(-1i64))?;
			decoder.set_property("expose-all-streams", &false)?;
			decoder.set_property(
				"caps",
				&Caps::new_simple("audio/x-raw", &[]) // Only decode audio streams
			)?;
			// Minecraft lets OpenAL "Frequency Shift by Pitch" implement pitch shifting
			// (see https://www.openal.org/documentation/openal-1.1-specification.pdf).
			// OpenAL achieves the pitch shift by resampling the audio, i.e. changing both
			// tempo and pitch, which introduces the "chipmunk effect". As we want the sound
			// to be played back at its original speed at target_pitch, we shift the pitch
			// to the inverse value. For instance, if the target pitch is 0.5 (half less pitch),
			// we shift the pitch here to 1 / 0.5 = 2 (double the pitch), so the pitch shifts
			// cancel each other out
			pitch_shifter.set_property("pitch", &(1.0 / self.settings.target_pitch))?;
			pitch_shifter.set_property("tempo", &(1.0 / self.settings.target_pitch))?;
			resampler.set_property("quality", &10)?; // Good quality resampling
			resampler_filter.set_property(
				"caps",
				&Caps::new_simple(
					"audio/x-raw",
					&[("rate", &self.settings.sampling_frequency.get())]
				)
			)?;
			encoder.set_property("min-bitrate", &self.settings.minimum_bitrate.get())?;
			encoder.set_property("max-bitrate", &self.settings.maximum_bitrate.get())?;
			appsink.set_property("sync", &false)?; // Output at max speed, not realtime

			// decodebin (demuxer + decoder) needs to be linked later with the next step, because in the
			// beginning it doesn't have a source pad: it acquires it on the fly after probing the input

			// Add and link the constant parts of the pipeline together
			gstreamer_pipeline.add_many(&[
				&appsrc,
				&decoder,
				&converter,
				&pitch_shifter,
				&resampler,
				&resampler_filter,
				&encoder,
				&muxer,
				&appsink
			])?;

			appsrc.link(&decoder)?;
			gstreamer::Element::link_many(&[
				&pitch_shifter,
				&resampler,
				&resampler_filter,
				&encoder,
				&muxer,
				&appsink
			])?;

			// Discard all event-provided tags. As the encoder is
			// not simultaneously a tag reader, and we not explicitly
			// add any tags, the resulting file will have no tags
			encoder
				.downcast_ref::<gstreamer::TagSetter>()
				.ok_or_else(|| SimpleError::new("Couldn't cast the encoder to a tag setter"))?
				.set_tag_merge_mode(gstreamer::TagMergeMode::KeepAll);

			// Handle the demuxer receiving a source pad
			let weak_pitch_shifter_ptr = Downgrade::downgrade(&pitch_shifter);
			let weak_gstreamer_pipeline_ptr = Downgrade::downgrade(&gstreamer_pipeline);
			let result_audio_channels = self.settings.channels;
			decoder.connect_pad_added(move |_, decoder_src_pad| {
				// The decoder has just received a audio source.
				// Get the sink pad of the converter, to connect the decoder source pad to it
				let converter_sink_pad = converter.get_static_pad("sink").unwrap();

				// Ignore event if the link is already set up
				if !converter_sink_pad.is_linked() {
					let pitch_shifter = weak_pitch_shifter_ptr.upgrade().unwrap();

					if result_audio_channels > 0 {
						let gstreamer_pipeline = weak_gstreamer_pipeline_ptr.upgrade().unwrap();

						// We want to mix to some number of channels, so configure the
						// necessary caps filter (channel mix filter)
						let channel_mix_filter = ElementFactory::make("capsfilter", None).unwrap();
						channel_mix_filter
							.set_property(
								"caps",
								&Caps::new_simple(
									"audio/x-raw",
									&[("channels", &result_audio_channels)]
								)
							)
							.unwrap();

						gstreamer_pipeline.add(&channel_mix_filter).unwrap();

						// Link the converter to the channel mix filter and
						// the channel mix filter to the pitch shifter:
						// ... -> converter -> channel_mix_filter -> pitch_shifter -> ...
						converter.link(&channel_mix_filter).unwrap();
						channel_mix_filter.link(&pitch_shifter).unwrap();

						// We also need to set the new element to play, or otherwise
						// the entire pipeline will pause
						channel_mix_filter.sync_state_with_parent().unwrap();
					} else {
						// No channel mixing is desired, so link the converter
						// directly to the pitch shifter:
						// ... -> converter -> pitch_shifter -> ...
						converter.link(&pitch_shifter).unwrap();
					}

					// The decoder received a audio source. Link the recently
					// created decoder source pad with the converter sink pad.
					// This will complete the pipeline, as we both added and
					// linked the rest of elements either statically or
					// dynamically
					decoder_src_pad.link(&converter_sink_pad).unwrap();
				}
			});

			// Handle GStreamer requesting data on the app source by handing it a
			// buffer with the next bytes provided by the source stream
			let data_lock = self.data.clone();
			let buffer_size = cmp::min(cmp::max(64 * 1024, self.data_length / 4), 4 * 1024 * 1024);
			appsrc
				.downcast_ref::<gstreamer_app::AppSrc>()
				.ok_or_else(|| SimpleError::new("Couldn't cast the app source"))?
				.set_callbacks(
					gstreamer_app::AppSrcCallbacks::builder()
						.need_data(move |appsrc, _| {
							// Create the buffer and get a mutable reference to it
							let mut buffer = gstreamer::Buffer::with_size(buffer_size).unwrap();
							let buffer_mut = buffer.get_mut().unwrap();

							// Map the buffer as writable and read to it
							let read_bytes;
							{
								let mut gstreamer_mapped_buffer = buffer_mut.map_writable().unwrap();
								let mut data = data_lock.lock().unwrap();

								read_bytes = data.read(&mut gstreamer_mapped_buffer).unwrap();
							}

							// Truncate it to the actual number of bytes we read
							buffer_mut.set_size(read_bytes);

							if read_bytes > 0 {
								appsrc.push_buffer(buffer).unwrap();
							} else {
								appsrc.end_of_stream().unwrap();
							}
						})
						.build()
				);

			// Handle data being received on the app sink by copying it to the result vector
			let result_ogg_lock_ptr = Arc::downgrade(&result_ogg_lock);
			appsink
				.downcast_ref::<gstreamer_app::AppSink>()
				.ok_or_else(|| SimpleError::new("Couldn't cast the app sink"))?
				.set_callbacks(
					gstreamer_app::AppSinkCallbacks::builder()
						.new_sample(move |sink| {
							// Get the incoming sample (container for audio data)
							let sample = sink.pull_sample().map_err(|_| gstreamer::FlowError::Eos)?;

							// Now get the buffer contained in the sample
							let buffer = sample.get_buffer().ok_or(gstreamer::FlowError::Error)?;

							// Request the buffer with read access
							let mapped_buffer = buffer
								.map_readable()
								.map_err(|_| gstreamer::FlowError::Error)?;

							// Now try to upgrade the weak pointer, that shouldn't be freed yet
							// because that's done when EOS is reached (and this closure is called no more)
							let result_ogg_lock = result_ogg_lock_ptr
								.upgrade()
								.ok_or(gstreamer::FlowError::Error)?;

							// Now acquire the write lock and add the bytes to the resulting OGG file
							result_ogg_lock
								.lock()
								.map_err(|_| gstreamer::FlowError::Error)?
								.extend_from_slice(&mapped_buffer[..]);

							Ok(gstreamer::FlowSuccess::Ok)
						})
						.build()
				);

			gstreamer_pipeline.set_state(gstreamer::State::Playing)?;

			// Handle errors and end of stream
			let bus = gstreamer_pipeline.get_bus().unwrap();
			for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
				match msg.view() {
					MessageView::Eos(..) => break,
					MessageView::Error(err) => {
						gstreamer_pipeline.set_state(gstreamer::State::Null)?;

						return Err(Box::new(SimpleError::new(
							err.get_debug()
								.unwrap_or_else(|| String::from("unknown GStreamer bus error"))
						)));
					}
					_ => ()
				}
			}

			// Clean up state
			gstreamer_pipeline.set_state(gstreamer::State::Null)?;

			// Unwrap ARC as it should have only one strong reference now (ours).
			// Also consume the RwLock
			let result_ogg = Arc::try_unwrap(result_ogg_lock)
				.map_err(|_| SimpleError::new("Couldn't unwrap the ARC"))?
				.into_inner()
				.map_err(|_| SimpleError::new("Couldn't consume RW lock"))?;

			Ok((Some(result_ogg), Cow::Borrowed("Transcoded")))
		} else {
			// The easy case: is OGG and the user does not want us to touch :)

			let mut buffer = Vec::with_capacity(self.data_length);
			self.data
				.lock()
				.map_err(|_| SimpleError::new("Couldn't acquire RW lock for writing"))?
				.read_to_end(&mut buffer)?;

			Ok((Some(buffer), Cow::Borrowed("Copied due to file settings")))
		}
	}

	fn canonical_extension(&self) -> &str {
		"ogg"
	}

	fn is_compressed(&self) -> bool {
		true
	}
}

struct ShaderFile<T: Read> {
	data: T,
	data_length: usize,
	extension: String
}

impl ShaderFile<BufReader<File>> {
	fn new(file_path: &Path, extension: String) -> Result<Self, Box<dyn Error>> {
		let file = File::open(file_path)?;
		Ok(Self {
			data_length: file
				.metadata()
				.map(|file_metadata| file_metadata.len().try_into().unwrap_or(usize::MAX))?,
			data: BufReader::new(file),
			extension
		})
	}
}

impl<T: Read> ResourcePackFile for ShaderFile<T> {
	fn process(&mut self) -> Result<ProcessResult, Box<dyn Error>> {
		let mut buffer = String::with_capacity(self.data_length);

		// Transfer the translation unit source code to the buffer
		self.data.read_to_string(&mut buffer)?;

		// Parse a translation unit struct from the contents of the buffer.
		// Then discard the contents of the buffer, without deallocating memory,
		// to reuse it to show its compacted version
		let translation_unit = TranslationUnit::parse(&mut buffer)?;
		buffer.clear();

		transpiler::glsl::show_translation_unit(&mut buffer, &translation_unit);

		Ok((Some(buffer.into_bytes()), Cow::Borrowed("Compacted")))
	}

	fn canonical_extension(&self) -> &str {
		&self.extension
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

struct PropertiesFile<T: Read> {
	data: T,
	data_length: usize,
	extension: String
}

impl PropertiesFile<BufReader<File>> {
	fn new(file_path: &Path, extension: String) -> Result<Self, Box<dyn Error>> {
		let file = File::open(file_path)?;
		Ok(Self {
			data_length: file
				.metadata()
				.map(|file_metadata| file_metadata.len().try_into().unwrap_or(usize::MAX))?,
			data: BufReader::new(file),
			extension
		})
	}
}

impl<T: Read> ResourcePackFile for PropertiesFile<T> {
	fn process(&mut self) -> Result<ProcessResult, Box<dyn Error>> {
		let mut compacted_properties = Vec::with_capacity(self.data_length);
		let mut compacted_properties_writer = PropertiesWriter::new(&mut compacted_properties);

		// Normalize line endings and separators
		compacted_properties_writer.set_line_ending(LineEnding::LF);
		compacted_properties_writer.set_kv_separator("=")?;

		// Read key-value pairs from the input file, and write them without comments
		// in the tersest way possible
		PropertiesIter::new(&mut self.data).read_into(|key, value| {
			compacted_properties_writer.write(&key, &value).unwrap();
		})?;

		Ok((Some(compacted_properties), Cow::Borrowed("Compacted")))
	}

	fn canonical_extension(&self) -> &str {
		// Passthrough
		&self.extension
	}

	fn is_compressed(&self) -> bool {
		false
	}
}

struct PassthroughFile<'a, T: Read> {
	data: T,
	data_length: usize,
	canonical_extension: &'a str,
	message: &'a str,
	is_compressed: bool
}

impl<'a> PassthroughFile<'a, BufReader<File>> {
	fn new(
		file_path: &Path,
		canonical_extension: &'a str,
		message: &'a str,
		is_compressed: bool
	) -> Result<Self, Box<dyn Error>> {
		let file = File::open(file_path)?;
		Ok(Self {
			data_length: file
				.metadata()
				.map(|file_metadata| file_metadata.len().try_into().unwrap_or(usize::MAX))?,
			data: BufReader::new(file),
			canonical_extension,
			message,
			is_compressed
		})
	}
}

impl<'a, T: Read> ResourcePackFile for PassthroughFile<'a, T> {
	fn process(&mut self) -> Result<ProcessResult, Box<dyn Error>> {
		// Just copy file contents to memory
		let mut buffer = Vec::with_capacity(self.data_length);
		self.data.read_to_end(&mut buffer)?;

		Ok((Some(buffer), Cow::Borrowed(self.message)))
	}

	fn canonical_extension(&self) -> &str {
		&self.canonical_extension
	}

	fn is_compressed(&self) -> bool {
		self.is_compressed
	}
}

struct SkippedFile {}

impl ResourcePackFile for SkippedFile {
	fn process(&mut self) -> Result<ProcessResult, Box<dyn Error>> {
		Ok((None, Cow::Borrowed("Skipped")))
	}

	fn canonical_extension(&self) -> &str {
		""
	}

	fn is_compressed(&self) -> bool {
		true
	}
}
