use std::collections::HashSet;
use std::error::Error;
use std::io::{Seek, SeekFrom, Write};
use std::ops::DerefMut;
use std::path::PathBuf;

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;

use tempfile::SpooledTempFile;

use simple_error::SimpleError;

use crc32fast::Hasher;

use pbr::ProgressBar;

/// Compression methods supported by Minecraft in a ZIP file.
enum ZipCompressionMethod {
	DEFLATE,
	STORE
}

impl ZipCompressionMethod {
	fn to_bitstream_field(&self) -> u16 {
		match self {
			ZipCompressionMethod::DEFLATE => 8,
			ZipCompressionMethod::STORE => 0
		}
	}

	/// Compresses the given data, optionally returning its CRC too.
	fn compress(&self, data: &[u8], calculate_crc: bool) -> (Vec<u8>, Option<u32>) {
		// Consider a 95% compression ratio for initial buffer sizing.
		// That should not be wasteful, but improve reallocation performance
		let mut vec_buffer = Vec::with_capacity(data.len() / 20);
		let calculated_crc;

		// Calculate the CRC if needed
		if calculate_crc {
			let mut crc_hasher = Hasher::new();
			crc_hasher.update(data);
			calculated_crc = Some(crc_hasher.finalize())
		} else {
			calculated_crc = None;
		}

		match self {
			ZipCompressionMethod::DEFLATE => {
				// Compress the data with Zopfli
				zopfli::compress(
					&zopfli::Options::default(),
					&zopfli::Format::Deflate,
					data,
					&mut vec_buffer
				)
				.unwrap();
			}
			ZipCompressionMethod::STORE => {
				// Passthrough original data
				vec_buffer.extend_from_slice(data);
			}
		};

		(vec_buffer, calculated_crc)
	}

	/// Returns the best compression algorithm for the given data, its compressed version
	/// and its CRC.
	fn best_compress(data: &[u8]) -> (ZipCompressionMethod, Vec<u8>, u32) {
		let (deflated_data, uncompressed_data_crc) =
			match ZipCompressionMethod::DEFLATE.compress(data, true) {
				(compressed_data, Some(crc)) => (compressed_data, crc),
				(_, None) => panic!("Assertion failed")
			};

		let stored_data = match ZipCompressionMethod::STORE.compress(data, false) {
			(passthrough_data, None) => passthrough_data,
			_ => panic!("Assertion failed")
		};

		if deflated_data.len() <= (stored_data.len() / 50) * 49 {
			// Consider DEFLATE better if its savings are better than 2% (~20 KiB per MiB)
			(
				ZipCompressionMethod::DEFLATE,
				deflated_data,
				uncompressed_data_crc
			)
		} else {
			(
				ZipCompressionMethod::STORE,
				stored_data,
				uncompressed_data_crc
			)
		}
	}
}

pub enum ZipFileType {
	RegularFile //Folder
}

impl ZipFileType {
	fn to_ms_dos_attributes(&self) -> u8 {
		match self {
			ZipFileType::RegularFile => 0x01 // FILE_ATTRIBUTE_READONLY
			//ZipFileType::Folder => 0x10    // FILE_ATTRIBUTE_DIRECTORY (16)
		}
	}
}

trait ZipStructure {
	/// Writes this ZIP file structure bytes to the specified writer.
	fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>>;
}

/// Contains the central directory header data that cannot be
/// computed when adding the central directory at the end of
/// the file.
struct PartialCentralDirectoryFileHeader {
	compression_method: u16,
	crc_32: u32,
	compressed_size: u32,
	uncompressed_size: u32,
	file_name: Arc<String>,
	external_file_attributes: u32,
	local_header_offset: u32
}

struct CentralDirectoryFileHeader {
	signature: u32,
	version_made_by: u16,
	version_needed_to_extract: u16,
	general_purpose_bit_flag: u16,
	compression_method: u16,
	last_mod_time: u16,
	last_mod_date: u16,
	crc_32: u32,
	compressed_size: u32,
	uncompressed_size: u32,
	file_name_length: u16,
	extra_field_length: u16,
	file_comment_length: u16,
	disk_number_start: u16,
	internal_file_attributes: u16,
	external_file_attributes: u32,
	local_header_offset: u32,
	file_name: Vec<u8>
}

