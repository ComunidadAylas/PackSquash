//! This module exports a single static [`SYSTEM_ID`] variable which contains
//! a system ID for the computer this application is running on.
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
//!
//! Along with a system ID, the [`SYSTEM_ID`] variable contains two booleans. The
//! first boolean indicates whether the system ID is expected to have high entropy
//! and unpredictability, while the second indicates that is a highly volatile
//! system ID that may not persist across reboots. These booleans can be used to
//! handle the shenanigans of the available system ID better.

// This module was inspired by, and expands upon, the following crates:
// https://docs.rs/crate/machine-uid/0.2.0
// https://github.com/tilda/rust-hwid

use std::lazy::SyncLazy;

mod os;

#[cfg(test)]
mod tests;

/// The panic error message to show in case a system ID could not be found.
static ERROR_MESSAGE: &str =
	"Couldn't get a suitable system ID for ZIP field obfuscation. Aborting execution for safety. \
	Please review your system configuration or report a bug.";

/// Provides more concise syntax for discarding known-bad values for system IDs.
macro_rules! filter_id {
	($id_expr:expr) => {
		$id_expr.filter(|(id, _, _)| *id != u128::MIN && *id != u128::MAX)
	};
}

/// Provides more concise syntax for reading a system ID from a special process
/// environment variable.
macro_rules! read_from_env {
	() => {
		|| -> Option<(u128, bool, bool)> {
			Some((
				uuid::Uuid::parse_str(&std::env::var("PACKSQUASH_SYSTEM_ID").ok()?)
					.ok()?
					.as_u128(),
				true,
				false
			))
		}()
	};
}

// What follows are target specific definitions of SYSTEM_ID. Obviously, SYSTEM_ID
// can't be defined twice for the same target, so the definitions should be mutually
// exclusive.
//
// Not all targets implemented here are tested.

#[cfg(target_os = "linux")]
pub(super) static SYSTEM_ID: SyncLazy<(u128, bool, bool)> = SyncLazy::new(|| {
	use self::os::{get_boot_id, get_dbus_machine_id, get_host_id};

	read_from_env!()
		.or_else(|| filter_id!(get_dbus_machine_id()))
		.or_else(|| filter_id!(get_boot_id()))
		.or_else(|| filter_id!(get_host_id()))
		.expect(ERROR_MESSAGE)
});

#[cfg(target_os = "android")]
pub(super) static SYSTEM_ID: SyncLazy<(u128, bool, bool)> = SyncLazy::new(|| {
	use self::os::{get_boot_id, get_host_id};

	read_from_env!()
		.or_else(|| filter_id!(get_boot_id()))
		.or_else(|| filter_id!(get_host_id()))
		.expect(ERROR_MESSAGE)
});

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
pub(super) static SYSTEM_ID: SyncLazy<(u128, bool, bool)> = SyncLazy::new(|| {
	use self::os::{get_dbus_machine_id, get_dmi_product_id, get_host_id, get_kernel_host_id};

	read_from_env!()
		.or_else(|| filter_id!(get_dbus_machine_id()))
		.or_else(|| filter_id!(get_kernel_host_id()))
		.or_else(|| filter_id!(get_dmi_product_id()))
		.or_else(|| filter_id!(get_host_id()))
		.expect(ERROR_MESSAGE)
});

#[cfg(any(target_os = "macos"))]
pub(super) static SYSTEM_ID: SyncLazy<(u128, bool, bool)> = SyncLazy::new(|| {
	use self::os::{get_host_id, get_platform_serial_number};

	read_from_env!()
		.or_else(|| filter_id!(get_platform_serial_number()))
		.or_else(|| filter_id!(get_host_id()))
		.expect(ERROR_MESSAGE)
});

#[cfg(any(target_os = "windows"))]
pub(super) static SYSTEM_ID: SyncLazy<(u128, bool, bool)> = SyncLazy::new(|| {
	use self::os::{get_install_date, get_machine_id, get_product_id};

	read_from_env!()
		.or_else(|| filter_id!(get_machine_id()))
		.or_else(|| filter_id!(get_product_id()))
		.or_else(|| filter_id!(get_install_date()))
		.expect(ERROR_MESSAGE)
});
