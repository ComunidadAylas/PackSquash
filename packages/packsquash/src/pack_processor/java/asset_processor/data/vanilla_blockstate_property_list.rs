//! This automatically generated file contains the list of block state properties for several vanilla Minecraft versions.
#![allow(clippy::type_complexity)]

use std::{
	fmt,
	fmt::{Display, Formatter},
	num::NonZeroU8,
	ops::RangeBounds
};

use ahash::AHashMap;
use packsquash_options::{minecraft_version, MinecraftVersion};
use tinyvec::{tiny_vec, TinyVec};

use crate::util::range_bounds_intersect::RangeBoundsIntersectExt;

/// A type of property that determines a block state.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum BlockStatePropertyType {
	/// An enumerated block state property, whose value belongs to a finite set of possible values.
	Enum { values: &'static [&'static str] },
	/// A boolean block state property, which is either `true` or `false`.
	#[default]
	Boolean,
	/// An integer block state property in the range `[0, maximum_value]`.
	PositiveInteger { maximum_value: NonZeroU8 },
	/// An integer block state property in the range `[1, maximum_value]`.
	StrictlyPositiveInteger { maximum_value: NonZeroU8 }
}

impl Display for BlockStatePropertyType {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Enum { values } => {
				if values.is_empty() {
					f.write_str("enum with no values")?;
				} else {
					f.write_str("enum and possible values: ")?;
					values.iter().try_fold(true, |is_first, value| {
						if !is_first {
							f.write_str(", ")?;
						}

						f.write_str(value)?;

						Ok(false)
					})?;
				}
			}
			Self::Boolean => f.write_str("boolean")?,
			Self::PositiveInteger { maximum_value } => f.write_fmt(format_args!(
				"positive integer between 0 and {maximum_value}"
			))?,
			Self::StrictlyPositiveInteger { maximum_value } => f.write_fmt(format_args!(
				"positive integer between 1 and {maximum_value}"
			))?
		}

		Ok(())
	}
}

