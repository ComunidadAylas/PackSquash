/// Takes care of build-time metadata.
fn main() {
	// Add platform-specific metadata to the executable
	add_executable_metadata();
}

#[cfg(windows)]
fn add_executable_metadata() {
	let mut windows_resource = winresource::WindowsResource::new();
	windows_resource.set("LegalCopyright", env!("CARGO_PKG_AUTHORS"));
	windows_resource.set(
		"FileDescription",
		&format!("PackSquash - {}", env!("CARGO_PKG_DESCRIPTION"))
	);

	windows_resource.set_language(0x0409); // English (US)
	windows_resource.set_icon("../../icons/packsquash_icon_256x256.ico");

	windows_resource
		.compile()
		.expect("Windows executable resource build failure");
}

#[cfg(not(windows))]
fn add_executable_metadata() {}
