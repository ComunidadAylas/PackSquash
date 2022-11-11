use std::cell::Cell;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::{env, io};

use atty::Stream;
use winapi::um::wincon::SetConsoleTitleW;
use winapi_util::console::Console;

use super::{write_ansi_set_window_title_escape_sequence, TerminalTitleSetterTrait};

/// A terminal title setter for Windows platforms.
pub struct WindowsTerminalTitleSetter {
	title_strategy: WindowsTitleStrategy
}

/// Represents all the known strategies to set a console or terminal title.
enum WindowsTitleStrategy {
	AnsiEscapeCodes(Stream),
	WindowsConsoleApi
}

impl TerminalTitleSetterTrait for WindowsTerminalTitleSetter {
	type TerminalTitleString = WindowsTerminalTitleString;

	fn init() -> Option<Self> {
		// Unix-style environment variable set by Unix-like terminal emulators on Windows (e.g. mintty)
		let terminal_emulator = env::var("TERM").ok();

		// The legacy Windows console host and Windows Terminal do not set TERM, but other Unix-like
		// terminal emulators do. The last ones usually support ANSI escape sequences. The Windows console
		// host might support them, and Windows Terminal always supports escape sequences.
		// If we're using a Unix-like dumb terminal emulator, then we know that ANSI escape codes should
		// not be used. Cygwin can do weird things which usually mean that ANSI escape codes are broken
		let terminal_emulator_might_support_ansi_escape_codes = !matches!(
			terminal_emulator.as_deref(),
			Some("dumb" | "unknown" | "cygwin")
		);

		// We know with more certainty that ANSI escape codes are supported if we're using an Unix-like
		// terminal emulator that's not known to lack support for them, or Windows Terminal
		let terminal_emulator_supports_ansi_escape_codes =
			terminal_emulator_might_support_ansi_escape_codes
				&& (terminal_emulator.is_some() || env::var_os("WT_SESSION").is_some());

		// We can use ANSI escape codes if the terminal might support them, and it's known that it
		// either supports them for sure or that we can enable support for them using the Windows API
		let ansi_escape_codes_stream = if terminal_emulator_might_support_ansi_escape_codes {
			if terminal_emulator_supports_ansi_escape_codes {
				// This is the easy case: just check if any stream is attached to an interactive
				// terminal. Use the atty crate to handle both standard Windows consoles, including
				// Windows Terminal, and some Unix-like terminals
				if atty::is(Stream::Stdout) {
					Some(Stream::Stdout)
				} else if atty::is(Stream::Stderr) {
					Some(Stream::Stderr)
				} else {
					None
				}
			} else {
				// We assume that the terminal emulator does not support ANSI escape codes, at least
				// without asking. We can try enabling ANSI escape code support, and fall back to
				// using the Windows console API if that fails.
				// We will usually have a console handle, because we target the command-line
				// subsystem. However, a console might not be available if we're run as a
				// detached process.
				// See: https://docs.microsoft.com/en-us/windows/console/creation-of-a-console
				// When a console is available, both stdout and stderr point to it by default
				// (IOW, both streams share the same console, so it doesn't matter what stream
				// we choose to get the console of). But check both anyway, to handle redirections.
				// See: https://docs.microsoft.com/en-us/windows/console/getstdhandle#remarks
				Console::stdout().map_or_else(
					|_| {
						// stdout is not associated with a console. Try with stderr
						Console::stderr().map_or_else(
							|_| {
								// stderr is not associated with a console either. Give up
								None
							},
							|console| {
								// stderr is associated with a console
								enable_vt_processing(console).and_then(|_| Some(Stream::Stderr))
							}
						)
					},
					|console| {
						// stdout is associated with a console
						enable_vt_processing(console).and_then(|_| Some(Stream::Stdout))
					}
				)
			}
		} else {
			// No way we can use ANSI escape codes
			None
		};

		ansi_escape_codes_stream.map_or_else(
			|| {
				// We can't use ANSI escape sequences, but let's try with the Windows console API
				// calls. They don't require us to pass a console and will work if there is a console
				// somewhere, even if it's only on the standard input stream. If there is not a console
				// anywhere we'll just waste time doing the call, but that's not that bad
				Some(Self {
					title_strategy: WindowsTitleStrategy::WindowsConsoleApi
				})
			},
			|escape_codes_stream| {
				// We can use ANSI escape sequences on some standard stream
				Some(Self {
					title_strategy: WindowsTitleStrategy::AnsiEscapeCodes(escape_codes_stream)
				})
			}
		)
	}

	fn set_title(&self, title: &WindowsTerminalTitleString) {
		match &self.title_strategy {
			WindowsTitleStrategy::AnsiEscapeCodes(Stream::Stdout) => {
				write_ansi_set_window_title_escape_sequence(io::stdout(), title.string)
			}
			WindowsTitleStrategy::AnsiEscapeCodes(Stream::Stderr) => {
				write_ansi_set_window_title_escape_sequence(io::stderr(), title.string)
			}
			WindowsTitleStrategy::AnsiEscapeCodes(_) => {
				// Can't print ANSI escape codes to stdin. This is an internal error
				unreachable!()
			}
			WindowsTitleStrategy::WindowsConsoleApi => {
				let mut utf16_string = title.utf16_string.take();

				// SAFETY: system calls are unsafe. We borrow a Vec whose lifetime
				// is at least as long as this function execution, so the pointer
				// passed to SetConsoleTitleW stays valid
				#[allow(unsafe_code)]
				unsafe {
					SetConsoleTitleW(
						utf16_string
							.get_or_insert_with(|| OsStr::new(title).encode_wide().collect())
							.as_ptr()
					);
				}

				title.utf16_string.set(utf16_string);
			}
		}
	}
}

/// A string that can be used to change a terminal title.
pub struct WindowsTerminalTitleString {
	string: &'static str,
	utf16_string: Cell<Option<Vec<u16>>>
}

impl From<&'static str> for WindowsTerminalTitleString {
	fn from(title: &'static str) -> Self {
		Self {
			string: title,
			utf16_string: Cell::new(None)
		}
	}
}

/// Enables virtual terminal proccessing (i.e. ANSI escape sequence support) for
/// the specified console. `Some(())` is returned if the VT processing mode
/// could be enabled; otherwise, `None` is returned.
fn enable_vt_processing(mut console: Console) -> Option<()> {
	console.set_virtual_terminal_processing(true).ok()
}
