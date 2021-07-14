use std::{
	borrow::Cow,
	cell::Cell,
	char::{ToLowercase, ToUppercase},
	io,
	iter::{self, Once}
};

use const_random::const_random;
use rand_xoshiro::{
	rand_core::{RngCore, SeedableRng},
	Xoshiro128Plus
};
use tokio::io::{AsyncWrite, AsyncWriteExt};

use crate::config::PercentageInteger;

use super::{
	zip_file_record::{
		CentralDirectoryHeader, CompressionMethod, EndOfCentralDirectory, LocalFileHeader
	},
	SquashZipSettings
};

const CRC32_KEY: u32 = {
	let k = const_random!(u32);

	if k == 0 {
		0xDEADBEEF
	} else {
		k
	}
};

thread_local!(static RNG: Cell<Option<Xoshiro128Plus>> = Cell::new(None));

/// Implements several obfuscation techniques for ZIP file records. This struct is
/// purposefully less documented than others.
//
// This obfuscation is not rocket science. You've won. Please consider whether
// circumventing it is ethical and worth your time. Please be civil and do not
// circumvent it publicly just because you hold a grudge against someone or
// something. Please use your knowledge to do good things and help shape a
// better world, not just for bragging, ripping off the work of others or other
// questionable means.
pub(super) enum ObfuscationEngine {
	NoObfuscation,
	Obfuscation {
		size_increasing_obfuscation: bool,
		obfuscation_discretion_records_percentage: PercentageInteger,
		workaround_old_java_obfuscation_quirks: bool
	}
}

impl ObfuscationEngine {
	/// Instantiates an [`ObfuscationEngine`] appropriate for the provided [`SquashZipSettings`].
	pub fn from_squash_zip_settings(squash_zip_settings: &SquashZipSettings) -> Self {
		if squash_zip_settings.enable_obfuscation {
			Self::Obfuscation {
				size_increasing_obfuscation: squash_zip_settings.enable_size_increasing_obfuscation,
				obfuscation_discretion_records_percentage: squash_zip_settings
					.percentage_of_records_tuned_for_obfuscation_discretion,
				workaround_old_java_obfuscation_quirks: squash_zip_settings
					.workaround_old_java_obfuscation_quirks
			}
		} else {
			Self::NoObfuscation
		}
	}

	pub async fn obfuscating_header<T: AsyncWrite + Unpin>(
		&self,
		mut output_zip: T,
		seed: u64
	) -> io::Result<()> {
		if let ObfuscationEngine::Obfuscation {
			size_increasing_obfuscation: true,
			..
		} = self
		{
			output_zip
				.write_all(&if random_u32(seed) % 5 == 0 {
					[0x50, 0x4B, 0x03, 0x04]
				} else {
					[0x50, 0x4B, 0x05, 0x06]
				})
				.await?;
		}

		Ok(())
	}

