//! Contains the ZIP file record objects that compose a ZIP file.

use std::{
	borrow::Cow,
	cmp,
	io::{self, Error, IoSlice, SeekFrom}
};

use enumset::{EnumSet, EnumSetType};

use memchr::memmem;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncSeekExt, AsyncWrite};

use crate::RelativePath;

use self::util::AsyncWriteAllVectoredExt;
use super::{PreviousZipParseError, zip_archive_comment_string::ZipArchiveCommentString};

mod util;

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
const DUMMY_SQUASH_TIME: [u8; 4] = ((0b0000000_0001_00001 << 16) as u32).to_le_bytes();

/// The MS-DOS read-only file attribute. Used to signal the intent for the files
/// to not be modified after extraction, although this isn't always honoured.
/// See: <https://docs.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants>
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
		.next() // Feature with the highest version needed to extract
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
	) -> Result<CompressionMethod, PreviousZipParseError> {
		match field {
			0 => Ok(CompressionMethod::Store),
			8 => Ok(CompressionMethod::Deflate),
			_ => Err(PreviousZipParseError::UnknownCompressionMethod(field))
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
	pub compression_method: CompressionMethod,
	pub squash_time: [u8; 4],
	pub crc32: u32,
	pub compressed_size: u32,
	pub uncompressed_size: u32,
	pub zero_out_version_needed_to_extract: bool,
	pub file_name: Cow<'a, RelativePath<'a>>
}

impl<'a> LocalFileHeader<'a> {
	/// Magic bytes defined in the ZIP specification whose purpose is signalling
	/// the beginning of a local file header record.
	pub(super) const SIGNATURE: [u8; 4] = 0x04_03_4B_50_u32.to_le_bytes();

	/// Creates a new local file header record. The caller must make sure that the
	/// following fields end up being initialized to an appropriate value before
	/// writing the header:
	/// - `compression_method` (by default it is STORE)
	/// - `crc32` (by default it is 0)
	/// - `compressed_size` (by default it is 0)
	/// - `uncompressed_size` (by default it is 0)
	/// - `squash_time` (by default it is a dummy value)
	/// - `zero_out_version_needed_to_extract` (by default is `false`)
	pub fn new(file_name: impl Into<Cow<'a, RelativePath<'a>>>) -> Self {
		let file_name = file_name.into();

		Self {
			compression_method: CompressionMethod::Store,
			squash_time: DUMMY_SQUASH_TIME,
			crc32: 0,
			compressed_size: 0,
			uncompressed_size: 0,
			zero_out_version_needed_to_extract: false,
			file_name
		}
	}

	/// Writes this ZIP file record to the specified output ZIP file. For top performance,
	/// it is recommended to use a sink with efficient vectored writes.
	pub async fn write(
		&self,
		mut output_zip: &mut (impl AsyncWrite + Unpin + ?Sized)
	) -> Result<(), Error> {
		let version_needed_to_extract = if !self.zero_out_version_needed_to_extract {
			// Compute the actual set of ZIP features needed to extract with the information we have
			let mut zip_features_needed_to_extract = EnumSet::empty();

			if self.compression_method == CompressionMethod::Deflate {
				zip_features_needed_to_extract |= ZipFeature::DeflateCompression;
			}

			version_needed_to_extract(&zip_features_needed_to_extract)
		} else {
			0
		};
		let general_purpose_bit_flag = get_general_purpose_bit_flag(self.file_name.as_str());
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

		output_zip
			.write_all_vectored(&mut [
				IoSlice::new(&Self::SIGNATURE),
				IoSlice::new(&version_needed_to_extract.to_le_bytes()),
				IoSlice::new(&general_purpose_bit_flag.to_le_bytes()),
				IoSlice::new(&compression_method.to_le_bytes()),
				IoSlice::new(&self.squash_time),
				IoSlice::new(&self.crc32.to_le_bytes()),
				IoSlice::new(&self.compressed_size.to_le_bytes()),
				IoSlice::new(&self.uncompressed_size.to_le_bytes()),
				IoSlice::new(&(self.file_name.as_str().len() as u16).to_le_bytes()),
				// We don't add extra fields in the local file header
				IoSlice::new(&0u16.to_le_bytes()),
				IoSlice::new(self.file_name.as_str().as_bytes())
			])
			.await
	}

	/// Returns the size that this ZIP file record would take on the file. This
	/// is the same number of bytes that would be written by [`Self::write()`].
	pub fn size(&self) -> u32 {
		30 + self.file_name.as_str().len() as u32
	}
}

/// A ZIP file central directory file header, defined in section 4.3.12
/// of the ZIP file specification.
pub(super) struct CentralDirectoryHeader<'a> {
	pub compression_method: CompressionMethod,
	pub squash_time: [u8; 4],
	pub crc32: u32,
	pub compressed_size: u32,
	pub uncompressed_size: u32,
	pub local_header_disk_number: u16,
	pub local_header_offset: u64,
	pub file_name: RelativePath<'a>,
	pub spoof_version_made_by: bool
}

