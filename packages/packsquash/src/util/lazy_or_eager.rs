use once_cell::sync::Lazy;
use std::ops::Deref;

pub enum LazyOrEager<'lazy, T, F: FnOnce() -> T> {
	Lazy(&'lazy Lazy<T, F>),
	Eager(T)
}

impl<T, F: FnOnce() -> T> Deref for LazyOrEager<'_, T, F> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		match self {
			Self::Lazy(lazy) => lazy,
			Self::Eager(inner) => inner
		}
	}
}
