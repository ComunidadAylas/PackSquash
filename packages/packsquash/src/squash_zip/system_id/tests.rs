//! NOTE: these tests should be run on devices that support all the platform-specific
//! methods to get system IDs that are applicable. It is also assumed that the ID has
//! not been tampered with and was generated as usual.

use super::*;

#[test]
fn works() {
	let system_id = get_or_compute_system_ids();

	eprintln!("System ID: {system_id:?}");
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
#[ignore = "Requires (possibly fake) root permissions to work"]
#[cfg(target_os = "linux")]
fn get_dmi_product_id() {
	use super::os::get_dmi_product_id;

	eprintln!(
		"dmi_product_id: {:?}",
		get_dmi_product_id()
			.expect("Assuming an appropriate environment, this should return a system ID")
	)
}

#[test]
#[cfg(target_os = "linux")]
#[cfg_attr(ci, ignore = "Fails on CI runners")]
fn get_aggregated_dmi_serial_numbers_id() {
	use super::os::get_aggregated_dmi_serial_numbers_id;

	eprintln!(
		"aggregated_dmi_serial_numbers_id: {:?}",
		get_aggregated_dmi_serial_numbers_id()
			.expect("Assuming an appropriate environment, this should return a system ID")
	)
}

#[test]
#[cfg(any(target_os = "linux", target_os = "android"))]
fn boot_id_works() {
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

	eprintln!(
		"host_id: {:?}",
		get_host_id().expect("Assuming an appropriate environment, this should return a system ID")
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
fn system_root_volume_id_works() {
	use super::os::get_system_root_volume_id;

	eprintln!(
		"system_root_volume_id: {:?}",
		get_system_root_volume_id()
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