impl CentralDirectoryHeader<'_> {
	/// Magic bytes defined in the ZIP specification whose purpose is signalling
	/// the beginning of a central directory header record.
	pub(super) const SIGNATURE: [u8; 4] = 0x02_01_4B_50_u32.to_le_bytes();

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
	/// it is recommended to use a sink with efficient vectored writes.
	pub async fn write(
		&self,
		mut output_zip: &mut (impl AsyncWrite + Unpin + ?Sized)
	) -> Result<(), Error> {
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

		let general_purpose_bit_flag = get_general_purpose_bit_flag(self.file_name.as_str());
		let compression_method = self.compression_method.to_compression_method_field();

		output_zip
			.write_all_vectored(&mut [
				IoSlice::new(&Self::SIGNATURE),
				IoSlice::new(&get_version_made_by(self.spoof_version_made_by)),
				// Same operations as local file header
				IoSlice::new(&version_needed_to_extract.to_le_bytes()),
				IoSlice::new(&general_purpose_bit_flag.to_le_bytes()),
				IoSlice::new(&compression_method.to_le_bytes()),
				IoSlice::new(&self.squash_time),
				IoSlice::new(&self.crc32.to_le_bytes()),
				IoSlice::new(&self.compressed_size.to_le_bytes()),
				IoSlice::new(&self.uncompressed_size.to_le_bytes()),
				// End of same operations as local file header
				IoSlice::new(&(self.file_name.as_str().len() as u16).to_le_bytes()),
				IoSlice::new(&extra_field_length.to_le_bytes()),
				IoSlice::new(&[0; 2]), // File comment length
				IoSlice::new(&self.local_header_disk_number.to_le_bytes()),
				// Internal file attributes (always zero so no sane program will mangle the file with
				// EOL conversion, for example)
				IoSlice::new(&[0; 2]),
				IoSlice::new(&FILE_ATTRIBUTE_READONLY.to_le_bytes()), // External file attributes
				IoSlice::new(
					&if local_header_offset_requires_zip64 {
						u32::MAX
					} else {
						self.local_header_offset as u32
					}
					.to_le_bytes()
				),
				IoSlice::new(self.file_name.as_str().as_bytes())
			])
			.await?;

		// ZIP64 extended information extra field
		if zip64_extensions_required {
			let local_header_offset_bytes = self.local_header_offset.to_le_bytes();

			output_zip
				.write_all_vectored(&mut [
					IoSlice::new(&0x0001_u16.to_le_bytes()), // Extra field tag/ID
					// Data size (does not include the 4 byte long header)
					IoSlice::new(&(extra_field_length - 4).to_le_bytes()),
					IoSlice::new(if local_header_offset_requires_zip64 {
						&local_header_offset_bytes
					} else {
						&[]
					})
				])
				.await?;
		}

		Ok(())
	}
}

/// A mid-level abstraction for a ZIP file central directory record. When written,
/// depending on the circumstances, this may generate a ZIP64 end of central directory
/// record and locator, in addition to the conventional end of central directory record.
/// These records are defined in sections 4.3.14, 4.3.15 and 4.3.16 of the ZIP file
/// specification.
pub(super) struct EndOfCentralDirectory<'a> {
	pub disk_number: u16,
	pub central_directory_start_disk_number: u16,
	pub central_directory_entry_count_current_disk: u64,
	pub total_central_directory_entry_count: u64,
	pub central_directory_size: u64,
	pub central_directory_start_offset: u64,
	pub total_number_of_disks: u32,
	pub current_file_offset: u64,
	pub zip64_record_size_offset: i8,
	pub spoof_version_made_by: bool,
	pub zero_out_unused_zip64_fields: bool,
	pub archive_comment: ZipArchiveCommentString<'a>
}