impl Default for CentralDirectoryFileHeader {
	fn default() -> CentralDirectoryFileHeader {
		CentralDirectoryFileHeader {
			signature: 0x02014b50,         // Magic number from the spec
			version_made_by: 20,           // MS-DOS OS, 2.0 ZIP spec compliance
			version_needed_to_extract: 20, // MS-DOS OS. Zip spec version may be higher than actually needed
			general_purpose_bit_flag: 0,   // Irrelevant metadata, always 0
			last_mod_time: 0,              // Useless metadata, set to 0
			last_mod_date: 0,              // Useless metadata, set to 0
			extra_field_length: 0,         // No extra fields
			file_comment_length: 0,        // No comments
			disk_number_start: 0,          // No multi-disk support
			internal_file_attributes: 0,   // Binary data, no variable record length control field
			// These fields are to be used by this application
			compression_method: 0xFFFF,
			crc_32: 0xFFFFFFFF,
			compressed_size: 0xFFFFFFFF,
			uncompressed_size: 0xFFFFFFFF,
			file_name_length: 0xFFFF,
			external_file_attributes: 0xFFFFFFFF,
			local_header_offset: 0xFFFFFFFF,
			file_name: Vec::new()
		}
	}
}

impl ZipStructure for CentralDirectoryFileHeader {
	fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
		// Write fields in the expected order and with the needed
		// endianness. Surprisingly, this kind of simple serialization
		// is not covered by crates like bincode (they add extra cruft)
		writer.write_all(&self.signature.to_le_bytes())?;
		writer.write_all(&self.version_made_by.to_le_bytes())?;
		writer.write_all(&self.version_needed_to_extract.to_le_bytes())?;
		writer.write_all(&self.general_purpose_bit_flag.to_le_bytes())?;
		writer.write_all(&self.compression_method.to_le_bytes())?;
		writer.write_all(&self.last_mod_time.to_le_bytes())?;
		writer.write_all(&self.last_mod_date.to_le_bytes())?;
		writer.write_all(&self.crc_32.to_le_bytes())?;
		writer.write_all(&self.compressed_size.to_le_bytes())?;
		writer.write_all(&self.uncompressed_size.to_le_bytes())?;
		writer.write_all(&self.file_name_length.to_le_bytes())?;
		writer.write_all(&self.extra_field_length.to_le_bytes())?;
		writer.write_all(&self.file_comment_length.to_le_bytes())?;
		writer.write_all(&self.disk_number_start.to_le_bytes())?;
		writer.write_all(&self.internal_file_attributes.to_le_bytes())?;
		writer.write_all(&self.external_file_attributes.to_le_bytes())?;
		writer.write_all(&self.local_header_offset.to_le_bytes())?;
		writer.write_all(&self.file_name)?;

		Ok(())
	}
}

// The fields of this struct should match the ones in the central directory
// (all but file data, as it is not present in the central directory)
struct LocalFileHeader<'a> {
	signature: u32,
	version_needed_to_extract: u16,
	general_purpose_bit_flag: u16,
	compression_method: u16,
	last_mod_time: u16,
	last_mod_date: u16,
	crc_32: u32,
	compressed_size: u32,
	uncompressed_size: u32,
	file_name_length: u16,
	extra_field_length: u16,
	file_name: &'a [u8],
	file_data: &'a [u8]
}

impl<'a> Default for LocalFileHeader<'a> {
	fn default() -> LocalFileHeader<'a> {
		LocalFileHeader {
			signature: 0x04034b50, // Magic number from the spec
			version_needed_to_extract: 20,
			general_purpose_bit_flag: 0,
			last_mod_time: 0,
			last_mod_date: 0,
			extra_field_length: 0,
			// These fields are to be used by this application
			compression_method: 0xFFFF,
			crc_32: 0xFFFFFFFF,
			compressed_size: 0xFFFFFFFF,
			uncompressed_size: 0xFFFFFFFF,
			file_name_length: 0xFFFF,
			file_name: &[],
			file_data: &[]
		}
	}
}

