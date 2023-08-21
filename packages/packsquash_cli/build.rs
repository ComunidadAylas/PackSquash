use chrono::{Datelike, TimeZone, Utc};
use git2::opts::set_verify_owner_validation;
use git2::{DescribeFormatOptions, DescribeOptions, Repository};
use std::env;
use std::env::current_dir;
use std::error::Error;
use std::time::SystemTime;

/// Initializes environment variables that will be accessible in the source
/// code via the env! macro, and takes care of build-time metadata.
fn main() {
	// The environment variables read here are documented at:
	// https://doc.rust-lang.org/cargo/reference/environment-variables.html

	// Set variable for the build version
	println!(
		"cargo:rustc-env=BUILD_VERSION={}",
		git_version().unwrap_or_else(|err| {
			println!(
				"cargo:warning=Could not get version via git: {err}. \
				Falling back to Cargo package version"
			);
			String::from(env!("CARGO_PKG_VERSION"))
		})
	);

	// Set variables for the target triple and build profile
	println!(
		"cargo:rustc-env=CARGO_TARGET_TRIPLE={}",
		env::var("TARGET").unwrap_or_else(|_| String::from("unknown"))
	);
	println!(
		"cargo:rustc-env=CARGO_PROFILE={}",
		env::var("PROFILE").unwrap_or_else(|_| String::from("unknown"))
	);

	// Set variables with the build dates, for copyright and version strings
	let build_date = Utc
		.timestamp_opt(
			SystemTime::UNIX_EPOCH
				.elapsed()
				.expect("The system clock is set behind the UNIX epoch")
				.as_secs()
				.try_into()
				.expect("The system clock is too far in the future"),
			0
		)
		.single()
		.expect("The system clock is too far in the future");

	println!(
		"cargo:rustc-env=BUILD_DATE={}-{:02}-{:02}",
		build_date.year(),
		build_date.month(),
		build_date.day()
	);
	println!("cargo:rustc-env=BUILD_YEAR={}", build_date.year());

	// Add platform-specific metadata to the executable
	add_executable_metadata();
}

fn git_version() -> Result<String, Box<dyn Error>> {
	// CI workflows run on Debian Bullseye, which ships with git 2.30. This older
	// version of Git is vulnerable to CVE-2022-24765 and doesn't implement the
	// configuration option that libgit2 polls for, so it causes a failure. However,
	// this security vulnerability is not applicable to CI runners, so if we're on
	// CI, disable the troublesome checks
	if env::var_os("CI").is_some() {
		unsafe { set_verify_owner_validation(false) }.unwrap();
	}

	// The current directory is set to the source file directory for this package.
	// Find the repo directory backtracking on the file tree
	let repo = Repository::discover(current_dir()?)?;

	// Make sure we're executed if HEAD changes
	println!("cargo:rerun-if-changed={:?}", repo.path().join("HEAD"));

	// Run the equivalent of git describe --tags --dirty=-custom --always
	let head_description = repo.describe(
		DescribeOptions::new()
			.describe_tags()
			.show_commit_oid_as_fallback(true)
	)?;

	let version_string =
		head_description.format(Some(DescribeFormatOptions::new().dirty_suffix("-custom")))?;

	Ok(version_string)
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
