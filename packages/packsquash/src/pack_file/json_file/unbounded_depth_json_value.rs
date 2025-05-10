use std::{io::Write, mem};

use serde::{Deserialize, Serialize};

/// A [`Value`](serde_json::Value) that may have arbitrarily deep nesting of JSON arrays
/// and objects, exceeding the usual `serde_json` depth limit.
pub struct UnboundedDepthJsonValue {
	value: serde_json::Value,
	exceeds_recursion_limit: bool
}

impl UnboundedDepthJsonValue {
	/// The minimum stack size that operations on an arbitrarily deep nested JSON value held
	/// by this struct will have available to them.
	///
	/// At the moment, this value is set to 2 MiB.
	pub const MINIMUM_STACK_SIZE: usize = 2 * 1024 * 1024;
	const NEW_STACK_SIZE: usize = Self::MINIMUM_STACK_SIZE * 2;

	/// Deserializes an arbitrarily deep JSON value from a JSON reader factory, taking care to
	/// avoid stack overflows. Such factory is assumed to be able to return an equivalent reader
	/// potentially multiple times.
	pub fn deserialize<'de, R: serde_json::de::Read<'de>>(
		mut reader_factory: impl FnMut() -> R
	) -> Result<Self, serde_json::Error> {
		let mut deserializer = serde_json::Deserializer::new(reader_factory());

		let (value, exceeds_recursion_limit) = match serde_json::Value::deserialize(&mut deserializer)
		{
			// If usual deserialization was successful, we know that the default recursion
			// limits were not exceeded, so we need not be careful about stack overflows
			Ok(value) => (value, false),
			// If deserialization failed due to exceeding the recursion limit, try again
			// with an overflow-safe deserializer. Relying on the error display impl is not
			// great, but there is no other safe way to do it at the moment (2025-05-10)
			Err(err) if format!("{err}").starts_with("recursion limit exceeded") => {
				deserializer = serde_json::Deserializer::new(reader_factory());
				deserializer.disable_recursion_limit();

				(
					serde_json::Value::deserialize(serde_stacker::Deserializer::new(
						&mut deserializer
					))?,
					true
				)
			}
			Err(err) => return Err(err)
		};

		// Check that non-whitespace trailing characters are not present in the input, as they
		// violate JSON specs and usually point to syntax errors that is worthwhile fixing
		deserializer.end()?;

		Ok(Self {
			value,
			exceeds_recursion_limit
		})
	}

	/// Serializes the arbitrarily deep JSON value held by this struct to a JSON writer, taking care to
	/// avoid stack overflows.
	pub fn serialize<W: Write, F: serde_json::ser::Formatter>(
		&self,
		serializer: &mut serde_json::Serializer<W, F>
	) -> Result<(), serde_json::Error> {
		if self.exceeds_recursion_limit {
			self.value
				.serialize(serde_stacker::Serializer::new(serializer))
		} else {
			self.value.serialize(serializer)
		}
	}

	/// Executes `callback` with a reference to the JSON value held by this struct, and at least
	/// [`MINIMUM_STACK_SIZE`](Self::MINIMUM_STACK_SIZE) bytes of stack space available if the
	/// JSON value is deeply nested.
	///
	/// Note that, despite the available stack space being plenty and known, deeply nested JSON
	/// structures may still cause stack overflows when used with uncaoutious code that recurses
	/// on them without a heap fallback. Thus, this method is best used as a compromise between
	/// allowing truly unbounded JSON structures, minimizing heap usage, and allowing reuse of
	/// existing recursive code.
	pub fn with_safe_stack<R>(&self, callback: impl FnOnce(&serde_json::Value) -> R) -> R {
		if self.exceeds_recursion_limit {
			stacker::maybe_grow(Self::MINIMUM_STACK_SIZE, Self::NEW_STACK_SIZE, || {
				callback(&self.value)
			})
		} else {
			callback(&self.value)
		}
	}

	/// Like [`with_large_stack`](Self::with_large_stack), but operates on a mutable reference to the
	/// JSON value held by this struct.
	pub fn with_safe_stack_mut<R>(
		&mut self,
		callback: impl FnOnce(&mut serde_json::Value) -> R
	) -> R {
		if self.exceeds_recursion_limit {
			stacker::maybe_grow(Self::MINIMUM_STACK_SIZE, Self::NEW_STACK_SIZE, || {
				callback(&mut self.value)
			})
		} else {
			callback(&mut self.value)
		}
	}

	/// Checks whether the JSON value held by this struct is deeply nested, i.e. whether it
	/// needs to be handled with an arbitrarily large stack.
	pub fn has_deeply_nested_value(&self) -> bool {
		self.exceeds_recursion_limit
	}
}

impl Drop for UnboundedDepthJsonValue {
	fn drop(&mut self) {
		// Non-deeply nested JSON values can be dropped normally, as doing so would
		// not use significant stack space.
		//
		// Deeply nested JSON values can only be safely dropped by moving them to a
		// drop stack in the heap, which has O(1) stack memory usage, at the cost of
		// some performance

		if !self.exceeds_recursion_limit {
			return;
		}

		let mut drop_stack = vec![mem::take(&mut self.value)];
		while let Some(value) = drop_stack.pop() {
			match value {
				serde_json::Value::Array(array) => {
					drop_stack.extend(array);
				}
				serde_json::Value::Object(object) => {
					drop_stack.extend(object.into_values());
				}
				_ => drop(value)
			}
		}
	}
}
