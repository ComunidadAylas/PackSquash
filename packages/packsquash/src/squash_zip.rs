//! Implements a custom ZIP compressor that is tailored for PackSquash use cases.

use std::{
	borrow::Cow,
	collections::hash_map::Entry,
	io::{self, ErrorKind, Read, Seek, SeekFrom, Write},
	num::{NonZeroU8, TryFromIntError},
	string::FromUtf8Error,
	sync::Mutex,
	time::SystemTime
};

use aes::Aes128;
use ahash::AHashMap;
use once_cell::sync::Lazy;
use packsquash_options::PercentageInteger;
use thiserror::Error;
use tinyvec::{tiny_vec, TinyVec};
use zopfli::Format;

use self::{
	obfuscation_engine::ObfuscationEngine,
	system_time_sanitizer::{SystemTimeSanitizationError, SystemTimeSanitizer},
	zip_file_record::{
		CentralDirectoryHeader, CompressionMethod, EndOfCentralDirectory, LocalFileHeader
	},
	zopfli_iterations_time_model::ZopfliIterationsTimeModel
};
use crate::{
	scratch_file::{ScratchFile, ScratchFilesBudget},
	util::crc32_hashing_read::Crc32HashingRead,
	RelativePath
};

mod obfuscation_engine;
pub mod system_id;
pub mod system_time_sanitizer;
mod zip_file_record;
mod zopfli_iterations_time_model;

#[cfg(test)]
mod tests;

/// The maximum number of Zopfli iterations that SquashZip will do, no matter the
/// input file size. Must be at least 1.
const MAXIMUM_ZOPFLI_ITERATIONS: u8 = 20;

/// Contains information about a file that was processed in a previous
/// run of PackSquash; i.e., already present in a generated ZIP file.
struct PreviousFile {
	/// Time when this file was processed in the previous run.
	squash_time: SystemTime,
	/// The offset to (seek position in the file of) the processed,
	/// compressed data.
	data_offset: u64,
	/// The CRC of the processed data in the previous ZIP file. This field
	/// will be passed through.
	crc32: u32,
	/// The compression method used in the previous ZIP file. This field
	/// will be passed through.
	compression_method: CompressionMethod,
	/// The size of the uncompressed version of the previous ZIP file data.
	/// This field will be passed through.
	uncompressed_size: u32,
	/// The size of the compressed version of the previous ZIP file data.
	/// This field will be passed through and used to copy the file data.
	compressed_size: u32
}

/// A partial central directory header record, which stores the minimal data
/// needed to generate the actual central directory header at some point.
struct PartialCentralDirectoryHeader {
	local_header_offset: u64,
	compression_method: CompressionMethod,
	squash_time: [u8; 4],
	crc32: u32,
	compressed_size: u32,
	uncompressed_size: u32
}

/// Represents a ZIP file hash and size pair.
#[derive(PartialEq, Eq, Hash)]
struct HashAndSize {
	hash: u32,
	size: u32
}

