/// Takes care of build-time metadata.
fn main() {
	// Add platform-specific metadata to the executable
	add_executable_metadata();
}

#[cfg(windows)]
fn add_executable_metadata() {
	inresource::WindowsResource::new()
		.set("LegalCopyright", env!("CARGO_PKG_AUTHORS"))
		.set(
			"FileDescription",
			&format!("PackSquash - {}", env!("CARGO_PKG_DESCRIPTION"))
		)
		.set_language(0x0409) // English (US)
		.set_icon("../../icons/packsquash_icon_256x256.ico")
		.compile()
		.expect("Windows executable resource build failure");
}

#[cfg(not(windows))]
fn add_executable_metadata() {}
