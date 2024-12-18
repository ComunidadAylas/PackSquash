//! Implements sanitization (i.e. format-preserving encryption) for [`SystemTime`]s to convert them
//! to Squash Times and back.

use std::time::{Duration, SystemTime, SystemTimeError};

use aes::{
	Aes256,
	cipher::{BlockCipher, BlockEncrypt}
};

use super::system_id::get_or_compute_system_ids;
use fpe::ff1::{BinaryNumeralString, FF1};
use hkdf::Hkdf;
use obfstr::random;
use sha2::Sha256;
use thiserror::Error;

#[cfg(test)]
mod tests;

/// Represents an error that may happen while sanitizing [SystemTime] structs.
#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum SystemTimeSanitizationError {
	#[error("The time is too far back in past")]
	PastSystemTime(#[from] SystemTimeError),
	#[error("The time is too far back in past")]
	PastSquashTime,
	#[error("The time is too far into the future")]
	FutureSquashTime,
	#[error("Invalid stick parity bit. Did the system ID or PackSquash build change?")]
	CorruptSquashTime
}

/// Sanitizes [SystemTime] structs to a 4-byte format that looks random, but can be
/// converted back to the original [SystemTime] only by PackSquash.
///
/// This 4-byte format is not a strictly conforming MS-DOS date and time, as defined
/// in the ZIP file specification. However, for performance reasons, most MS-DOS date
/// and time parsing implementations are lenient and accept invalid values.
//
// If you're trying to reverse the sanitized format, please consider whether doing
// so is really worth your time and that PackSquash developers made it harder for
// you because pack files modification dates are intended to be private.
pub struct SystemTimeSanitizer<C: BlockCipher + BlockEncrypt + Clone> {
	ff1_cipher: FF1<C>,
	cipher_key_is_volatile: bool,
	cipher_key_is_predictable: bool
}

/// An application-wide salt that is used for time sanitization.
///
/// This salt may be deterministically generated via the `OBFSTR_SEED`
/// environment variable on build time. A good way to generate a 512-bit
/// seed on a Linux system with GNU Coreutils is:
///
/// ```bash
/// $ dd if=/dev/urandom bs=1 count=64 2>/dev/null | base64 -w 0
/// ```
#[rustfmt::skip] // array::from_fn is not const and rustfmt turns this into one macro per line
const TIME_SANITIZATION_SALT: [u8; 32] = [
	random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8),
	random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8),
	random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8),
	random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8), random!(u8),
];

/// A 32-bit unsigned integer with the MSB set.
const STICK_PARITY_BIT_MASK: u32 = 1 << 31;

impl SystemTimeSanitizer<Aes256> {
	/// Creates a new system time sanitizer that uses AES-128 in CBC mode as its
	/// underlying cipher. The key used for encryption is derived by key stretching
	/// system IDs with an application-wide salt, extracting all the available
	/// entropy from them.
	pub(super) fn new() -> Self {
		let system_ids = get_or_compute_system_ids();
		let mut persistent_system_ids = system_ids
			.iter()
			.filter(|&system_id| !system_id.is_volatile)
			.peekable();

		// We want the key material to be deterministic between runs, so filter
		// volatile system IDs out, unless they're all we have
		let (key_material, cipher_key_is_volatile) = if persistent_system_ids.peek().is_some() {
			(
				persistent_system_ids
					.flat_map(|system_id| &system_id.id)
					.copied()
					.collect::<Vec<_>>(),
				false
			)
		} else {
			(
				system_ids
					.iter()
					.flat_map(|system_id| &system_id.id)
					.copied()
					.collect(),
				true
			)
		};

		// 14 bytes of material is a somewhat conservative threshold set by NIST for key size:
		// https://csrc.nist.gov/CSRC/media/Projects/Lightweight-Cryptography/documents/final-lwc-submission-requirements-august2018.pdf
		let cipher_key_is_predictable = key_material.len() < 14;

		let kdf = Hkdf::<Sha256>::new(Some(&TIME_SANITIZATION_SALT), &key_material);
		let mut key = [0; 32];
		kdf.expand(&[], &mut key).unwrap();

		Self {
			ff1_cipher: FF1::<Aes256>::new(&key, 2).unwrap(),
			cipher_key_is_volatile,
			cipher_key_is_predictable
		}
	}
}

