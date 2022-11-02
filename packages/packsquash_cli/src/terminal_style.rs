use std::env;

fn terminal_can_display_color() -> bool {
	let term = env::var_os("TERM");
	let term_is_dumb = |term| term != "dumb" && term != "unknown";

	term.map_or(
		// On Unix-like platforms, TERM must be set, or else things are probably
		// broken and colors won't work. On Windows and other platforms that is not
		// the case. We assume that if the terminal is not dumb then colors are supported
		cfg!(not(unix)),
		term_is_dumb
	)
}

/// Checks whether the running environment (i.e. terminal and environment variables combination)
/// allows displaying color.
pub fn environment_allows_color(is_tty: bool) -> bool {
	if let Some(color_mode) = env::var_os("PACKSQUASH_COLOR").or_else(|| env::var_os("COLOR")) {
		// The PACKSQUASH_COLOR and COLOR environment variables, in that priority order,
		// allow overriding any decision made
		color_mode == "show"
	} else if env::var_os("NO_COLOR").is_some() {
		// The NO_COLOR environment variable allows disabling color no matter what
		false
	} else {
		// Otherwise, if no special environment variable is set, just check if the
		// terminal can reliably display color
		is_tty && terminal_can_display_color()
	}
}

fn terminal_can_display_emoji() -> bool {
	if cfg!(windows) {
		// On Windows, the new Windows Terminal and Unix-like terminals support emojis.
		// Any sane Windows install has fonts set up in a way that will show emojis
		env::var_os("WT_SESSION").is_some() || env::var_os("TERM").is_some()
	} else {
		// Only macOS reliably supports emojis. UTF-8 support is almost universal on Unixes,
		// but the monospaced fonts used in terminal emulators usually do not contain emojis,
		// and the fallback to fonts that contain them requires a cooperative terminal emulator
		// and is brittle. It may be necessary to tweak fontconfig files in a way that does not
		// break anything else. See: https://gist.github.com/IgnoredAmbience/7c99b6cf9a8b73c9312a71d1209d9bbb
		cfg!(target_os = "macos")
	}
}

/// Checks whether the running environment (i.e. terminal and environment variables combination)
/// allows displaying emoji.
pub fn environment_allows_emoji(is_tty: bool) -> bool {
	if let Some(emoji_mode) = env::var_os("PACKSQUASH_EMOJI").or_else(|| env::var_os("EMOJI")) {
		// The PACKSQUASH_EMOJI and EMOJI environment variables, in that priority order,
		// allow overriding any decision made
		emoji_mode == "show"
	} else if env::var_os("NO_EMOJI").is_some() {
		// The NO_EMOJI environment variable allows disabling emojis no matter what
		false
	} else {
		// Otherwise, if no special environment variable is set, just check if the
		// terminal can reliably display emojis
		is_tty && terminal_can_display_emoji()
	}
}
