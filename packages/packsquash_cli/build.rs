//! Build helper for the `PackSquash` CLI.

use std::env;
use std::process::Command;
use std::time::SystemTime;
use winresource::WindowsResource;

/// Initializes environment variables that will be accessible in the source
/// code via the env! macro, and takes care of build-time metadata.
fn main() {
	// The environment variables read here are documented at:

	let git_version = Command::new("git")
		.args(["describe", "--tags", "--dirty=-custom", "--always"])
		.output()
		.ok()
		.filter(|output| output.status.success())
		.and_then(|output| String::from_utf8(output.stdout).ok());

	// Set environment variables for the build version and time
	println!(
		"cargo:rustc-env=PACKSQUASH_BUILD_VERSION={}",
		match git_version {
			Some(git_version) => git_version.lines().next().unwrap().into(),
			None => format!(
				"v{}{}",
				env!("CARGO_PKG_VERSION"),
				option_env!("CARGO_PRIMARY_PACKAGE")
					.map(|_| "-custom")
					.unwrap_or_default()
			)
		}
	);
	println!(
		"cargo:rustc-env=PACKSQUASH_BUILD_TIMESTAMP={}",
		SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.expect("System time before UNIX epoch")
			.as_secs()
	);

	// Set variables for the target triple and build profile.
	// See: https://doc.rust-lang.org/cargo/reference/environment-variables.html
	println!(
		"cargo:rustc-env=CARGO_TARGET_TRIPLE={}",
		env::var("TARGET").unwrap()
	);
	println!(
		"cargo:rustc-env=CARGO_PROFILE={}",
		env::var("PROFILE").unwrap()
	);

	set_windows_executable_metadata();
}

fn set_windows_executable_metadata() {
	// Build scripts are always compiled for the host target, so to make sure metadata is
	// added when cross-compiling to Windows, we need to check the target OS during runtime
	if env::var_os("CARGO_CFG_WINDOWS").is_none() {
		return;
	}

	let mut windows_resource = WindowsResource::new();
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
