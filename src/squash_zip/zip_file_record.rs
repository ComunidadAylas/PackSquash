use std::{
	convert::TryInto,
	io::{Cursor, Error, Write}
};

use enumset::{EnumSet, EnumSetType};

use tokio::io::{self, AsyncWrite};

use super::SquashZipError;

#[cfg(test)]
mod tests;

/// A dummy value for the last modification time and date in the local file header and central
/// directory ZIP file records.
// The DOS date time format used in ZIP files is documented at
// https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-dosdatetimetofiletime.
// The lower two bytes, that map to a DOS time, are set to zero. This means 0 seconds (bits 0-4),
// 0 minutes (bits 5-10) and 0 hours (bits 11-15).
// The upper two bytes map to a DOS date, and are set to day 1 (bits 0-4), month 1 (bits 5-8),
// year 0 + 1980 = 1980 (bits 9-15).
// We set day and month to one because the documentation only seems to
// consider days and months in their 1-31 and 1-12 range. Most DOS date handling
// functions are lenient and performant and accept days and months outside of
// this range, overflowing other date fields, like Wine's, because they just
// use bitwise operations and do not perform any checks. However, a compliant
// program could reject these dates because they're undefined
#[allow(clippy::unusual_byte_groupings)] // Grouped according to fields
pub(super) const DUMMY_SQUASH_TIME: [u8; 4] = ((0b0000000_0001_00001 << 16) as u32).to_le_bytes();

/// The MS-DOS read-only file attribute. Used to signal the intent for the files
/// to not be modified after extraction, although this isn't always honoured.
/// See: https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
const FILE_ATTRIBUTE_READONLY: u32 = 0x1;

/// A ZIP file format feature needed to extract a file in a ZIP file, as defined in
/// section 4.4.3.1 of the ZIP file specification.
#[derive(EnumSetType)]
#[non_exhaustive]
pub(super) enum ZipFeature {
	// It is assumed that these features are in descending version
	// needed to extract order (i.e. highest version needed first).
	// If a new feature is added above the highest one,
	// CentralDirectoryHeader::write must be changed
	Zip64Extensions,
	DeflateCompression,
	BasicFeatures
}

impl ZipFeature {
	/// Converts this ZIP file format feature to the minimum ZIP file specification
	/// needed to extract the affected file.
	const fn to_version_needed_to_extract(self) -> u16 {
		match self {
			ZipFeature::Zip64Extensions => 45,    // 4.5
			ZipFeature::DeflateCompression => 20, // 2.0
			ZipFeature::BasicFeatures => 10       // 1.0
		}
	}
}

/// Returns the ZIP file specification version compliance needed to extract
/// a ZIP file that uses the specified ZIP file format features. This is the
/// highest ZIP file specification version that is needed by any of the
/// features.
fn version_needed_to_extract(zip_features: &EnumSet<ZipFeature>) -> u16 {
	zip_features
		.iter()
		.next() // Feature with highest version needed to extract
		.unwrap_or(ZipFeature::BasicFeatures) // Default to basic feature set
		.to_version_needed_to_extract()
}

/// Returns a value for the "version made by" field that appears in several ZIP file records,
/// taking into account whether it is desired to spoof it or not.
///
/// Spoofing may be desired because the ZIP standard says the compressor should write in this
/// field the maximum ZIP specification version that it supports. However, some programs (i.e.
/// Info-ZIP zip) write their own version here, which is incorrect. Therefore, this field is a
/// somewhat unreliable way of identifying the software that generated the ZIP file. When
/// spoofing is enabled, we mask ourselves as Info-ZIP zip 3.0 for Unix systems (a pretty common
/// command line utility to generate ZIP files), to give an attacker the least information
/// possible.
const fn get_version_made_by(spoof_version_made_by: bool) -> [u8; 2] {
	if spoof_version_made_by {
		[30, 3] // First byte (lower) = "specification version"
	} else {
		ZipFeature::Zip64Extensions
			.to_version_needed_to_extract()
			.to_le_bytes()
	}
}

/// Represents a compression method, as defined in the section 4.4.5 of the
/// ZIP file specification, that may be used to compress the data of files
/// within a ZIP file.
#[derive(Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub(super) enum CompressionMethod {
	Store,
	Deflate
}

