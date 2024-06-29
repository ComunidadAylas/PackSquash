//! Build helper for the `PackSquash` package.

use std::env;

fn main() {
	println!("cargo:rerun-if-env-changed=CI");
	println!("cargo:rustc-check-cfg=cfg(ci)");

	// Set the "ci" cfg flag to enable conditional compilation tailored to the CI environment
	if env::var_os("CI").is_some() {
		println!("cargo:rustc-cfg=ci");
	}
}