impl<'a> ZipStructure for LocalFileHeader<'a> {
	fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
		writer.write_all(&self.signature.to_le_bytes())?;
		writer.write_all(&self.version_needed_to_extract.to_le_bytes())?;
		writer.write_all(&self.general_purpose_bit_flag.to_le_bytes())?;
		writer.write_all(&self.compression_method.to_le_bytes())?;
		writer.write_all(&self.last_mod_time.to_le_bytes())?;
		writer.write_all(&self.last_mod_date.to_le_bytes())?;
		writer.write_all(&self.crc_32.to_le_bytes())?;
		writer.write_all(&self.compressed_size.to_le_bytes())?;
		writer.write_all(&self.uncompressed_size.to_le_bytes())?;
		writer.write_all(&self.file_name_length.to_le_bytes())?;
		writer.write_all(&self.extra_field_length.to_le_bytes())?;
		writer.write_all(&self.file_name)?;
		writer.write_all(&self.file_data)?;

		Ok(())
	}
}

struct CentralDirectoryEndRecord {
	signature: u32,
	disk_number: u16,
	central_directory_start_disk: u16,
	central_directory_entries_in_this_disk: u16,
	central_directory_entries_total: u16,
	central_directory_size: u32,
	central_directory_offset: u32,
	comment_length: u16
}

impl Default for CentralDirectoryEndRecord {
	fn default() -> CentralDirectoryEndRecord {
		CentralDirectoryEndRecord {
			signature: 0x06054b50,           // Magic number from the spec
			disk_number: 0,                  // No multi-disk support
			central_directory_start_disk: 0, // No multi-disk support
			comment_length: 0,               // No comments
			// These fields are to be used by this application
			central_directory_entries_in_this_disk: 0xFFFF,
			central_directory_entries_total: 0xFFFF,
			central_directory_size: 0xFFFFFFFF,
			central_directory_offset: 0xFFFFFFFF
		}
	}
}

impl ZipStructure for CentralDirectoryEndRecord {
	fn write<W: Write>(&self, writer: &mut W) -> Result<(), Box<dyn Error>> {
		writer.write_all(&self.signature.to_le_bytes())?;
		writer.write_all(&self.disk_number.to_le_bytes())?;
		writer.write_all(&self.central_directory_start_disk.to_le_bytes())?;
		writer.write_all(&self.central_directory_entries_in_this_disk.to_le_bytes())?;
		writer.write_all(&self.central_directory_entries_total.to_le_bytes())?;
		writer.write_all(&self.central_directory_size.to_le_bytes())?;
		writer.write_all(&self.central_directory_offset.to_le_bytes())?;
		writer.write_all(&self.comment_length.to_le_bytes())?;

		Ok(())
	}
}

/// A custom, minimalistic and flexible ZIP compressor, whose output
/// is optimized for size.
pub struct MicroZip {
	temp_out_file: RwLock<SpooledTempFile>,
	stored_file_paths: RwLock<HashSet<Arc<String>>>,
	partial_central_directory_entries: RwLock<Vec<PartialCentralDirectoryFileHeader>>,
	obfuscate: bool,
	writing_central_directory: AtomicBool
}

impl MicroZip {
	pub fn new(file_count_estimate: usize, use_zip_obfuscation: bool) -> MicroZip {
		MicroZip {
			temp_out_file: RwLock::new(SpooledTempFile::new(64 * 1024 * 1024)), // 64 MiB
			stored_file_paths: RwLock::new(HashSet::with_capacity(file_count_estimate)),
			partial_central_directory_entries: RwLock::new(Vec::with_capacity(file_count_estimate)),
			obfuscate: use_zip_obfuscation,
			writing_central_directory: AtomicBool::new(false)
		}
	}