impl CompressionMethod {
	/// Gets the compression method field value that represents this compression
	/// method.
	const fn to_compression_method_field(self) -> u16 {
		match self {
			CompressionMethod::Store => 0,
			CompressionMethod::Deflate => 8
		}
	}

	/// Gets the compression method represented by the specified compression method
	/// field value.
	pub(super) const fn from_compression_method_field(
		field: u16
	) -> Result<CompressionMethod, SquashZipError> {
		match field {
			0 => Ok(CompressionMethod::Store),
			8 => Ok(CompressionMethod::Deflate),
			_ => Err(SquashZipError::UnknownCompressionMethod(field))
		}
	}
}

/// Computes the general purpose bit flag for this ZIP file record from the file name
/// it contains, used to specify its UTF-8 encoding.
fn get_general_purpose_bit_flag(file_name: &str) -> u16 {
	// Set Language encoding flag (EFS) at bit 11 to indicate UTF-8 encoded file names
	// only if the file name is not ASCII (i.e. some byte is greater than 127). This allows
	// for maybe improved compressibility in some edge cases and better compatibility
	// with ancient or weird ZIP programs that don't implement this properly
	(!file_name.is_ascii() as u16) << 11
}

/// A ZIP file local file header, defined in section 4.3.7 of the ZIP
/// specification.
pub(super) struct LocalFileHeader<'a> {
	pub(super) compression_method: CompressionMethod,
	pub(super) squash_time: [u8; 4],
	pub(super) crc32: u32,
	pub(super) compressed_size: u32,
	pub(super) uncompressed_size: u32,
	file_name_length: u16,
	file_name: &'a str
}

/// Magic bytes defined in the ZIP specification whose purpose is signalling
/// the beginning of a local file header record.
const LOCAL_FILE_HEADER_SIGNATURE: [u8; 4] = 0x04034B50_u32.to_le_bytes();

impl<'a> LocalFileHeader<'a> {
	/// Creates a new local file header record. This operation may fail if the
	/// file name is too big. The caller must make sure that the following fields
	/// end up being initialized to an appropriate value before writing the header:
	/// - `compression_method` (by default it is STORE)
	/// - `crc32` (by default it is 0)
	/// - `compressed_size` (by default it is 0)
	/// - `uncompressed_size` (by default it is 0)
	/// - `squash_time` (by default it is a dummy value)
	pub fn new(file_name: &'a str) -> Result<Self, SquashZipError> {
		Ok(Self {
			compression_method: CompressionMethod::Store,
			squash_time: DUMMY_SQUASH_TIME,
			crc32: 0,
			compressed_size: 0,
			uncompressed_size: 0,
			file_name_length: file_name.len().try_into()?,
			file_name
		})
	}

	/// Writes this ZIP file record to the specified output ZIP file. For top performance,
	/// it is recommended to use a buffered sink.
	pub async fn write<W: AsyncWrite + Unpin + ?Sized>(
		&self,
		output_zip: &mut W
	) -> Result<(), Error> {
		let mut buf = [0; 30];
		let mut cursor = Cursor::new(&mut buf[..]);

		// Compute the actual set of ZIP features needed to extract with the information we have
		let mut zip_features_needed_to_extract = EnumSet::empty();
		if self.compression_method == CompressionMethod::Deflate {
			zip_features_needed_to_extract |= ZipFeature::DeflateCompression;
		}

		let version_needed_to_extract = version_needed_to_extract(&zip_features_needed_to_extract);
		let general_purpose_bit_flag = get_general_purpose_bit_flag(self.file_name);
		let compression_method = self.compression_method.to_compression_method_field();

		// A 4-byte Squash Time timestamp is stored in the two little-endian two bytes fields
		// that the ZIP file specification reserves for date and time. This way we effectively
		// treat both fields as a single logical four bytes little-endian field.
		// This does not conform to any ZIP file specification, and it is not guaranteed to
		// generate specification-compliant results for all Squash Times. However, some of them,
		// including the dummy one we use, can be interpreted as the specification mandates with
		// no problems.
		// Example:
		// squash_time = 0xAABBCCDD (LE bytes on disk: 0xDDCCBBAA)
		// -> last_mod_time = 0xCCDD (LE bytes on disk: 0xDDCC)
		// -> last_mod_date = 0xAABB (LE bytes on disk: 0xBBAA)
		// Therefore, writing squash_time bytes in LE order is enough to achieve this

		cursor.write_all(&LOCAL_FILE_HEADER_SIGNATURE)?;
		cursor.write_all(&version_needed_to_extract.to_le_bytes())?;
		cursor.write_all(&general_purpose_bit_flag.to_le_bytes())?;
		cursor.write_all(&compression_method.to_le_bytes())?;
		cursor.write_all(&self.squash_time)?;
		cursor.write_all(&self.crc32.to_le_bytes())?;
		cursor.write_all(&self.compressed_size.to_le_bytes())?;
		cursor.write_all(&self.uncompressed_size.to_le_bytes())?;
		cursor.write_all(&self.file_name_length.to_le_bytes())?;
		// We don't add extra fields in the local file header
		cursor.write_all(&0u16.to_le_bytes())?;

		io::AsyncWriteExt::write_all(output_zip, &buf).await?;

		io::AsyncWriteExt::write_all(output_zip, self.file_name.as_bytes()).await
	}