/// Represents an error that may happen during a fallible SquashZip operation.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SquashZipError {
	#[error("Could not read the previous ZIP: {0}")]
	PreviousZipParseError(#[from] PreviousZipParseError),
	#[error("Tried to handle a value that is off limits: {0}. Is a file too big?")]
	Overflow(#[from] TryFromIntError),
	#[error("A file size exceeds the 4 GiB limit")]
	FileTooBig,
	#[error("ZIP file timestamp error: {0}")]
	SystemTimeSanitizationError(#[from] SystemTimeSanitizationError),
	#[error("No such file in the previous ZIP: {0}")]
	NoSuchPreviousFile(String),
	#[error("Tried to add a file with the same path twice: {0}")]
	FileAlreadyAdded(RelativePath<'static>),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

/// Represents an error that may occur while parsing a previously generated ZIP file.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PreviousZipParseError {
	#[error("Invalid data: {0}")]
	Invalid(&'static str),
	#[error("A filename is not valid UTF-8: {0}")]
	InvalidFileName(#[from] FromUtf8Error),
	#[error("Unknown compression method: {0}")]
	UnknownCompressionMethod(u16),
	#[error("ZIP file timestamp error: {0}")]
	SystemTimeSanitizationError(#[from] SystemTimeSanitizationError),
	#[error("I/O error: {0}")]
	Io(#[from] io::Error)
}

/// Contains settings that tweak SquashZip operation.
pub struct SquashZipSettings {
	/// The number of Zopfli compression iterations that will be done on input files of
	/// 1 MiB magnitude, if they are to be deflated by SquashZip. This number is adjusted
	/// to the actual input file magnitude via a combination of a linear regression model
	/// and non-linear file magnitude formula, the objective being to minimize compression
	/// time differences between files of different sizes, by compressing smaller files
	/// more and bigger files less. Zero is treated in a special way, meaning to never
	/// perform any compression.
	pub zopfli_iterations: u8,
	/// Whether Squash Time timestamps will be stored to the output ZIP file or not.
	/// This allows reusing the contents of previously generated ZIP files to skip
	/// processing unchanged files again.
	pub store_squash_time: bool,
	/// Whether to enable ZIP file records obfuscation or not, expressly aimed at
	/// increasing compressibility and/or protection.
	pub enable_obfuscation: bool,
	/// Whether to enable deduplication of identical processed input files or not.
	/// This is a good thing for space savings, but can cause many ZIP file manipulation
	/// programs to choke. It also takes a bit of time to make sure whether two files are
	/// indeed duplicates, because doing so requires comparing their contents, although
	/// these operations are reduced to the minimum possible by comparing the hash and
	/// file size first.
	pub enable_deduplication: bool,
	/// Whether to enable ZIP file obfuscations that may increase file size for extra
	/// protection or not.
	pub enable_size_increasing_obfuscation: bool,
	/// Controls the percentage of ZIP file records that will be stored favoring increased
	/// resistance against some potentially protection-breaking activities vs. increased
	/// compressibility.
	pub percentage_of_records_tuned_for_obfuscation_discretion: PercentageInteger,
	/// Whether obfuscation acceptance quirks that are specific to older Java versions
	/// need to be worked around or not.
	pub workaround_old_java_obfuscation_quirks: bool
}

/// A custom, minimalistic ZIP compressor, which exploits its great control
/// over the low-level details of the ZIP format to make some PackSquash
/// optimizations and use cases possible.
pub struct SquashZip<'settings, 'budget, F: Read + Seek> {
	settings: &'settings SquashZipSettings,
	scratch_files_budget: &'budget ScratchFilesBudget,
	zopfli_iterations_time_model: ZopfliIterationsTimeModel,
	obfuscation_engine: ObfuscationEngine,
	previous_zip_contents: AHashMap<RelativePath<'static>, PreviousFile>,
	state: Mutex<MutableSquashZipState<'budget, F>>
}

/// Holds the mutable state of a [`SquashZip`] struct, allowing to mutate
/// several fields of data by acquiring just a single lock.
struct MutableSquashZipState<'budget, F: Read + Seek> {
	output_zip: ScratchFile<'budget>,
	previous_zip: Option<F>,
	processed_local_headers: AHashMap<HashAndSize, TinyVec<[(u64, u32); 1]>>,
	central_directory_data: AHashMap<RelativePath<'static>, PartialCentralDirectoryHeader>
}

/// The system time sanitizer that SquashZip will use for sanitizing and
/// desanitizing dates to and from ZIP files, respectively.
// TODO can this static be moved to a field in SquashZip?
static SYSTEM_TIME_SANITIZER: Lazy<SystemTimeSanitizer<Aes128>> = Lazy::new(SystemTimeSanitizer::new);

impl<'settings, 'budget, F: Read + Seek> SquashZip<'settings, 'budget, F> {
	/// Creates a new instance of this struct, that may leverage the
	/// results of a ZIP file generated in a previous run to speed up
	/// the process of compressing the current pack.
	///
	/// Any previous ZIP file passed to this method is assumed to have
	/// been generated and/or modified only by SquashZip. This method
	/// does some sanity checks to verify that assumption, but they are
	/// not completely reliable by design. That previous ZIP file is
	/// also assumed to have Squash Time information (i.e. that it was
	/// generated with the `store_squash_time` option set).
	///
	/// Any error that may occur during the creation of the instance,
	/// including (but not limited to) errors related to reading the
	/// previous ZIP file, doesn't have to be fatal. The error
	/// information is returned back, so client code can handle the
	/// situation as it deems fit. For example, it can retry the
	/// operation with new parameters.
	pub fn new(
		mut previous_zip: Option<F>,
		settings: &'settings SquashZipSettings,
		scratch_files_budget: &'budget ScratchFilesBudget
	) -> Result<Self, SquashZipError> {
		let obfuscation_engine = ObfuscationEngine::from_squash_zip_settings(settings);
		let mut output_zip = ScratchFile::new(scratch_files_budget)?;

		let previous_zip_contents = if let Some(previous_zip) = &mut previous_zip {
			read_previous_zip_contents(previous_zip, &obfuscation_engine)?
		} else {
			// No previous contents if no previous file to read their data from
			AHashMap::new()
		};

		obfuscation_engine.obfuscating_header(
			&mut output_zip,
			(previous_zip_contents.len() ^ scratch_files_budget.remaining()) as u64
		)?;

		Ok(Self {
			scratch_files_budget,
			zopfli_iterations_time_model: ZopfliIterationsTimeModel::new(
				settings.zopfli_iterations,
				5.0 / 6.0
			),
			settings,
			obfuscation_engine,
			state: Mutex::new(MutableSquashZipState {
				output_zip,
				previous_zip,
				processed_local_headers: AHashMap::with_capacity(previous_zip_contents.len()),
				central_directory_data: AHashMap::with_capacity(previous_zip_contents.len())
			}),
			previous_zip_contents
		})
	}

	/// Adds a new file to the result ZIP file from its path and a seekable byte source of
	/// its processed contents.
	///
	/// Callers should take into account whether a suitable previous version of
	/// the file, in order to add it more cheaply by calling [`Self::add_previous_file()`].
	/// In that case, it is an error to call both methods for the same file: behavior is
	/// **undefined**.
	///
	/// Adding several files with the same path will cause this function to fail, as doing
	/// so would generate ZIP files that make little sense on a semantic level for no good
	/// reasons.
	///
	/// The result ZIP file may be left in an inconsistent state if this method returns
	/// an error. The caller probably should discard the ZIP file if this happens, by
	/// not calling any further methods on this instance.
	pub fn add_file(
		&self,
		path: &RelativePath<'_>,
		processed_data: impl Read + Seek,
		skip_compression: bool
	) -> Result<(), SquashZipError> {
		let (mut local_file_header, mut compressed_data_scratch_file) =
			self.compress_and_generate_local_header(path, processed_data, skip_compression)?;

		let state = &mut *self.state.lock().unwrap();
		let mut output_zip = &mut state.output_zip;

		let mut empty_vec = tiny_vec!();
		let matching_local_headers = if !self.settings.enable_deduplication {
			// We can't reuse local file headers if deduplication is disabled.
			// Consider that no headers ever match
			&mut empty_vec
		} else {
			state
				.processed_local_headers
				.entry(HashAndSize {
					hash: local_file_header.crc32,
					size: local_file_header.compressed_size
				})
				.or_insert_with(|| tiny_vec!()) // This list will usually be small
		};

		let mut already_stored = false;
		let mut initial_output_zip_stream_offset = None;
		for (matching_header_offset, matching_header_size) in &*matching_local_headers {
			let matching_data_start_offset = matching_header_offset + *matching_header_size as u64;

			compressed_data_scratch_file.rewind()?;

			// Move the output ZIP file cursor to where the matching data starts. If this is our
			// first seek, make sure to note where it was, so we can go back there
			if initial_output_zip_stream_offset.is_none() {
				initial_output_zip_stream_offset = Some(output_zip.stream_position()?);
			}
			output_zip.seek(SeekFrom::Start(matching_data_start_offset))?;

			let matching_output_zip_data = output_zip
				.take(local_file_header.compressed_size as u64)
				.bytes();

			already_stored = are_byte_iterators_equal(
				Read::by_ref(&mut compressed_data_scratch_file).bytes(),
				matching_output_zip_data,
				local_file_header.compressed_size
			)?;

			if already_stored {
				// We know for sure we found a matching file, so just add another pointer to an
				// existing local header in the central directory
				add_partial_central_directory_header(
					path,
					&local_file_header,
					*matching_header_offset,
					&mut state.central_directory_data
				)?;

				// Seek to where the next local header would be
				output_zip.seek(SeekFrom::Start(initial_output_zip_stream_offset.unwrap()))?;

				// Found a match. No point in finding more
				break;
			}
		}

		if !already_stored {
			let new_local_file_header_offset = if let Some(offset) = initial_output_zip_stream_offset
			{
				// Reuse the offset we already have. Make sure we write the new local file header
				// in its expected position
				output_zip.seek(SeekFrom::Start(offset))
			} else {
				// No matches found. There's no seeking around in the output ZIP to undo
				output_zip.stream_position()
			}?;

			add_partial_central_directory_header(
				path,
				&local_file_header,
				new_local_file_header_offset,
				&mut state.central_directory_data
			)?;

			self.obfuscation_engine
				.obfuscate_local_file_header(&mut local_file_header);

			// Avoid allocating memory for the dummy vector
			if self.settings.enable_deduplication {
				matching_local_headers.push((new_local_file_header_offset, local_file_header.size()));
			}

			// Write the local header
			local_file_header.write(&mut output_zip)?;

			// Write the compressed data
			compressed_data_scratch_file.rewind()?;

			match compressed_data_scratch_file {
				MaybeScratchFile::ScratchFile(mut scratch_file) => {
					scratch_file.copy_to(output_zip)?
				}
				MaybeScratchFile::Other(mut inner) => io::copy(&mut inner, output_zip)?
			};
		}

		Ok(())
	}

	/// Returns the time the specified file was added to the ZIP file generated by
	/// SquashZip in a previous run. `None` may be returned if, for instance, the
	/// file didn't exist before, or there is no available data about when this file
	/// was added.
	pub fn file_process_time(&self, file_path: &RelativePath<'_>) -> Option<SystemTime> {
		self.previous_zip_contents
			.get(file_path)
			.map(|previous_file| previous_file.squash_time)
	}

	/// Cheaply adds the specified previous run file to the ZIP file that is being generated
	/// right now. By default, all previous run files are not added again to the output ZIP
	/// file.
	///
	/// It is an error to call both [`Self::add_file()`] and [`Self::add_previous_file()`].
	/// As with [`Self::add_file()`], if this method returns an error, this SquashZip instance
	/// will become poisoned: it's no longer guaranteed that the output ZIP file will be correct.
	///
	/// A [`SquashZipError::NoSuchPreviousFile`] error is returned if the specified file path
	/// was not present in the previous ZIP file. In this case it is guaranteed that no bad
	/// state was introduced in the result output ZIP file, and the instance can still used
	/// normally.
	pub fn add_previous_file(&self, path: &RelativePath<'_>) -> Result<(), SquashZipError> {
		// For this method we implement a simpler version of the algorithm of add_file. It can be
		// summarised as follows:
		// 1. Check if the file is in map 1) (hash, size) -> (LOC offset list).
		//    1.1. It is (there is an entry and a comparison is successful): don't add LOC,
		//         just add CEN inserting to 2) (partial CEN data map).
		//    1.2. It isn't (there is no entry or comparisons are unsuccessful): add LOC,
		//         add new LOC to 1), add CEN entry to 2) and copy previous file data to the
		//         output file.

		let previous_file = if let Some(previous_file) = self.previous_zip_contents.get(path) {
			previous_file
		} else {
			return Err(SquashZipError::NoSuchPreviousFile(String::from(
				path.as_str()
			)));
		};

		// We can sanitize the Squash Time no matter what because we fail early if there was
		// no previous file, and any previous file has Squash Time data
		let sanitized_squash_time = SYSTEM_TIME_SANITIZER.sanitize(
			&previous_file.squash_time,
			&previous_file.crc32.to_le_bytes()
		)?;

		// Reconstruct the local file header this file would have
		let mut local_file_header = LocalFileHeader::new(Cow::Borrowed(path));
		local_file_header.squash_time = sanitized_squash_time;
		local_file_header.crc32 = previous_file.crc32;
		local_file_header.compression_method = previous_file.compression_method;
		local_file_header.uncompressed_size = previous_file.uncompressed_size;
		local_file_header.compressed_size = previous_file.compressed_size;

		let state = &mut *self.state.lock().unwrap();
		let mut output_zip = &mut state.output_zip;

		let mut empty_vec = tiny_vec!();
		let matching_local_headers = if !self.settings.enable_deduplication {
			// We can't reuse local file headers if deduplication is disabled.
			// Consider that no headers ever match
			&mut empty_vec
		} else {
			state
				.processed_local_headers
				.entry(HashAndSize {
					hash: previous_file.crc32,
					size: previous_file.compressed_size
				})
				.or_insert_with(|| tiny_vec!()) // This list will usually be small
		};

		let mut already_stored = false;
		let previous_zip = state.previous_zip.as_mut().unwrap();
		let mut initial_output_zip_stream_offset = None;
		for (matching_header_offset, matching_header_size) in &*matching_local_headers {
			let matching_data_start_offset = matching_header_offset + *matching_header_size as u64;

			// Position the previous ZIP to read the compressed data
			previous_zip.seek(SeekFrom::Start(previous_file.data_offset))?;

			let previous_zip_data = previous_zip
				.take(previous_file.compressed_size as u64)
				.bytes();

			// Move the output ZIP file cursor to where the matching data starts. If this is our
			// first seek, make sure to note where it was, so we can go back there
			if initial_output_zip_stream_offset.is_none() {
				initial_output_zip_stream_offset = Some(output_zip.stream_position()?);
			}
			output_zip.seek(SeekFrom::Start(matching_data_start_offset))?;

			let matching_output_zip_data = output_zip
				.take(previous_file.compressed_size as u64)
				.bytes();

			already_stored = are_byte_iterators_equal(
				previous_zip_data,
				matching_output_zip_data,
				previous_file.compressed_size
			)?;

			if already_stored {
				// We know for sure we found a matching file, so just add another pointer to
				// existing data in the central directory (but with different metadata)
				add_partial_central_directory_header(
					path,
					&local_file_header,
					*matching_header_offset,
					&mut state.central_directory_data
				)?;

				// Seek to where the next local header would be
				output_zip.seek(SeekFrom::Start(initial_output_zip_stream_offset.unwrap()))?;

				// Found a match. No point in finding more
				break;
			}
		}

		if !already_stored {
			let new_local_file_header_offset = if let Some(offset) = initial_output_zip_stream_offset
			{
				// Reuse the offset we already have. Make sure we write the new local file header
				// in its expected position
				output_zip.seek(SeekFrom::Start(offset))
			} else {
				// No matches found. There's no seeking around in the output ZIP to undo
				output_zip.stream_position()
			}?;

			add_partial_central_directory_header(
				path,
				&local_file_header,
				new_local_file_header_offset,
				&mut state.central_directory_data
			)?;

			self.obfuscation_engine
				.obfuscate_local_file_header(&mut local_file_header);

			// Avoid allocating memory for the dummy vector
			if self.settings.enable_deduplication {
				matching_local_headers.push((new_local_file_header_offset, local_file_header.size()));
			}

			// Write the local header
			local_file_header.write(&mut output_zip)?;

			// Write the compressed data
			previous_zip.seek(SeekFrom::Start(previous_file.data_offset))?;

			io::copy(
				&mut previous_zip.take(previous_file.compressed_size as u64),
				output_zip
			)?;
		}

		Ok(())
	}

	/// Finishes this ZIP file, writing any needed remaining data structures and flushing all
	/// the data to the sink returned by the provided function. The function will be called
	/// as late as possible, to minimize the chance of data loss caused by errors.
	///
	/// This operation ends the lifecycle of this SquashZip instance, consuming it, so no
	/// further operations can be done on the ZIP file after this method returns.
	pub fn finish<W: Write>(
		self,
		writer_provider: impl FnOnce() -> Result<W, SquashZipError>
	) -> Result<(), SquashZipError> {
		let state = self.state.into_inner().unwrap();
		let central_directory_data = state.central_directory_data;
		let mut output_zip = state.output_zip;

		let central_directory_entry_count = u64::try_from(central_directory_data.len())?;
		let central_directory_start_offset = output_zip.stream_position()?;

		// First, write the central directory file headers
		for (file_name, header_data) in central_directory_data {
			let mut central_directory_header = CentralDirectoryHeader {
				compression_method: header_data.compression_method,
				squash_time: header_data.squash_time,
				crc32: header_data.crc32,
				compressed_size: header_data.compressed_size,
				uncompressed_size: header_data.uncompressed_size,
				local_header_disk_number: 0,
				local_header_offset: header_data.local_header_offset,
				file_name,
				spoof_version_made_by: false
			};

			self.obfuscation_engine
				.obfuscate_central_directory_header(&mut central_directory_header);

			central_directory_header.write(&mut output_zip)?;
		}

		let central_directory_end_offset = output_zip.stream_position()?;

		// Now write the end of central directory
		let mut end_of_central_directory = EndOfCentralDirectory {
			disk_number: 0,
			central_directory_start_disk_number: 0,
			central_directory_entry_count_current_disk: central_directory_entry_count,
			total_central_directory_entry_count: central_directory_entry_count,
			central_directory_size: central_directory_end_offset - central_directory_start_offset,
			central_directory_start_offset,
			total_number_of_disks: 1,
			current_file_offset: central_directory_end_offset,
			zip64_record_size_offset: 0,
			spoof_version_made_by: false,
			zero_out_unused_zip64_fields: false
		};

		self.obfuscation_engine
			.obfuscate_end_of_central_directory(&mut end_of_central_directory);

		end_of_central_directory.write(&mut output_zip)?;

		// Finally, write the generated ZIP file to its place!
		// This also implicitly flushes any buffer, so any error during flushing will be returned
		output_zip.rewind()?;
		output_zip.copy_to(writer_provider()?)?;

		Ok(())
	}

	/// Compresses a seekable stream of processed data for the given ZIP file path, returning its
	/// corresponding local file header and a scratch data file that contains its most efficient
	/// representation in terms of size. The scratch data file stream position is just after the
	/// compressed contents, so to read the compressed data back client code may need to rewind
	/// the file first.
	fn compress_and_generate_local_header<'path, R: Read + Seek>(
		&self,
		path: &'path RelativePath<'path>,
		mut processed_data: R,
		skip_compression: bool
	) -> Result<(LocalFileHeader<'path>, MaybeScratchFile<'_, R>), SquashZipError> {
		// Get the Squash Time right now, so it is as close as possible to the time when
		// we saw whether it was modified or not, which is a good thing. Instantiate the
		// local file header now so we validate the path as early as possible
		let squash_time = self.settings.store_squash_time.then(SystemTime::now);
		let mut local_file_header = LocalFileHeader::new(Cow::Borrowed(path));

		// Get the processed data size
		let processed_data_size = u32::try_from(processed_data.seek(SeekFrom::End(0))?)
			.map_err(|_| SquashZipError::FileTooBig)?;
		processed_data.rewind()?;

		// Set up our scratch compressed data file
		let mut compressed_data_scratch_file = ScratchFile::with_capacity(
			self.scratch_files_budget,
			processed_data_size.try_into().unwrap_or(0)
		)?;

		// Set up our CRC32 digest and wrapper to update it as bytes are consumed from the source
		let mut crc32_hasher = crc32fast::Hasher::new();
		let mut processed_data_wrapper =
			Crc32HashingRead::new(&mut processed_data, &mut crc32_hasher);

		let mut compressed_data_size;
		if skip_compression || self.settings.zopfli_iterations == 0 || processed_data_size == 0 {
			// Perform no compression and treat uncompressed data as if it was compressed.
			// Because this never saves space, we don't actually get to use compressed_data_scratch_file

			// Compute the CRC32 checksum and advance the stream position anyway
			io::copy(&mut processed_data_wrapper, &mut io::sink())?;

			compressed_data_size = processed_data_size as u64;
		} else {
			zopfli::compress(
				&zopfli::Options {
					iteration_count: NonZeroU8::new(
						self.zopfli_iterations_time_model.iterations_for_data_size(
							processed_data_size,
							1,
							MAXIMUM_ZOPFLI_ITERATIONS
						)
					)
					.unwrap(),
					..Default::default()
				},
				&Format::Deflate,
				&mut processed_data_wrapper,
				&mut compressed_data_scratch_file
			)?;

			compressed_data_size = compressed_data_scratch_file.stream_position()?;
		}

		let processed_data_crc = crc32_hasher.finalize();

		let compression_method;
		let smallest_file;
		if compressed_data_size < processed_data_size as u64 {
			// Storing the compressed data in the ZIP saves space, so use the compressed version
			compression_method = CompressionMethod::Deflate;

			drop(processed_data);

			smallest_file = MaybeScratchFile::ScratchFile(compressed_data_scratch_file);
		} else {
			// Compressed data is equal or bigger in size than uncompressed data.
			// Favor uncompressed data, treating it as compressed
			compression_method = CompressionMethod::Store;
			compressed_data_size = processed_data_size as u64;

			smallest_file = MaybeScratchFile::Other(processed_data);
		}

		// Now populate all the local file header fields
		local_file_header.compression_method = compression_method;
		local_file_header.crc32 = processed_data_crc;
		local_file_header.uncompressed_size = processed_data_size;
		// The cast is always okay because compressed_data_size <= processed_data_size
		local_file_header.compressed_size = compressed_data_size as u32;
		if let Some(squash_time) = squash_time {
			local_file_header.squash_time =
				SYSTEM_TIME_SANITIZER.sanitize(&squash_time, &processed_data_crc.to_le_bytes())?;
		}

		Ok((local_file_header, smallest_file))
	}
}

/// Reads the relevant previous ZIP contents to a map, ready to be used to efficiently process
/// files that were already present in the previous version of the ZIP file.
fn read_previous_zip_contents<F: Read + Seek>(
	mut previous_zip: F,
	obfuscation_engine: &ObfuscationEngine
) -> Result<AHashMap<RelativePath<'static>, PreviousFile>, PreviousZipParseError> {
	let mut buffer = [0u8; 52];
	let record_offset = obfuscation_engine.obfuscating_header_size();

	// ZIP files generated by SquashZip have no comments, and always have
	// their mandatory end of central directory record at the very end. We can't
	// support ZIP files generated by other programs easily and reliably, so this
	// simplification also serves as a weak sanity check
	match previous_zip.seek(SeekFrom::End(-22)) {
		Ok(_) => Ok(()),
		// Handle seeking to an invalid position due to the file being too small
		// with a better message
		Err(err) if err.kind() == ErrorKind::InvalidInput => {
			Err(PreviousZipParseError::Invalid("Too small to be a ZIP file"))
		}
		Err(err) => Err(err.into())
	}?;

	previous_zip.read_exact(&mut buffer[..4])?;
	if buffer[..4] != [0x50, 0x4B, 0x05, 0x06] {
		return Err(PreviousZipParseError::Invalid(
			"EOCD signature not found at the expected position"
		));
	}

	// Now read fields that are relevant to populate our previous ZIP contents map

	// We are just after the end of central directory signature. Read several
	// fields of that record at once for speed: number of this disk, number of disk
	// with start of CD, number of CD entries in this disk, number of total CD entries,
	// CD size, and offset to CD (2 + 2 + 2 + 2 + 4 + 4 = 16 bytes)
	previous_zip.read_exact(&mut buffer[..16])?;

	// This entry count may be incorrect if we either are using ZIP64 extensions
	// or a proper count was not written. We may get a better hint if we check the ZIP64
	// extensions, but doing so opens the possibility of exhausting all the available memory
	// with specially crafted ZIP files, so we don't do that and instead reallocate later if
	// really needed
	let cdh_entry_count_hint = u16::from_le_bytes(buffer[6..8].try_into().unwrap()) as usize;
	let mut central_directory_offset = u32::from_le_bytes(buffer[12..16].try_into().unwrap()) as u64;

	if central_directory_offset == 0xFFFFFFFF {
		// We maybe have a proper offset in a ZIP64 end of central directory
		// record. Use its locator to find it

		// Move to the beginning of the locator. It's just before the end of CD
		previous_zip.seek(SeekFrom::Current(-40))?;

		// Read signature, disk number and offset (4 + 4 + 8 = 16 bytes)
		previous_zip.read_exact(&mut buffer[..16])?;

		// Check locator signature
		if buffer[..4] == [0x50, 0x4B, 0x06, 0x07] {
			// Get where the ZIP64 end of central directory record is
			let zip64_end_of_central_directory_offset =
				u64::from_le_bytes(buffer[8..16].try_into().unwrap());

			// Check its signature
			previous_zip.seek(SeekFrom::Start(
				zip64_end_of_central_directory_offset + record_offset
			))?;
			previous_zip.read_exact(&mut buffer[..52])?;
			if buffer[..4] != [0x50, 0x4B, 0x06, 0x06] {
				return Err(PreviousZipParseError::Invalid(
					"EOCD64 signature expected, but not found"
				));
			}

			// Hooray, we found the proper central directory offset!
			central_directory_offset = u64::from_le_bytes(buffer[44..52].try_into().unwrap());
		} else {
			// This is not an error. The offset may indeed be all-ones, although
			// this is very rare. Continue anyway; if the file is corrupt, we will
			// likely error out later
		}
	}

	// Seek to the offset of the first central directory header
	previous_zip.seek(SeekFrom::Start(central_directory_offset + record_offset))?;

	// Create a map with the most appropriate capacity, given the limitations
	// of the entry count hint
	let mut previous_zip_contents = AHashMap::with_capacity(cdh_entry_count_hint);

	// Keep adding files to the map until there are no more central directory headers
	while {
		previous_zip.read_exact(&mut buffer[..4])?;
		buffer[..4] == [0x50, 0x4B, 0x01, 0x02]
	} {
		// Read all the remaining central directory header fields
		previous_zip.read_exact(&mut buffer[..42])?;

		let file_comment_length = u16::from_le_bytes(buffer[28..30].try_into().unwrap());
		let extra_field_length = u16::from_le_bytes(buffer[26..28].try_into().unwrap());

		// SquashZip never generates comments in ZIP files
		if file_comment_length > 0 {
			return Err(PreviousZipParseError::Invalid(
				"File comment found, but not expected"
			));
		}

		// SquashZip either generates no extra fields or a single ZIP64 data field
		// with a extended local header offset (2 + 2 + 8 = 12 bytes)
		if extra_field_length != 0 && extra_field_length != 12 {
			return Err(PreviousZipParseError::Invalid(
				"Unexpected extra fields size in CDH"
			));
		}

		// Read the fields that will be stored in the map
		let crc = obfuscation_engine
			.deobfuscate_crc32(u32::from_le_bytes(buffer[12..16].try_into().unwrap()));
		let squash_time = SYSTEM_TIME_SANITIZER
			.desanitize(buffer[8..12].try_into().unwrap(), &crc.to_le_bytes())?;
		let compression_method = CompressionMethod::from_compression_method_field(
			u16::from_le_bytes(buffer[6..8].try_into().unwrap())
		)?;
		let compressed_size = u32::from_le_bytes(buffer[16..20].try_into().unwrap());
		let uncompressed_size = u32::from_le_bytes(buffer[20..24].try_into().unwrap());

		// Read the fields that we will use for further parsing
		let file_name_length = u16::from_le_bytes(buffer[24..26].try_into().unwrap()) as usize;
		let mut local_file_header_offset =
			u32::from_le_bytes(buffer[38..42].try_into().unwrap()) as u64;

		// Now get the relative path
		let relative_path = {
			// The filename may not only be larger than our stack-allocated buffer,
			// but we also need a owned string because that buffer is dropped when
			// this function ends
			let mut filename_buf = vec![0; file_name_length];

			previous_zip.read_exact(&mut filename_buf)?;

			// In the unlikely case this relative path is corrupt and/or invalid, but
			// still valid UTF-8, it'll be effectively ignored, so it doesn't really
			// matter
			RelativePath::from_inner(String::from_utf8(filename_buf)?)
		};

		if extra_field_length == 12 && local_file_header_offset == 0xFFFFFFFF {
			// We maybe have a proper local file header offset in a ZIP64 extended information
			// extra field. Read it. It's just after the file name
			previous_zip.read_exact(&mut buffer[..12])?;

			// Check the ZIP64 field tag to make sure it really is the field we're looking for
			if buffer[..2] == [0x01, 0x00] {
				local_file_header_offset = u64::from_le_bytes(buffer[4..12].try_into().unwrap());
			} else {
				// This wouldn't be a format error, as the extra fields are just a list of blocks.
				// However, SquashZip doesn't generate ZIP files with extra fields other than this,
				// so this definitely means that the ZIP file was modified or corrupted
				return Err(PreviousZipParseError::Invalid(
					"Found extra field in CDH that is not a ZIP64 extended information field"
				));
			}
		}

		local_file_header_offset += record_offset;

		// Assume that current offset is where the next central directory header starts.
		// This is true because we have read the extra fields, if any, and there are no
		// comments. If there were extra fields, but we didn't read them, we'll error out
		// when looking for the next central directory header, because the seek position
		// will point to those fields. This is intentional, as that signals a non-SquashZip
		// ZIP file, and we should error out with such a file
		let next_central_directory_header_offset = previous_zip.stream_position()?;

		// Now go to the local file header. We need to parse it a bit to compute the
		// compressed data offset, as the compressed data is after the file name, which
		// has variable length. We can't assume that the local file header filename
		// length is the same as in the central directory because we intentionally
		// can set it to different values due to deduplication and other reasons
		previous_zip.seek(SeekFrom::Start(local_file_header_offset))?;

		// Read all the local file header fields, barring file name and extra fields
		previous_zip.read_exact(&mut buffer[..30])?;

		// Check signature
		if buffer[..4] != [0x50, 0x4B, 0x03, 0x04] {
			return Err(PreviousZipParseError::Invalid(
				"LFH signature not found at expected position"
			));
		}

		let local_header_file_name_length =
			u16::from_le_bytes(buffer[26..28].try_into().unwrap()) as u64;
		let extra_field_length = u16::from_le_bytes(buffer[28..30].try_into().unwrap());

		if extra_field_length > 0 {
			// SquashZip ZIP files never contain extra fields in local file headers
			return Err(PreviousZipParseError::Invalid(
				"Unexpected extra field length in LFH"
			));
		}

		// After all this work, we can finally insert the file data in the map :)
		previous_zip_contents.insert(
			relative_path,
			PreviousFile {
				squash_time,
				data_offset: local_file_header_offset + 30 + local_header_file_name_length,
				crc32: crc,
				compression_method,
				uncompressed_size,
				compressed_size
			}
		);

		// Make sure the seek position points to the next central directory header for the
		// next iteration
		previous_zip.seek(SeekFrom::Start(next_central_directory_header_offset))?;
	}

	Ok(previous_zip_contents)
}

/// Adds a partial central directory header to the specified partial central directory headers map,
/// which is used when finishing up the ZIP file to generate the central directory.
///
/// An error will be returned if the central directory headers map already contains an entry for
/// the specified file path, as in that case the generated ZIP file would be semantically nonsensical
/// (i.e. there's not enough information for a program to know what version of the file to extract,
/// and choosing one over another means that the other just wastes space anyway).
fn add_partial_central_directory_header(
	path: &RelativePath<'_>,
	local_file_header: &LocalFileHeader<'_>,
	local_file_header_offset: u64,
	central_directory_data: &mut AHashMap<RelativePath<'static>, PartialCentralDirectoryHeader>
) -> Result<(), SquashZipError> {
	match central_directory_data.entry(path.as_owned()) {
		Entry::Vacant(entry) => {
			entry.insert(PartialCentralDirectoryHeader {
				local_header_offset: local_file_header_offset,
				compression_method: local_file_header.compression_method,
				squash_time: local_file_header.squash_time,
				crc32: local_file_header.crc32,
				compressed_size: local_file_header.compressed_size,
				uncompressed_size: local_file_header.uncompressed_size
			});

			Ok(())
		}
		Entry::Occupied(entry) => Err(SquashZipError::FileAlreadyAdded(entry.replace_key()))
	}
}

/// Checks whether two fallible byte iterators return the same byte sequence, exhausting them.
/// Any error returned by any of the iterators will be propagated to the caller.
fn are_byte_iterators_equal(
	data: impl Iterator<Item = Result<u8, io::Error>>,
	other: impl Iterator<Item = Result<u8, io::Error>>,
	data_size: u32
) -> Result<bool, SquashZipError> {
	let mut bytes_compared = 0;

	match data.zip(other).find(|(byte, other_byte)| {
		bytes_compared += 1;

		if let (Ok(byte), Ok(other_byte)) = (byte, other_byte) {
			*byte != *other_byte
		} else {
			true
		}
	}) {
		Some((Ok(_), Ok(_))) => {
			// Found a different byte
			Ok(false)
		}
		Some((Err(err), _) | (_, Err(err))) => {
			// An I/O error occurred. Propagate it
			Err(err.into())
		}
		None => {
			// A different byte was not found (i.e., the bytes read from both iterators
			// were equal). Make sure we read the same number of bytes from both, as one
			// may still be shorter than the other
			Ok(bytes_compared == data_size)
		}
	}
}

/// Wraps either a [`ScratchFile`] or an instance of any other type that implements
/// `Read + Seek` to allow accessing either type as instances of the same type.
enum MaybeScratchFile<'budget, R: Read + Seek> {
	ScratchFile(ScratchFile<'budget>),
	Other(R)
}

impl<R: Read + Seek> Read for MaybeScratchFile<'_, R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		match self {
			Self::ScratchFile(scratch_file) => scratch_file.read(buf),
			Self::Other(inner) => inner.read(buf)
		}
	}
}

impl<R: Read + Seek> Seek for MaybeScratchFile<'_, R> {
	fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
		match self {
			Self::ScratchFile(scratch_file) => scratch_file.seek(pos),
			Self::Other(inner) => inner.seek(pos)
		}
	}
}
