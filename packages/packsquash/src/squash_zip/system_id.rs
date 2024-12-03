//! This module exports functions to efficiently get a system ID for the computer
//! this application is running on.
//!
//! The particular properties of a system ID may vary across platforms, but at
//! least is guaranteed to be more repeatable across executions than a randomly
//! generated value, and fairly unpredictable for an attacker that does not know
//! any specifics of the computer that generated the system ID. A best effort is
//! made to make it suitable to generate a key for cryptographic primitives,
//! although it is **strongly discouraged** to use it as a key directly, because
//! its entropy is not guaranteed, and a successful crack could reveal the system
//! ID to an attacker. It should be assumed that the user is able to change the
//! system ID.

// This module was inspired by, and expands upon, the following crates:
// https://docs.rs/crate/machine-uid/0.2.0
// https://github.com/tilda/rust-hwid

use std::{cmp::Ordering, env, sync::OnceLock};

use tokio::task;
use uuid::Uuid;

mod os;

#[cfg(test)]
mod tests;

/// The panic error message to show in case a system ID could not be found.
static ERROR_MESSAGE: &str = "Couldn't get a suitable system ID for ZIP field obfuscation. Aborting execution for safety. \
	Please review your system configuration or report a bug.";

/// A struct that contains a system ID and some of its most relevant characteristics.
///
/// System identifiers have a defined sort order and equality condition that does not
/// look at the actual ID value. Instead, two system identifiers are considered equal
/// when they have the same entropy, volatileness and priority, and a system ID is
/// greater than another when it has higher entropy, lower volatileness or higher
/// priority. Therefore, if a system ID is "greater" than another, it is "better".
#[derive(Debug)]
pub struct SystemId {
	pub id: u128,
	pub has_low_entropy: bool,
	pub is_volatile: bool,
	entropy: f32,
	priority: u8
}

impl SystemId {
	/// Creates a new system identifier from the specified identifier and characteristics.
	fn new(id: u128, is_volatile: bool, priority: u8) -> Option<Self> {
		// Discard too unsafe identifiers (all zeros or all ones)
		if id != u128::MIN && id != u128::MAX {
			let entropy = entropy::metric_entropy(id.to_le_bytes());

			Some(Self {
				id,
				// Metric entropy is the Shannon entropy normalized by the length of the string.
				// Therefore, the optimum metric entropy is 0.25, which means that every bit in the
				// ID is unpredictable. A threshold at 0.2 seems okay
				has_low_entropy: entropy <= 0.2,
				is_volatile,
				entropy,
				priority
			})
		} else {
			None
		}
	}
}

impl PartialOrd for SystemId {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for SystemId {
	fn cmp(&self, other: &Self) -> Ordering {
		// - Consider self greater than other if self entropy is higher than other entropy, but
		//   discount quite a bit of entropy for volatile IDs, so we only use them if they have
		//   substantially more entropy than persistent IDs.
		// - If entropies are equal, or some weird floating point numbers for which comparison
		//   is not well-defined, consider self greater than other if it is not volatile.
		// - If entropies and volatilenesses are equal, prefer the ID with higher priority value.
		let self_volatile_penalization = self.is_volatile as u16 as f32 * 0.05;
		let other_volatile_penalization = other.is_volatile as u16 as f32 * 0.05;

		(self.entropy - self_volatile_penalization)
			.partial_cmp(&(other.entropy - other_volatile_penalization))
			.unwrap_or(Ordering::Equal)
			.then(self.is_volatile.cmp(&other.is_volatile).reverse())
			.then(self.priority.cmp(&other.priority))
	}
}

impl PartialEq for SystemId {
	fn eq(&self, other: &Self) -> bool {
		matches!(self.cmp(other), Ordering::Equal)
	}
}

impl Eq for SystemId {}

/// The cell that will be used to memoize the result of computing the system ID, so
/// it's done only once in the lifetime of the process.
static SYSTEM_ID: OnceLock<SystemId> = OnceLock::new();

/// Returns the system ID, calculating it if that was not done yet.
///
/// The result is memoized, so calling this function several times is free after the first use,
/// barring any thread synchronization that may be needed. This also means that a system ID can't
/// change during the lifetime of the process. However, obtaining a system ID for the first time
/// may be a relatively expensive operation, involving I/O and system calls.
pub(super) fn get_or_compute_system_id() -> &'static SystemId {
	SYSTEM_ID.get_or_init(|| task::block_in_place(compute_system_id))
}

/// Gets the system ID if and only if it was calculated at some point. If not, this function
/// returns `None`. This function is guaranteed to never block.
pub fn get_system_id() -> Option<&'static SystemId> {
	SYSTEM_ID.get()
}

/// Reads a system ID from a special process environment variable. This environment variable
/// may not exist or be valid, in which case `None` is returned.
fn read_from_env() -> Option<SystemId> {
	SystemId::new(
		Uuid::parse_str(&env::var("PACKSQUASH_SYSTEM_ID").ok()?)
			.ok()?
			.as_u128(),
		false,
		u8::MAX
	)
}

// What follows are target specific definitions of the function that computes the system ID.
// Obviously, it can't be defined twice for the same target.
//
// Not all targets implemented here are tested.

#[cfg(target_os = "linux")]
fn compute_system_id() -> SystemId {
	use self::os::{get_boot_id, get_dbus_machine_id, get_host_id};

	read_from_env()
		.or_else(|| {
			get_dbus_machine_id()
				.into_iter()
				.chain(get_boot_id())
				.chain(get_host_id())
				.max()
		})
		.expect(ERROR_MESSAGE)
}

#[cfg(target_os = "android")]
fn compute_system_id() -> SystemId {
	use self::os::{get_boot_id, get_host_id};

	read_from_env()
		.or_else(|| {
			get_boot_id()
				.into_iter()
				.chain(get_host_id().into_iter())
				.max()
		})
		.expect(ERROR_MESSAGE)
}

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
fn compute_system_id() -> SystemId {
	use self::os::{get_dbus_machine_id, get_dmi_product_id, get_host_id, get_kernel_host_id};

	read_from_env()
		.or_else(|| {
			get_dbus_machine_id()
				.into_iter()
				.chain(get_kernel_host_id().into_iter())
				.chain(get_dmi_product_id().into_iter())
				.chain(get_host_id().into_iter())
				.max()
		})
		.expect(ERROR_MESSAGE)
}

#[cfg(target_os = "macos")]
fn compute_system_id() -> SystemId {
	use self::os::{get_host_id, get_platform_serial_number};

	read_from_env()
		.or_else(|| {
			get_platform_serial_number()
				.into_iter()
				.chain(get_host_id().into_iter())
				.max()
		})
		.expect(ERROR_MESSAGE)
}

#[cfg(target_os = "windows")]
fn compute_system_id() -> SystemId {
	use self::os::{get_install_date, get_machine_id, get_product_id};

	read_from_env()
		.or_else(|| {
			get_machine_id()
				.into_iter()
				.chain(get_product_id().into_iter())
				.chain(get_install_date().into_iter())
				.max()
		})
		.expect(ERROR_MESSAGE)
}