	/// Returns the size that this ZIP file record would take on the file. This
	/// is the same number of bytes that would be written by [`Self::write()`].
	pub const fn size(&self) -> u32 {
		30 + self.file_name_length as u32
	}
}

/// A ZIP file central directory file header, defined in section 4.3.12
/// of the ZIP file specification.
pub(super) struct CentralDirectoryHeader<'a> {
	compression_method: CompressionMethod,
	squash_time: [u8; 4],
	crc32: u32,
	compressed_size: u32,
	uncompressed_size: u32,
	local_header_disk_number: u16,
	local_header_offset: u64,
	file_name: &'a str,
	spoof_version_made_by: bool
}

/// Magic bytes defined in the ZIP specification whose purpose is signalling
/// the beginning of a central directory header record.
const CENTRAL_DIRECTORY_HEADER_SIGNATURE: [u8; 4] = 0x02014B50_u32.to_le_bytes();

impl<'a> CentralDirectoryHeader<'a> {
	/// Creates a new central directory file header record.
	/// # Assumptions
	/// This constructor assumes that the file name is 65535 bytes long or less,
	/// as limited by the ZIP specification. Failure to uphold this assumption
	/// will lead to incorrect results. This should not be a problem because the file
	/// name length should already have been checked previously, while building the
	/// local file header.
	#[allow(clippy::too_many_arguments)]
	pub const fn new(
		file_name: &'a str,
		local_header_offset: u64,
		compression_method: CompressionMethod,
		squash_time: [u8; 4],
		crc32: u32,
		compressed_size: u32,
		uncompressed_size: u32,
		local_header_disk_number: u16,
		spoof_version_made_by: bool
	) -> Self {
		Self {
			compression_method,
			squash_time,
			crc32,
			compressed_size,
			uncompressed_size,
			local_header_disk_number,
			local_header_offset,
			file_name,
			spoof_version_made_by
		}
	}

	/// Returns whether this central directory header record requires ZIP64 extensions
	/// to be stored correctly.
	const fn requires_zip64_extensions(&self) -> bool {
		self.local_header_offset_requires_zip64_extensions()
	}

	/// Checks whether this central directory header record requires ZIP64 extensions
	/// because the local header offset would overflow the 32-bit unsigned integer range.
	const fn local_header_offset_requires_zip64_extensions(&self) -> bool {
		// We use ZIP64 extensions in case the local file header offset can't be stored
		// in 4 bytes
		self.local_header_offset > u32::MAX as u64
	}

	/// Calculates the total length of the extra fields that should be appended to this
	/// central directory header. If extra fields are not needed, this returns zero.
	const fn compute_extra_field_length(&self) -> u16 {
		// Currently, PackSquash only uses the ZIP64 extended information extra field.
		// That extra field length is the result of the following formula:
		// Header size (2 byte ID/tag + 2 byte data size) + data size
		// Where data size = local header offset size (8 bytes)
		4 * self.requires_zip64_extensions() as u16
			+ 8 * self.local_header_offset_requires_zip64_extensions() as u16
	}

