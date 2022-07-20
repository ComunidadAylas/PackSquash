use time::OffsetDateTime;
use vergen::{vergen, SemverKind};

/// Initializes environment variables that will be accessible in the source
/// code via the env! macro, and takes care of build-time metadata.
fn main() {
	let mut vergen_config = vergen::Config::default();
	*vergen_config.cargo_mut().features_mut() = false;
	*vergen_config.cargo_mut().profile_mut() = true;
	*vergen_config.cargo_mut().target_triple_mut() = true;
	*vergen_config.git_mut().branch_mut() = false;
	*vergen_config.git_mut().commit_timestamp_mut() = false;
	*vergen_config.git_mut().semver_mut() = true;
	*vergen_config.git_mut().semver_kind_mut() = SemverKind::Lightweight;
	*vergen_config.git_mut().semver_dirty_mut() = Some("-custom");
	*vergen_config.git_mut().sha_mut() = false;

	// Generate the 'cargo:' key output that populate the target triple and version envrionment variables
	vergen(vergen_config).expect("Vergen failure");

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
