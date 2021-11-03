use std::{cell::Cell, lazy::OnceCell};

use jsonpath_lib::SelectorMut;
use serde_json::Value;

use super::JsonFileAssetType;

/// Provides a short syntax to create a [SelectorMut] from a JSONPath string.
macro_rules! jsonpath_selectormut {
	($path:expr) => {{
		let mut selector_mut = SelectorMut::new();
		selector_mut.str_path($path).unwrap();
		selector_mut
	}};
}

/// Allows debloating Minecraft JSON files. Debloating deletes values that
/// may be fairly common to find in JSON objects but are ignored by the game,
/// reducing the size of the file.
///
/// Instantiating this struct involves compiling JSONPath expressions, so users
/// are encouraged to reuse instances across JSON files.
pub(super) struct Debloater {
	minecraft_model_bloat_selectors: OnceCell<Cell<[SelectorMut; 1]>>,
	#[cfg(feature = "mtr3-support")]
	mtr3_train_model_bloat_selectors: OnceCell<Cell<Vec<SelectorMut>>>
}

impl Debloater {
	/// Creates a new Minecraft JSON file debloater.
	pub const fn new() -> Self {
		Self {
			minecraft_model_bloat_selectors: OnceCell::new(),
			#[cfg(feature = "mtr3-support")]
			mtr3_train_model_bloat_selectors: OnceCell::new()
		}
	}

	/// Debloats an already parsed Minecraft JSON file, according to its asset type.
	/// A boolean value is returned indicating whether a debloat attempt was made to
	/// this file. Note that, even if such an attempt was made, the JSON might not
	/// have been modified.
	pub fn debloat(&self, parsed_json: &mut Value, asset_type: JsonFileAssetType) -> bool {
		// Use the appropriate JSONPath selectors for this asset type. If this
		// asset type has no applicable selectors, bail out early
		match asset_type {
			JsonFileAssetType::MinecraftModel => debloat_value(
				parsed_json,
				&self.minecraft_model_bloat_selectors,
				compile_minecraft_model_bloat_selectors
			),
			#[cfg(feature = "mtr3-support")]
			JsonFileAssetType::Mtr3CustomTrainModel => debloat_value(
				parsed_json,
				&self.mtr3_train_model_bloat_selectors,
				compile_mtr3_train_model_bloat_selectors
			),
			_ => return false
		};

		true
	}
}

/// Debloats a JSON value using the provided JSONPath selectors, which are stored in
/// a [Cell] inside a [OnceCell]. The [OnceCell] is used to guarantee that the selectors
/// are constructed only once using the provided init function, and the inner [Cell] is
/// used to restore the inner mutability needed by the JSONPath library. The JSONPath
/// selectors may be stored in anything that can be reference converted to a mutable
/// slice of them.
fn debloat_value<T: AsMut<[SelectorMut]> + Default, F: FnOnce() -> Cell<T>>(
	value: &mut Value,
	bloat_value_selectors_cell: &OnceCell<Cell<T>>,
	bloat_value_selectors_cell_init: F
) {
	let bloat_value_selectors_inner_cell =
		bloat_value_selectors_cell.get_or_init(bloat_value_selectors_cell_init);

	// Restore inner mutability by moving the cell value out
	let mut bloat_value_selectors = bloat_value_selectors_inner_cell.take();

	// Put the value in a Cell to be able to get its ownership by moving it in each iteration
	let value = Cell::from_mut(value);

	for selector in bloat_value_selectors.as_mut() {
		selector.value(value.take());

		// The documentation is not so clear about this, but after reading the source code
		// of the crate, thinking how it works and testing, we can assume that any error here
		// is due to a usage mistake, and we should panic on that
		selector.remove().unwrap();

		value.set(selector.take().unwrap());
	}

	bloat_value_selectors_inner_cell.set(bloat_value_selectors);
}

/// Compiles JSONPath selectors to remove bloat from Minecraft model assets.
fn compile_minecraft_model_bloat_selectors() -> Cell<[SelectorMut; 1]> {
	Cell::new([
		// Blockbench credits (can be disabled in its options)
		jsonpath_selectormut!("$.credit")
	])
}

/// Compiles JSONPath selectors to remove bloat from Minecraft Transit Railway 3
/// train model assets.
#[cfg(feature = "mtr3-support")]
#[doc(cfg(feature = "mtr3-support"))]
fn compile_mtr3_train_model_bloat_selectors() -> Cell<Vec<SelectorMut>> {
	// An array of these many selectors would be 5168 bytes big (152 bytes
	// per selector). We don't want to put an array that big in the stack
	Cell::new(vec![
		// This list of expressions was deduced from the following source code files:
		// https://github.com/JannisX11/blockbench/blob/f687d97b77d748d5a07cc8ef3e594033a21dd305/js/io/formats/bbmodel.js#L93
		// https://github.com/JannisX11/blockbench/blob/f687d97b77d748d5a07cc8ef3e594033a21dd305/js/io/project.js#L294
		// https://github.com/JannisX11/blockbench/blob/f687d97b77d748d5a07cc8ef3e594033a21dd305/js/outliner/cube.js#L225
		// https://github.com/JannisX11/blockbench/blob/f687d97b77d748d5a07cc8ef3e594033a21dd305/js/outliner/group.js#L357
		// https://github.com/jonafanho/Minecraft-Transit-Railway/blob/f11faa38a0df5a7cff9907c1d64f9c58f1f1f83c/src/main/java/mtr/config/DynamicTrainModel.java#L23
		jsonpath_selectormut!("$.meta"),
		jsonpath_selectormut!("$.name"),
		jsonpath_selectormut!("$.geometry_name"),
		jsonpath_selectormut!("$.modded_entity_version"),
		jsonpath_selectormut!("$.ambientocclusion"),
		jsonpath_selectormut!("$.front_gui_light"),
		jsonpath_selectormut!("$.visible_box"),
		jsonpath_selectormut!("$.variable_placeholders"),
		jsonpath_selectormut!("$.overrides"),
		jsonpath_selectormut!("$.flag"),
		jsonpath_selectormut!("$.textures"),
		jsonpath_selectormut!("$.animations"),
		jsonpath_selectormut!("$['animation_variable_placeholders']"),
		jsonpath_selectormut!("$.display"),
		jsonpath_selectormut!("$.backgrounds"),
		jsonpath_selectormut!("$.history"),
		jsonpath_selectormut!("$.history_index"),
		jsonpath_selectormut!("$.fabricOptions"),
		jsonpath_selectormut!("$.elements[*].autouv"),
		jsonpath_selectormut!("$.elements[*].color"),
		jsonpath_selectormut!("$.elements[*].visibility"),
		jsonpath_selectormut!("$.elements[*].export"),
		jsonpath_selectormut!("$.elements[*].faces"),
		jsonpath_selectormut!("$.outliner[*]..origin"),
		jsonpath_selectormut!("$.outliner[*]..color"),
		jsonpath_selectormut!("$.outliner[*]..uuid"),
		jsonpath_selectormut!("$.outliner[*]..export"),
		jsonpath_selectormut!("$.outliner[*]..isOpen"),
		jsonpath_selectormut!("$.outliner[*]..locked"),
		jsonpath_selectormut!("$.outliner[*]..visibility"),
		jsonpath_selectormut!("$.outliner[*]..autouv"),
		jsonpath_selectormut!("$.outliner[*]..shade"),
		jsonpath_selectormut!("$.outliner[*]..rotation"),
		jsonpath_selectormut!("$.outliner[*]..reset"),
	])
}