	/// Writes this ZIP file record to the specified output ZIP file. For top performance,
	/// it is recommended to use a buffered sink.
	pub async fn write<W: AsyncWrite + Unpin + ?Sized>(
		&self,
		output_zip: &mut W
	) -> Result<(), Error> {
		let mut buf = [0; 46];
		let mut cursor = Cursor::new(&mut buf[..]);

		// We use ZIP64 extensions in case the local file header offset can't be stored
		// in 4 bytes
		let local_header_offset_requires_zip64 = self.local_header_offset_requires_zip64_extensions();
		let zip64_extensions_required = self.requires_zip64_extensions();
		let extra_field_length = self.compute_extra_field_length();

		// Compute the actual set of ZIP features needed to extract with the information we have
		let mut zip_features_needed_to_extract = EnumSet::empty();
		if self.compression_method == CompressionMethod::Deflate {
			zip_features_needed_to_extract |= ZipFeature::DeflateCompression;
		}
		if zip64_extensions_required {
			zip_features_needed_to_extract |= ZipFeature::Zip64Extensions;
		}

		let version_needed_to_extract = version_needed_to_extract(&zip_features_needed_to_extract);

		let general_purpose_bit_flag = get_general_purpose_bit_flag(self.file_name);
		let compression_method = self.compression_method.to_compression_method_field();

		cursor.write_all(&CENTRAL_DIRECTORY_HEADER_SIGNATURE)?;
		cursor.write_all(&get_version_made_by(self.spoof_version_made_by))?;
		// Same operations as local file header
		cursor.write_all(&version_needed_to_extract.to_le_bytes())?;
		cursor.write_all(&general_purpose_bit_flag.to_le_bytes())?;
		cursor.write_all(&compression_method.to_le_bytes())?;
		cursor.write_all(&self.squash_time)?;
		cursor.write_all(&self.crc32.to_le_bytes())?;
		cursor.write_all(&self.compressed_size.to_le_bytes())?;
		cursor.write_all(&self.uncompressed_size.to_le_bytes())?;
		// End of same operations as local file header
		cursor.write_all(&(self.file_name.len() as u16).to_le_bytes())?;
		cursor.write_all(&extra_field_length.to_le_bytes())?;
		// File comment length
		cursor.write_all(&[0; 2])?;
		// Number of the disk where the local file header is
		cursor.write_all(&self.local_header_disk_number.to_le_bytes())?;
		// Internal file attributes (always zero so no sane program will mangle the file with
		// EOL conversion, for example)
		cursor.write_all(&[0; 2])?;
		// External file attributes
		cursor.write_all(&FILE_ATTRIBUTE_READONLY.to_le_bytes())?;
		// Local header offset
		cursor.write_all(
			&if local_header_offset_requires_zip64 {
				u32::MAX
			} else {
				self.local_header_offset as u32
			}
			.to_le_bytes()
		)?;
		io::AsyncWriteExt::write_all(output_zip, &buf).await?;
		io::AsyncWriteExt::write_all(output_zip, self.file_name.as_bytes()).await?;

		// ZIP64 extended information extra field
		if zip64_extensions_required {
			let mut buf = [0; 4];
			let mut cursor = Cursor::new(&mut buf[..]);

			// Extra field tag/ID
			cursor.write_all(&0x0001_u16.to_le_bytes())?;
			// Data size (does not include the 4 byte long header)
			cursor.write_all(&(extra_field_length - 4).to_le_bytes())?;
			io::AsyncWriteExt::write_all(output_zip, &buf).await?;

			if local_header_offset_requires_zip64 {
				io::AsyncWriteExt::write_all(output_zip, &self.local_header_offset.to_le_bytes())
					.await?;
			}
		}

		Ok(())
	}
}

/// A mid-level abstraction for a ZIP file central directory record. When written,
/// depending on the circumstances, this may generate a ZIP64 end of central directory
/// record and locator, in addition to the conventional end of central directory record.
/// These records are defined in sections 4.3.14, 4.3.15 and 4.3.16 of the ZIP file
/// specification.
pub(super) struct EndOfCentralDirectory {
	disk_number: u16,
	central_directory_start_disk_number: u16,
	central_directory_entry_count_current_disk: u64,
	total_central_directory_entry_count: u64,
	central_directory_size: u64,
	central_directory_start_offset: u64,
	total_number_of_disks: u32,
	current_file_offset: u64,
	zip64_record_size_offset: i8,
	spoof_version_made_by: bool,
	zero_out_unused_zip64_fields: bool
}

