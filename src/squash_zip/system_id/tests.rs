//! NOTE: these tests should be run on devices that support all the platform-specific
//! methods to get system IDs that are applicable. It is also assumed that the ID has
//! not been tampered with and was generated as usual.

use super::*;

#[test]
fn works() {
	let system_id = get_or_compute_system_id();

	eprintln!("System ID: {:?}", system_id);

	assert!(
		system_id.has_high_entropy,
		"Expected a high entropy system ID in test environments"
	)
}

#[test]
#[cfg(target_os = "linux")]
fn dbus_machine_id_works() {
	use super::os::get_dbus_machine_id;

	eprintln!(
		"dbus_machine_id: {:?}",
		get_dbus_machine_id()
			.expect("Assuming an appropriate environment, this should return a system ID")
	)
}

#[test]
#[cfg(any(target_os = "linux", target_os = "android"))]
fn get_boot_id_works() {
	use super::os::get_boot_id;

	eprintln!(
		"boot_id: {:?}",
		get_boot_id().expect("Assuming an appropriate environment, this should return a system ID")
	)
}

#[test]
#[cfg(target_os = "macos")]
fn platform_serial_number_works() {
	use super::os::get_platform_serial_number;

	eprintln!(
		"platform_serial_number: {:?}",
		get_platform_serial_number()
			.expect("Assuming an appropriate environment, this should return a system ID")
	)
}

#[test]
#[cfg(unix)]
fn host_id_works() {
	use super::os::get_host_id;

	let host_id = get_host_id();

	eprintln!(
		"host_id: {:?}",
		if cfg!(target_os = "macos") {
			// gethostid() is known to be buggy on macOS and return all zeros sometimes, so this can fail.
			// See: https://bug-coreutils.gnu.narkive.com/4cnKKtfD/workaround-for-hostid-on-darwin-8-macppc
			host_id
		} else {
			host_id.expect("Assuming an appropriate environment, this should return a system ID")
		}
	)
}

#[test]
#[cfg(windows)]
fn machine_id_works() {
	use super::os::get_machine_id;

	eprintln!(
		"machine_id: {:?}",
		get_machine_id()
			.expect("Assuming an appropriate environment, this should return a system ID")
	)
}

#[test]
#[cfg(windows)]
fn product_id_works() {
	use super::os::get_product_id;

	eprintln!(
		"product_id: {:?}",
		get_product_id()
			.expect("Assuming an appropriate environment, this should return a system ID")
	)
}

#[test]
#[cfg(windows)]
fn install_date_works() {
	use super::os::get_install_date;

	eprintln!(
		"install_date: {:?}",
		get_install_date()
			.expect("Assuming an appropriate environment, this should return a system ID")
	)
}
