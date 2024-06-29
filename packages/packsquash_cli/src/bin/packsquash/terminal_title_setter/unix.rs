use std::fs::{File, OpenOptions};
use std::{
	env,
	io::{self, IsTerminal}
};

use super::{write_ansi_set_window_title_escape_sequence, TerminalTitleSetterTrait};

/// A terminal title setter for Unix-like platforms.
pub struct UnixTerminalTitleSetter {
	escape_codes_stream: AnsiEscapeCodesStream
}

/// Enumerates all the possible output streams where ANSI escape sequences can be
/// written to change the terminal title.
enum AnsiEscapeCodesStream {
	Stdout,
	Stderr,
	ControllingTty(File)
}

impl<'title> TerminalTitleSetterTrait<'title> for UnixTerminalTitleSetter {
	type TerminalTitleString = UnixTerminalTitleString<'title>;

	fn init() -> Option<Self> {
		if matches!(
			env::var("TERM").ok().as_deref(),
			None | Some("dumb" | "unknown")
		) {
			// This is a dumb or unknown terminal that very likely does not support ANSI escape codes
			None
		} else {
			// It is very likely that this terminal supports ANSI escape codes for setting the title.
			// Check if any output stream is connected to a terminal. Even if no stream is connected
			// to a terminal, the controlling terminal of the process may exist and be writable, so
			// try that too
			if io::stdout().is_terminal() {
				Some(Self {
					escape_codes_stream: AnsiEscapeCodesStream::Stdout
				})
			} else if io::stderr().is_terminal() {
				Some(Self {
					escape_codes_stream: AnsiEscapeCodesStream::Stderr
				})
			} else {
				controlling_terminal()
					.and_then(|ctty_path| OpenOptions::new().write(true).open(ctty_path).ok())
					.map(|ctty| Self {
						escape_codes_stream: AnsiEscapeCodesStream::ControllingTty(ctty)
					})
			}
		}
	}

	fn set_title(&self, title: &'title UnixTerminalTitleString) {
		match &self.escape_codes_stream {
			AnsiEscapeCodesStream::Stdout => {
				write_ansi_set_window_title_escape_sequence(io::stdout(), title.0);
			}
			AnsiEscapeCodesStream::Stderr => {
				write_ansi_set_window_title_escape_sequence(io::stderr(), title.0);
			}
			AnsiEscapeCodesStream::ControllingTty(ctty) => {
				write_ansi_set_window_title_escape_sequence(ctty, title.0);
			}
		}
	}
}

/// A string that can be used to change a terminal title.
#[repr(transparent)]
pub struct UnixTerminalTitleString<'title>(&'title str);

impl<'title> From<&'title str> for UnixTerminalTitleString<'title> {
	fn from(title: &'title str) -> Self {
		Self(title)
	}
}

/// Returns a file path to the controlling terminal of this process. If this
/// process has no controlling terminal, `None` will be returned.
fn controlling_terminal() -> Option<String> {
	use std::ffi::CStr;
	use std::os::raw::c_char;

	extern "C" {
		/// `char* ctermid(char* s)`, from `#include <stdio.h>`.
		///
		/// Documentation: <https://pubs.opengroup.org/onlinepubs/9699919799/functions/ctermid.html>
		fn ctermid(s: *mut c_char) -> *mut c_char;
	}

	let mut path_buf = [0; 256];

	// SAFETY: system calls are unsafe. ctermid is required by the standard to populate the passed
	// pointer with a valid string, always. If some error happens, then the string is empty, but the
	// pointer contents are valid. Because we bring our own buffer, safe Rust is in full control of
	// the data lifetime. L_ctermid is 9 on Linux, but allocating space for 256 characters should
	// guarantee that the buffer size is always greater than L_ctermid. c_uchar has the same memory
	// layout than c_char and are interchangeable
	#[allow(unsafe_code)]
	unsafe {
		ctermid(path_buf.as_mut_ptr().cast::<c_char>())
	};

	let path = CStr::from_bytes_until_nul(&path_buf[..]);

	path.ok()
		.and_then(|path_cstr| path_cstr.to_str().ok().map(String::from))
		.filter(|path| !path.is_empty())
}
