//! Helper traits to emulate generic type constructors without lifetime bounds for the purpose
//! of interfacing zero-copy deserializable structs that hold data stored in the current stack
//! frame with callbacks. Based on the code and ideas from:
//! https://users.rust-lang.org/t/generic-parameter-bound-by-deserialize-de-is-this-pattern-possible-in-rust/82163/3
//!
//! Callbacks are not defined here because client code may desire to modify their signature.
//!
//! The deserializable type to be used with this pattern must be represented by a dummy struct that
//! implements `ZeroCopyDeserializable` for any lifetime of the struct they represent.

use serde::de::{Deserialize, DeserializeOwned};

pub trait ZeroCopyDeserializable<'data> {
	type Type: Deserialize<'data>;
}
impl<T: DeserializeOwned> ZeroCopyDeserializable<'_> for T {
	type Type = T;
}

pub trait AlwaysZeroCopyDeserializable: for<'any> ZeroCopyDeserializable<'any> {}
impl<T: ?Sized + for<'any> ZeroCopyDeserializable<'any>> AlwaysZeroCopyDeserializable for T {}
