//! Contains miscellaneous small helper functions that are used for processing
//! resource pack files.

/// Contains helper functions to strip Unicode byte order marks from text files.
pub(super) mod bom_stripper {
	/// Returns a slice of the input buffer that skips the initial byte order
	/// mark character that some programs (most notably, Microsoft software,
	/// like Notepad) add to UTF-8 encoded files. If no BOM is present, the
	/// buffer is returned as-is.
	pub fn strip_utf8_bom(buf: &[u8]) -> &[u8] {
		// These bytes are the UTF-8 representation of
		// character 0xFEFF (BYTE ORDER MARK)
		if buf.len() > 2 && buf[..3] == [0xEF, 0xBB, 0xBF] {
			&buf[3..]
		} else {
			&buf
		}
	}
}