impl<C: BlockCipher + BlockEncrypt + Clone> SystemTimeSanitizer<C> {
	/// Sanitizes the specified [SystemTime], using a tweak for the underlying cipher.
	/// The tweak is somewhat similar in role to a salt and need not be secret.
	/// The sanitization process may represent the system time with reduced precision,
	/// so desanitizing the result may not yield exactly the same system time.
	pub(super) fn sanitize(
		&self,
		time: &SystemTime,
		tweak: &[u8]
	) -> Result<[u8; 4], SystemTimeSanitizationError> {
		// Squash Time is defined as the count of half-seconds since Monday, 22 December 2014
		// 0:00:00 (UTC), as adjustments to the Unix time, following the formula
		// squash_time = (ms_unix_time - squash_epoch) / 500.
		// With 31 bits to store the magnitude, timestamps up to Wednesday, 30 December 2048
		// 13:37:03 (UTC) can be represented, which is better than 32-bit, second-precision
		// Unix timestamps
		let squash_time = u32::try_from(
			time.duration_since(SystemTime::UNIX_EPOCH)?
				.as_millis()
				.checked_sub(1419206400000)
				.ok_or(SystemTimeSanitizationError::PastSquashTime)?
				/ 500
		)
		.map_err(|_| SystemTimeSanitizationError::FutureSquashTime)?;

		// Make sure that the most significant bit is not set, because we use as it as a
		// stick parity bit that is always set to zero to check that the decoding is
		// plausibly correct
		if squash_time >= STICK_PARITY_BIT_MASK {
			return Err(SystemTimeSanitizationError::FutureSquashTime);
		}

		// Now use our block cipher in FF1 FPE mode to encrypt the Squash Time
		let sanitized_squash_time_bytes = self
			.ff1_cipher
			.encrypt(
				tweak,
				&BinaryNumeralString::from_bytes_le(&squash_time.to_le_bytes())
			)
			.unwrap()
			.to_bytes_le();

		Ok(sanitized_squash_time_bytes.try_into().unwrap())
	}

	/// Desanitizes the specified four bytes back to a system time. The tweak
	/// must be the same that was used for sanitization. Some authenticity is
	/// guaranteed using a stick parity bit, but such bit can only detect
	/// non-authentic bytes as such with 50% probability, as any change in the
	/// tweak, key or bytes is assumed to desanitize to an incorrect random number.
	pub(super) fn desanitize(
		&self,
		sanitized_time: &[u8; 4],
		tweak: &[u8]
	) -> Result<SystemTime, SystemTimeSanitizationError> {
		let squash_time = u32::from_le_bytes(
			self.ff1_cipher
				.decrypt(tweak, &BinaryNumeralString::from_bytes_le(sanitized_time))
				.unwrap()
				.to_bytes_le()
				.try_into()
				.unwrap()
		);

		// If the stick parity bit is set, we know for sure that this Squash Time
		// is not authentic, because it was tampered with or a key has changed
		if squash_time >= STICK_PARITY_BIT_MASK {
			return Err(SystemTimeSanitizationError::CorruptSquashTime);
		}

		// Convert Squash Time back to Unix time, in ms. The result value
		// needs at most 42 bits, so it fits nicely in a 64-bit integer
		let ms_unix_time = 500 * squash_time as u64 + 1419206400000;

		Ok(SystemTime::UNIX_EPOCH + Duration::from_millis(ms_unix_time))
	}

	/// Returns whether the sanitizer cipher is using a volatile key.
	///
	/// Volatile keys are those derived from a set of system IDs where at least one
	/// is volatile, meaning that it can change between runs, and thus the key.
	pub fn using_volatile_key(&self) -> bool {
		self.cipher_key_is_volatile
	}

	/// Returns whether the sanitizer cipher is using a predictable key.
	///
	/// Predictable keys are those derived from an empty set of system IDs, or where
	/// their concatenation to produce key material is less than 14 bytes long.
	pub fn using_predictable_key(&self) -> bool {
		self.cipher_key_is_predictable
	}
}
