use std::process::{Command, Stdio};
use std::{env, fs};

fn main() {
	generate_debloated_options_file_json_schema();
	tauri_build::build();
}

fn generate_debloated_options_file_json_schema() {
	let options_file_json_schema_path = env::var("DEP_PACKSQUASH_OPTIONS_FILE_JSON_SCHEMA")
		.expect("Missing PackSquash options file JSON schema file");

	// Debloat descriptions we won't need from the schema. jq also implicitly minifies the JSON data
	let debloated_options_file_json_schema = String::from_utf8(
		Command::new("jq")
			.args([
				if env::var("PROFILE") == Ok("release".into()) {
					"--compact-output"
				} else {
					// No-op option
					"-r"
				},
				"del(.. | .description?) | del(.title)",
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
		&debloated_options_file_json_schema
	)
	.expect("Could not write the PackSquash options file JSON schema");
}