	/// Adds a file to the to-be-finished ZIP file represented by this struct,
	/// returning an error if something went wrong during the operation.
	pub fn add_file<W: Write>(
		&self,
		relativized_path: &PathBuf,
		file_type: ZipFileType,
		data: &[u8],
		skip_compression: bool,
		progress: &mut ProgressBar<W>
	) -> Result<(), Box<dyn Error>> {
		if self.writing_central_directory.load(Ordering::SeqCst) {
			return Err(Box::new(SimpleError::new(
				"Can't add a file after writing the central directory"
			)));
		}

		if data.len() > 0xFFFFFFFF {
			return Err(Box::new(SimpleError::new("The data is too big for ZIP32")));
		}

		let path_str = Arc::new(MicroZip::relativized_path_to_string(relativized_path)?);
		let path_str_arc = path_str.clone();

		if path_str.len() > 0xFFFF {
			return Err(Box::new(SimpleError::new("The path is too long")));
		}

		// Compute stored status in a subscope, so the check and insert is atomic
		let already_stored;
		{
			let mut stored_file_paths_guard = match self.stored_file_paths.write() {
				Ok(guard) => guard,
				_ => {
					return Err(Box::new(SimpleError::new(
						"An error occurred while acquiring a RW lock for adding a ZIP file"
					)))
				}
			};

			already_stored = stored_file_paths_guard.contains(&path_str);

			if !already_stored {
				stored_file_paths_guard.insert(path_str);
			}
		}

		// Only proceed if the file is not already stored
		if !already_stored {
			// If it is a folder, make sure we add its parent folders are also included
			// Get the best compression for this file
			let (zip_compression_method, compressed_data, crc) = match file_type {
				ZipFileType::RegularFile => {
					if skip_compression {
						match ZipCompressionMethod::STORE.compress(data, true) {
							(compressed_data, Some(crc)) => {
								(ZipCompressionMethod::STORE, compressed_data, crc)
							}
							_ => return Err(Box::new(
								SimpleError::new("The contract of compress was violated"))
							)
						}
					} else {
						ZipCompressionMethod::best_compress(data)
					}
				} //ZipFileType::Folder => (ZipCompressionMethod::STORE, Vec::new(), 0)
			};
			let zip_compression_method_field = zip_compression_method.to_bitstream_field();

			if compressed_data.len() > 0xFFFFFFFF {
				return Err(Box::new(SimpleError::new(
					"The compressed data is too big for ZIP32"
				)));
			}

			progress.tick();

			// Generate the local file header
			let actual_path_str = path_str_arc.as_ref().as_bytes();
			let local_file_header = LocalFileHeader {
				compression_method: zip_compression_method_field,
				crc_32: crc,
				compressed_size: compressed_data.len() as u32
					& ((!self.obfuscate as u32) * 0xFFFFFFFF),
				uncompressed_size: data.len() as u32 & ((!self.obfuscate as u32) * 0xFFFFFFFF),
				file_name_length: path_str_arc.len() as u16 & ((!self.obfuscate as u16) * 0xFFFF),
				file_name: if !self.obfuscate {
					actual_path_str
				} else {
					&[]
				},
				file_data: &compressed_data[..],
				..Default::default()
			};

			let local_file_header_offset;
			{
				// Get the offset where the header would be
				let mut temp_out_file = match self.temp_out_file.write() {
					Ok(guard) => guard,
					_ => {
						return Err(Box::new(SimpleError::new(
							"An error occurred while acquiring a RW lock for adding a ZIP file"
						)))
					}
				};

				local_file_header_offset = temp_out_file.seek(SeekFrom::Current(0))?;
				if local_file_header_offset > 0xFFFFFFFF {
					return Err(Box::new(SimpleError::new(
						"The ZIP32 file can't take more than 4 GiB"
					)));
				}

				// Append it to the result ZIP file
				local_file_header.write(temp_out_file.by_ref())?;
			}

			// Also store the necessary information to generate the central directory entry later
			let mut partial_central_directory_entries =
				match self.partial_central_directory_entries.write() {
					Ok(guard) => guard,
					_ => {
						return Err(Box::new(SimpleError::new(
							"An error occurred while acquiring a RW lock for adding a ZIP file"
						)))
					}
				};

			progress.tick();

			partial_central_directory_entries.push(PartialCentralDirectoryFileHeader {
				compression_method: zip_compression_method_field,
				crc_32: crc,
				compressed_size: compressed_data.len() as u32,
				uncompressed_size: data.len() as u32,
				file_name: path_str_arc,
				external_file_attributes: file_type.to_ms_dos_attributes() as u32,
				local_header_offset: local_file_header_offset as u32
			});
		}

		Ok(())
	}

