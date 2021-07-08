use std::cell::Cell;

use jsonpath_lib::{JsonPathError, SelectorMut};
use serde_json::Value;
use thiserror::Error;

/// Provides a short syntax to create a [SelectorMut] from a JSONPath string.
macro_rules! jsonpath_selector_mut {
	($path:expr) => {{
		let mut selector_mut = SelectorMut::new();
		selector_mut.str_path($path).unwrap();
		selector_mut
	}};
}

/// Allows debloating Minecraft JSON files. Debloating here deletes values that
/// may be fairly common to find in JSON objects but are ignored by the game,
/// reducing the size of the file.
///
/// Instantiating this struct involves compiling JSONPath expressions, so users
/// are encouraged to reuse instances across JSON files.
pub(super) struct Debloater {
	bloat_values_selectors: Cell<[SelectorMut; 1]>
}

/// An error that may occur during debloat operations.
#[derive(Error, Debug)]
#[error("JSONPath error: {0}")]
pub struct DebloatError(JsonPathError);

impl From<JsonPathError> for DebloatError {
	fn from(err: JsonPathError) -> Self {
		Self(err)
	}
}

impl Debloater {
	/// Creates a new Minecraft JSON file debloater.
	pub fn new() -> Self {
		Self {
			bloat_values_selectors: Cell::new([
				// Blockbench credits
				jsonpath_selector_mut!("$.credit")
			])
		}
	}

	/// Debloats the specified Minecraft JSON file, previously parsed as a JSON value.
	pub fn debloat(&self, root_value: &mut Value) -> Result<(), DebloatError> {
		// Put the value in a Cell to be able to get its ownership by replacing it
		// temporarily during each iteration
		let value = Cell::from_mut(root_value);

		// Do something similar with the selectors array
		let mut bloat_values_selectors = self.bloat_values_selectors.take();

		for selector in &mut bloat_values_selectors {
			selector.value(value.take());

			match selector.remove() {
				Ok(_) => (),
				Err(err) => match err {
					JsonPathError::EmptyValue => (),
					other_error => {
						// Always make sure we set the cell back to its original
						// content before returning, to not affect future invocations
						self.bloat_values_selectors.set(bloat_values_selectors);
						return Err(other_error.into());
					}
				}
			};

			value.set(selector.take().unwrap());
		}

		self.bloat_values_selectors.set(bloat_values_selectors);

		Ok(())
	}
}
