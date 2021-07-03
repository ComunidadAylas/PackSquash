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
	let build_year = build_date.year();
	println!("cargo:rustc-env=BUILD_DATE={}", build_date.format("%F"));
	println!("cargo:rustc-env=BUILD_YEAR={}", build_year);

	// For Windows, generate a script to set the executable resource data
	set_windows_executable_resource_data();
}

/// Generates a PowerShell script that sets the resource data of the executable generated
/// by this build. This resource data embeds an icon in the file and makes it look nice.
fn set_windows_executable_resource_data() {
	#[cfg(windows)]
	{
		use std::{env, fs};

		fn escape_quotations(string: String) -> String {
			string.replace('\'', "''").replace('"', "\\\"")
		}

		let script = format!(
			r#"
# Automatically generated PowerShell script to set PackSquash executable resource data.
# Invoke after the executable is built

# -----
$rcedit_download_url = 'https://github.com/electron/rcedit/releases/download/v1.1.1/rcedit-x64.exe'
$packsquash_exe = '{}\target\{}\{}\{}.exe'
$icon_path = '{}\src\app_icon.ico'
$basename = Split-Path "$packsquash_exe" -Leaf
$name = 'PackSquash'
$company = 'Comunidad Aylas'
$description = '{}'
$cargo_version = '{}'
$semver_version = '{}'
$authors = '{}'
# -----

$rcedit = New-TemporaryFile | Rename-Item -NewName {{ $_.Name -replace '.tmp', '.exe' }} -PassThru
Invoke-WebRequest -Uri "$rcedit_download_url" -OutFile $rcedit
& "$rcedit" "$packsquash_exe" `
--set-version-string 'ProductName' "$name" `
--set-version-string 'FileDescription' "$name - $description" `
--set-version-string 'LegalCopyright' "$authors" `
--set-version-string 'CompanyName' "$company" `
--set-version-string 'FileVersion' "$cargo_version" `
--set-version-string 'ProductVersion' "$semver_version" `
--set-version-string 'OriginalFilename' "$basename" `
--set-version-string 'InternalName' "$basename" `
--set-icon "$icon_path""#,
			escape_quotations(env::var("CARGO_MANIFEST_DIR").unwrap()),
			escape_quotations(env::var("TARGET").unwrap()),
			escape_quotations(env::var("PROFILE").unwrap()),
			escape_quotations(env::var("CARGO_PKG_NAME").unwrap()),
			escape_quotations(env::var("CARGO_MANIFEST_DIR").unwrap()),
			escape_quotations(env::var("CARGO_PKG_DESCRIPTION").unwrap()),
			escape_quotations(env::var("CARGO_PKG_VERSION").unwrap()),
			// Can't actually get the Git semver here due to vergen limitations,
			// so use the closest thing available instead
			escape_quotations(env::var("CARGO_PKG_VERSION").unwrap()),
			escape_quotations(env::var("CARGO_PKG_AUTHORS").unwrap())
		);

		fs::write("target/set_executable_resource_data.ps1", script)
			.expect("Couldn't create the script that changes the executable resource metadata");
	}
}
