//! Contains miscellaneous small helper functions that are used for processing pack files.

/// The Unicode byte order mark character.
#[cfg(test)]
pub static BOM: char = '\u{feff}';
/// The UTF-8 representation of the Unicode byte order mark character.
pub static BOM_UTF8: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// Returns a slice of the input buffer that skips the initial byte order
/// mark character that some programs (most notably, Microsoft software,
/// like Notepad) add to UTF-8 encoded files. If no BOM is present, the
/// buffer is returned as-is.
pub fn strip_utf8_bom(buf: &[u8]) -> &[u8] {
	let start_index = if starts_with_bom(buf) {
		BOM_UTF8.len()
	} else {
		0
	};

	&buf[start_index..]
}

/// Checks whether the specified byte buffer begins with a byte order mark
/// character.
pub fn starts_with_bom<T: AsRef<[u8]>>(buf: T) -> bool {
	let buf = buf.as_ref();

	// These bytes are the UTF-8 representation of character 0xFEFF
	// (BYTE ORDER MARK)
	buf.len() >= BOM_UTF8.len() && buf[..BOM_UTF8.len()] == BOM_UTF8
}
