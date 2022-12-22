use std::borrow::Cow;

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