/// Magic bytes defined in the ZIP specification whose purpose is signalling
/// the beginning of a ZIP64 end of central directory header record.
const ZIP64_END_OF_CENTRAL_DIRECTORY_SIGNATURE: [u8; 4] = 0x06064B50_u32.to_le_bytes();

/// Magic bytes defined in the ZIP specification whose purpose is signalling
/// the beginning of a ZIP64 end of central directory header locator record.
const ZIP64_END_OF_CENTRAL_DIRECTORY_LOCATOR_SIGNATURE: [u8; 4] = 0x07064B50_u32.to_le_bytes();

/// Magic bytes defined in the ZIP specification whose purpose is signalling
/// the beginning of a end of central directory header record.
const END_OF_CENTRAL_DIRECTORY_SIGNATURE: [u8; 4] = 0x06054B50_u32.to_le_bytes();

impl EndOfCentralDirectory {
	/// Creates a end of central directory.
	#[allow(clippy::too_many_arguments)]
	pub const fn new(
		disk_number: u16,
		central_directory_start_disk_number: u16,
		central_directory_entry_count_current_disk: u64,
		total_central_directory_entry_count: u64,
		central_directory_size: u64,
		central_directory_start_offset: u64,
		total_number_of_disks: u32,
		current_file_offset: u64,
		zip64_size_offset: i8,
		spoof_version_made_by: bool,
		zero_out_unused_zip64_fields: bool
	) -> Self {
		Self {
			disk_number,
			central_directory_start_disk_number,
			central_directory_entry_count_current_disk,
			total_central_directory_entry_count,
			central_directory_size,
			central_directory_start_offset,
			total_number_of_disks,
			current_file_offset,
			zip64_record_size_offset: zip64_size_offset,
			spoof_version_made_by,
			zero_out_unused_zip64_fields
		}
	}

	/// Returns whether this end of central directory requires ZIP64 extensions to be
	/// stored correctly.
	const fn requires_zip64_extensions(&self) -> bool {
		self.entry_count_current_disk_requires_zip64_extensions()
			|| self.total_entry_count_requires_zip64_extensions()
			|| self.central_directory_size_requires_zip64_extensions()
			|| self.central_directory_start_offset_requires_zip64_extensions()
	}

	/// Checks whether this end of central directory requires ZIP64 extensions because
	/// the number of entries in the central directory in the current disk exceeds the
	/// 16-bit unsigned integer range.
	const fn entry_count_current_disk_requires_zip64_extensions(&self) -> bool {
		self.central_directory_entry_count_current_disk > u16::MAX as u64
	}

	/// Checks whether this end of central directory requires ZIP64 extensions because
	/// the total number of entries in the central directory exceeds the 16-bit unsigned
	/// integer range.
	const fn total_entry_count_requires_zip64_extensions(&self) -> bool {
		self.total_central_directory_entry_count > u16::MAX as u64
	}

	/// Checks whether this end of central directory requires ZIP64 extensions because
	/// the size of the central directory file headers exceeds the 32-bit unsigned
	/// integer range.
	const fn central_directory_size_requires_zip64_extensions(&self) -> bool {
		self.central_directory_size > u32::MAX as u64
	}

	/// Checks whether this end of central directory requires ZIP64 extensions because
	/// the offset where the first central directory file header is exceeds the 32-bit
	/// unsigned integer range.
	const fn central_directory_start_offset_requires_zip64_extensions(&self) -> bool {
		self.central_directory_start_offset > u32::MAX as u64
	}

