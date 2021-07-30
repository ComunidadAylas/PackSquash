use std::time::SystemTime;

use super::*;

static TWEAK: &[u8; 4] = &[1, 2, 3, 4];

/// Checks that sanitizing a [SystemTime] and then desanitizing it back
/// yields back the closest representable [SystemTime].
fn successful_sanitize_desanitize_cycle_test(time: &SystemTime) {
	let sanitizer = SystemTimeSanitizer::new();

	let desanitized_time = sanitizer.desanitize(
		&sanitizer
			.sanitize(time, TWEAK)
			.expect("No error should occur while sanitizing the specified time"),
		TWEAK
	);

	let unix_timestamp = time
		.duration_since(SystemTime::UNIX_EPOCH)
		.expect("The system clock is set to an invalid date")
		.as_millis();

	let desanitized_unix_timestamp = desanitized_time
		.expect("Expected a sanitized time with a proper stick parity bit value")
		.duration_since(SystemTime::UNIX_EPOCH)
		.expect("The system clock is set to an invalid date")
		.as_millis();

	assert_eq!(
		// Because we actually sanitize the Squash Time, we only have
		// half-second precision
		unix_timestamp - unix_timestamp % 500,
		desanitized_unix_timestamp,
		"The sanitization is not reversible"
	);
}

#[test]
fn sanitization_is_reversible() {
	successful_sanitize_desanitize_cycle_test(&SystemTime::now())
}

#[test]
fn sanitization_accomodates_future_dates() {
	successful_sanitize_desanitize_cycle_test(
		// Approximately ten years into the future
		&(SystemTime::now() + Duration::from_secs(86400 * 365 * 10))
	)
}
