#[cfg(test)]
mod tests;

/// Decodes an hexadecimal string composed of an even number of hex digits (`0-9`,
/// `a-f`, `A-F`) to a freshly allocated byte vector.
///
/// `None` is returned if the string contains invalid hex digits or has an odd length.
///
/// The implementation of this function is optimized for a balance between performance,
/// simplicity, and code size. It performs faster and is simpler than the `hex` crate
/// for many inputs by virtue of not having detailed error reporting and using a small
/// lookup table, but it does not go to the great lengths of using explicit SIMD
/// intrinsics.
pub fn decode_hex(hex_str: &str) -> Option<Vec<u8>> {
	const INVALID_HEX_DIGIT_MARKER: u8 = u8::MAX;
	const HEX_DIGIT_TO_NIBBLE: [u8; 256] = {
		let mut table = [INVALID_HEX_DIGIT_MARKER; 256];

		let mut i = 0;
		while i < table.len() {
			table[i] = match i as u8 {
				b'0'..=b'9' => i as u8 - b'0',
				b'a'..=b'f' => 10 + (i as u8 - b'a'),
				b'A'..=b'F' => 10 + (i as u8 - b'A'),
				_ => INVALID_HEX_DIGIT_MARKER
			};

			i += 1;
		}

		table
	};

	let (hex_digit_pairs, remainder) = hex_str.as_bytes().as_chunks::<2>();

	// We must have an even number of hex digits, as each pair is a byte
	if !remainder.is_empty() {
		return None;
	}

	let mut decoded_bytes = vec![0; hex_digit_pairs.len()];
	for (i, [high_nibble, low_nibble]) in hex_digit_pairs.into_iter().enumerate() {
		let high_nibble = HEX_DIGIT_TO_NIBBLE[*high_nibble as usize];
		let low_nibble = HEX_DIGIT_TO_NIBBLE[*low_nibble as usize];

		// Using a single lookup table and shifting one of the nibbles leads to slightly less assembly
		// code being generated on x86_64
		decoded_bytes[i] = (high_nibble << 4) | low_nibble;

		// Checking for invalid hex digits after the assignment above leads to 25% less assembly
		// lines of code on x86_64
		if high_nibble == INVALID_HEX_DIGIT_MARKER || low_nibble == INVALID_HEX_DIGIT_MARKER {
			return None;
		}
	}

	Some(decoded_bytes)
}
