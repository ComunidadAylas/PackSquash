// Parses a date string encoded on the Rust's time crate OffsetDateTime debug format.
// If the string is not valid according to the format, an invalid date will be returned.
export default (dateString: string) => {
  // The debug format of a Rust's time crate OffsetDateTime struct is defined by
  // the output of its fmt::Display implementation, which is straightforward enough,
  // but does not conform to any standard user agents already know how to parse.
  // Therefore, start by splitting the date into its components

  const [date, time, offset] = dateString.split(" ", 3);

  // Components may be padded with zeros to fill some fixed length
  const [year, month, day] = date.split("-", 3);
  const [timeComponent, nanosecond] = time.split(".", 2);
  const [hour, minute, second] = timeComponent.split(":", 3);

  // offsetHours is prefixed by + or -. Every component here is padded to length 2 with zeros
  const [offsetHours, offsetMinutes, offsetSeconds] = offset.split(":", 3);

  // We've all the time components we need. However, we can't use the Date constructor directly
  // because it interprets arguments in local time, and the local time offset may not match the
  // date offset. To deal with that:
  // - Parse the date as UTC, ignoring the offset. For example, for the 05:00+01:00 time, this
  //   would result in 05:00+00:00 (which is NOT the same instant; it'd be equivalent to
  //   06:00+01:00).
  // - Subtract the offset from the UTC timestamp we've parsed. Continuing the previous example,
  //   this would result in 04:00+00:00, which represents the same instant in time as 05:00+01:00.

  const timestamp = Date.UTC(
    Number.parseInt(year),
    Number.parseInt(month) - 1,
    Number.parseInt(day),
    Number.parseInt(hour),
    Number.parseInt(minute),
    Number.parseInt(second),
    Number.parseInt(nanosecond) / 1000000
  );

  return new Date(
    timestamp -
      Number.parseInt(offsetHours) * 3600000 -
      Number.parseInt(offsetMinutes) * 60000 -
      Number.parseInt(offsetSeconds) * 1000
  );
};