	/// Appends the central directory and writes the complete ZIP file to the specified
	/// destination. add_file would fail after this function returns.
	/// It is assumed that this function is called when no thread is executing add_file.
	pub fn finish_and_write<D: Write + Seek>(&self, dst: &mut D) -> Result<(), Box<dyn Error>> {
		// Swap false writing flag for true. If it was false, proceed
		if !self
			.writing_central_directory
			.compare_and_swap(false, true, Ordering::SeqCst)
		{
			let mut temp_out_file =
				match self.temp_out_file.write() {
					Ok(guard) => guard,
					_ => return Err(Box::new(SimpleError::new(
						"An error occurred while acquiring a RW lock for finishing the ZIP file"
					)))
				};

			let partial_central_directory_entries =
				match self.partial_central_directory_entries.read() {
					Ok(guard) => guard,
					_ => return Err(Box::new(SimpleError::new(
						"An error occurred while acquiring a RW lock for finishing the ZIP file"
					)))
				};

			// First, rewind the temporary output file and write to the destination all its contents
			// (local file headers)
			temp_out_file.seek(SeekFrom::Start(0))?;
			std::io::copy(temp_out_file.deref_mut(), dst)?;

			// Write each central directory entry
			let central_file_header_start = dst.seek(SeekFrom::Current(0))?;
			for partial_directory_entry in &*partial_central_directory_entries {
				let mut file_name_vec = Vec::new();
				file_name_vec.extend_from_slice(partial_directory_entry.file_name.as_bytes());

				CentralDirectoryFileHeader {
					compression_method: partial_directory_entry.compression_method,
					crc_32: partial_directory_entry.crc_32,
					compressed_size: partial_directory_entry.compressed_size,
					uncompressed_size: partial_directory_entry.uncompressed_size,
					file_name_length: file_name_vec.len() as u16,
					external_file_attributes: partial_directory_entry.external_file_attributes,
					local_header_offset: partial_directory_entry.local_header_offset,
					file_name: file_name_vec,
					..Default::default()
				}
				.write(dst)?;
			}

			// Write the central directory end record
			let central_file_header_end = dst.seek(SeekFrom::Current(0))?;
			CentralDirectoryEndRecord {
				central_directory_entries_in_this_disk: partial_central_directory_entries.len()
					as u16,
				central_directory_entries_total: partial_central_directory_entries.len() as u16,
				central_directory_size: (central_file_header_end - central_file_header_start)
					as u32,
				central_directory_offset: central_file_header_start as u32,
				..Default::default()
			}
			.write(dst)?;
		}

		Ok(())
	}

	/// Converts a relativized path to a string, ready to be used in
	/// ZIP files structures. Non-ASCII characters are forbidden.
	fn relativized_path_to_string(relativized_path: &PathBuf) -> Result<String, Box<dyn Error>> {
		let mut result_string = String::new();

		for component in relativized_path.components() {
			result_string.push_str(match component.as_os_str().to_str() {
				Some(string) => {
					if string.is_ascii() {
						string
					} else {
						return Err(Box::new(SimpleError::new(
							"The path contains non-ASCII characters"
						)));
					}
				}
				None => {
					return Err(Box::new(SimpleError::new(
						"The path contains invalid UTF-8 codepoints"
					)))
				}
			});
			result_string.push_str("/");
		}

		// Remove trailing dash
		result_string.pop();

		Ok(result_string)
	}
}
