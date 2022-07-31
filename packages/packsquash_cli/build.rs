use git2::{DescribeFormatOptions, DescribeOptions, Repository};
use std::env;
use std::env::current_dir;
use std::error::Error;
use time::OffsetDateTime;

/// Initializes environment variables that will be accessible in the source
/// code via the env! macro, and takes care of build-time metadata.
fn main() {
	// Set variable for the build version
	println!(
		"cargo:rustc-env=BUILD_VERSION={}",
		git_version().unwrap_or_else(|err| {
			println!(
				"cargo:warning=Could not get version via git: {}. \
				Falling back to Cargo package version",
				err
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
	let build_date = OffsetDateTime::now_utc();
	let (build_year, build_month, build_day) = build_date.to_calendar_date();
	println!(
		"cargo:rustc-env=BUILD_DATE={}-{:02}-{:02}",
		build_year, build_month as u8, build_day
	);
	println!("cargo:rustc-env=BUILD_YEAR={}", build_year);

	// Add platform-specific metadata to the executable
	add_executable_metadata();
}

fn git_version() -> Result<String, Box<dyn Error>> {
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
	let mut windows_resource = winres::WindowsResource::new();
	windows_resource.set("LegalCopyright", env!("CARGO_PKG_AUTHORS"));
	windows_resource.set(
		"FileDescription",
		&format!(
			"{} - {}",
			env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_DESCRIPTION")
		)
	);

	windows_resource.set_language(0x0409); // English (US)
	windows_resource.set_icon("../../common/assets/app_icon.ico");

	windows_resource
		.compile()
		.expect("Windows executable resource build failure");
}

#[cfg(not(windows))]
fn add_executable_metadata() {}
