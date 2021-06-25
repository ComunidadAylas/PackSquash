//! NOTE: these tests should be run on devices that support all the platform-specific
//! methods to get system IDs that are applicable.

use super::*;

#[test]
fn works() {
	SYSTEM_ID.0;
}

#[test]
#[cfg(target_os = "linux")]
fn dbus_machine_id_works() {
	use super::os::get_dbus_machine_id;

	get_dbus_machine_id()
		.expect("Assuming an appropriate environment, this should return a system ID");
}

#[test]
#[cfg(any(target_os = "linux", target_os = "android"))]
fn get_boot_id_works() {
	use super::os::get_boot_id;

	get_boot_id().expect("Assuming an appropriate environment, this should return a system ID");
}

#[test]
#[cfg(target_os = "macos")]
fn platform_serial_number_works() {
	use super::os::get_platform_serial_number;

	get_platform_serial_number()
		.expect("Assuming an appropriate environment, this should return a system ID");
}

#[test]
#[cfg(unix)]
fn host_id_works() {
	use super::os::get_host_id;

	get_host_id().expect("Assuming an appropriate environment, this should return a system ID");
}

#[test]
#[cfg(windows)]
fn machine_id_works() {
	use super::os::get_machine_id;

	get_machine_id().expect("Assuming an appropriate environment, this should return a system ID");
}

#[test]
#[cfg(windows)]
fn product_id_works() {
	use super::os::get_product_id;

	get_product_id().expect("Assuming an appropriate environment, this should return a system ID");
}

#[test]
#[cfg(windows)]
fn install_date_works() {
	use super::os::get_install_date;

	get_install_date().expect("Assuming an appropriate environment, this should return a system ID");
}
