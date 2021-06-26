use std::time::{Duration, SystemTime, SystemTimeError};
use std::{convert::TryFrom, convert::TryInto, num::TryFromIntError};

use aes::cipher::generic_array::GenericArray;
use aes::{Aes128, Block, BlockCipher, BlockDecrypt, BlockEncrypt, NewBlockCipher};
use const_random::const_random;

use fpe::ff1::{BinaryNumeralString, FF1};
use thiserror::Error;

use super::{system_id::get_or_compute_system_id, zip_file_record::DUMMY_SQUASH_TIME};

#[cfg(test)]
mod tests;

/// Represents an error that may happen while sanitizing [SystemTime] structs.
#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub(super) enum SystemTimeSanitizationError {
	#[error("The time is too far back in past")]
	PastSystemTime(#[from] SystemTimeError),
	#[error("The time is too far back in past")]
	PastSquashTime,
	#[error("The time is too far into the future")]
	FutureSquashTime(#[from] TryFromIntError),
	#[error("The time has a reserved value. Please try again")]
	ReservedSquashTime
}

/// Sanitizes [SystemTime] structs to a 4-byte format that looks random, but can be
/// converted back to the original [SystemTime] only by PackSquash.
///
/// This 4-byte format is not a strictly conforming MS-DOS date and time, as defined
/// in the ZIP file specification. However, for performance reasons, most MS-DOS date
/// and time parsing implementations are lenient and accept invalid values.
///
/// It is guaranteed that a [SystemTime] will never sanitize to the same value as
/// [DUMMY_SQUASH_TIME].
//
// If you're trying to reverse the sanitized format, please consider whether doing
// so is really worth your time and that PackSquash developers made it harder for
// you because resource pack files modification dates are inteded to be private.
pub(super) struct SystemTimeSanitizer<
	C: NewBlockCipher + BlockCipher + BlockEncrypt + BlockDecrypt + Clone
> {
	ff1_cipher: FF1<C>
}

/// An application-wide salt that is used for time sanitization.
///
/// This salt may be deterministically generated via the `CONST_RANDOM_SEED`
/// environment variable on build time. A good way to generate a 512-bit
/// seed on a Linux system with GNU Coreutils is:
///
/// ```bash
/// $ dd if=/dev/urandom bs=1 count=64 2>/dev/null | base64 -w 0
/// ```
const TIME_SANITIZATION_SALT: [u8; 16] = const_random!(u128).to_le_bytes();

impl SystemTimeSanitizer<Aes128> {
	/// Creates a new system time sanitizer that uses AES-128 in CBC mode as its
	/// underlying cipher.
	pub fn new() -> Self {
		// Now generate the actual encryption key by encrypting the system-specific
		// system ID with our application-wide salt. This makes it much harder to
		// derive the system ID from the ciphertext, even if we know the salt
		let mut key = get_or_compute_system_id().id.to_le_bytes();
		Aes128::new(GenericArray::from_slice(&TIME_SANITIZATION_SALT))
			.encrypt_block(Block::from_mut_slice(&mut key));

		Self {
			ff1_cipher: FF1::<Aes128>::new(&key, 2).unwrap()
		}
	}
}

impl<C: NewBlockCipher + BlockCipher + BlockEncrypt + BlockDecrypt + Clone> SystemTimeSanitizer<C> {
	/// Sanitizes the specified [SystemTime], using a tweak for the underlying cipher.
	/// The tweak is somewhat similar in role to a salt and need not be secret.
	/// The sanitization process may represent the system time with reduced precision,
	/// so desanitizing the result may not yield exactly the same system time.
	pub fn sanitize(
		&self,
		time: &SystemTime,
		tweak: &[u8]
	) -> Result<[u8; 4], SystemTimeSanitizationError> {
		// Squash Time is defined as quarter-seconds since Monday, 22 December 2014 0:00:00 (UTC),
		// as adjustements to the Unix time, following the formula squash_time = (ms_unix_time -
		// squash_epoch) / 250.
		// This will be able to represent dates until Wednesday, 30 December 2048 13:37:03 (UTC),
		// which is better than 32-bit, second-precision Unix timestamps
		let squash_time = u32::try_from(
			time.duration_since(SystemTime::UNIX_EPOCH)?
				.as_millis()
				.checked_sub(1419206400000)
				.ok_or(SystemTimeSanitizationError::PastSquashTime)?
				/ 250
		)?;

		// Now use our block cipher in FF1 FPE mode to encrypt the Squash Time
		let sanitized_squash_time_bytes = self
			.ff1_cipher
			.encrypt(
				tweak,
				&BinaryNumeralString::from_bytes_le(&squash_time.to_le_bytes())
			)
			.unwrap()
			.to_bytes_le();

		match sanitized_squash_time_bytes.try_into().unwrap() {
			DUMMY_SQUASH_TIME => Err(SystemTimeSanitizationError::ReservedSquashTime),
			sanitized_time => Ok(sanitized_time)
		}
	}

	/// Desanitizes the specified four bytes back to a system time. The tweak
	/// must be the same that was used for sanitization. Note that any four
	/// bytes desanitize to lexically valid system times, but they may not be
	/// _semantically_ valid.
	pub fn desanitize(&self, sanitized_time: &[u8; 4], tweak: &[u8]) -> SystemTime {
		let squash_time = u32::from_le_bytes(
			self.ff1_cipher
				.decrypt(tweak, &BinaryNumeralString::from_bytes_le(sanitized_time))
				.unwrap()
				.to_bytes_le()
				.try_into()
				.unwrap()
		);

		// Convert Squash Time back to Unix time, in ms. The result value
		// needs at most 42 bits, so it fits nicely in a 64 bit integer
		let ms_unix_time = 250 * squash_time as u64 + 1419206400000;

		SystemTime::UNIX_EPOCH + Duration::from_millis(ms_unix_time)
	}
}
