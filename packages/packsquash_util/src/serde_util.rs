use std::{
	error::Error,
	fmt::{Debug, Display, Formatter}
};

use serde::{de::Deserialize, Deserializer};

/// Newtype wrapper struct to prettify how `serde` errors with path information
/// are displayed.
#[derive(Clone)]
#[repr(transparent)]
pub struct PrettySerdePathErrorWrapper<E>(serde_path_to_error::Error<E>);

impl<E: Display> Display for PrettySerdePathErrorWrapper<E> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let path = self.0.path();
		let inner = self.0.inner();

		// When the path is empty, the serde_path_to_error Display implementation
		// writes a dot ("."), which looks ugly and is somewhat confusing. Show
		// the inner error directly instead
		if path.iter().next().is_some() {
			write!(f, "{path}: {inner}")
		} else {
			inner.fmt(f)
		}
	}
}

impl<E: Debug> Debug for PrettySerdePathErrorWrapper<E> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl<E: Error + 'static> Error for PrettySerdePathErrorWrapper<E> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.0)
	}
}

impl<E> From<serde_path_to_error::Error<E>> for PrettySerdePathErrorWrapper<E> {
	fn from(value: serde_path_to_error::Error<E>) -> Self {
		Self(value)
	}
}

/// Like [`serde_path_to_error::deserialize`], but wraps the returned error in a
/// [`PrettySerdePathErrorWrapper`].
pub fn deserialize_with_pretty_path_on_error<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
	deserializer: D
) -> Result<T, PrettySerdePathErrorWrapper<D::Error>> {
	serde_path_to_error::deserialize(deserializer).map_err(|err| err.into())
}