impl EndOfCentralDirectory<'_> {
	/// Magic bytes defined in the ZIP specification whose purpose is signalling
	/// the beginning of a ZIP64 end of central directory header record.
	pub(super) const ZIP64_SIGNATURE: [u8; 4] = 0x06_06_4B_50_u32.to_le_bytes();

	/// Magic bytes defined in the ZIP specification whose purpose is signalling
	/// the beginning of a ZIP64 end of central directory header locator record.
	pub(super) const ZIP64_LOCATOR_SIGNATURE: [u8; 4] = 0x07_06_4B_50_u32.to_le_bytes();

	/// Magic bytes defined in the ZIP specification whose purpose is signalling
	/// the beginning of an end of central directory header record.
	pub(super) const SIGNATURE: [u8; 4] = 0x06_05_4B_50_u32.to_le_bytes();

	/// The maximum size that an end of central directory record can have in a
	/// ZIP file, in bytes, without taking into account any ZIP64 extensions.
	const MAX_SIZE: u32 = 22 + u16::MAX as u32;

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

	/// Returns the file position of the last (and only) end of central directory record
	/// in the provided ZIP file, if any, assuming that there is no trailing data after
	/// such record. Such position is relative to the beginning of the file, and points
	/// to the first byte of the record's signature.
	///
	/// The algorithm implemented by this method is highly optimized for the very common
	/// case where the end of central directory record is located near the end of the
	/// file, and a single file read call can fill a scratch buffer large enough to hold
	/// the region of the file that may hold the record. In this case, the algorithm can
	/// find the record with a single file read operation and, due to searching for
	/// signatures in reverse, a few comparisons with the bytes at the end of the file.
	///
	/// Should the case occur where the end of central directory record is located far from
	/// the end of the file, or file read calls yield small amounts of data at a time, the
	/// algorithm will still be asympotically efficient due to the usage of [two-way searching]
	/// for searching the end of central directory record signature, but the overheads will
	/// increase due to the additional read calls and/or the reverse search order doing more
	/// and more expensive comparisons. No heap allocations are made in either case.
	///
	/// [two-way searching]: https://en.wikipedia.org/wiki/Two-way_string-matching_algorithm
	pub async fn locate(
		mut zip_file: impl AsyncRead + AsyncSeek + Unpin
	) -> Result<Option<u64>, io::Error> {
		/// The maximum length of a partial EOCD signature: its length, minus one byte.
		const MAX_PARTIAL_EOCD_SIGNATURE_LENGTH: usize = EndOfCentralDirectory::SIGNATURE.len() - 1;

		// Opportunistically seek to the file region where EOCD has to be when no trailing data
		// comes after it, which notably reduces search time without sacrificing compatibility
		// with the ZIP files we generate. If that fails, which can be a normal occurrence when
		// the ZIP file is pretty small (and thus we cannot seek to a negative position),
		// ensure we are at the beginning of the file, to not miss any records
		let record_region_start = match zip_file.seek(SeekFrom::End(-(Self::MAX_SIZE as i64))).await {
			Ok(eocd_region_start) => eocd_region_start,
			Err(_) => zip_file.seek(SeekFrom::Start(0)).await?
		};

		let mut last_signature_offset = None;

		let signature_finder = memmem::FinderRev::new(&Self::SIGNATURE);
		let mut candidate_record_buf =
			[0; MAX_PARTIAL_EOCD_SIGNATURE_LENGTH + Self::MAX_SIZE as usize];

		let mut total_bytes_read = 0;
		let mut bytes_carried_over = 0;

		loop {
			let bytes_read = zip_file
				.read(&mut candidate_record_buf[MAX_PARTIAL_EOCD_SIGNATURE_LENGTH..])
				.await?;

			if bytes_read == 0 {
				// No further bytes read. The bytes carried over from the previous read can be assumed
				// to not be followed by any more bytes, so we never will find a new signature
				break;
			}

			// We either have no bytes from the previous read (if it was the first read), or we have
			// some. In either case, try to find the signature in the previous bytes (if any) and the
			// bytes we just read, carrying over at most the last `MAX_PARTIAL_EOCD_SIGNATURE_LENGTH`
			// bytes for the next read, to handle the case where the EOCD signature gets split between
			// several reads
			let bytes_to_carry_over = cmp::min(MAX_PARTIAL_EOCD_SIGNATURE_LENGTH, bytes_read);
			let (find_start, find_end) = (
				MAX_PARTIAL_EOCD_SIGNATURE_LENGTH - bytes_carried_over,
				MAX_PARTIAL_EOCD_SIGNATURE_LENGTH + bytes_read
			);

			last_signature_offset = signature_finder
				.rfind(&candidate_record_buf[find_start..find_end])
				.map(|window_offset| {
					total_bytes_read + window_offset as u64 - bytes_carried_over as u64
				})
				.or(last_signature_offset);

			// Update the sliding window of at most `MAX_PARTIAL_EOCD_SIGNATURE_LENGTH` previous bytes by
			// discarding its oldest `bytes_to_carry_over` bytes, and then copying the latest bytes read
			// to the end of such window
			candidate_record_buf[..MAX_PARTIAL_EOCD_SIGNATURE_LENGTH]
				.rotate_left(bytes_to_carry_over);
			candidate_record_buf.copy_within(
				MAX_PARTIAL_EOCD_SIGNATURE_LENGTH + bytes_read - bytes_to_carry_over
					..MAX_PARTIAL_EOCD_SIGNATURE_LENGTH + bytes_read,
				MAX_PARTIAL_EOCD_SIGNATURE_LENGTH - bytes_to_carry_over
			);

			bytes_carried_over = cmp::min(
				bytes_carried_over + bytes_to_carry_over,
				MAX_PARTIAL_EOCD_SIGNATURE_LENGTH
			);
			total_bytes_read += bytes_read as u64;
		}

		Ok(last_signature_offset.map(|offset| record_region_start + offset))
	}

	/// Writes this ZIP file record to the specified output ZIP file. For top performance,
	/// it is recommended to use a sink with efficient vectored writes.
	pub async fn write(
		&self,
		mut output_zip: &mut (impl AsyncWrite + Unpin + ?Sized)
	) -> Result<(), Error> {
		// If ZIP64 extensions are required, we must generate a ZIP64 end of central directory
		// record, with its corresponding locator
		if self.requires_zip64_extensions() {
			output_zip
				.write_all_vectored(&mut [
					IoSlice::new(&Self::ZIP64_SIGNATURE),
					IoSlice::new(&(44 + self.zip64_record_size_offset as i64).to_le_bytes()),
					IoSlice::new(&get_version_made_by(self.spoof_version_made_by)),
					// Luckily, ZIP64 is the highest specification version we support, so this is
					// always correct. It also achieves more compressibility if we didn't spoof
					// the made by version
					IoSlice::new(
						&ZipFeature::Zip64Extensions
							.to_version_needed_to_extract()
							.to_le_bytes()
					),
					IoSlice::new(
						&(if self.zero_out_unused_zip64_fields {
							0
						} else {
							self.disk_number
						} as u32)
							.to_le_bytes()
					),
					IoSlice::new(
						&(if self.zero_out_unused_zip64_fields {
							0
						} else {
							self.central_directory_start_disk_number
						} as u32)
							.to_le_bytes()
					),
					IoSlice::new(
						&(if self.zero_out_unused_zip64_fields
							&& !self.entry_count_current_disk_requires_zip64_extensions()
						{
							0
						} else {
							self.central_directory_entry_count_current_disk
						}
						.to_le_bytes())
					),
					IoSlice::new(
						&(if self.zero_out_unused_zip64_fields
							&& !self.total_entry_count_requires_zip64_extensions()
						{
							0
						} else {
							self.total_central_directory_entry_count
						}
						.to_le_bytes())
					),
					IoSlice::new(
						&(if self.zero_out_unused_zip64_fields
							&& !self.central_directory_size_requires_zip64_extensions()
						{
							0
						} else {
							self.central_directory_size
						}
						.to_le_bytes())
					),
					IoSlice::new(
						&(if self.zero_out_unused_zip64_fields
							&& !self.central_directory_start_offset_requires_zip64_extensions()
						{
							0
						} else {
							self.central_directory_start_offset
						}
						.to_le_bytes())
					),
					// Now go for the ZIP64 EOCD locator, which is always needed
					IoSlice::new(&Self::ZIP64_LOCATOR_SIGNATURE),
					IoSlice::new(
						&(if self.zero_out_unused_zip64_fields {
							0
						} else {
							self.central_directory_start_disk_number
						} as u32)
							.to_le_bytes()
					),
					IoSlice::new(&self.current_file_offset.to_le_bytes()),
					IoSlice::new(&self.total_number_of_disks.to_le_bytes())
				])
				.await?;
		}

		// After the ZIP64 EOCD record, if any, goes the traditional EOCD record. Write it
		output_zip
			.write_all_vectored(&mut [
				IoSlice::new(&Self::SIGNATURE),
				IoSlice::new(&self.disk_number.to_le_bytes()),
				IoSlice::new(&self.central_directory_start_disk_number.to_le_bytes()),
				IoSlice::new(
					&if self.entry_count_current_disk_requires_zip64_extensions() {
						u16::MAX
					} else {
						self.central_directory_entry_count_current_disk as u16
					}
					.to_le_bytes()
				),
				IoSlice::new(
					&if self.total_entry_count_requires_zip64_extensions() {
						u16::MAX
					} else {
						self.total_central_directory_entry_count as u16
					}
					.to_le_bytes()
				),
				IoSlice::new(
					&if self.central_directory_size_requires_zip64_extensions() {
						u32::MAX
					} else {
						self.central_directory_size as u32
					}
					.to_le_bytes()
				),
				IoSlice::new(
					&if self.central_directory_start_offset_requires_zip64_extensions() {
						u32::MAX
					} else {
						self.central_directory_start_offset as u32
					}
					.to_le_bytes()
				),
				IoSlice::new(&(self.archive_comment.len() as u16).to_le_bytes()),
				IoSlice::new(self.archive_comment.as_bytes())
			])
			.await
	}
}
