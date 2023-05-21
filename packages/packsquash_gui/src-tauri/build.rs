use std::{
	env, fs,
	process::{Command, Stdio}
};

use serde::Serialize;

fn main() {
	generate_debloated_options_file_json_schema();
	generate_dependencies_list();
	tauri_build::build();
}

fn generate_debloated_options_file_json_schema() {
	let options_file_json_schema_path = env::var("DEP_PACKSQUASH_OPTIONS_FILE_JSON_SCHEMA")
		.expect("Missing PackSquash options file JSON schema file");

	// Debloat descriptions and system-specific defaults we won't need from the schema. jq also minifies the JSON data
	let debloated_options_file_json_schema = String::from_utf8(
		Command::new("jq")
			.args([
				if env::var("PROFILE") == Ok("release".into()) {
					"--compact-output"
				} else {
					// No-op option
					"-r"
				},
				r#"
					del(.. | .description?) |
					del(.title)
				"#,
				&options_file_json_schema_path
			])
			.stderr(Stdio::null())
			.output()
			.expect("Could not invoke jq")
			.stdout
	)
	.expect("jq output was not UTF-8 encoded");

	fs::write(
		"../src/data/optionsSchema.json",
		debloated_options_file_json_schema
	)
	.expect("Could not write the PackSquash options file JSON schema");
}

fn generate_dependencies_list() {
	let cargo_about_config = toml::from_str(
		&fs::read_to_string("../../../about.toml").expect("Could not read cargo-about configuration")
	)
	.expect("Could not deserialize cargo-about configuration");

	let dependencies = cargo_about::get_all_crates(
		krates::Utf8Path::new("../../../Cargo.toml"),
		false,
		false,
		vec![],
		false,
		&cargo_about_config
	)
	.expect("Could not get crate dependencies");

	#[derive(Serialize)]
	struct Dependency {
		name: String,
		version: String,
		license: Option<String>,
		homepage: String,
		authors: Vec<String>
	}

	let mut dependency_list = Vec::with_capacity(dependencies.len());
	for (name, version, license, homepage, authors) in dependencies.krates().filter_map(|krate| {
		// Ignoring private workspace crates in about.toml does not have any effect when using the cargo-about API.
		// Work around that limitation by ignoring all crates that start with "packsquash"
		(!krate.name.starts_with("packsquash")).then(|| {
			(
				krate.name.clone(),
				&krate.version,
				krate.license.clone(),
				krate.homepage.clone(),
				krate.authors.clone()
			)
		})
	}) {
		dependency_list.push(Dependency {
			// The format! macro below assumes that crates only contain URL-safe characters,
			// which is the case for crates pulled from crates.io. Reference:
			// https://github.com/rust-lang/cargo/issues/2388#issuecomment-420212252
			homepage: homepage.unwrap_or_else(|| format!("https://crates.io/crates/{name}")),
			name,
			version: version.to_string(),
			license,
			authors
		});
	}

	fs::write(
		"../src/data/cargoDependencies.json",
		if env::var("PROFILE") == Ok("release".into()) {
			serde_json::to_string(&dependency_list)
		} else {
			serde_json::to_string_pretty(&dependency_list)
		}
		.expect("JSON serialization error")
	)
	.expect("Could not write dependencies file");
}
