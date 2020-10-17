use time::OffsetDateTime;
use vergen::{ConstantsFlags, generate_cargo_keys};

/// Initializes environment variables that will be accessible in the source
/// code via the env! macro, and takes care of build-time metadata.
fn main() {
	// Setup the flags that enable the required environment variables
	let flags = ConstantsFlags::from_bits(
		ConstantsFlags::BUILD_DATE.bits() |
		ConstantsFlags::TARGET_TRIPLE.bits() |
		ConstantsFlags::SEMVER_LIGHTWEIGHT.bits()
	).unwrap();

	// Generate the 'cargo:' key output that will do the magic
	if let Err(error) = generate_cargo_keys(flags) {
		eprintln!(
			"W: Couldn't generate Cargo keys. This is normal for custom builds outside a repository. Details: {}",
			error
		);
	}

	// Set a variable with the build year, for copyright strings
	let build_year = OffsetDateTime::now_utc().year();
	println!("cargo:rustc-env=BUILD_YEAR={}", build_year);

	// Add platform-specific metadata to the executable
	add_exe_metadata(build_year);
}

#[cfg(windows)]
fn add_exe_metadata(build_year: i32) {
	let mut windows_resource = winres::WindowsResource::new();
	windows_resource.set("ProductName", "PackSquash");
	windows_resource.set(
		"LegalCopyright",
		&format!("Copyright (C) {} {}", build_year, env!("CARGO_PKG_AUTHORS"))[..]
	);
	windows_resource.set_language(0x0409); // English (US)
	windows_resource.set_icon("src/app_icon.ico");

    if let Err(error) = windows_resource.compile() {
		eprintln!(
			"W: Couldn't set the metadata of the Windows executable. Details: {}",
			error
		);
	}
}

#[cfg(not(windows))]
fn add_exe_metadata(_build_year: i32) {}
