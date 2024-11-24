//! Implements several obfuscation techniques for ZIP file records. This module is purposefully
//! less documented than others.
//
// This obfuscation is not rocket science. You've won. Please consider whether
// circumventing it is ethical and worth your time. Please be civil and do not
// circumvent it publicly just because you hold a grudge against someone or
// something. Please use your knowledge to do good things and help shape a
// better world, not just for bragging, ripping off the work of others or other
// questionable means.

use obfstr::random;
use rand_xoshiro::{
	rand_core::{RngCore, SeedableRng},
	Xoshiro128Plus
};
use std::{
	borrow::Cow,
	cell::Cell,
	char::{ToLowercase, ToUppercase},
	io,
	iter::{self, Once}
};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub use self::pseudodir_concealment::FileListingCircumstances;
use self::pseudodir_concealment::PseudodirConcealer;
use crate::{config::PercentageInteger, RelativePath};

use super::{
	zip_file_record::{
		CentralDirectoryHeader, CompressionMethod, EndOfCentralDirectory, LocalFileHeader
	},
	SquashZipSettings
};

mod pseudodir_concealment;

const CRC32_KEY: u32 = {
	let k = random!(u32);

	if k == 0 {
		0xDEADBEEF
	} else {
		k
	}
};

enum SizeIncreasingObfuscation {
	Disabled,
	Enabled {
		pseudodir_concealer: PseudodirConcealer
	}
}

#[repr(transparent)]
pub(super) struct ObfuscationEngine(SealedObfuscationEngine);

enum SealedObfuscationEngine {
	NoObfuscation,
	Obfuscation {
		size_increasing_obfuscation: SizeIncreasingObfuscation,
		obfuscation_discretion_records_percentage: PercentageInteger,
		workaround_old_java_obfuscation_quirks: bool
	}
}

impl ObfuscationEngine {
	/// Instantiates an [`ObfuscationEngine`] appropriate for the provided [`SquashZipSettings`].
	pub fn from_squash_zip_settings(squash_zip_settings: &SquashZipSettings) -> Self {
		Self(if squash_zip_settings.enable_obfuscation {
			SealedObfuscationEngine::Obfuscation {
				size_increasing_obfuscation: if squash_zip_settings.enable_size_increasing_obfuscation
				{
					SizeIncreasingObfuscation::Enabled {
						pseudodir_concealer: PseudodirConcealer::new()
					}
				} else {
					SizeIncreasingObfuscation::Disabled
				},
				obfuscation_discretion_records_percentage: squash_zip_settings
					.percentage_of_records_tuned_for_obfuscation_discretion,
				workaround_old_java_obfuscation_quirks: squash_zip_settings
					.workaround_old_java_obfuscation_quirks
			}
		} else {
			SealedObfuscationEngine::NoObfuscation
		})
	}

