use std::env;
use std::process::Command;

fn main() {
	println!("cargo:rerun-if-changed=front");

	// Ensure that the frontend distribution assets are present and updated when compiling in release
	// mode, so they get bundled in the executable image. During development (i.e. debug builds), the
	// frontend is either served by a separate process that hot-reloads on its own, and rust-embed ends
	// up reading files directly from the distribution assets directory anyway
	if env::var("PROFILE").unwrap() == "release" {
		assert_eq!(
			Command::new("pnpm")
				.args(["run", "build"])
				.current_dir("front")
				.spawn()
				.expect("Could not invoke the frontend bundler")
				.wait()
				.expect("Could not wait for the frontend bundler to complete")
				.code()
				.unwrap_or(1),
			0,
			"The frontend bundler did not complete successfully"
		);
	}
}
