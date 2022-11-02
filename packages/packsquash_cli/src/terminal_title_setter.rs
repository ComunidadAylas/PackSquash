use std::io::Write;

#[cfg(unix)]
pub use unix::{
	UnixTerminalTitleSetter as TerminalTitleSetter, UnixTerminalTitleString as TerminalTitleString
};
#[cfg(windows)]
pub use windows::{
	WindowsTerminalTitleSetter as TerminalTitleSetter,
	WindowsTerminalTitleString as TerminalTitleString
};

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

/// Defines the contract that any struct capable of setting the title of the process
/// controlling terminal must follow.
pub trait TerminalTitleSetterTrait<'title> {
	type TerminalTitleString: From<&'title str>;

	/// Initializes a new terminal title setter, possibly checking whether a terminal title
	/// can be set and setting things up for doing so. `None` will be returned if the setup
	/// can't be done for some reason, or there is no way that the terminal title can be
	/// changed (because the process is not associated to any terminal, for example).
	fn init() -> Option<Self>
	where
		Self: Sized;

	/// Sets the current terminal title to the specified string. Any errors that might happen
	/// doing so are silently ignored.
	fn set_title(&self, title: &Self::TerminalTitleString);
}

/// Writes the ANSI escape sequence that sets the terminal title to the provided string to
/// some byte sink that represents a terminal. More precisely, it is an Operating System
/// Control sequence introduced by xterm, which most terminal emulators emulate nowadays.
fn write_ansi_set_window_title_escape_sequence<W: Write>(mut w: W, string: &str) {
	// OSC = ESC ]
	// Ps = 2 = Change Window Title to Pt
	// OSC Ps ; Pt BEL
	// See: https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h3-Tektronix-4014-Mode
	write!(w, "\x1B]2;{string}\x07").ok();

	// Flushing is needed because the stream may be (line-)buffered, preventing
	// the title from being updated promptly
	w.flush().ok();
}