	pub async fn obfuscating_header<T: AsyncWrite + Unpin>(
		&self,
		mut output_zip: T,
		seed: u64
	) -> io::Result<()> {
		if let SealedObfuscationEngine::Obfuscation {
			size_increasing_obfuscation: SizeIncreasingObfuscation::Enabled { .. },
			..
		} = self.0
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

	pub fn obfuscate_local_file_header(&self, local_file_header: &mut LocalFileHeader<'_>) {
		if let SealedObfuscationEngine::Obfuscation { .. } = self.0 {
			let seed = local_file_header.crc32 as u64;
			let discretion = self.use_discretion(seed);

			if discretion {
				local_file_header.crc32 ^= CRC32_KEY;
				local_file_header.compressed_size += random_u32(seed) % 8 + 1;
				local_file_header.uncompressed_size += random_u32(seed) % 8 + 1;
				local_file_header.file_name = {
					let digit_displacement = (random_u32(seed) % 10 + 1) as u8;
					let mut remaining_file_name_bytes =
						local_file_header.file_name.as_str().len() as i32;

					Cow::Owned(RelativePath::from_inner(
						local_file_header
							.file_name
							.as_str()
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
					))
				};
			} else {
				local_file_header.compression_method = CompressionMethod::Store;
				local_file_header.squash_time = [0; 4];
				local_file_header.crc32 = 0;
				local_file_header.compressed_size = 0;
				local_file_header.uncompressed_size = 0;
				local_file_header.file_name = Cow::Owned(RelativePath::from_inner(""));
				local_file_header.zero_out_version_needed_to_extract = true;
			}
		}
	}

	pub fn obfuscate_central_directory_header(
		&self,
		central_directory_header: &mut CentralDirectoryHeader<'_>,
		listing_circumstances: FileListingCircumstances
	) {
		if let SealedObfuscationEngine::Obfuscation {
			workaround_old_java_obfuscation_quirks,
			size_increasing_obfuscation,
			..
		} = &self.0
		{
			let seed = central_directory_header.crc32 as u64;
			let discretion = self.use_discretion(seed);

			let obfuscate_uncompressed_size = !*workaround_old_java_obfuscation_quirks
				|| (central_directory_header.compression_method != CompressionMethod::Store
					&& central_directory_header.compressed_size != 0);

			if discretion {
				if obfuscate_uncompressed_size
					&& central_directory_header.compression_method != CompressionMethod::Store
				{
					central_directory_header.uncompressed_size =
						4096 + central_directory_header.compressed_size % 4096;
				}
				central_directory_header.local_header_disk_number =
					(random_u32(seed) % 32768) as u16 + 32767;
			} else {
				if obfuscate_uncompressed_size {
					central_directory_header.uncompressed_size = 0xFFFFFF7F;
				}
				central_directory_header.local_header_disk_number = u16::MAX - 1;
			}

			if let SizeIncreasingObfuscation::Enabled {
				pseudodir_concealer
			} = size_increasing_obfuscation
			{
				pseudodir_concealer.conceal(
					&mut central_directory_header.file_name,
					listing_circumstances
				);
			}

			central_directory_header.crc32 ^= CRC32_KEY;
			central_directory_header.local_header_offset -= self.obfuscating_header_size();
			central_directory_header.spoof_version_made_by = true;
		}
	}

	pub fn obfuscate_end_of_central_directory(
		&self,
		end_of_central_directory: &mut EndOfCentralDirectory
	) {
		if let SealedObfuscationEngine::Obfuscation { .. } = self.0 {
			let seed = end_of_central_directory.total_central_directory_entry_count
				^ end_of_central_directory.current_file_offset;
			let discretion = self.use_discretion(seed);

			if discretion {
				let offset = random_u32(seed) as u64 % 7 + 1;

				end_of_central_directory.central_directory_entry_count_current_disk += offset;
				end_of_central_directory.total_central_directory_entry_count += offset;
				end_of_central_directory.total_number_of_disks = random_u32(seed) % 65536 + 65536;
				end_of_central_directory.zip64_record_size_offset = (random_u32(seed) % 17) as i8 - 8;
			} else {
				end_of_central_directory.central_directory_entry_count_current_disk = 0;
				end_of_central_directory.total_central_directory_entry_count = 0;
				end_of_central_directory.total_number_of_disks = 0;
				end_of_central_directory.zip64_record_size_offset = -44;
			}

			end_of_central_directory.disk_number = 0xFFFF;
			end_of_central_directory.central_directory_start_offset -= self.obfuscating_header_size();
			end_of_central_directory.current_file_offset -= self.obfuscating_header_size();
			end_of_central_directory.spoof_version_made_by = true;
			end_of_central_directory.zero_out_unused_zip64_fields = !discretion;
		}
	}

	pub fn deobfuscate_crc32(&self, obfuscated_crc32: u32) -> u32 {
		if let SealedObfuscationEngine::Obfuscation { .. } = self.0 {
			obfuscated_crc32 ^ CRC32_KEY
		} else {
			obfuscated_crc32
		}
	}

	pub fn obfuscating_header_size(&self) -> u64 {
		if let SealedObfuscationEngine::Obfuscation {
			size_increasing_obfuscation: SizeIncreasingObfuscation::Enabled { .. },
			..
		} = self.0
		{
			4
		} else {
			0
		}
	}

	fn use_discretion(&self, seed: u64) -> bool {
		if let SealedObfuscationEngine::Obfuscation {
			obfuscation_discretion_records_percentage,
			..
		} = &self.0
		{
			random_u32(seed) % 100 < u8::from(*obfuscation_discretion_records_percentage) as u32
		} else {
			false
		}
	}
}

fn random_u32(seed: u64) -> u32 {
	thread_local!(static RNG: Cell<Option<Xoshiro128Plus>> = const { Cell::new(None) });

	RNG.with(|rng_cell| {
		let mut rng = rng_cell
			.take()
			.unwrap_or_else(|| Xoshiro128Plus::seed_from_u64(seed));

		let random_number = rng.next_u32();

		rng_cell.set(Some(rng));

		random_number
	})
}
