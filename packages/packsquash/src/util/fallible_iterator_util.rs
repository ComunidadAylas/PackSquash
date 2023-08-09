pub trait FallibleIterExt<T, E> {
	/// Determines if the elements of this [`Iterator`] are equal to those
	/// of another. Any error returned by any of the iterators will abort
	/// comparison and yield it back to the caller.
	fn try_eq<U: PartialEq<T>, I: IntoIterator<Item = Result<U, E>>>(
		self,
		other: I
	) -> Result<bool, E>;
}

impl<T, E, I: Iterator<Item = Result<T, E>>> FallibleIterExt<T, E> for I {
	// Implementation inspired by
	// https://docs.rs/fallible-iterator/0.3.0/src/fallible_iterator/lib.rs.html#777-796
	fn try_eq<U: PartialEq<T>, J: IntoIterator<Item = Result<U, E>>>(
		mut self,
		other: J
	) -> Result<bool, E> {
		let mut other = other.into_iter();
		loop {
			match (self.next().transpose()?, other.next().transpose()?) {
				// Both iterators successfully ended at the same time: they read the
				// same amount of equal items, so they are equal
				(None, None) => return Ok(true),
				// One iterator ended, but the other didn't: they have different amounts
				// of equal items, so they are not equal
				(None, Some(_)) | (Some(_), None) => return Ok(false),
				// Both iterators yielded an item. If the items are different, return
				// false. If they are equal, continue checking until they are different
				// or the end of some iterator is reached
				(Some(data_byte_or_err), Some(other_byte_or_err)) => {
					if other_byte_or_err != data_byte_or_err {
						return Ok(false);
					}
				}
			}
		}
	}
}