#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_13_TO_1_13_2_EXCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	594] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("air", &[]),
	("allium", &[]),
	("andesite", &[]),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azure_bluet", &[]),
	("barrier", &[]),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	("brain_coral", &[]),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral", &[]),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cauldron",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("cave_air", &[]),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("conduit", &[]),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cut_red_sandstone", &[]),
	("cut_sandstone", &[]),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	("dirt", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("fire_coral", &[]),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("flower_pot", &[]),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("grass_path", &[]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	("horn_coral", &[]),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("nether_bricks", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("packed_ice", &[]),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	("polished_andesite", &[]),
	("polished_diorite", &[]),
	("polished_granite", &[]),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azure_bluet", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_jungle_sapling", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_white_tulip", &[]),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[(
			"shape",
			BlockStatePropertyType::Enum {
				values: &[
					"north_south",
					"east_west",
					"ascending_east",
					"ascending_west",
					"ascending_north",
					"ascending_south",
					"south_east",
					"south_west",
					"north_west",
					"north_east"
				]
			}
		)]
	),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	("smooth_quartz", &[]),
	("smooth_red_sandstone", &[]),
	("smooth_sandstone", &[]),
	("smooth_stone", &[]),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3", "4", "5", "6", "7", "8"]
			}
		)]
	),
	("snow_block", &[]),
	("soul_sand", &[]),
	("spawner", &[]),
	("sponge", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("spruce_planks", &[]),
	(
		"spruce_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"spruce_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"spruce_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"sticky_piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	("stone", &[]),
	(
		"stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("stone_bricks", &[]),
	(
		"stone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stripped_acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"structure_block",
		&[(
			"mode",
			BlockStatePropertyType::Enum {
				values: &["save", "load", "corner", "data"]
			}
		)]
	),
	("structure_void", &[]),
	(
		"sugar_cane",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"sunflower",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_grass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_seagrass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("terracotta", &[]),
	("tnt", &[]),
	("torch", &[]),
	(
		"trapped_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire",
		&[
			("attached", BlockStatePropertyType::Boolean),
			("disarmed", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire_hook",
		&[
			("attached", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("tube_coral", &[]),
	("tube_coral_block", &[]),
	(
		"tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"turtle_egg",
		&[
			(
				"eggs",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"hatch",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			)
		]
	),
	(
		"vine",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("void_air", &[]),
	(
		"wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"water",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("wet_sponge", &[]),
	(
		"wheat",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"white_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"white_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("white_carpet", &[]),
	("white_concrete", &[]),
	("white_concrete_powder", &[]),
	(
		"white_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"white_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("white_stained_glass", &[]),
	(
		"white_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("white_terracotta", &[]),
	("white_tulip", &[]),
	(
		"white_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("white_wool", &[]),
	(
		"wither_skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"wither_skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"yellow_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("yellow_carpet", &[]),
	("yellow_concrete", &[]),
	("yellow_concrete_powder", &[]),
	(
		"yellow_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("yellow_stained_glass", &[]),
	(
		"yellow_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("yellow_terracotta", &[]),
	(
		"yellow_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("yellow_wool", &[]),
	(
		"zombie_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"zombie_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	)
];
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_13_2_TO_1_14_EXCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	599] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("air", &[]),
	("allium", &[]),
	("andesite", &[]),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azure_bluet", &[]),
	("barrier", &[]),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	(
		"brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cauldron",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("cave_air", &[]),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"conduit",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cut_red_sandstone", &[]),
	("cut_sandstone", &[]),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	(
		"dead_brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	(
		"dead_fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	("dirt", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("flower_pot", &[]),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("grass_path", &[]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("nether_bricks", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("packed_ice", &[]),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	("polished_andesite", &[]),
	("polished_diorite", &[]),
	("polished_granite", &[]),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azure_bluet", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_jungle_sapling", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_white_tulip", &[]),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[(
			"shape",
			BlockStatePropertyType::Enum {
				values: &[
					"north_south",
					"east_west",
					"ascending_east",
					"ascending_west",
					"ascending_north",
					"ascending_south",
					"south_east",
					"south_west",
					"north_west",
					"north_east"
				]
			}
		)]
	),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	("smooth_quartz", &[]),
	("smooth_red_sandstone", &[]),
	("smooth_sandstone", &[]),
	("smooth_stone", &[]),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3", "4", "5", "6", "7", "8"]
			}
		)]
	),
	("snow_block", &[]),
	("soul_sand", &[]),
	("spawner", &[]),
	("sponge", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("spruce_planks", &[]),
	(
		"spruce_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"spruce_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"spruce_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"sticky_piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	("stone", &[]),
	(
		"stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("stone_bricks", &[]),
	(
		"stone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stripped_acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"structure_block",
		&[(
			"mode",
			BlockStatePropertyType::Enum {
				values: &["save", "load", "corner", "data"]
			}
		)]
	),
	("structure_void", &[]),
	(
		"sugar_cane",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"sunflower",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_grass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_seagrass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("terracotta", &[]),
	("tnt", &[("unstable", BlockStatePropertyType::Boolean)]),
	("torch", &[]),
	(
		"trapped_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire",
		&[
			("attached", BlockStatePropertyType::Boolean),
			("disarmed", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire_hook",
		&[
			("attached", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("tube_coral_block", &[]),
	(
		"tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"turtle_egg",
		&[
			(
				"eggs",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"hatch",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			)
		]
	),
	(
		"vine",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("void_air", &[]),
	(
		"wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"water",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("wet_sponge", &[]),
	(
		"wheat",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"white_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"white_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("white_carpet", &[]),
	("white_concrete", &[]),
	("white_concrete_powder", &[]),
	(
		"white_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"white_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("white_stained_glass", &[]),
	(
		"white_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("white_terracotta", &[]),
	("white_tulip", &[]),
	(
		"white_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("white_wool", &[]),
	(
		"wither_skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"wither_skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"yellow_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("yellow_carpet", &[]),
	("yellow_concrete", &[]),
	("yellow_concrete_powder", &[]),
	(
		"yellow_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("yellow_stained_glass", &[]),
	(
		"yellow_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("yellow_terracotta", &[]),
	(
		"yellow_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("yellow_wool", &[]),
	(
		"zombie_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"zombie_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	)
];
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_14_TO_1_15_EXCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	677] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("air", &[]),
	("allium", &[]),
	("andesite", &[]),
	(
		"andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azure_bluet", &[]),
	(
		"bamboo",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			),
			(
				"leaves",
				BlockStatePropertyType::Enum {
					values: &["none", "small", "large"]
				}
			),
			(
				"stage",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			)
		]
	),
	("bamboo_sapling", &[]),
	(
		"barrel",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("open", BlockStatePropertyType::Boolean)
		]
	),
	("barrier", &[]),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"bell",
		&[
			(
				"attachment",
				BlockStatePropertyType::Enum {
					values: &["floor", "ceiling", "single_wall", "double_wall"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	(
		"blast_furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	(
		"brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	(
		"campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("cartography_table", &[]),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cauldron",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("cave_air", &[]),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"composter",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	(
		"conduit",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("cornflower", &[]),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cut_red_sandstone", &[]),
	(
		"cut_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_sandstone", &[]),
	(
		"cut_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	(
		"dead_brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	(
		"dead_fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	(
		"diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("dirt", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	(
		"end_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("fletching_table", &[]),
	("flower_pot", &[]),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	(
		"granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("grass_path", &[]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"grindstone",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jigsaw",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lantern", &[("hanging", BlockStatePropertyType::Boolean)]),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lectern",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("has_book", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_of_the_valley", &[]),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	(
		"loom",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("nether_bricks", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone",
						"iron_xylophone",
						"cow_bell",
						"didgeridoo",
						"bit",
						"banjo",
						"pling"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("packed_ice", &[]),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	("polished_andesite", &[]),
	(
		"polished_andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_diorite", &[]),
	(
		"polished_diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_granite", &[]),
	(
		"polished_granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azure_bluet", &[]),
	("potted_bamboo", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_cornflower", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_jungle_sapling", &[]),
	("potted_lily_of_the_valley", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_white_tulip", &[]),
	("potted_wither_rose", &[]),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[(
			"shape",
			BlockStatePropertyType::Enum {
				values: &[
					"north_south",
					"east_west",
					"ascending_east",
					"ascending_west",
					"ascending_north",
					"ascending_south",
					"south_east",
					"south_west",
					"north_west",
					"north_east"
				]
			}
		)]
	),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"scaffolding",
		&[
			("bottom", BlockStatePropertyType::Boolean),
			(
				"distance",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	("smithing_table", &[]),
	(
		"smoker",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_quartz", &[]),
	(
		"smooth_quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_red_sandstone", &[]),
	(
		"smooth_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_sandstone", &[]),
	(
		"smooth_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_stone", &[]),
	(
		"smooth_stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3", "4", "5", "6", "7", "8"]
			}
		)]
	),
	("snow_block", &[]),
	("soul_sand", &[]),
	("spawner", &[]),
	("sponge", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("spruce_planks", &[]),
	(
		"spruce_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"spruce_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"spruce_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"sticky_piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	("stone", &[]),
	(
		"stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("stone_bricks", &[]),
	(
		"stone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stonecutter",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"stripped_acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"structure_block",
		&[(
			"mode",
			BlockStatePropertyType::Enum {
				values: &["save", "load", "corner", "data"]
			}
		)]
	),
	("structure_void", &[]),
	(
		"sugar_cane",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"sunflower",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"sweet_berry_bush",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"tall_grass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_seagrass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("terracotta", &[]),
	("tnt", &[("unstable", BlockStatePropertyType::Boolean)]),
	("torch", &[]),
	(
		"trapped_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire",
		&[
			("attached", BlockStatePropertyType::Boolean),
			("disarmed", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire_hook",
		&[
			("attached", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("tube_coral_block", &[]),
	(
		"tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"turtle_egg",
		&[
			(
				"eggs",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"hatch",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			)
		]
	),
	(
		"vine",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("void_air", &[]),
	(
		"wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"water",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("wet_sponge", &[]),
	(
		"wheat",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"white_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"white_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("white_carpet", &[]),
	("white_concrete", &[]),
	("white_concrete_powder", &[]),
	(
		"white_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"white_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("white_stained_glass", &[]),
	(
		"white_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("white_terracotta", &[]),
	("white_tulip", &[]),
	(
		"white_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("white_wool", &[]),
	("wither_rose", &[]),
	(
		"wither_skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"wither_skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"yellow_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("yellow_carpet", &[]),
	("yellow_concrete", &[]),
	("yellow_concrete_powder", &[]),
	(
		"yellow_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("yellow_stained_glass", &[]),
	(
		"yellow_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("yellow_terracotta", &[]),
	(
		"yellow_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("yellow_wool", &[]),
	(
		"zombie_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"zombie_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	)
];
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_15_TO_1_16_EXCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	681] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("air", &[]),
	("allium", &[]),
	("andesite", &[]),
	(
		"andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azure_bluet", &[]),
	(
		"bamboo",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			),
			(
				"leaves",
				BlockStatePropertyType::Enum {
					values: &["none", "small", "large"]
				}
			),
			(
				"stage",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			)
		]
	),
	("bamboo_sapling", &[]),
	(
		"barrel",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("open", BlockStatePropertyType::Boolean)
		]
	),
	("barrier", &[]),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"bee_nest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beehive",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"bell",
		&[
			(
				"attachment",
				BlockStatePropertyType::Enum {
					values: &["floor", "ceiling", "single_wall", "double_wall"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	(
		"blast_furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	(
		"brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	(
		"campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("cartography_table", &[]),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cauldron",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("cave_air", &[]),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"composter",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	(
		"conduit",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("cornflower", &[]),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cut_red_sandstone", &[]),
	(
		"cut_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_sandstone", &[]),
	(
		"cut_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	(
		"dead_brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	(
		"dead_fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	(
		"diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("dirt", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	(
		"end_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("fletching_table", &[]),
	("flower_pot", &[]),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	(
		"granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("grass_path", &[]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"grindstone",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("honey_block", &[]),
	("honeycomb_block", &[]),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jigsaw",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lantern", &[("hanging", BlockStatePropertyType::Boolean)]),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lectern",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("has_book", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_of_the_valley", &[]),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	(
		"loom",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("nether_bricks", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone",
						"iron_xylophone",
						"cow_bell",
						"didgeridoo",
						"bit",
						"banjo",
						"pling"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("packed_ice", &[]),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	("polished_andesite", &[]),
	(
		"polished_andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_diorite", &[]),
	(
		"polished_diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_granite", &[]),
	(
		"polished_granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azure_bluet", &[]),
	("potted_bamboo", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_cornflower", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_jungle_sapling", &[]),
	("potted_lily_of_the_valley", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_white_tulip", &[]),
	("potted_wither_rose", &[]),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[(
			"shape",
			BlockStatePropertyType::Enum {
				values: &[
					"north_south",
					"east_west",
					"ascending_east",
					"ascending_west",
					"ascending_north",
					"ascending_south",
					"south_east",
					"south_west",
					"north_west",
					"north_east"
				]
			}
		)]
	),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"scaffolding",
		&[
			("bottom", BlockStatePropertyType::Boolean),
			(
				"distance",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	("smithing_table", &[]),
	(
		"smoker",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_quartz", &[]),
	(
		"smooth_quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_red_sandstone", &[]),
	(
		"smooth_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_sandstone", &[]),
	(
		"smooth_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_stone", &[]),
	(
		"smooth_stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3", "4", "5", "6", "7", "8"]
			}
		)]
	),
	("snow_block", &[]),
	("soul_sand", &[]),
	("spawner", &[]),
	("sponge", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("spruce_planks", &[]),
	(
		"spruce_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"spruce_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"spruce_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"sticky_piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	("stone", &[]),
	(
		"stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_wall",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("stone_bricks", &[]),
	(
		"stone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stonecutter",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"stripped_acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"structure_block",
		&[(
			"mode",
			BlockStatePropertyType::Enum {
				values: &["save", "load", "corner", "data"]
			}
		)]
	),
	("structure_void", &[]),
	(
		"sugar_cane",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"sunflower",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"sweet_berry_bush",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"tall_grass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_seagrass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("terracotta", &[]),
	("tnt", &[("unstable", BlockStatePropertyType::Boolean)]),
	("torch", &[]),
	(
		"trapped_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire",
		&[
			("attached", BlockStatePropertyType::Boolean),
			("disarmed", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire_hook",
		&[
			("attached", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("tube_coral_block", &[]),
	(
		"tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"turtle_egg",
		&[
			(
				"eggs",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"hatch",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			)
		]
	),
	(
		"vine",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("void_air", &[]),
	(
		"wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"water",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("wet_sponge", &[]),
	(
		"wheat",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"white_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"white_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("white_carpet", &[]),
	("white_concrete", &[]),
	("white_concrete_powder", &[]),
	(
		"white_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"white_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("white_stained_glass", &[]),
	(
		"white_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("white_terracotta", &[]),
	("white_tulip", &[]),
	(
		"white_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("white_wool", &[]),
	("wither_rose", &[]),
	(
		"wither_skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"wither_skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"yellow_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("yellow_carpet", &[]),
	("yellow_concrete", &[]),
	("yellow_concrete_powder", &[]),
	(
		"yellow_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("yellow_stained_glass", &[]),
	(
		"yellow_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("yellow_terracotta", &[]),
	(
		"yellow_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("yellow_wool", &[]),
	(
		"zombie_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"zombie_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	)
];
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_16_TO_1_16_2_EXCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	764] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("air", &[]),
	("allium", &[]),
	("ancient_debris", &[]),
	("andesite", &[]),
	(
		"andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azure_bluet", &[]),
	(
		"bamboo",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			),
			(
				"leaves",
				BlockStatePropertyType::Enum {
					values: &["none", "small", "large"]
				}
			),
			(
				"stage",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			)
		]
	),
	("bamboo_sapling", &[]),
	(
		"barrel",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("open", BlockStatePropertyType::Boolean)
		]
	),
	("barrier", &[]),
	(
		"basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"bee_nest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beehive",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"bell",
		&[
			(
				"attachment",
				BlockStatePropertyType::Enum {
					values: &["floor", "ceiling", "single_wall", "double_wall"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	("blackstone", &[]),
	(
		"blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"blast_furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	(
		"brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	(
		"campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("cartography_table", &[]),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cauldron",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("cave_air", &[]),
	("chain", &[("waterlogged", BlockStatePropertyType::Boolean)]),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_nether_bricks", &[]),
	("chiseled_polished_blackstone", &[]),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"composter",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	(
		"conduit",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("cornflower", &[]),
	("cracked_nether_bricks", &[]),
	("cracked_polished_blackstone_bricks", &[]),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"crimson_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("crimson_fungus", &[]),
	(
		"crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("crimson_nylium", &[]),
	("crimson_planks", &[]),
	(
		"crimson_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("crimson_roots", &[]),
	(
		"crimson_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"crimson_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("crying_obsidian", &[]),
	("cut_red_sandstone", &[]),
	(
		"cut_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_sandstone", &[]),
	(
		"cut_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	(
		"dead_brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	(
		"dead_fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	(
		"diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("dirt", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	(
		"end_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("fletching_table", &[]),
	("flower_pot", &[]),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("gilded_blackstone", &[]),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	(
		"granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("grass_path", &[]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"grindstone",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("honey_block", &[]),
	("honeycomb_block", &[]),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jigsaw",
		&[(
			"orientation",
			BlockStatePropertyType::Enum {
				values: &[
					"down_east",
					"down_north",
					"down_south",
					"down_west",
					"up_east",
					"up_north",
					"up_south",
					"up_west",
					"west_up",
					"east_up",
					"north_up",
					"south_up"
				]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lantern", &[("hanging", BlockStatePropertyType::Boolean)]),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lectern",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("has_book", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_of_the_valley", &[]),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	("lodestone", &[]),
	(
		"loom",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"mossy_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("nether_bricks", &[]),
	("nether_gold_ore", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	("nether_sprouts", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherite_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone",
						"iron_xylophone",
						"cow_bell",
						"didgeridoo",
						"bit",
						"banjo",
						"pling"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("packed_ice", &[]),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	("polished_andesite", &[]),
	(
		"polished_andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("polished_blackstone", &[]),
	(
		"polished_blackstone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_blackstone_bricks", &[]),
	(
		"polished_blackstone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"polished_blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_diorite", &[]),
	(
		"polished_diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_granite", &[]),
	(
		"polished_granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azure_bluet", &[]),
	("potted_bamboo", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_cornflower", &[]),
	("potted_crimson_fungus", &[]),
	("potted_crimson_roots", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_jungle_sapling", &[]),
	("potted_lily_of_the_valley", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_warped_fungus", &[]),
	("potted_warped_roots", &[]),
	("potted_white_tulip", &[]),
	("potted_wither_rose", &[]),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	("quartz_bricks", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[(
			"shape",
			BlockStatePropertyType::Enum {
				values: &[
					"north_south",
					"east_west",
					"ascending_east",
					"ascending_west",
					"ascending_north",
					"ascending_south",
					"south_east",
					"south_west",
					"north_west",
					"north_east"
				]
			}
		)]
	),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"respawn_anchor",
		&[(
			"charges",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
			}
		)]
	),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"scaffolding",
		&[
			("bottom", BlockStatePropertyType::Boolean),
			(
				"distance",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	("shroomlight", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	("smithing_table", &[]),
	(
		"smoker",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_quartz", &[]),
	(
		"smooth_quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_red_sandstone", &[]),
	(
		"smooth_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_sandstone", &[]),
	(
		"smooth_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_stone", &[]),
	(
		"smooth_stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3", "4", "5", "6", "7", "8"]
			}
		)]
	),
	("snow_block", &[]),
	(
		"soul_campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_fire", &[]),
	(
		"soul_lantern",
		&[("hanging", BlockStatePropertyType::Boolean)]
	),
	("soul_sand", &[]),
	("soul_soil", &[]),
	("soul_torch", &[]),
	(
		"soul_wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("spawner", &[]),
	("sponge", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("spruce_planks", &[]),
	(
		"spruce_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"spruce_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"spruce_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"sticky_piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	("stone", &[]),
	(
		"stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("stone_bricks", &[]),
	(
		"stone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stonecutter",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"stripped_acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_warped_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_warped_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"structure_block",
		&[(
			"mode",
			BlockStatePropertyType::Enum {
				values: &["save", "load", "corner", "data"]
			}
		)]
	),
	("structure_void", &[]),
	(
		"sugar_cane",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"sunflower",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"sweet_berry_bush",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"tall_grass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_seagrass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"target",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("terracotta", &[]),
	("tnt", &[("unstable", BlockStatePropertyType::Boolean)]),
	("torch", &[]),
	(
		"trapped_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire",
		&[
			("attached", BlockStatePropertyType::Boolean),
			("disarmed", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire_hook",
		&[
			("attached", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("tube_coral_block", &[]),
	(
		"tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"turtle_egg",
		&[
			(
				"eggs",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"hatch",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			)
		]
	),
	(
		"twisting_vines",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("twisting_vines_plant", &[]),
	(
		"vine",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("void_air", &[]),
	(
		"wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"warped_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("warped_fungus", &[]),
	(
		"warped_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("warped_nylium", &[]),
	("warped_planks", &[]),
	(
		"warped_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("warped_roots", &[]),
	(
		"warped_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"warped_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("warped_wart_block", &[]),
	(
		"water",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"weeping_vines",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("weeping_vines_plant", &[]),
	("wet_sponge", &[]),
	(
		"wheat",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"white_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"white_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("white_carpet", &[]),
	("white_concrete", &[]),
	("white_concrete_powder", &[]),
	(
		"white_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"white_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("white_stained_glass", &[]),
	(
		"white_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("white_terracotta", &[]),
	("white_tulip", &[]),
	(
		"white_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("white_wool", &[]),
	("wither_rose", &[]),
	(
		"wither_skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"wither_skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"yellow_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("yellow_carpet", &[]),
	("yellow_concrete", &[]),
	("yellow_concrete_powder", &[]),
	(
		"yellow_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("yellow_stained_glass", &[]),
	(
		"yellow_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("yellow_terracotta", &[]),
	(
		"yellow_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("yellow_wool", &[]),
	(
		"zombie_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"zombie_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	)
];
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_16_2_TO_1_17_EXCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	764] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("air", &[]),
	("allium", &[]),
	("ancient_debris", &[]),
	("andesite", &[]),
	(
		"andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azure_bluet", &[]),
	(
		"bamboo",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			),
			(
				"leaves",
				BlockStatePropertyType::Enum {
					values: &["none", "small", "large"]
				}
			),
			(
				"stage",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			)
		]
	),
	("bamboo_sapling", &[]),
	(
		"barrel",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("open", BlockStatePropertyType::Boolean)
		]
	),
	("barrier", &[]),
	(
		"basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"bee_nest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beehive",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"bell",
		&[
			(
				"attachment",
				BlockStatePropertyType::Enum {
					values: &["floor", "ceiling", "single_wall", "double_wall"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	("blackstone", &[]),
	(
		"blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"blast_furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	(
		"brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	(
		"campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("cartography_table", &[]),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cauldron",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("cave_air", &[]),
	(
		"chain",
		&[
			(
				"axis",
				BlockStatePropertyType::Enum {
					values: &["x", "y", "z"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_nether_bricks", &[]),
	("chiseled_polished_blackstone", &[]),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"composter",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	(
		"conduit",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("cornflower", &[]),
	("cracked_nether_bricks", &[]),
	("cracked_polished_blackstone_bricks", &[]),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"crimson_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("crimson_fungus", &[]),
	(
		"crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("crimson_nylium", &[]),
	("crimson_planks", &[]),
	(
		"crimson_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("crimson_roots", &[]),
	(
		"crimson_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"crimson_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("crying_obsidian", &[]),
	("cut_red_sandstone", &[]),
	(
		"cut_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_sandstone", &[]),
	(
		"cut_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	(
		"dead_brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	(
		"dead_fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	(
		"diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("dirt", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	(
		"end_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("fletching_table", &[]),
	("flower_pot", &[]),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("gilded_blackstone", &[]),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	(
		"granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("grass_path", &[]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"grindstone",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("honey_block", &[]),
	("honeycomb_block", &[]),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jigsaw",
		&[(
			"orientation",
			BlockStatePropertyType::Enum {
				values: &[
					"down_east",
					"down_north",
					"down_south",
					"down_west",
					"up_east",
					"up_north",
					"up_south",
					"up_west",
					"west_up",
					"east_up",
					"north_up",
					"south_up"
				]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lantern",
		&[
			("hanging", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lectern",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("has_book", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_of_the_valley", &[]),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	("lodestone", &[]),
	(
		"loom",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"mossy_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("nether_bricks", &[]),
	("nether_gold_ore", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	("nether_sprouts", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherite_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone",
						"iron_xylophone",
						"cow_bell",
						"didgeridoo",
						"bit",
						"banjo",
						"pling"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("packed_ice", &[]),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	("polished_andesite", &[]),
	(
		"polished_andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("polished_blackstone", &[]),
	(
		"polished_blackstone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_blackstone_bricks", &[]),
	(
		"polished_blackstone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"polished_blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_diorite", &[]),
	(
		"polished_diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_granite", &[]),
	(
		"polished_granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azure_bluet", &[]),
	("potted_bamboo", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_cornflower", &[]),
	("potted_crimson_fungus", &[]),
	("potted_crimson_roots", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_jungle_sapling", &[]),
	("potted_lily_of_the_valley", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_warped_fungus", &[]),
	("potted_warped_roots", &[]),
	("potted_white_tulip", &[]),
	("potted_wither_rose", &[]),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	("quartz_bricks", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[(
			"shape",
			BlockStatePropertyType::Enum {
				values: &[
					"north_south",
					"east_west",
					"ascending_east",
					"ascending_west",
					"ascending_north",
					"ascending_south",
					"south_east",
					"south_west",
					"north_west",
					"north_east"
				]
			}
		)]
	),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"respawn_anchor",
		&[(
			"charges",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
			}
		)]
	),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"scaffolding",
		&[
			("bottom", BlockStatePropertyType::Boolean),
			(
				"distance",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	("shroomlight", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	("smithing_table", &[]),
	(
		"smoker",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_quartz", &[]),
	(
		"smooth_quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_red_sandstone", &[]),
	(
		"smooth_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_sandstone", &[]),
	(
		"smooth_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_stone", &[]),
	(
		"smooth_stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3", "4", "5", "6", "7", "8"]
			}
		)]
	),
	("snow_block", &[]),
	(
		"soul_campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_fire", &[]),
	(
		"soul_lantern",
		&[
			("hanging", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_sand", &[]),
	("soul_soil", &[]),
	("soul_torch", &[]),
	(
		"soul_wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("spawner", &[]),
	("sponge", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("spruce_planks", &[]),
	(
		"spruce_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"spruce_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"spruce_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"sticky_piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	("stone", &[]),
	(
		"stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("stone_bricks", &[]),
	(
		"stone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stonecutter",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"stripped_acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_warped_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_warped_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"structure_block",
		&[(
			"mode",
			BlockStatePropertyType::Enum {
				values: &["save", "load", "corner", "data"]
			}
		)]
	),
	("structure_void", &[]),
	(
		"sugar_cane",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"sunflower",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"sweet_berry_bush",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"tall_grass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_seagrass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"target",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("terracotta", &[]),
	("tnt", &[("unstable", BlockStatePropertyType::Boolean)]),
	("torch", &[]),
	(
		"trapped_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire",
		&[
			("attached", BlockStatePropertyType::Boolean),
			("disarmed", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire_hook",
		&[
			("attached", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("tube_coral_block", &[]),
	(
		"tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"turtle_egg",
		&[
			(
				"eggs",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"hatch",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			)
		]
	),
	(
		"twisting_vines",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("twisting_vines_plant", &[]),
	(
		"vine",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("void_air", &[]),
	(
		"wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"warped_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("warped_fungus", &[]),
	(
		"warped_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("warped_nylium", &[]),
	("warped_planks", &[]),
	(
		"warped_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("warped_roots", &[]),
	(
		"warped_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"warped_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("warped_wart_block", &[]),
	(
		"water",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"weeping_vines",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("weeping_vines_plant", &[]),
	("wet_sponge", &[]),
	(
		"wheat",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"white_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"white_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("white_carpet", &[]),
	("white_concrete", &[]),
	("white_concrete_powder", &[]),
	(
		"white_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"white_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("white_stained_glass", &[]),
	(
		"white_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("white_terracotta", &[]),
	("white_tulip", &[]),
	(
		"white_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("white_wool", &[]),
	("wither_rose", &[]),
	(
		"wither_skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"wither_skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"yellow_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	("yellow_carpet", &[]),
	("yellow_concrete", &[]),
	("yellow_concrete_powder", &[]),
	(
		"yellow_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("yellow_stained_glass", &[]),
	(
		"yellow_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("yellow_terracotta", &[]),
	(
		"yellow_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("yellow_wool", &[]),
	(
		"zombie_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"zombie_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	)
];
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_17_TO_1_18_EXCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	900] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("air", &[]),
	("allium", &[]),
	("amethyst_block", &[]),
	(
		"amethyst_cluster",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ancient_debris", &[]),
	("andesite", &[]),
	(
		"andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azalea", &[]),
	(
		"azalea_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	("azure_bluet", &[]),
	(
		"bamboo",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			),
			(
				"leaves",
				BlockStatePropertyType::Enum {
					values: &["none", "small", "large"]
				}
			),
			(
				"stage",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			)
		]
	),
	("bamboo_sapling", &[]),
	(
		"barrel",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("open", BlockStatePropertyType::Boolean)
		]
	),
	("barrier", &[]),
	(
		"basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"bee_nest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beehive",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"bell",
		&[
			(
				"attachment",
				BlockStatePropertyType::Enum {
					values: &["floor", "ceiling", "single_wall", "double_wall"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"big_dripleaf",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"tilt",
				BlockStatePropertyType::Enum {
					values: &["none", "unstable", "partial", "full"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"big_dripleaf_stem",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"black_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"black_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	("blackstone", &[]),
	(
		"blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"blast_furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"blue_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	(
		"brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"brown_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("budding_amethyst", &[]),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	("calcite", &[]),
	(
		"campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("candle_cake", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("cartography_table", &[]),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cauldron", &[]),
	("cave_air", &[]),
	(
		"cave_vines",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
				}
			),
			("berries", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cave_vines_plant",
		&[("berries", BlockStatePropertyType::Boolean)]
	),
	(
		"chain",
		&[
			(
				"axis",
				BlockStatePropertyType::Enum {
					values: &["x", "y", "z"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_deepslate", &[]),
	("chiseled_nether_bricks", &[]),
	("chiseled_polished_blackstone", &[]),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobbled_deepslate", &[]),
	(
		"cobbled_deepslate_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobbled_deepslate_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobbled_deepslate_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"composter",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	(
		"conduit",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("copper_block", &[]),
	("copper_ore", &[]),
	("cornflower", &[]),
	("cracked_deepslate_bricks", &[]),
	("cracked_deepslate_tiles", &[]),
	("cracked_nether_bricks", &[]),
	("cracked_polished_blackstone_bricks", &[]),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"crimson_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("crimson_fungus", &[]),
	(
		"crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("crimson_nylium", &[]),
	("crimson_planks", &[]),
	(
		"crimson_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("crimson_roots", &[]),
	(
		"crimson_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"crimson_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("crying_obsidian", &[]),
	("cut_copper", &[]),
	(
		"cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_red_sandstone", &[]),
	(
		"cut_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_sandstone", &[]),
	(
		"cut_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"cyan_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	(
		"dead_brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	(
		"dead_fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"deepslate_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("deepslate_bricks", &[]),
	("deepslate_coal_ore", &[]),
	("deepslate_copper_ore", &[]),
	("deepslate_diamond_ore", &[]),
	("deepslate_emerald_ore", &[]),
	("deepslate_gold_ore", &[]),
	("deepslate_iron_ore", &[]),
	("deepslate_lapis_ore", &[]),
	(
		"deepslate_redstone_ore",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"deepslate_tile_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_tile_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_tile_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("deepslate_tiles", &[]),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	(
		"diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("dirt", &[]),
	("dirt_path", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	("dripstone_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	(
		"end_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("exposed_copper", &[]),
	("exposed_cut_copper", &[]),
	(
		"exposed_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"exposed_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("fletching_table", &[]),
	("flower_pot", &[]),
	("flowering_azalea", &[]),
	(
		"flowering_azalea_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("gilded_blackstone", &[]),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"glow_item_frame",
		&[("map", BlockStatePropertyType::Boolean)]
	),
	(
		"glow_lichen",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	(
		"granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"gray_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"gray_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"green_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"green_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"grindstone",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"hanging_roots",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("honey_block", &[]),
	("honeycomb_block", &[]),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	(
		"infested_deepslate",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jigsaw",
		&[(
			"orientation",
			BlockStatePropertyType::Enum {
				values: &[
					"down_east",
					"down_north",
					"down_south",
					"down_west",
					"up_east",
					"up_north",
					"up_south",
					"up_west",
					"west_up",
					"east_up",
					"north_up",
					"south_up"
				]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lantern",
		&[
			("hanging", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("lava_cauldron", &[]),
	(
		"lectern",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("has_book", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light",
		&[
			(
				"level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"light_blue_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"light_gray_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_gray_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lightning_rod",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_of_the_valley", &[]),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"lime_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lime_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	("lodestone", &[]),
	(
		"loom",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"magenta_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"magenta_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	(
		"medium_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("moss_block", &[]),
	("moss_carpet", &[]),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"mossy_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("nether_bricks", &[]),
	("nether_gold_ore", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	("nether_sprouts", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherite_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone",
						"iron_xylophone",
						"cow_bell",
						"didgeridoo",
						"bit",
						"banjo",
						"pling"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"orange_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"orange_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("oxidized_copper", &[]),
	("oxidized_cut_copper", &[]),
	(
		"oxidized_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oxidized_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("packed_ice", &[]),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"pink_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"pointed_dripstone",
		&[
			(
				"thickness",
				BlockStatePropertyType::Enum {
					values: &["tip_merge", "tip", "frustum", "middle", "base"]
				}
			),
			(
				"vertical_direction",
				BlockStatePropertyType::Enum {
					values: &["up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_andesite", &[]),
	(
		"polished_andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("polished_blackstone", &[]),
	(
		"polished_blackstone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_blackstone_bricks", &[]),
	(
		"polished_blackstone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"polished_blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_deepslate", &[]),
	(
		"polished_deepslate_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_deepslate_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_deepslate_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_diorite", &[]),
	(
		"polished_diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_granite", &[]),
	(
		"polished_granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azalea_bush", &[]),
	("potted_azure_bluet", &[]),
	("potted_bamboo", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_cornflower", &[]),
	("potted_crimson_fungus", &[]),
	("potted_crimson_roots", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_flowering_azalea_bush", &[]),
	("potted_jungle_sapling", &[]),
	("potted_lily_of_the_valley", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_warped_fungus", &[]),
	("potted_warped_roots", &[]),
	("potted_white_tulip", &[]),
	("potted_wither_rose", &[]),
	("powder_snow", &[]),
	(
		"powder_snow_cauldron",
		&[(
			"level",
			BlockStatePropertyType::StrictlyPositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"purple_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purple_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	("quartz_bricks", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south",
						"south_east",
						"south_west",
						"north_west",
						"north_east"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("raw_copper_block", &[]),
	("raw_gold_block", &[]),
	("raw_iron_block", &[]),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"red_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"respawn_anchor",
		&[(
			"charges",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
			}
		)]
	),
	("rooted_dirt", &[]),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"scaffolding",
		&[
			("bottom", BlockStatePropertyType::Boolean),
			(
				"distance",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sculk_sensor",
		&[
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"sculk_sensor_phase",
				BlockStatePropertyType::Enum {
					values: &["inactive", "active", "cooldown"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	("shroomlight", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	(
		"small_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"small_dripleaf",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smithing_table", &[]),
	(
		"smoker",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_basalt", &[]),
	("smooth_quartz", &[]),
	(
		"smooth_quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_red_sandstone", &[]),
	(
		"smooth_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_sandstone", &[]),
	(
		"smooth_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_stone", &[]),
	(
		"smooth_stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::StrictlyPositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	("snow_block", &[]),
	(
		"soul_campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_fire", &[]),
	(
		"soul_lantern",
		&[
			("hanging", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_sand", &[]),
	("soul_soil", &[]),
	("soul_torch", &[]),
	(
		"soul_wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("spawner", &[]),
	("sponge", &[]),
	("spore_blossom", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("spruce_planks", &[]),
	(
		"spruce_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"spruce_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"spruce_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"sticky_piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	("stone", &[]),
	(
		"stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("stone_bricks", &[]),
	(
		"stone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stonecutter",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"stripped_acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_warped_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_warped_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"structure_block",
		&[(
			"mode",
			BlockStatePropertyType::Enum {
				values: &["save", "load", "corner", "data"]
			}
		)]
	),
	("structure_void", &[]),
	(
		"sugar_cane",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"sunflower",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"sweet_berry_bush",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"tall_grass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_seagrass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"target",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("terracotta", &[]),
	("tinted_glass", &[]),
	("tnt", &[("unstable", BlockStatePropertyType::Boolean)]),
	("torch", &[]),
	(
		"trapped_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire",
		&[
			("attached", BlockStatePropertyType::Boolean),
			("disarmed", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire_hook",
		&[
			("attached", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("tube_coral_block", &[]),
	(
		"tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("tuff", &[]),
	(
		"turtle_egg",
		&[
			(
				"eggs",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			(
				"hatch",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			)
		]
	),
	(
		"twisting_vines",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("twisting_vines_plant", &[]),
	(
		"vine",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("void_air", &[]),
	(
		"wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"warped_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("warped_fungus", &[]),
	(
		"warped_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("warped_nylium", &[]),
	("warped_planks", &[]),
	(
		"warped_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("warped_roots", &[]),
	(
		"warped_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"warped_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("warped_wart_block", &[]),
	(
		"water",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"water_cauldron",
		&[(
			"level",
			BlockStatePropertyType::StrictlyPositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("waxed_copper_block", &[]),
	("waxed_cut_copper", &[]),
	(
		"waxed_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"waxed_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("waxed_exposed_copper", &[]),
	("waxed_exposed_cut_copper", &[]),
	(
		"waxed_exposed_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"waxed_exposed_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("waxed_oxidized_copper", &[]),
	("waxed_oxidized_cut_copper", &[]),
	(
		"waxed_oxidized_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"waxed_oxidized_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("waxed_weathered_copper", &[]),
	("waxed_weathered_cut_copper", &[]),
	(
		"waxed_weathered_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"waxed_weathered_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("weathered_copper", &[]),
	("weathered_cut_copper", &[]),
	(
		"weathered_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"weathered_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"weeping_vines",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("weeping_vines_plant", &[]),
	("wet_sponge", &[]),
	(
		"wheat",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"white_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"white_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"white_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"white_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("white_carpet", &[]),
	("white_concrete", &[]),
	("white_concrete_powder", &[]),
	(
		"white_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"white_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("white_stained_glass", &[]),
	(
		"white_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("white_terracotta", &[]),
	("white_tulip", &[]),
	(
		"white_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("white_wool", &[]),
	("wither_rose", &[]),
	(
		"wither_skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"wither_skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"yellow_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"yellow_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"yellow_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("yellow_carpet", &[]),
	("yellow_concrete", &[]),
	("yellow_concrete_powder", &[]),
	(
		"yellow_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("yellow_stained_glass", &[]),
	(
		"yellow_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("yellow_terracotta", &[]),
	(
		"yellow_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("yellow_wool", &[]),
	(
		"zombie_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"zombie_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	)
];
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_18_TO_1_19_EXCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	900] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("air", &[]),
	("allium", &[]),
	("amethyst_block", &[]),
	(
		"amethyst_cluster",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ancient_debris", &[]),
	("andesite", &[]),
	(
		"andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azalea", &[]),
	(
		"azalea_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	("azure_bluet", &[]),
	(
		"bamboo",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			),
			(
				"leaves",
				BlockStatePropertyType::Enum {
					values: &["none", "small", "large"]
				}
			),
			(
				"stage",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			)
		]
	),
	("bamboo_sapling", &[]),
	(
		"barrel",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("open", BlockStatePropertyType::Boolean)
		]
	),
	("barrier", &[]),
	(
		"basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"bee_nest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beehive",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"bell",
		&[
			(
				"attachment",
				BlockStatePropertyType::Enum {
					values: &["floor", "ceiling", "single_wall", "double_wall"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"big_dripleaf",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"tilt",
				BlockStatePropertyType::Enum {
					values: &["none", "unstable", "partial", "full"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"big_dripleaf_stem",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"black_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"black_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	("blackstone", &[]),
	(
		"blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"blast_furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"blue_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	(
		"brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"brown_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("budding_amethyst", &[]),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	("calcite", &[]),
	(
		"campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("candle_cake", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("cartography_table", &[]),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cauldron", &[]),
	("cave_air", &[]),
	(
		"cave_vines",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
				}
			),
			("berries", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cave_vines_plant",
		&[("berries", BlockStatePropertyType::Boolean)]
	),
	(
		"chain",
		&[
			(
				"axis",
				BlockStatePropertyType::Enum {
					values: &["x", "y", "z"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_deepslate", &[]),
	("chiseled_nether_bricks", &[]),
	("chiseled_polished_blackstone", &[]),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobbled_deepslate", &[]),
	(
		"cobbled_deepslate_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobbled_deepslate_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobbled_deepslate_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"composter",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	(
		"conduit",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("copper_block", &[]),
	("copper_ore", &[]),
	("cornflower", &[]),
	("cracked_deepslate_bricks", &[]),
	("cracked_deepslate_tiles", &[]),
	("cracked_nether_bricks", &[]),
	("cracked_polished_blackstone_bricks", &[]),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"crimson_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("crimson_fungus", &[]),
	(
		"crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("crimson_nylium", &[]),
	("crimson_planks", &[]),
	(
		"crimson_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("crimson_roots", &[]),
	(
		"crimson_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"crimson_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("crying_obsidian", &[]),
	("cut_copper", &[]),
	(
		"cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_red_sandstone", &[]),
	(
		"cut_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_sandstone", &[]),
	(
		"cut_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"cyan_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	(
		"dead_brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	(
		"dead_fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"deepslate_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("deepslate_bricks", &[]),
	("deepslate_coal_ore", &[]),
	("deepslate_copper_ore", &[]),
	("deepslate_diamond_ore", &[]),
	("deepslate_emerald_ore", &[]),
	("deepslate_gold_ore", &[]),
	("deepslate_iron_ore", &[]),
	("deepslate_lapis_ore", &[]),
	(
		"deepslate_redstone_ore",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"deepslate_tile_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_tile_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_tile_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("deepslate_tiles", &[]),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	(
		"diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("dirt", &[]),
	("dirt_path", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	("dripstone_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	(
		"end_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("exposed_copper", &[]),
	("exposed_cut_copper", &[]),
	(
		"exposed_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"exposed_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("fletching_table", &[]),
	("flower_pot", &[]),
	("flowering_azalea", &[]),
	(
		"flowering_azalea_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("gilded_blackstone", &[]),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"glow_item_frame",
		&[("map", BlockStatePropertyType::Boolean)]
	),
	(
		"glow_lichen",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	(
		"granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"gray_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"gray_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"green_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"green_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"grindstone",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"hanging_roots",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("honey_block", &[]),
	("honeycomb_block", &[]),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	(
		"infested_deepslate",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jigsaw",
		&[(
			"orientation",
			BlockStatePropertyType::Enum {
				values: &[
					"down_east",
					"down_north",
					"down_south",
					"down_west",
					"up_east",
					"up_north",
					"up_south",
					"up_west",
					"west_up",
					"east_up",
					"north_up",
					"south_up"
				]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lantern",
		&[
			("hanging", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("lava_cauldron", &[]),
	(
		"lectern",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("has_book", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light",
		&[
			(
				"level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"light_blue_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"light_gray_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_gray_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lightning_rod",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_of_the_valley", &[]),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"lime_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lime_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	("lodestone", &[]),
	(
		"loom",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"magenta_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"magenta_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	(
		"medium_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("moss_block", &[]),
	("moss_carpet", &[]),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"mossy_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("nether_bricks", &[]),
	("nether_gold_ore", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	("nether_sprouts", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherite_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone",
						"iron_xylophone",
						"cow_bell",
						"didgeridoo",
						"bit",
						"banjo",
						"pling"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"orange_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"orange_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("oxidized_copper", &[]),
	("oxidized_cut_copper", &[]),
	(
		"oxidized_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oxidized_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("packed_ice", &[]),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"pink_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"pointed_dripstone",
		&[
			(
				"thickness",
				BlockStatePropertyType::Enum {
					values: &["tip_merge", "tip", "frustum", "middle", "base"]
				}
			),
			(
				"vertical_direction",
				BlockStatePropertyType::Enum {
					values: &["up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_andesite", &[]),
	(
		"polished_andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("polished_blackstone", &[]),
	(
		"polished_blackstone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_blackstone_bricks", &[]),
	(
		"polished_blackstone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"polished_blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_deepslate", &[]),
	(
		"polished_deepslate_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_deepslate_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_deepslate_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_diorite", &[]),
	(
		"polished_diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_granite", &[]),
	(
		"polished_granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azalea_bush", &[]),
	("potted_azure_bluet", &[]),
	("potted_bamboo", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_cornflower", &[]),
	("potted_crimson_fungus", &[]),
	("potted_crimson_roots", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_flowering_azalea_bush", &[]),
	("potted_jungle_sapling", &[]),
	("potted_lily_of_the_valley", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_warped_fungus", &[]),
	("potted_warped_roots", &[]),
	("potted_white_tulip", &[]),
	("potted_wither_rose", &[]),
	("powder_snow", &[]),
	(
		"powder_snow_cauldron",
		&[(
			"level",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3"]
			}
		)]
	),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"purple_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purple_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	("quartz_bricks", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south",
						"south_east",
						"south_west",
						"north_west",
						"north_east"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("raw_copper_block", &[]),
	("raw_gold_block", &[]),
	("raw_iron_block", &[]),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"red_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"respawn_anchor",
		&[(
			"charges",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
			}
		)]
	),
	("rooted_dirt", &[]),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"scaffolding",
		&[
			("bottom", BlockStatePropertyType::Boolean),
			(
				"distance",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sculk_sensor",
		&[
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"sculk_sensor_phase",
				BlockStatePropertyType::Enum {
					values: &["inactive", "active", "cooldown"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	("shroomlight", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	(
		"small_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"small_dripleaf",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smithing_table", &[]),
	(
		"smoker",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_basalt", &[]),
	("smooth_quartz", &[]),
	(
		"smooth_quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_red_sandstone", &[]),
	(
		"smooth_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_sandstone", &[]),
	(
		"smooth_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_stone", &[]),
	(
		"smooth_stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3", "4", "5", "6", "7", "8"]
			}
		)]
	),
	("snow_block", &[]),
	(
		"soul_campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_fire", &[]),
	(
		"soul_lantern",
		&[
			("hanging", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_sand", &[]),
	("soul_soil", &[]),
	("soul_torch", &[]),
	(
		"soul_wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("spawner", &[]),
	("sponge", &[]),
	("spore_blossom", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4", "5", "6", "7"]
				}
			),
			("persistent", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("spruce_planks", &[]),
	(
		"spruce_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"spruce_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"spruce_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"sticky_piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	("stone", &[]),
	(
		"stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("stone_bricks", &[]),
	(
		"stone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"stonecutter",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"stripped_acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_spruce_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_warped_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"stripped_warped_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"structure_block",
		&[(
			"mode",
			BlockStatePropertyType::Enum {
				values: &["save", "load", "corner", "data"]
			}
		)]
	),
	("structure_void", &[]),
	(
		"sugar_cane",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"sunflower",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"sweet_berry_bush",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"tall_grass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"tall_seagrass",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"target",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("terracotta", &[]),
	("tinted_glass", &[]),
	("tnt", &[("unstable", BlockStatePropertyType::Boolean)]),
	("torch", &[]),
	(
		"trapped_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire",
		&[
			("attached", BlockStatePropertyType::Boolean),
			("disarmed", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tripwire_hook",
		&[
			("attached", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("tube_coral_block", &[]),
	(
		"tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("tuff", &[]),
	(
		"turtle_egg",
		&[
			(
				"eggs",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			(
				"hatch",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			)
		]
	),
	(
		"twisting_vines",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("twisting_vines_plant", &[]),
	(
		"vine",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("void_air", &[]),
	(
		"wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"warped_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("warped_fungus", &[]),
	(
		"warped_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("warped_nylium", &[]),
	("warped_planks", &[]),
	(
		"warped_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("warped_roots", &[]),
	(
		"warped_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"warped_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"warped_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("warped_wart_block", &[]),
	(
		"water",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"water_cauldron",
		&[(
			"level",
			BlockStatePropertyType::Enum {
				values: &["1", "2", "3"]
			}
		)]
	),
	("waxed_copper_block", &[]),
	("waxed_cut_copper", &[]),
	(
		"waxed_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"waxed_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("waxed_exposed_copper", &[]),
	("waxed_exposed_cut_copper", &[]),
	(
		"waxed_exposed_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"waxed_exposed_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("waxed_oxidized_copper", &[]),
	("waxed_oxidized_cut_copper", &[]),
	(
		"waxed_oxidized_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"waxed_oxidized_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("waxed_weathered_copper", &[]),
	("waxed_weathered_cut_copper", &[]),
	(
		"waxed_weathered_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"waxed_weathered_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("weathered_copper", &[]),
	("weathered_cut_copper", &[]),
	(
		"weathered_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"weathered_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"weeping_vines",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("weeping_vines_plant", &[]),
	("wet_sponge", &[]),
	(
		"wheat",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"white_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"white_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"white_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"white_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("white_carpet", &[]),
	("white_concrete", &[]),
	("white_concrete_powder", &[]),
	(
		"white_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"white_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("white_stained_glass", &[]),
	(
		"white_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("white_terracotta", &[]),
	("white_tulip", &[]),
	(
		"white_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("white_wool", &[]),
	("wither_rose", &[]),
	(
		"wither_skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"wither_skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"yellow_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"yellow_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::Enum {
					values: &["1", "2", "3", "4"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"yellow_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("yellow_carpet", &[]),
	("yellow_concrete", &[]),
	("yellow_concrete_powder", &[]),
	(
		"yellow_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"yellow_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("yellow_stained_glass", &[]),
	(
		"yellow_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("yellow_terracotta", &[]),
	(
		"yellow_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("yellow_wool", &[]),
	(
		"zombie_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"zombie_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	)
];
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_19_TO_1_19_INCLUSIVE: [(&str, &[(&str, BlockStatePropertyType)]);
	935] = [
	(
		"acacia_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("acacia_planks", &[]),
	(
		"acacia_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"acacia_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"acacia_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"acacia_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"activator_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("air", &[]),
	("allium", &[]),
	("amethyst_block", &[]),
	(
		"amethyst_cluster",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ancient_debris", &[]),
	("andesite", &[]),
	(
		"andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"andesite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_melon_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"attached_pumpkin_stem",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("azalea", &[]),
	(
		"azalea_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("azure_bluet", &[]),
	(
		"bamboo",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			),
			(
				"leaves",
				BlockStatePropertyType::Enum {
					values: &["none", "small", "large"]
				}
			),
			(
				"stage",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			)
		]
	),
	("bamboo_sapling", &[]),
	(
		"barrel",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("open", BlockStatePropertyType::Boolean)
		]
	),
	("barrier", &[]),
	(
		"basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("beacon", &[]),
	("bedrock", &[]),
	(
		"bee_nest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beehive",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"honey_level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
				}
			)
		]
	),
	(
		"beetroots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"bell",
		&[
			(
				"attachment",
				BlockStatePropertyType::Enum {
					values: &["floor", "ceiling", "single_wall", "double_wall"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"big_dripleaf",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"tilt",
				BlockStatePropertyType::Enum {
					values: &["none", "unstable", "partial", "full"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"big_dripleaf_stem",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("birch_planks", &[]),
	(
		"birch_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"birch_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"birch_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"birch_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"black_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"black_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"black_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"black_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("black_carpet", &[]),
	("black_concrete", &[]),
	("black_concrete_powder", &[]),
	(
		"black_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"black_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("black_stained_glass", &[]),
	(
		"black_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("black_terracotta", &[]),
	(
		"black_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("black_wool", &[]),
	("blackstone", &[]),
	(
		"blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"blast_furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"blue_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"blue_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("blue_carpet", &[]),
	("blue_concrete", &[]),
	("blue_concrete_powder", &[]),
	(
		"blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_ice", &[]),
	("blue_orchid", &[]),
	(
		"blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("blue_stained_glass", &[]),
	(
		"blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("blue_terracotta", &[]),
	(
		"blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("blue_wool", &[]),
	(
		"bone_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("bookshelf", &[]),
	(
		"brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("brain_coral_block", &[]),
	(
		"brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brewing_stand",
		&[
			("has_bottle_0", BlockStatePropertyType::Boolean),
			("has_bottle_1", BlockStatePropertyType::Boolean),
			("has_bottle_2", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("bricks", &[]),
	(
		"brown_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"brown_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"brown_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("brown_carpet", &[]),
	("brown_concrete", &[]),
	("brown_concrete_powder", &[]),
	(
		"brown_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_mushroom", &[]),
	(
		"brown_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"brown_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("brown_stained_glass", &[]),
	(
		"brown_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("brown_terracotta", &[]),
	(
		"brown_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("brown_wool", &[]),
	(
		"bubble_column",
		&[("drag", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("bubble_coral_block", &[]),
	(
		"bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("budding_amethyst", &[]),
	(
		"cactus",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cake",
		&[(
			"bites",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(6) }
			}
		)]
	),
	("calcite", &[]),
	(
		"campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("candle_cake", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"carrots",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("cartography_table", &[]),
	(
		"carved_pumpkin",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cauldron", &[]),
	("cave_air", &[]),
	(
		"cave_vines",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
				}
			),
			("berries", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cave_vines_plant",
		&[("berries", BlockStatePropertyType::Boolean)]
	),
	(
		"chain",
		&[
			(
				"axis",
				BlockStatePropertyType::Enum {
					values: &["x", "y", "z"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chain_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["single", "left", "right"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"chipped_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("chiseled_deepslate", &[]),
	("chiseled_nether_bricks", &[]),
	("chiseled_polished_blackstone", &[]),
	("chiseled_quartz_block", &[]),
	("chiseled_red_sandstone", &[]),
	("chiseled_sandstone", &[]),
	("chiseled_stone_bricks", &[]),
	(
		"chorus_flower",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(5) }
			}
		)]
	),
	(
		"chorus_plant",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("clay", &[]),
	("coal_block", &[]),
	("coal_ore", &[]),
	("coarse_dirt", &[]),
	("cobbled_deepslate", &[]),
	(
		"cobbled_deepslate_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobbled_deepslate_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobbled_deepslate_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("cobblestone", &[]),
	(
		"cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("cobweb", &[]),
	(
		"cocoa",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(2) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"comparator",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"mode",
				BlockStatePropertyType::Enum {
					values: &["compare", "subtract"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"composter",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	(
		"conduit",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("copper_block", &[]),
	("copper_ore", &[]),
	("cornflower", &[]),
	("cracked_deepslate_bricks", &[]),
	("cracked_deepslate_tiles", &[]),
	("cracked_nether_bricks", &[]),
	("cracked_polished_blackstone_bricks", &[]),
	("cracked_stone_bricks", &[]),
	("crafting_table", &[]),
	(
		"creeper_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"creeper_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"crimson_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("crimson_fungus", &[]),
	(
		"crimson_hyphae",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("crimson_nylium", &[]),
	("crimson_planks", &[]),
	(
		"crimson_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	("crimson_roots", &[]),
	(
		"crimson_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_stem",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"crimson_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"crimson_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("crying_obsidian", &[]),
	("cut_copper", &[]),
	(
		"cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_red_sandstone", &[]),
	(
		"cut_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("cut_sandstone", &[]),
	(
		"cut_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"cyan_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"cyan_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"cyan_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("cyan_carpet", &[]),
	("cyan_concrete", &[]),
	("cyan_concrete_powder", &[]),
	(
		"cyan_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"cyan_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("cyan_stained_glass", &[]),
	(
		"cyan_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("cyan_terracotta", &[]),
	(
		"cyan_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("cyan_wool", &[]),
	(
		"damaged_anvil",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dandelion", &[]),
	(
		"dark_oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_oak_planks", &[]),
	(
		"dark_oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"dark_oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"dark_oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("dark_prismarine", &[]),
	(
		"dark_prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dark_prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"daylight_detector",
		&[
			("inverted", BlockStatePropertyType::Boolean),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			)
		]
	),
	(
		"dead_brain_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_brain_coral_block", &[]),
	(
		"dead_brain_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_brain_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_bubble_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_bubble_coral_block", &[]),
	(
		"dead_bubble_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_bubble_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("dead_bush", &[]),
	(
		"dead_fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_fire_coral_block", &[]),
	(
		"dead_fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_horn_coral_block", &[]),
	(
		"dead_horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"dead_tube_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("dead_tube_coral_block", &[]),
	(
		"dead_tube_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"dead_tube_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"deepslate_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("deepslate_bricks", &[]),
	("deepslate_coal_ore", &[]),
	("deepslate_copper_ore", &[]),
	("deepslate_diamond_ore", &[]),
	("deepslate_emerald_ore", &[]),
	("deepslate_gold_ore", &[]),
	("deepslate_iron_ore", &[]),
	("deepslate_lapis_ore", &[]),
	(
		"deepslate_redstone_ore",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"deepslate_tile_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_tile_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"deepslate_tile_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("deepslate_tiles", &[]),
	(
		"detector_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("diamond_block", &[]),
	("diamond_ore", &[]),
	("diorite", &[]),
	(
		"diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"diorite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("dirt", &[]),
	("dirt_path", &[]),
	(
		"dispenser",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("dragon_egg", &[]),
	(
		"dragon_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"dragon_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("dried_kelp_block", &[]),
	("dripstone_block", &[]),
	(
		"dropper",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("triggered", BlockStatePropertyType::Boolean)
		]
	),
	("emerald_block", &[]),
	("emerald_ore", &[]),
	("enchanting_table", &[]),
	("end_gateway", &[]),
	("end_portal", &[]),
	(
		"end_portal_frame",
		&[
			("eye", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"end_rod",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("end_stone", &[]),
	(
		"end_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"end_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("end_stone_bricks", &[]),
	(
		"ender_chest",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("exposed_copper", &[]),
	("exposed_cut_copper", &[]),
	(
		"exposed_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"exposed_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"farmland",
		&[(
			"moisture",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("fern", &[]),
	(
		"fire",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"fire_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("fire_coral_block", &[]),
	(
		"fire_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"fire_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("fletching_table", &[]),
	("flower_pot", &[]),
	("flowering_azalea", &[]),
	(
		"flowering_azalea_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("frogspawn", &[]),
	(
		"frosted_ice",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"furnace",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("gilded_blackstone", &[]),
	("glass", &[]),
	(
		"glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"glow_item_frame",
		&[("map", BlockStatePropertyType::Boolean)]
	),
	(
		"glow_lichen",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("glowstone", &[]),
	("gold_block", &[]),
	("gold_ore", &[]),
	("granite", &[]),
	(
		"granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"granite_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("grass", &[]),
	("grass_block", &[("snowy", BlockStatePropertyType::Boolean)]),
	("gravel", &[]),
	(
		"gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"gray_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"gray_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("gray_carpet", &[]),
	("gray_concrete", &[]),
	("gray_concrete_powder", &[]),
	(
		"gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("gray_stained_glass", &[]),
	(
		"gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("gray_terracotta", &[]),
	(
		"gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("gray_wool", &[]),
	(
		"green_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"green_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"green_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"green_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("green_carpet", &[]),
	("green_concrete", &[]),
	("green_concrete_powder", &[]),
	(
		"green_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"green_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("green_stained_glass", &[]),
	(
		"green_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("green_terracotta", &[]),
	(
		"green_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("green_wool", &[]),
	(
		"grindstone",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"hanging_roots",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"hay_block",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"heavy_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("honey_block", &[]),
	("honeycomb_block", &[]),
	(
		"hopper",
		&[
			("enabled", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["down", "north", "south", "west", "east"]
				}
			)
		]
	),
	(
		"horn_coral",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	("horn_coral_block", &[]),
	(
		"horn_coral_fan",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"horn_coral_wall_fan",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("ice", &[]),
	("infested_chiseled_stone_bricks", &[]),
	("infested_cobblestone", &[]),
	("infested_cracked_stone_bricks", &[]),
	(
		"infested_deepslate",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("infested_mossy_stone_bricks", &[]),
	("infested_stone", &[]),
	("infested_stone_bricks", &[]),
	(
		"iron_bars",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("iron_block", &[]),
	(
		"iron_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("iron_ore", &[]),
	(
		"iron_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("item_frame", &[("map", BlockStatePropertyType::Boolean)]),
	(
		"jack_o_lantern",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"jigsaw",
		&[(
			"orientation",
			BlockStatePropertyType::Enum {
				values: &[
					"down_east",
					"down_north",
					"down_south",
					"down_west",
					"up_east",
					"up_north",
					"up_south",
					"up_west",
					"west_up",
					"east_up",
					"north_up",
					"south_up"
				]
			}
		)]
	),
	(
		"jukebox",
		&[("has_record", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("jungle_planks", &[]),
	(
		"jungle_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"jungle_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"jungle_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"jungle_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"kelp",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(25) }
			}
		)]
	),
	("kelp_plant", &[]),
	(
		"ladder",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lantern",
		&[
			("hanging", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("lapis_block", &[]),
	("lapis_ore", &[]),
	(
		"large_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"large_fern",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"lava",
		&[(
			"level",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	("lava_cauldron", &[]),
	(
		"lectern",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("has_book", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lever",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light",
		&[
			(
				"level",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_blue_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"light_blue_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_blue_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("light_blue_carpet", &[]),
	("light_blue_concrete", &[]),
	("light_blue_concrete_powder", &[]),
	(
		"light_blue_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_blue_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_blue_stained_glass", &[]),
	(
		"light_blue_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_blue_terracotta", &[]),
	(
		"light_blue_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_blue_wool", &[]),
	(
		"light_gray_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"light_gray_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"light_gray_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"light_gray_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("light_gray_carpet", &[]),
	("light_gray_concrete", &[]),
	("light_gray_concrete_powder", &[]),
	(
		"light_gray_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"light_gray_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("light_gray_stained_glass", &[]),
	(
		"light_gray_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("light_gray_terracotta", &[]),
	(
		"light_gray_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("light_gray_wool", &[]),
	(
		"light_weighted_pressure_plate",
		&[(
			"power",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lightning_rod",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lilac",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("lily_of_the_valley", &[]),
	("lily_pad", &[]),
	(
		"lime_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"lime_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"lime_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"lime_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("lime_carpet", &[]),
	("lime_concrete", &[]),
	("lime_concrete_powder", &[]),
	(
		"lime_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"lime_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("lime_stained_glass", &[]),
	(
		"lime_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("lime_terracotta", &[]),
	(
		"lime_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("lime_wool", &[]),
	("lodestone", &[]),
	(
		"loom",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"magenta_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"magenta_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"magenta_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("magenta_carpet", &[]),
	("magenta_concrete", &[]),
	("magenta_concrete_powder", &[]),
	(
		"magenta_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"magenta_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("magenta_stained_glass", &[]),
	(
		"magenta_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("magenta_terracotta", &[]),
	(
		"magenta_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("magenta_wool", &[]),
	("magma_block", &[]),
	(
		"mangrove_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("mangrove_planks", &[]),
	(
		"mangrove_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"mangrove_propagule",
		&[
			(
				"age",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("hanging", BlockStatePropertyType::Boolean),
			(
				"stage",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_roots",
		&[("waterlogged", BlockStatePropertyType::Boolean)]
	),
	(
		"mangrove_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mangrove_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"medium_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("melon", &[]),
	(
		"melon_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("moss_block", &[]),
	("moss_carpet", &[]),
	("mossy_cobblestone", &[]),
	(
		"mossy_cobblestone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_cobblestone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"mossy_stone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mossy_stone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("mossy_stone_bricks", &[]),
	(
		"moving_piston",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	("mud", &[]),
	(
		"mud_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mud_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"mud_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("mud_bricks", &[]),
	(
		"muddy_mangrove_roots",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"mushroom_stem",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("mycelium", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"nether_brick_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("nether_bricks", &[]),
	("nether_gold_ore", &[]),
	(
		"nether_portal",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "z"]
			}
		)]
	),
	("nether_quartz_ore", &[]),
	("nether_sprouts", &[]),
	(
		"nether_wart",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	("nether_wart_block", &[]),
	("netherite_block", &[]),
	("netherrack", &[]),
	(
		"note_block",
		&[
			(
				"instrument",
				BlockStatePropertyType::Enum {
					values: &[
						"harp",
						"basedrum",
						"snare",
						"hat",
						"bass",
						"flute",
						"bell",
						"guitar",
						"chime",
						"xylophone",
						"iron_xylophone",
						"cow_bell",
						"didgeridoo",
						"bit",
						"banjo",
						"pling"
					]
				}
			),
			(
				"note",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(24) }
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			(
				"hinge",
				BlockStatePropertyType::Enum {
					values: &["left", "right"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_fence_gate",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("in_wall", BlockStatePropertyType::Boolean),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_leaves",
		&[
			(
				"distance",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("persistent", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_log",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("oak_planks", &[]),
	(
		"oak_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"oak_sapling",
		&[(
			"stage",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(1) }
			}
		)]
	),
	(
		"oak_sign",
		&[
			(
				"rotation",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_trapdoor",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			("open", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wall_sign",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oak_wood",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"observer",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	("obsidian", &[]),
	(
		"ochre_froglight",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"orange_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"orange_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"orange_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"orange_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("orange_carpet", &[]),
	("orange_concrete", &[]),
	("orange_concrete_powder", &[]),
	(
		"orange_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"orange_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("orange_stained_glass", &[]),
	(
		"orange_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("orange_terracotta", &[]),
	("orange_tulip", &[]),
	(
		"orange_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("orange_wool", &[]),
	("oxeye_daisy", &[]),
	("oxidized_copper", &[]),
	("oxidized_cut_copper", &[]),
	(
		"oxidized_cut_copper_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"oxidized_cut_copper_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("packed_ice", &[]),
	("packed_mud", &[]),
	(
		"pearlescent_froglight",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"peony",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	(
		"petrified_oak_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"pink_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"pink_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"pink_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("pink_carpet", &[]),
	("pink_concrete", &[]),
	("pink_concrete_powder", &[]),
	(
		"pink_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"pink_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("pink_stained_glass", &[]),
	(
		"pink_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("pink_terracotta", &[]),
	("pink_tulip", &[]),
	(
		"pink_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("pink_wool", &[]),
	(
		"piston",
		&[
			("extended", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"piston_head",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("short", BlockStatePropertyType::Boolean),
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["normal", "sticky"]
				}
			)
		]
	),
	(
		"player_head",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"player_wall_head",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("podzol", &[("snowy", BlockStatePropertyType::Boolean)]),
	(
		"pointed_dripstone",
		&[
			(
				"thickness",
				BlockStatePropertyType::Enum {
					values: &["tip_merge", "tip", "frustum", "middle", "base"]
				}
			),
			(
				"vertical_direction",
				BlockStatePropertyType::Enum {
					values: &["up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_andesite", &[]),
	(
		"polished_andesite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_andesite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_basalt",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	("polished_blackstone", &[]),
	(
		"polished_blackstone_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_blackstone_bricks", &[]),
	(
		"polished_blackstone_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_pressure_plate",
		&[("powered", BlockStatePropertyType::Boolean)]
	),
	(
		"polished_blackstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_blackstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_deepslate", &[]),
	(
		"polished_deepslate_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_deepslate_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_deepslate_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("polished_diorite", &[]),
	(
		"polished_diorite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_diorite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("polished_granite", &[]),
	(
		"polished_granite_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"polished_granite_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("poppy", &[]),
	(
		"potatoes",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	("potted_acacia_sapling", &[]),
	("potted_allium", &[]),
	("potted_azalea_bush", &[]),
	("potted_azure_bluet", &[]),
	("potted_bamboo", &[]),
	("potted_birch_sapling", &[]),
	("potted_blue_orchid", &[]),
	("potted_brown_mushroom", &[]),
	("potted_cactus", &[]),
	("potted_cornflower", &[]),
	("potted_crimson_fungus", &[]),
	("potted_crimson_roots", &[]),
	("potted_dandelion", &[]),
	("potted_dark_oak_sapling", &[]),
	("potted_dead_bush", &[]),
	("potted_fern", &[]),
	("potted_flowering_azalea_bush", &[]),
	("potted_jungle_sapling", &[]),
	("potted_lily_of_the_valley", &[]),
	("potted_mangrove_propagule", &[]),
	("potted_oak_sapling", &[]),
	("potted_orange_tulip", &[]),
	("potted_oxeye_daisy", &[]),
	("potted_pink_tulip", &[]),
	("potted_poppy", &[]),
	("potted_red_mushroom", &[]),
	("potted_red_tulip", &[]),
	("potted_spruce_sapling", &[]),
	("potted_warped_fungus", &[]),
	("potted_warped_roots", &[]),
	("potted_white_tulip", &[]),
	("potted_wither_rose", &[]),
	("powder_snow", &[]),
	(
		"powder_snow_cauldron",
		&[(
			"level",
			BlockStatePropertyType::StrictlyPositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(3) }
			}
		)]
	),
	(
		"powered_rail",
		&[
			("powered", BlockStatePropertyType::Boolean),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine", &[]),
	(
		"prismarine_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("prismarine_bricks", &[]),
	(
		"prismarine_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"prismarine_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("pumpkin", &[]),
	(
		"pumpkin_stem",
		&[(
			"age",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
			}
		)]
	),
	(
		"purple_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"purple_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"purple_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purple_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("purple_carpet", &[]),
	("purple_concrete", &[]),
	("purple_concrete_powder", &[]),
	(
		"purple_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	(
		"purple_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("purple_stained_glass", &[]),
	(
		"purple_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("purple_terracotta", &[]),
	(
		"purple_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("purple_wool", &[]),
	("purpur_block", &[]),
	(
		"purpur_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"purpur_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"purpur_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("quartz_block", &[]),
	("quartz_bricks", &[]),
	(
		"quartz_pillar",
		&[(
			"axis",
			BlockStatePropertyType::Enum {
				values: &["x", "y", "z"]
			}
		)]
	),
	(
		"quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"rail",
		&[
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"north_south",
						"east_west",
						"ascending_east",
						"ascending_west",
						"ascending_north",
						"ascending_south",
						"south_east",
						"south_west",
						"north_west",
						"north_east"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("raw_copper_block", &[]),
	("raw_gold_block", &[]),
	("raw_iron_block", &[]),
	(
		"red_banner",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"red_bed",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("occupied", BlockStatePropertyType::Boolean),
			(
				"part",
				BlockStatePropertyType::Enum {
					values: &["head", "foot"]
				}
			)
		]
	),
	(
		"red_candle",
		&[
			(
				"candles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_candle_cake",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	("red_carpet", &[]),
	("red_concrete", &[]),
	("red_concrete_powder", &[]),
	(
		"red_glazed_terracotta",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_mushroom", &[]),
	(
		"red_mushroom_block",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_nether_brick_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	("red_nether_bricks", &[]),
	("red_sand", &[]),
	("red_sandstone", &[]),
	(
		"red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"red_sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"red_shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	("red_stained_glass", &[]),
	(
		"red_stained_glass_pane",
		&[
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("red_terracotta", &[]),
	("red_tulip", &[]),
	(
		"red_wall_banner",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("red_wool", &[]),
	("redstone_block", &[]),
	("redstone_lamp", &[("lit", BlockStatePropertyType::Boolean)]),
	("redstone_ore", &[("lit", BlockStatePropertyType::Boolean)]),
	(
		"redstone_torch",
		&[("lit", BlockStatePropertyType::Boolean)]
	),
	(
		"redstone_wall_torch",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	(
		"redstone_wire",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["up", "side", "none"]
				}
			)
		]
	),
	("reinforced_deepslate", &[]),
	(
		"repeater",
		&[
			(
				"delay",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("locked", BlockStatePropertyType::Boolean),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"repeating_command_block",
		&[
			("conditional", BlockStatePropertyType::Boolean),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			)
		]
	),
	(
		"respawn_anchor",
		&[(
			"charges",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
			}
		)]
	),
	("rooted_dirt", &[]),
	(
		"rose_bush",
		&[(
			"half",
			BlockStatePropertyType::Enum {
				values: &["upper", "lower"]
			}
		)]
	),
	("sand", &[]),
	("sandstone", &[]),
	(
		"sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sandstone_wall",
		&[
			(
				"east",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"north",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			(
				"south",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			(
				"west",
				BlockStatePropertyType::Enum {
					values: &["none", "low", "tall"]
				}
			)
		]
	),
	(
		"scaffolding",
		&[
			("bottom", BlockStatePropertyType::Boolean),
			(
				"distance",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(7) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("sculk", &[]),
	(
		"sculk_catalyst",
		&[("bloom", BlockStatePropertyType::Boolean)]
	),
	(
		"sculk_sensor",
		&[
			(
				"power",
				BlockStatePropertyType::PositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
				}
			),
			(
				"sculk_sensor_phase",
				BlockStatePropertyType::Enum {
					values: &["inactive", "active", "cooldown"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sculk_shrieker",
		&[
			("can_summon", BlockStatePropertyType::Boolean),
			("shrieking", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"sculk_vein",
		&[
			("down", BlockStatePropertyType::Boolean),
			("east", BlockStatePropertyType::Boolean),
			("north", BlockStatePropertyType::Boolean),
			("south", BlockStatePropertyType::Boolean),
			("up", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean),
			("west", BlockStatePropertyType::Boolean)
		]
	),
	("sea_lantern", &[]),
	(
		"sea_pickle",
		&[
			(
				"pickles",
				BlockStatePropertyType::StrictlyPositiveInteger {
					maximum_value: unsafe { NonZeroU8::new_unchecked(4) }
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("seagrass", &[]),
	("shroomlight", &[]),
	(
		"shulker_box",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "east", "south", "west", "up", "down"]
			}
		)]
	),
	(
		"skeleton_skull",
		&[(
			"rotation",
			BlockStatePropertyType::PositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(15) }
			}
		)]
	),
	(
		"skeleton_wall_skull",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("slime_block", &[]),
	(
		"small_amethyst_bud",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "east", "south", "west", "up", "down"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"small_dripleaf",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["upper", "lower"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smithing_table", &[]),
	(
		"smoker",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_basalt", &[]),
	("smooth_quartz", &[]),
	(
		"smooth_quartz_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_quartz_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_red_sandstone", &[]),
	(
		"smooth_red_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_red_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_sandstone", &[]),
	(
		"smooth_sandstone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"smooth_sandstone_stairs",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom"]
				}
			),
			(
				"shape",
				BlockStatePropertyType::Enum {
					values: &[
						"straight",
						"inner_left",
						"inner_right",
						"outer_left",
						"outer_right"
					]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("smooth_stone", &[]),
	(
		"smooth_stone_slab",
		&[
			(
				"type",
				BlockStatePropertyType::Enum {
					values: &["top", "bottom", "double"]
				}
			),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	(
		"snow",
		&[(
			"layers",
			BlockStatePropertyType::StrictlyPositiveInteger {
				maximum_value: unsafe { NonZeroU8::new_unchecked(8) }
			}
		)]
	),
	("snow_block", &[]),
	(
		"soul_campfire",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("lit", BlockStatePropertyType::Boolean),
			("signal_fire", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_fire", &[]),
	(
		"soul_lantern",
		&[
			("hanging", BlockStatePropertyType::Boolean),
			("waterlogged", BlockStatePropertyType::Boolean)
		]
	),
	("soul_sand", &[]),
	("soul_soil", &[]),
	("soul_torch", &[]),
	(
		"soul_wall_torch",
		&[(
			"facing",
			BlockStatePropertyType::Enum {
				values: &["north", "south", "west", "east"]
			}
		)]
	),
	("spawner", &[]),
	("sponge", &[]),
	("spore_blossom", &[]),
	(
		"spruce_button",
		&[
			(
				"face",
				BlockStatePropertyType::Enum {
					values: &["floor", "wall", "ceiling"]
				}
			),
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			("powered", BlockStatePropertyType::Boolean)
		]
	),
	(
		"spruce_door",
		&[
			(
				"facing",
				BlockStatePropertyType::Enum {
					values: &["north", "south", "west", "east"]
				}
			),
			(
				"half",
				BlockStatePropertyType::Enum {
				}
			),
			(
				}