	pub fn obfuscate_local_file_header<'a>(
		&'a self,
		local_file_header: LocalFileHeader<'a>
	) -> LocalFileHeader<'a> {
		let mut obfuscated_header = local_file_header;

		if let ObfuscationEngine::Obfuscation { .. } = self {
			let seed = obfuscated_header.crc32 as u64;
			let discretion = self.use_discretion(seed);

			let compression_method;
			let squash_time;
			let crc32;
			let compressed_size;
			let uncompressed_size;
			let file_name;
			let zero_out_version_needed_to_extract;

			if discretion {
				compression_method = obfuscated_header.compression_method;
				squash_time = obfuscated_header.squash_time;
				crc32 = obfuscated_header.crc32 ^ CRC32_KEY;
				compressed_size = obfuscated_header.compressed_size + random_u32(seed) % 8 + 1;
				uncompressed_size = obfuscated_header.uncompressed_size + random_u32(seed) % 8 + 1;
				file_name = {
					let digit_displacement = (random_u32(seed) % 10 + 1) as u8;
					let mut remaining_file_name_bytes =
						obfuscated_header.file_name().as_bytes().len() as i32;

					Cow::Owned(
						obfuscated_header
							.file_name()
							.chars()
							.flat_map(|c| {
								enum ObfuscatedChars {
									ToUppercase(ToUppercase),
									ToLowercase(ToLowercase),
									Passthrough(Once<char>)
								}

								impl Iterator for ObfuscatedChars {
									type Item = char;

									fn next(&mut self) -> Option<Self::Item> {
										match self {
											Self::ToUppercase(iter) => iter.next(),
											Self::ToLowercase(iter) => iter.next(),
											Self::Passthrough(iter) => iter.next()
										}
									}
								}

								if c.is_ascii_digit() {
									ObfuscatedChars::Passthrough(iter::once(
										((c as u8 - 48 + digit_displacement) % 10 + 48) as char
									))
								} else if c.is_lowercase() {
									ObfuscatedChars::ToUppercase(c.to_uppercase())
								} else if c.is_uppercase() {
									ObfuscatedChars::ToLowercase(c.to_lowercase())
								} else {
									ObfuscatedChars::Passthrough(iter::once(c))
								}
							})
							.take_while(|c| {
								remaining_file_name_bytes -= c.len_utf8() as i32;

								remaining_file_name_bytes >= 0
							})
							.collect::<String>()
					)
				};
				zero_out_version_needed_to_extract = false;
			} else {
				compression_method = CompressionMethod::Store;
				squash_time = [0; 4];
				crc32 = 0;
				compressed_size = 0;
				uncompressed_size = 0;
				file_name = Cow::Borrowed("");
				zero_out_version_needed_to_extract = true;
			}

			obfuscated_header = LocalFileHeader::new(file_name).unwrap();
			obfuscated_header.compression_method = compression_method;
			obfuscated_header.crc32 = crc32;
			obfuscated_header.compressed_size = compressed_size;
			obfuscated_header.uncompressed_size = uncompressed_size;
			obfuscated_header.squash_time = squash_time;
			obfuscated_header.zero_out_version_needed_to_extract = zero_out_version_needed_to_extract;
		}

		obfuscated_header
	}

	pub fn obfuscate_central_directory_header<'a>(
		&'a self,
		central_directory_header: CentralDirectoryHeader<'a>
	) -> CentralDirectoryHeader<'a> {
		let mut obfuscated_header = central_directory_header;

		if let ObfuscationEngine::Obfuscation {
			workaround_old_java_obfuscation_quirks,
			..
		} = self
		{
			let seed = obfuscated_header.crc32 as u64;
			let discretion = self.use_discretion(seed);

			let uncompressed_size;
			let local_header_disk_number;

			let obfuscate_uncompressed_size = !workaround_old_java_obfuscation_quirks
				|| (obfuscated_header.compression_method != CompressionMethod::Store
					&& obfuscated_header.compressed_size != 0);

			if discretion {
				uncompressed_size = if !obfuscate_uncompressed_size
					|| obfuscated_header.compression_method == CompressionMethod::Store
				{
					obfuscated_header.uncompressed_size
				} else {
					4096 + obfuscated_header.compressed_size % 4096
				};
				local_header_disk_number = (random_u32(seed) % 32768) as u16 + 32768;
			} else {
				uncompressed_size = if obfuscate_uncompressed_size {
					0
				} else {
					obfuscated_header.uncompressed_size
				};
				local_header_disk_number = 0;
			}

			obfuscated_header = CentralDirectoryHeader {
				compression_method: obfuscated_header.compression_method,
				squash_time: obfuscated_header.squash_time,
				crc32: obfuscated_header.crc32 ^ CRC32_KEY,
				compressed_size: obfuscated_header.compressed_size,
				uncompressed_size,
				local_header_disk_number,
				local_header_offset: obfuscated_header.local_header_offset
					- self.obfuscating_header_size(),
				file_name: obfuscated_header.file_name,
				spoof_version_made_by: true
			};
		}

		obfuscated_header
	}

	pub fn obfuscate_end_of_central_directory(
		&self,
		end_of_central_directory: EndOfCentralDirectory
	) -> EndOfCentralDirectory {
		let mut obfuscated_header = end_of_central_directory;

		if let ObfuscationEngine::Obfuscation { .. } = self {
			let seed = obfuscated_header.total_central_directory_entry_count
				^ obfuscated_header.current_file_offset;
			let discretion = self.use_discretion(seed);

			let central_directory_entry_count_current_disk;
			let total_central_directory_entry_count;
			let total_number_of_disks;
			let zip64_record_size_offset;

			if discretion {
				let offset = random_u32(seed) as u64 % 7 + 1;

				central_directory_entry_count_current_disk =
					obfuscated_header.central_directory_entry_count_current_disk + offset;
				total_central_directory_entry_count =
					obfuscated_header.total_central_directory_entry_count + offset;
				total_number_of_disks = random_u32(seed) % 65536 + 65536;
				zip64_record_size_offset = (random_u32(seed) % 17) as i8 - 8;
			} else {
				central_directory_entry_count_current_disk = 0;
				total_central_directory_entry_count = 0;
				total_number_of_disks = 0;
				zip64_record_size_offset = -44;
			}

			obfuscated_header = EndOfCentralDirectory {
				disk_number: u16::MAX,
				central_directory_start_disk_number: 0,
				central_directory_entry_count_current_disk,
				total_central_directory_entry_count,
				central_directory_size: obfuscated_header.central_directory_size,
				central_directory_start_offset: obfuscated_header.central_directory_start_offset
					- self.obfuscating_header_size(),
				total_number_of_disks,
				current_file_offset: obfuscated_header.current_file_offset
					- self.obfuscating_header_size(),
				zip64_record_size_offset,
				spoof_version_made_by: true,
				zero_out_unused_zip64_fields: !discretion
			};
		}

		obfuscated_header
	}

	pub fn deobfuscate_crc32(&self, obfuscated_crc32: u32) -> u32 {
		if let ObfuscationEngine::Obfuscation { .. } = self {
			obfuscated_crc32 ^ CRC32_KEY
		} else {
			obfuscated_crc32
		}
	}

	pub fn obfuscating_header_size(&self) -> u64 {
		if let ObfuscationEngine::Obfuscation {
			size_increasing_obfuscation: true,
			..
		} = self
		{
			4
		} else {
			0
		}
	}

	fn use_discretion(&self, seed: u64) -> bool {
		if let ObfuscationEngine::Obfuscation {
			obfuscation_discretion_records_percentage,
			..
		} = self
		{
			random_u32(seed) % 100 < u8::from(*obfuscation_discretion_records_percentage) as u32
		} else {
			false
		}
	}
}

fn random_u32(seed: u64) -> u32 {
	RNG.with(|rng_cell| {
		let mut rng = rng_cell
			.take()
			.or_else(|| Some(Xoshiro128Plus::seed_from_u64(seed)))
			.unwrap();

		let random_number = rng.next_u32();

		rng_cell.set(Some(rng));

		random_number
	})
}
