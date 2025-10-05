//! This module exports functions to efficiently get system IDs for the computer
//! this application is running on.
//!
//! The particular properties of a system ID may vary across platforms, but at
//! least is guaranteed to be more repeatable across executions than a randomly
//! generated value, and fairly unpredictable for an attacker that does not know
//! any specifics of the computer that generated the system ID. This property
//! does not necessarily preclude the system ID from being unsuitable for using
//! as a key for cryptographic primitives, so it is **strongly discouraged** to
//! use it as a key directly, because its entropy is not guaranteed, and a
//! successful crack could reveal the system ID to an attacker. It should be
//! assumed that the user is able to change the system ID.

// This module was inspired by, and expands upon, the following crates:
// https://docs.rs/crate/machine-uid/0.2.0
// https://github.com/tilda/rust-hwid

use std::{env, iter, sync::OnceLock};
use tinyvec::TinyVec;
use tokio::task;
use uuid::Uuid;

mod os;
mod util;

#[cfg(test)]
mod tests;

/// The type of the internal system ID list used.
type SystemIdVec = TinyVec<[SystemId; 5]>;

/// A struct that contains a system ID and some of its most relevant characteristics.
#[derive(Debug, Default)]
pub(super) struct SystemId {
	pub(super) id: TinyVec<[u8; 16]>,
	pub(super) is_volatile: bool
}

impl SystemId {
	/// Creates a new system identifier from the specified identifier and characteristics.
	fn new(id: impl AsRef<[u8]>, is_volatile: bool) -> Self {
		Self {
			id: id.as_ref().into(),
			is_volatile
		}
	}
}

/// The cell that will be used to memoize the result of computing the system ID, so
/// it's done only once in the lifetime of the process.
static SYSTEM_IDS: OnceLock<SystemIdVec> = OnceLock::new();

/// Returns identifiers for this system, calculating them if that was not done yet.
///
/// The result is memoized, so calling this function several times is free after the first use,
/// barring any thread synchronization that may be needed. This also means that system IDs can't
/// change during the lifetime of the process. However, obtaining system IDs for the first time
/// may be a relatively expensive operation, involving I/O and system calls.
pub(super) fn get_or_compute_system_ids() -> &'static [SystemId] {
	SYSTEM_IDS.get_or_init(|| {
		task::block_in_place(|| {
			read_system_id_from_env().map_or_else(compute_system_ids, |system_id| {
				iter::once(system_id).collect()
			})
		})
	})
}

/// Reads a system ID from a special process environment variable. This environment variable
/// may not exist or be valid, in which case `None` is returned.
fn read_system_id_from_env() -> Option<SystemId> {
	env::var("PACKSQUASH_SYSTEM_ID").ok().and_then(|system_id| {
		Uuid::try_parse(&system_id)
			.map(|uuid| SystemId::new(uuid.into_bytes(), false))
			.ok()
			.or_else(|| util::decode_hex(&system_id).map(|id| SystemId::new(id, false)))
	})
}

fn compute_system_ids() -> SystemIdVec {
	// Not all targets implemented here are tested

	#[cfg(target_os = "linux")]
	{
		use self::os::{
			get_aggregated_dmi_serial_numbers_id, get_boot_id, get_dbus_machine_id,
			get_dmi_product_id, get_host_id
		};

		get_dbus_machine_id()
			.into_iter()
			.chain(get_dmi_product_id())
			.chain(get_aggregated_dmi_serial_numbers_id())
			.chain(get_boot_id())
			.chain(get_host_id())
			.collect()
	}

	#[cfg(target_os = "android")]
	{
		use self::os::{get_boot_id, get_host_id};

		get_boot_id().into_iter().chain(get_host_id()).collect()
	}

	#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
	{
		use self::os::{get_dbus_machine_id, get_dmi_product_id, get_host_id, get_kernel_host_id};

		get_dbus_machine_id()
			.into_iter()
			.chain(get_kernel_host_id())
			.chain(get_dmi_product_id())
			.chain(get_host_id())
			.collect()
	}

	#[cfg(target_os = "macos")]
	{
		use self::os::{get_host_id, get_platform_serial_number};

		get_platform_serial_number()
			.into_iter()
			.chain(get_host_id())
			.collect()
	}

	#[cfg(target_os = "windows")]
	{
		use self::os::{
			get_dmi_product_id, get_install_date, get_machine_id, get_system_root_volume_id
		};

		get_machine_id()
			.into_iter()
			.chain(get_dmi_product_id())
			.chain(get_system_root_volume_id())
			.chain(get_install_date())
			.collect()
	}
}
