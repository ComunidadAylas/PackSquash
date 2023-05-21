use std::borrow::Cow;

pub trait ReborrowExt<B: ?Sized + ToOwned> {
	fn reborrow(&self) -> Cow<'_, B>;
}

impl<'this, B: ?Sized + ToOwned> ReborrowExt<B> for Cow<'this, B> {
	/// Reborrows this `Cow`, returning an equivalent `Cow` by value without cloning
	/// the owned data it holds, if any.
	///
	/// More precisely, reborrowing works as follows:
	/// - For borrowed `Cow`s, it constructs another borrowed `Cow` that borrows
	///   the same data.
	/// - For owned `Cow`s, it constructs a borrowed `Cow` that borrows the owned
	///   data. This is possible because this method borrows the owned `Cow` in this
	///   case, so the caller implicitly guarantees that its owned data lives at least
	///   as long as the borrow to the owned `Cow`.
	///
	/// As a consequence of the above, reborrowing always returns a `Cow` with the
	/// lifetime parameter set to that of the borrow of the original `Cow`, which may
	/// be shorter than the lifetime parameter of that original `Cow`.
	fn reborrow(&self) -> Cow<'_, B> {
		Cow::Borrowed(&**self)
	}
}

pub trait SplitOnceByColonWithDefaultPrefixExt<'this> {
	fn split_once_with_default_prefix(
		self,
		delimiter: &str,
		default_prefix: impl Into<Cow<'this, str>>
	) -> (Cow<'this, str>, Cow<'this, str>);
}

impl<'this> SplitOnceByColonWithDefaultPrefixExt<'this> for Cow<'this, str> {
	fn split_once_with_default_prefix(
		self,
		delimiter: &str,
		default_prefix: impl Into<Cow<'this, str>>
	) -> (Cow<'this, str>, Cow<'this, str>) {
		match self {
			Cow::Borrowed(string) => string
				.split_once(delimiter)
				.map(|(prefix, suffix)| (Cow::Borrowed(prefix), Cow::Borrowed(suffix)))
				.unwrap_or((default_prefix.into(), Cow::Borrowed(string))),
			Cow::Owned(mut string) => {
				if let Some(index) = string.find(delimiter) {
					let suffix = string.split_off(index + delimiter.len());

					// Strip the delimiter from the prefix
					string.truncate(string.len() - delimiter.len());

					(Cow::Owned(string), Cow::Owned(suffix))
				} else {
					(default_prefix.into(), Cow::Owned(string))
				}
			}
		}
	}
}

pub trait StripPrefixExt<'this> {
	fn strip_prefix(self, prefix: &str) -> Result<Cow<'this, str>, Self>
	where
		Self: Sized;
}

impl<'this> StripPrefixExt<'this> for Cow<'this, str> {
	fn strip_prefix(self, prefix: &str) -> Result<Cow<'this, str>, Self> {
		match self {
			Cow::Borrowed(string) => string.strip_prefix(prefix).map(Cow::Borrowed).ok_or(self),
			Cow::Owned(mut string) => {
				if string.starts_with(prefix) {
					string.replace_range(..prefix.len(), "");

					Ok(Cow::Owned(string))
				} else {
					Err(Cow::Owned(string))
				}
			}
		}
	}
}
