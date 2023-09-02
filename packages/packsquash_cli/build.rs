use std::env;

/// Initializes environment variables that will be accessible in the source
/// code via the env! macro, and takes care of build-time metadata.
fn main() {
	// The environment variables read here are documented at:
	// https://doc.rust-lang.org/cargo/reference/environment-variables.html

	println!("cargo:rerun-if-env-changed=PACKSQUASH_BUILD_VERSION");
	println!("cargo:rerun-if-env-changed=PACKSQUASH_BUILD_DATE");

	// Set variable for the build version if absent
	if option_env!("PACKSQUASH_BUILD_VERSION").is_none() {
		println!(
			"cargo:rustc-env=PACKSQUASH_BUILD_VERSION=v{}{}",
			env!("CARGO_PKG_VERSION"),
			option_env!("CARGO_PRIMARY_PACKAGE")
				.map(|_| "-custom")
				.unwrap_or_default()
		);
	}

	// Set variables for the target triple and build profile
	println!(
		"cargo:rustc-env=CARGO_TARGET_TRIPLE={}",
		env::var("TARGET").unwrap_or_else(|_| String::from("unknown"))
	);
	println!(
		"cargo:rustc-env=CARGO_PROFILE={}",
		env::var("PROFILE").unwrap_or_else(|_| String::from("unknown"))
	);

	// Set variables for build date
	if option_env!("PACKSQUASH_BUILD_DATE").is_none() {
		println!("cargo:rustc-env=PACKSQUASH_BUILD_DATE=unknown build date");
	}
	let build_year =
		option_env!("PACKSQUASH_BUILD_DATE").and_then(|build_date| build_date.split('-').next());
	println!(
		"cargo:rustc-env=PACKSQUASH_COPYRIGHT_BUILD_YEAR_SUFFIX={}{}",
		if build_year.is_some() { " " } else { "" },
		build_year.unwrap_or_default()
	);

	// Add platform-specific metadata to the executable
	add_executable_metadata();
}

#[cfg(windows)]
fn add_executable_metadata() {
	let mut windows_resource = winresource::WindowsResource::new();
	windows_resource.set("LegalCopyright", env!("CARGO_PKG_AUTHORS"));
	windows_resource.set(
		"FileDescription",
		&format!("PackSquash - {}", env!("CARGO_PKG_DESCRIPTION"))
	);

	windows_resource.set_language(0x0409); // English (US)
	windows_resource.set_icon("../../common/assets/app_icon.ico");

	windows_resource
		.compile()
		.expect("Windows executable resource build failure");
}

#[cfg(not(windows))]
fn add_executable_metadata() {}
