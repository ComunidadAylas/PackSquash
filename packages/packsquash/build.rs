use git2::opts::set_verify_owner_validation;
use git2::{DescribeFormatOptions, DescribeOptions, Repository};
use packsquash_options::{GlobalOptionsFieldName, SquashOptions};
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::schema::Schema;
use std::env::current_dir;
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};
use time::OffsetDateTime;

fn main() {
	set_build_metadata_environment_variables();
	generate_options_file_json_schema();
}

fn set_build_metadata_environment_variables() {
	// The environment variables read here are documented at:
	// https://doc.rust-lang.org/cargo/reference/environment-variables.html

	println!(
		"cargo:rustc-env=BUILD_VERSION={}",
		git_version().unwrap_or_else(|err| {
			println!(
				"cargo:warning=Could not get version via git: {err}. \
				Falling back to Cargo package version"
			);
			format!("v{}", env!("CARGO_PKG_VERSION"))
		})
	);

	println!(
		"cargo:rustc-env=CARGO_TARGET_TRIPLE={}",
		env::var("TARGET").unwrap_or_else(|_| String::from("unknown"))
	);
	println!(
		"cargo:rustc-env=CARGO_PROFILE={}",
		env::var("PROFILE").unwrap_or_else(|_| String::from("unknown"))
	);

	let build_date = OffsetDateTime::now_utc();
	let (build_year, build_month, build_day) = build_date.to_calendar_date();
	println!(
		"cargo:rustc-env=BUILD_DATE={}-{:02}-{:02}",
		build_year, build_month as u8, build_day
	);
	println!("cargo:rustc-env=BUILD_YEAR={build_year}");
}

fn generate_options_file_json_schema() {
	let options_schema_path =
		PathBuf::from(env::var("OUT_DIR").unwrap()).join("options_file_schema.json");

	let mut options_schema_settings = SchemaSettings::draft07();
	// Inlining subschemas generates a smaller, more readable schema that
	// works better with VS Code extensions such as tamasfe.even-better-toml
	options_schema_settings.inline_subschemas = true;

	let mut options_schema =
		SchemaGenerator::new(options_schema_settings).into_root_schema_for::<SquashOptions>();

	// Remove system-specific default values for non-required options. This is necessary
	// because #[schemars(skip_serializing)] makes the option required and does not represent
	// the desired semantics
	for global_option in [
		GlobalOptionsFieldName::Threads.name(),
		GlobalOptionsFieldName::MaximumScratchFilesBuffersSize.name()
	] {
		if let Some(Schema::Object(schema_object)) = options_schema
			.schema
			.object()
			.properties
			.get_mut(global_option)
		{
			schema_object.metadata().default = None;
		}
	}

	fs::write(
		&options_schema_path,
		serde_json::to_string_pretty(&options_schema).unwrap()
	)
	.expect("Could not generate options file JSON schema");

	println!(
		"cargo:OPTIONS_FILE_JSON_SCHEMA={}",
		options_schema_path.to_str().unwrap()
	);
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
