use std::env;

fn main() {
	println!("cargo:rerun-if-env-changed=CI");

	// Set the "ci" cfg flag to enable conditional compilation tailored to the CI environment
	if env::var_os("CI").is_some() {
		println!("cargo:rustc-cfg=ci");
	}
}