	/// Writes this ZIP file record to the specified output ZIP file. For top performance,
	/// it is recommended to use a buffered sink.
	pub async fn write<W: AsyncWrite + Unpin + ?Sized>(
		&self,
		output_zip: &mut W
	) -> Result<(), Error> {
		// If ZIP64 extensions are required, we must generate a ZIP64 end of central directory
		// record, with its corresponding locator
		if self.requires_zip64_extensions() {
			let mut buf = [0; 56 + 20];
			let mut cursor = Cursor::new(&mut buf[..]);

			cursor.write_all(&ZIP64_END_OF_CENTRAL_DIRECTORY_SIGNATURE)?;
			cursor.write_all(&(44 + self.zip64_record_size_offset as i64).to_le_bytes())?;
			cursor.write_all(&get_version_made_by(self.spoof_version_made_by))?;
			// Luckily, ZIP64 is the highest specification version we support, so this is
			// always correct. It also achieves more compressibility if we didn't spoof
			// the made by version
			cursor.write_all(
				&ZipFeature::Zip64Extensions
					.to_version_needed_to_extract()
					.to_le_bytes()
			)?;
			cursor.write_all(
				&(if self.zero_out_unused_zip64_fields {
					0
				} else {
					self.disk_number
				} as u32)
					.to_le_bytes()
			)?;
			cursor.write_all(
				&(if self.zero_out_unused_zip64_fields {
					0
				} else {
					self.central_directory_start_disk_number
				} as u32)
					.to_le_bytes()
			)?;
			cursor.write_all(
				&if self.zero_out_unused_zip64_fields
					&& !self.entry_count_current_disk_requires_zip64_extensions()
				{
					0
				} else {
					self.central_directory_entry_count_current_disk
				}
				.to_le_bytes()
			)?;
			cursor.write_all(
				&if self.zero_out_unused_zip64_fields
					&& !self.total_entry_count_requires_zip64_extensions()
				{
					0
				} else {
					self.total_central_directory_entry_count
				}
				.to_le_bytes()
			)?;
			cursor.write_all(
				&if self.zero_out_unused_zip64_fields
					&& !self.central_directory_size_requires_zip64_extensions()
				{
					0
				} else {
					self.central_directory_size
				}
				.to_le_bytes()
			)?;
			cursor.write_all(
				&if self.zero_out_unused_zip64_fields
					&& !self.central_directory_start_offset_requires_zip64_extensions()
				{
					0
				} else {
					self.central_directory_start_offset
				}
				.to_le_bytes()
			)?;

			// Now go for the ZIP64 EOCD locator, which is always needed
			cursor.write_all(&ZIP64_END_OF_CENTRAL_DIRECTORY_LOCATOR_SIGNATURE)?;
			cursor.write_all(
				&(if self.zero_out_unused_zip64_fields {
					0
				} else {
					self.central_directory_start_disk_number
				} as u32)
					.to_le_bytes()
			)?;
			cursor.write_all(&self.current_file_offset.to_le_bytes())?;
			cursor.write_all(&self.total_number_of_disks.to_le_bytes())?;

			io::AsyncWriteExt::write_all(output_zip, &buf).await?;
		}

		let mut buf = [0; 22];
		let mut cursor = Cursor::new(&mut buf[..]);

		// After the ZIP64 EOCD record, if any, goes the traditional EOCD record. Write it
		cursor.write_all(&END_OF_CENTRAL_DIRECTORY_SIGNATURE)?;
		cursor.write_all(&self.disk_number.to_le_bytes())?;
		cursor.write_all(&self.central_directory_start_disk_number.to_le_bytes())?;
		cursor.write_all(
			&if self.entry_count_current_disk_requires_zip64_extensions() {
				u16::MAX
			} else {
				self.central_directory_entry_count_current_disk as u16
			}
			.to_le_bytes()
		)?;
		cursor.write_all(
			&if self.total_entry_count_requires_zip64_extensions() {
				u16::MAX
			} else {
				self.total_central_directory_entry_count as u16
			}
			.to_le_bytes()
		)?;
		cursor.write_all(
			&if self.central_directory_size_requires_zip64_extensions() {
				u32::MAX
			} else {
				self.central_directory_size as u32
			}
			.to_le_bytes()
		)?;
		cursor.write_all(
			&if self.central_directory_start_offset_requires_zip64_extensions() {
				u32::MAX
			} else {
				self.central_directory_start_offset as u32
			}
			.to_le_bytes()
		)?;
		// No comments (zero comment length)
		cursor.write_all(&[0; 2])?;

		io::AsyncWriteExt::write_all(output_zip, &buf).await
	}
}
