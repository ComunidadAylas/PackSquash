//! This automatically generated file contains the list of block state properties for several vanilla Minecraft versions.

use crate::minecraft_version::MinecraftVersion;
use crate::util::range_bounds_intersect::RangeBoundsIntersectExt;
use ahash::AHashMap;
use phf::phf_map;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU8;
use std::ops::RangeBounds;
use tinyvec::{tiny_vec, TinyVec};

/// A type of property that determines a block state.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(super) enum BlockStatePropertyType {
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
					f.write_str("enum of no values")?;
				} else {
					f.write_str("enum of: ")?;
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
				"positive integer between 0 and {}",
				maximum_value
			))?,
			Self::StrictlyPositiveInteger { maximum_value } => f.write_fmt(format_args!(
				"positive integer between 1 and {}",
				maximum_value
			))?
		}

		Ok(())
	}
}

#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_13_TO_1_13_2_EXCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azure_bluet" => phf_map! {
	},
	"barrier" => phf_map! {
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"cave_air" => phf_map! {
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"conduit" => phf_map! {
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_sandstone" => phf_map! {
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"dirt" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"flower_pot" => phf_map! {
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"grass_path" => phf_map! {
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_bricks" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"packed_ice" => phf_map! {
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_diorite" => phf_map! {
	},
	"polished_granite" => phf_map! {
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_stone" => phf_map! {
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7", "8" ] },
	},
	"snow_block" => phf_map! {
	},
	"soul_sand" => phf_map! {
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"terracotta" => phf_map! {
	},
	"tnt" => phf_map! {
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_13_2_TO_1_14_EXCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azure_bluet" => phf_map! {
	},
	"barrier" => phf_map! {
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"cave_air" => phf_map! {
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"conduit" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_sandstone" => phf_map! {
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"dirt" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"flower_pot" => phf_map! {
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"grass_path" => phf_map! {
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_bricks" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"packed_ice" => phf_map! {
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_diorite" => phf_map! {
	},
	"polished_granite" => phf_map! {
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_stone" => phf_map! {
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7", "8" ] },
	},
	"snow_block" => phf_map! {
	},
	"soul_sand" => phf_map! {
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"terracotta" => phf_map! {
	},
	"tnt" => phf_map! {
		"unstable" => BlockStatePropertyType::Boolean,
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_14_TO_1_15_EXCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azure_bluet" => phf_map! {
	},
	"bamboo" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
		"leaves" => BlockStatePropertyType::Enum { values: &[ "none", "small", "large" ] },
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"bamboo_sapling" => phf_map! {
	},
	"barrel" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"open" => BlockStatePropertyType::Boolean,
	},
	"barrier" => phf_map! {
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"bell" => phf_map! {
		"attachment" => BlockStatePropertyType::Enum { values: &[ "floor", "ceiling", "single_wall", "double_wall" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blast_furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"cartography_table" => phf_map! {
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"cave_air" => phf_map! {
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"composter" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"conduit" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cornflower" => phf_map! {
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_sandstone" => phf_map! {
	},
	"cut_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dirt" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fletching_table" => phf_map! {
	},
	"flower_pot" => phf_map! {
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"grass_path" => phf_map! {
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"grindstone" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jigsaw" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lectern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"has_book" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_of_the_valley" => phf_map! {
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"loom" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_bricks" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone", "iron_xylophone", "cow_bell", "didgeridoo", "bit", "banjo", "pling" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"packed_ice" => phf_map! {
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite" => phf_map! {
	},
	"polished_diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite" => phf_map! {
	},
	"polished_granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_bamboo" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_cornflower" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_lily_of_the_valley" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"potted_wither_rose" => phf_map! {
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"scaffolding" => phf_map! {
		"bottom" => BlockStatePropertyType::Boolean,
		"distance" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"smithing_table" => phf_map! {
	},
	"smoker" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_stone" => phf_map! {
	},
	"smooth_stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7", "8" ] },
	},
	"snow_block" => phf_map! {
	},
	"soul_sand" => phf_map! {
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stonecutter" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sweet_berry_bush" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"terracotta" => phf_map! {
	},
	"tnt" => phf_map! {
		"unstable" => BlockStatePropertyType::Boolean,
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_rose" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_15_TO_1_16_EXCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azure_bluet" => phf_map! {
	},
	"bamboo" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
		"leaves" => BlockStatePropertyType::Enum { values: &[ "none", "small", "large" ] },
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"bamboo_sapling" => phf_map! {
	},
	"barrel" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"open" => BlockStatePropertyType::Boolean,
	},
	"barrier" => phf_map! {
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"bee_nest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beehive" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"bell" => phf_map! {
		"attachment" => BlockStatePropertyType::Enum { values: &[ "floor", "ceiling", "single_wall", "double_wall" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blast_furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"cartography_table" => phf_map! {
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"cave_air" => phf_map! {
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"composter" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"conduit" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cornflower" => phf_map! {
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_sandstone" => phf_map! {
	},
	"cut_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dirt" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fletching_table" => phf_map! {
	},
	"flower_pot" => phf_map! {
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"grass_path" => phf_map! {
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"grindstone" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"honey_block" => phf_map! {
	},
	"honeycomb_block" => phf_map! {
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jigsaw" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lectern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"has_book" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_of_the_valley" => phf_map! {
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"loom" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_bricks" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone", "iron_xylophone", "cow_bell", "didgeridoo", "bit", "banjo", "pling" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"packed_ice" => phf_map! {
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite" => phf_map! {
	},
	"polished_diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite" => phf_map! {
	},
	"polished_granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_bamboo" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_cornflower" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_lily_of_the_valley" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"potted_wither_rose" => phf_map! {
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"scaffolding" => phf_map! {
		"bottom" => BlockStatePropertyType::Boolean,
		"distance" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"smithing_table" => phf_map! {
	},
	"smoker" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_stone" => phf_map! {
	},
	"smooth_stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7", "8" ] },
	},
	"snow_block" => phf_map! {
	},
	"soul_sand" => phf_map! {
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stonecutter" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sweet_berry_bush" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"terracotta" => phf_map! {
	},
	"tnt" => phf_map! {
		"unstable" => BlockStatePropertyType::Boolean,
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_rose" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_16_TO_1_16_2_EXCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"ancient_debris" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azure_bluet" => phf_map! {
	},
	"bamboo" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
		"leaves" => BlockStatePropertyType::Enum { values: &[ "none", "small", "large" ] },
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"bamboo_sapling" => phf_map! {
	},
	"barrel" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"open" => BlockStatePropertyType::Boolean,
	},
	"barrier" => phf_map! {
	},
	"basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"bee_nest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beehive" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"bell" => phf_map! {
		"attachment" => BlockStatePropertyType::Enum { values: &[ "floor", "ceiling", "single_wall", "double_wall" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blackstone" => phf_map! {
	},
	"blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"blast_furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"cartography_table" => phf_map! {
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"cave_air" => phf_map! {
	},
	"chain" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_nether_bricks" => phf_map! {
	},
	"chiseled_polished_blackstone" => phf_map! {
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"composter" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"conduit" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cornflower" => phf_map! {
	},
	"cracked_nether_bricks" => phf_map! {
	},
	"cracked_polished_blackstone_bricks" => phf_map! {
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"crimson_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fungus" => phf_map! {
	},
	"crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_nylium" => phf_map! {
	},
	"crimson_planks" => phf_map! {
	},
	"crimson_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_roots" => phf_map! {
	},
	"crimson_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crying_obsidian" => phf_map! {
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_sandstone" => phf_map! {
	},
	"cut_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"dirt" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fletching_table" => phf_map! {
	},
	"flower_pot" => phf_map! {
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"gilded_blackstone" => phf_map! {
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"grass_path" => phf_map! {
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"grindstone" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"honey_block" => phf_map! {
	},
	"honeycomb_block" => phf_map! {
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jigsaw" => phf_map! {
		"orientation" => BlockStatePropertyType::Enum { values: &[ "down_east", "down_north", "down_south", "down_west", "up_east", "up_north", "up_south", "up_west", "west_up", "east_up", "north_up", "south_up" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lectern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"has_book" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_of_the_valley" => phf_map! {
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"lodestone" => phf_map! {
	},
	"loom" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"nether_bricks" => phf_map! {
	},
	"nether_gold_ore" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_sprouts" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherite_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone", "iron_xylophone", "cow_bell", "didgeridoo", "bit", "banjo", "pling" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"packed_ice" => phf_map! {
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"polished_blackstone" => phf_map! {
	},
	"polished_blackstone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_blackstone_bricks" => phf_map! {
	},
	"polished_blackstone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_diorite" => phf_map! {
	},
	"polished_diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite" => phf_map! {
	},
	"polished_granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_bamboo" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_cornflower" => phf_map! {
	},
	"potted_crimson_fungus" => phf_map! {
	},
	"potted_crimson_roots" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_lily_of_the_valley" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_warped_fungus" => phf_map! {
	},
	"potted_warped_roots" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"potted_wither_rose" => phf_map! {
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_bricks" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"respawn_anchor" => phf_map! {
		"charges" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"scaffolding" => phf_map! {
		"bottom" => BlockStatePropertyType::Boolean,
		"distance" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shroomlight" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"smithing_table" => phf_map! {
	},
	"smoker" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_stone" => phf_map! {
	},
	"smooth_stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7", "8" ] },
	},
	"snow_block" => phf_map! {
	},
	"soul_campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_fire" => phf_map! {
	},
	"soul_lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
	},
	"soul_sand" => phf_map! {
	},
	"soul_soil" => phf_map! {
	},
	"soul_torch" => phf_map! {
	},
	"soul_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stonecutter" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sweet_berry_bush" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"target" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"terracotta" => phf_map! {
	},
	"tnt" => phf_map! {
		"unstable" => BlockStatePropertyType::Boolean,
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"twisting_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"twisting_vines_plant" => phf_map! {
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"warped_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"warped_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fungus" => phf_map! {
	},
	"warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_nylium" => phf_map! {
	},
	"warped_planks" => phf_map! {
	},
	"warped_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_roots" => phf_map! {
	},
	"warped_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wart_block" => phf_map! {
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"weeping_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"weeping_vines_plant" => phf_map! {
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_rose" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_16_2_TO_1_17_EXCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"ancient_debris" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azure_bluet" => phf_map! {
	},
	"bamboo" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
		"leaves" => BlockStatePropertyType::Enum { values: &[ "none", "small", "large" ] },
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"bamboo_sapling" => phf_map! {
	},
	"barrel" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"open" => BlockStatePropertyType::Boolean,
	},
	"barrier" => phf_map! {
	},
	"basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"bee_nest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beehive" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"bell" => phf_map! {
		"attachment" => BlockStatePropertyType::Enum { values: &[ "floor", "ceiling", "single_wall", "double_wall" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blackstone" => phf_map! {
	},
	"blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"blast_furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"cartography_table" => phf_map! {
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"cave_air" => phf_map! {
	},
	"chain" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_nether_bricks" => phf_map! {
	},
	"chiseled_polished_blackstone" => phf_map! {
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"composter" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"conduit" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cornflower" => phf_map! {
	},
	"cracked_nether_bricks" => phf_map! {
	},
	"cracked_polished_blackstone_bricks" => phf_map! {
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"crimson_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fungus" => phf_map! {
	},
	"crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_nylium" => phf_map! {
	},
	"crimson_planks" => phf_map! {
	},
	"crimson_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_roots" => phf_map! {
	},
	"crimson_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crying_obsidian" => phf_map! {
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_sandstone" => phf_map! {
	},
	"cut_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"dirt" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fletching_table" => phf_map! {
	},
	"flower_pot" => phf_map! {
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"gilded_blackstone" => phf_map! {
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"grass_path" => phf_map! {
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"grindstone" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"honey_block" => phf_map! {
	},
	"honeycomb_block" => phf_map! {
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jigsaw" => phf_map! {
		"orientation" => BlockStatePropertyType::Enum { values: &[ "down_east", "down_north", "down_south", "down_west", "up_east", "up_north", "up_south", "up_west", "west_up", "east_up", "north_up", "south_up" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lectern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"has_book" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_of_the_valley" => phf_map! {
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"lodestone" => phf_map! {
	},
	"loom" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"nether_bricks" => phf_map! {
	},
	"nether_gold_ore" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_sprouts" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherite_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone", "iron_xylophone", "cow_bell", "didgeridoo", "bit", "banjo", "pling" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"packed_ice" => phf_map! {
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"polished_blackstone" => phf_map! {
	},
	"polished_blackstone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_blackstone_bricks" => phf_map! {
	},
	"polished_blackstone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_diorite" => phf_map! {
	},
	"polished_diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite" => phf_map! {
	},
	"polished_granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_bamboo" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_cornflower" => phf_map! {
	},
	"potted_crimson_fungus" => phf_map! {
	},
	"potted_crimson_roots" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_lily_of_the_valley" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_warped_fungus" => phf_map! {
	},
	"potted_warped_roots" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"potted_wither_rose" => phf_map! {
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_bricks" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"respawn_anchor" => phf_map! {
		"charges" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"scaffolding" => phf_map! {
		"bottom" => BlockStatePropertyType::Boolean,
		"distance" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shroomlight" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"smithing_table" => phf_map! {
	},
	"smoker" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_stone" => phf_map! {
	},
	"smooth_stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7", "8" ] },
	},
	"snow_block" => phf_map! {
	},
	"soul_campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_fire" => phf_map! {
	},
	"soul_lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_sand" => phf_map! {
	},
	"soul_soil" => phf_map! {
	},
	"soul_torch" => phf_map! {
	},
	"soul_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stonecutter" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sweet_berry_bush" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"target" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"terracotta" => phf_map! {
	},
	"tnt" => phf_map! {
		"unstable" => BlockStatePropertyType::Boolean,
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"twisting_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"twisting_vines_plant" => phf_map! {
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"warped_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"warped_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fungus" => phf_map! {
	},
	"warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_nylium" => phf_map! {
	},
	"warped_planks" => phf_map! {
	},
	"warped_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_roots" => phf_map! {
	},
	"warped_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wart_block" => phf_map! {
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"weeping_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"weeping_vines_plant" => phf_map! {
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_rose" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_17_TO_1_18_EXCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"amethyst_block" => phf_map! {
	},
	"amethyst_cluster" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ancient_debris" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azalea" => phf_map! {
	},
	"azalea_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"azure_bluet" => phf_map! {
	},
	"bamboo" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
		"leaves" => BlockStatePropertyType::Enum { values: &[ "none", "small", "large" ] },
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"bamboo_sapling" => phf_map! {
	},
	"barrel" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"open" => BlockStatePropertyType::Boolean,
	},
	"barrier" => phf_map! {
	},
	"basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"bee_nest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beehive" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"bell" => phf_map! {
		"attachment" => BlockStatePropertyType::Enum { values: &[ "floor", "ceiling", "single_wall", "double_wall" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"big_dripleaf" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"tilt" => BlockStatePropertyType::Enum { values: &[ "none", "unstable", "partial", "full" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"big_dripleaf_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"black_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blackstone" => phf_map! {
	},
	"blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"blast_furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blue_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brown_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"budding_amethyst" => phf_map! {
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"calcite" => phf_map! {
	},
	"campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"cartography_table" => phf_map! {
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
	},
	"cave_air" => phf_map! {
	},
	"cave_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
		"berries" => BlockStatePropertyType::Boolean,
	},
	"cave_vines_plant" => phf_map! {
		"berries" => BlockStatePropertyType::Boolean,
	},
	"chain" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_deepslate" => phf_map! {
	},
	"chiseled_nether_bricks" => phf_map! {
	},
	"chiseled_polished_blackstone" => phf_map! {
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobbled_deepslate" => phf_map! {
	},
	"cobbled_deepslate_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobbled_deepslate_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobbled_deepslate_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"composter" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"conduit" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"copper_block" => phf_map! {
	},
	"copper_ore" => phf_map! {
	},
	"cornflower" => phf_map! {
	},
	"cracked_deepslate_bricks" => phf_map! {
	},
	"cracked_deepslate_tiles" => phf_map! {
	},
	"cracked_nether_bricks" => phf_map! {
	},
	"cracked_polished_blackstone_bricks" => phf_map! {
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"crimson_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fungus" => phf_map! {
	},
	"crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_nylium" => phf_map! {
	},
	"crimson_planks" => phf_map! {
	},
	"crimson_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_roots" => phf_map! {
	},
	"crimson_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crying_obsidian" => phf_map! {
	},
	"cut_copper" => phf_map! {
	},
	"cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_sandstone" => phf_map! {
	},
	"cut_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"deepslate_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"deepslate_bricks" => phf_map! {
	},
	"deepslate_coal_ore" => phf_map! {
	},
	"deepslate_copper_ore" => phf_map! {
	},
	"deepslate_diamond_ore" => phf_map! {
	},
	"deepslate_emerald_ore" => phf_map! {
	},
	"deepslate_gold_ore" => phf_map! {
	},
	"deepslate_iron_ore" => phf_map! {
	},
	"deepslate_lapis_ore" => phf_map! {
	},
	"deepslate_redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"deepslate_tiles" => phf_map! {
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"dirt" => phf_map! {
	},
	"dirt_path" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dripstone_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"exposed_copper" => phf_map! {
	},
	"exposed_cut_copper" => phf_map! {
	},
	"exposed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"exposed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fletching_table" => phf_map! {
	},
	"flower_pot" => phf_map! {
	},
	"flowering_azalea" => phf_map! {
	},
	"flowering_azalea_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"gilded_blackstone" => phf_map! {
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glow_item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"glow_lichen" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"gray_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"green_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"grindstone" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"hanging_roots" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"honey_block" => phf_map! {
	},
	"honeycomb_block" => phf_map! {
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_deepslate" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jigsaw" => phf_map! {
		"orientation" => BlockStatePropertyType::Enum { values: &[ "down_east", "down_north", "down_south", "down_west", "up_east", "up_north", "up_south", "up_west", "west_up", "east_up", "north_up", "south_up" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lava_cauldron" => phf_map! {
	},
	"lectern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"has_book" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_blue_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_gray_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lightning_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_of_the_valley" => phf_map! {
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lime_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"lodestone" => phf_map! {
	},
	"loom" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"magenta_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"medium_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"moss_block" => phf_map! {
	},
	"moss_carpet" => phf_map! {
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"nether_bricks" => phf_map! {
	},
	"nether_gold_ore" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_sprouts" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherite_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone", "iron_xylophone", "cow_bell", "didgeridoo", "bit", "banjo", "pling" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"orange_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"oxidized_copper" => phf_map! {
	},
	"oxidized_cut_copper" => phf_map! {
	},
	"oxidized_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oxidized_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"packed_ice" => phf_map! {
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"pointed_dripstone" => phf_map! {
		"thickness" => BlockStatePropertyType::Enum { values: &[ "tip_merge", "tip", "frustum", "middle", "base" ] },
		"vertical_direction" => BlockStatePropertyType::Enum { values: &[ "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"polished_blackstone" => phf_map! {
	},
	"polished_blackstone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_blackstone_bricks" => phf_map! {
	},
	"polished_blackstone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_deepslate" => phf_map! {
	},
	"polished_deepslate_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_deepslate_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_deepslate_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_diorite" => phf_map! {
	},
	"polished_diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite" => phf_map! {
	},
	"polished_granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azalea_bush" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_bamboo" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_cornflower" => phf_map! {
	},
	"potted_crimson_fungus" => phf_map! {
	},
	"potted_crimson_roots" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_flowering_azalea_bush" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_lily_of_the_valley" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_warped_fungus" => phf_map! {
	},
	"potted_warped_roots" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"potted_wither_rose" => phf_map! {
	},
	"powder_snow" => phf_map! {
	},
	"powder_snow_cauldron" => phf_map! {
		"level" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purple_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_bricks" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"raw_copper_block" => phf_map! {
	},
	"raw_gold_block" => phf_map! {
	},
	"raw_iron_block" => phf_map! {
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"respawn_anchor" => phf_map! {
		"charges" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
	},
	"rooted_dirt" => phf_map! {
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"scaffolding" => phf_map! {
		"bottom" => BlockStatePropertyType::Boolean,
		"distance" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sculk_sensor" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"sculk_sensor_phase" => BlockStatePropertyType::Enum { values: &[ "inactive", "active", "cooldown" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shroomlight" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"small_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"small_dripleaf" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smithing_table" => phf_map! {
	},
	"smoker" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"smooth_basalt" => phf_map! {
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_stone" => phf_map! {
	},
	"smooth_stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"snow_block" => phf_map! {
	},
	"soul_campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_fire" => phf_map! {
	},
	"soul_lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_sand" => phf_map! {
	},
	"soul_soil" => phf_map! {
	},
	"soul_torch" => phf_map! {
	},
	"soul_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spore_blossom" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stonecutter" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sweet_berry_bush" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"target" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"terracotta" => phf_map! {
	},
	"tinted_glass" => phf_map! {
	},
	"tnt" => phf_map! {
		"unstable" => BlockStatePropertyType::Boolean,
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tuff" => phf_map! {
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"twisting_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"twisting_vines_plant" => phf_map! {
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"warped_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"warped_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fungus" => phf_map! {
	},
	"warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_nylium" => phf_map! {
	},
	"warped_planks" => phf_map! {
	},
	"warped_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_roots" => phf_map! {
	},
	"warped_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wart_block" => phf_map! {
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"water_cauldron" => phf_map! {
		"level" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"waxed_copper_block" => phf_map! {
	},
	"waxed_cut_copper" => phf_map! {
	},
	"waxed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_exposed_copper" => phf_map! {
	},
	"waxed_exposed_cut_copper" => phf_map! {
	},
	"waxed_exposed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_exposed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_oxidized_copper" => phf_map! {
	},
	"waxed_oxidized_cut_copper" => phf_map! {
	},
	"waxed_oxidized_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_oxidized_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_weathered_copper" => phf_map! {
	},
	"waxed_weathered_cut_copper" => phf_map! {
	},
	"waxed_weathered_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_weathered_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weathered_copper" => phf_map! {
	},
	"weathered_cut_copper" => phf_map! {
	},
	"weathered_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weathered_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weeping_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"weeping_vines_plant" => phf_map! {
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"white_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_rose" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"yellow_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_18_TO_1_19_EXCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"amethyst_block" => phf_map! {
	},
	"amethyst_cluster" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ancient_debris" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azalea" => phf_map! {
	},
	"azalea_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"azure_bluet" => phf_map! {
	},
	"bamboo" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
		"leaves" => BlockStatePropertyType::Enum { values: &[ "none", "small", "large" ] },
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"bamboo_sapling" => phf_map! {
	},
	"barrel" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"open" => BlockStatePropertyType::Boolean,
	},
	"barrier" => phf_map! {
	},
	"basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"bee_nest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beehive" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"bell" => phf_map! {
		"attachment" => BlockStatePropertyType::Enum { values: &[ "floor", "ceiling", "single_wall", "double_wall" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"big_dripleaf" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"tilt" => BlockStatePropertyType::Enum { values: &[ "none", "unstable", "partial", "full" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"big_dripleaf_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"black_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blackstone" => phf_map! {
	},
	"blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"blast_furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blue_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brown_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"budding_amethyst" => phf_map! {
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"calcite" => phf_map! {
	},
	"campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"cartography_table" => phf_map! {
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
	},
	"cave_air" => phf_map! {
	},
	"cave_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
		"berries" => BlockStatePropertyType::Boolean,
	},
	"cave_vines_plant" => phf_map! {
		"berries" => BlockStatePropertyType::Boolean,
	},
	"chain" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_deepslate" => phf_map! {
	},
	"chiseled_nether_bricks" => phf_map! {
	},
	"chiseled_polished_blackstone" => phf_map! {
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobbled_deepslate" => phf_map! {
	},
	"cobbled_deepslate_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobbled_deepslate_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobbled_deepslate_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"composter" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"conduit" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"copper_block" => phf_map! {
	},
	"copper_ore" => phf_map! {
	},
	"cornflower" => phf_map! {
	},
	"cracked_deepslate_bricks" => phf_map! {
	},
	"cracked_deepslate_tiles" => phf_map! {
	},
	"cracked_nether_bricks" => phf_map! {
	},
	"cracked_polished_blackstone_bricks" => phf_map! {
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"crimson_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fungus" => phf_map! {
	},
	"crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_nylium" => phf_map! {
	},
	"crimson_planks" => phf_map! {
	},
	"crimson_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_roots" => phf_map! {
	},
	"crimson_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crying_obsidian" => phf_map! {
	},
	"cut_copper" => phf_map! {
	},
	"cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_sandstone" => phf_map! {
	},
	"cut_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"deepslate_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"deepslate_bricks" => phf_map! {
	},
	"deepslate_coal_ore" => phf_map! {
	},
	"deepslate_copper_ore" => phf_map! {
	},
	"deepslate_diamond_ore" => phf_map! {
	},
	"deepslate_emerald_ore" => phf_map! {
	},
	"deepslate_gold_ore" => phf_map! {
	},
	"deepslate_iron_ore" => phf_map! {
	},
	"deepslate_lapis_ore" => phf_map! {
	},
	"deepslate_redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"deepslate_tiles" => phf_map! {
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"dirt" => phf_map! {
	},
	"dirt_path" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dripstone_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"exposed_copper" => phf_map! {
	},
	"exposed_cut_copper" => phf_map! {
	},
	"exposed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"exposed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fletching_table" => phf_map! {
	},
	"flower_pot" => phf_map! {
	},
	"flowering_azalea" => phf_map! {
	},
	"flowering_azalea_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"gilded_blackstone" => phf_map! {
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glow_item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"glow_lichen" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"gray_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"green_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"grindstone" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"hanging_roots" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"honey_block" => phf_map! {
	},
	"honeycomb_block" => phf_map! {
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_deepslate" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jigsaw" => phf_map! {
		"orientation" => BlockStatePropertyType::Enum { values: &[ "down_east", "down_north", "down_south", "down_west", "up_east", "up_north", "up_south", "up_west", "west_up", "east_up", "north_up", "south_up" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lava_cauldron" => phf_map! {
	},
	"lectern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"has_book" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_blue_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_gray_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lightning_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_of_the_valley" => phf_map! {
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lime_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"lodestone" => phf_map! {
	},
	"loom" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"magenta_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"medium_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"moss_block" => phf_map! {
	},
	"moss_carpet" => phf_map! {
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"nether_bricks" => phf_map! {
	},
	"nether_gold_ore" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_sprouts" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherite_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone", "iron_xylophone", "cow_bell", "didgeridoo", "bit", "banjo", "pling" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"orange_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"oxidized_copper" => phf_map! {
	},
	"oxidized_cut_copper" => phf_map! {
	},
	"oxidized_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oxidized_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"packed_ice" => phf_map! {
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"pointed_dripstone" => phf_map! {
		"thickness" => BlockStatePropertyType::Enum { values: &[ "tip_merge", "tip", "frustum", "middle", "base" ] },
		"vertical_direction" => BlockStatePropertyType::Enum { values: &[ "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"polished_blackstone" => phf_map! {
	},
	"polished_blackstone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_blackstone_bricks" => phf_map! {
	},
	"polished_blackstone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_deepslate" => phf_map! {
	},
	"polished_deepslate_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_deepslate_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_deepslate_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_diorite" => phf_map! {
	},
	"polished_diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite" => phf_map! {
	},
	"polished_granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azalea_bush" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_bamboo" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_cornflower" => phf_map! {
	},
	"potted_crimson_fungus" => phf_map! {
	},
	"potted_crimson_roots" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_flowering_azalea_bush" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_lily_of_the_valley" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_warped_fungus" => phf_map! {
	},
	"potted_warped_roots" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"potted_wither_rose" => phf_map! {
	},
	"powder_snow" => phf_map! {
	},
	"powder_snow_cauldron" => phf_map! {
		"level" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3" ] },
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purple_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_bricks" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"raw_copper_block" => phf_map! {
	},
	"raw_gold_block" => phf_map! {
	},
	"raw_iron_block" => phf_map! {
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"respawn_anchor" => phf_map! {
		"charges" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
	},
	"rooted_dirt" => phf_map! {
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"scaffolding" => phf_map! {
		"bottom" => BlockStatePropertyType::Boolean,
		"distance" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sculk_sensor" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"sculk_sensor_phase" => BlockStatePropertyType::Enum { values: &[ "inactive", "active", "cooldown" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shroomlight" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"small_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"small_dripleaf" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smithing_table" => phf_map! {
	},
	"smoker" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"smooth_basalt" => phf_map! {
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_stone" => phf_map! {
	},
	"smooth_stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7", "8" ] },
	},
	"snow_block" => phf_map! {
	},
	"soul_campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_fire" => phf_map! {
	},
	"soul_lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_sand" => phf_map! {
	},
	"soul_soil" => phf_map! {
	},
	"soul_torch" => phf_map! {
	},
	"soul_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spore_blossom" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4", "5", "6", "7" ] },
		"persistent" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stonecutter" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sweet_berry_bush" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"target" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"terracotta" => phf_map! {
	},
	"tinted_glass" => phf_map! {
	},
	"tnt" => phf_map! {
		"unstable" => BlockStatePropertyType::Boolean,
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tuff" => phf_map! {
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"twisting_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"twisting_vines_plant" => phf_map! {
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"warped_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"warped_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fungus" => phf_map! {
	},
	"warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_nylium" => phf_map! {
	},
	"warped_planks" => phf_map! {
	},
	"warped_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_roots" => phf_map! {
	},
	"warped_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wart_block" => phf_map! {
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"water_cauldron" => phf_map! {
		"level" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3" ] },
	},
	"waxed_copper_block" => phf_map! {
	},
	"waxed_cut_copper" => phf_map! {
	},
	"waxed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_exposed_copper" => phf_map! {
	},
	"waxed_exposed_cut_copper" => phf_map! {
	},
	"waxed_exposed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_exposed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_oxidized_copper" => phf_map! {
	},
	"waxed_oxidized_cut_copper" => phf_map! {
	},
	"waxed_oxidized_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_oxidized_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_weathered_copper" => phf_map! {
	},
	"waxed_weathered_cut_copper" => phf_map! {
	},
	"waxed_weathered_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_weathered_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weathered_copper" => phf_map! {
	},
	"weathered_cut_copper" => phf_map! {
	},
	"weathered_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weathered_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weeping_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"weeping_vines_plant" => phf_map! {
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"white_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_rose" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_candle" => phf_map! {
		"candles" => BlockStatePropertyType::Enum { values: &[ "1", "2", "3", "4" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"yellow_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};
#[allow(unsafe_code)]
static BLOCKSTATE_PROPERTIES_1_19_TO_1_19_INCLUSIVE: phf::Map<
	&str,
	phf::Map<&str, BlockStatePropertyType>
> = phf_map! {
	"acacia_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"acacia_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"acacia_planks" => phf_map! {
	},
	"acacia_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"acacia_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"acacia_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"activator_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"air" => phf_map! {
	},
	"allium" => phf_map! {
	},
	"amethyst_block" => phf_map! {
	},
	"amethyst_cluster" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ancient_debris" => phf_map! {
	},
	"andesite" => phf_map! {
	},
	"andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"andesite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_melon_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"attached_pumpkin_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"azalea" => phf_map! {
	},
	"azalea_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"azure_bluet" => phf_map! {
	},
	"bamboo" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
		"leaves" => BlockStatePropertyType::Enum { values: &[ "none", "small", "large" ] },
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"bamboo_sapling" => phf_map! {
	},
	"barrel" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"open" => BlockStatePropertyType::Boolean,
	},
	"barrier" => phf_map! {
	},
	"basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"beacon" => phf_map! {
	},
	"bedrock" => phf_map! {
	},
	"bee_nest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beehive" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"honey_level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"beetroots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"bell" => phf_map! {
		"attachment" => BlockStatePropertyType::Enum { values: &[ "floor", "ceiling", "single_wall", "double_wall" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"big_dripleaf" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"tilt" => BlockStatePropertyType::Enum { values: &[ "none", "unstable", "partial", "full" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"big_dripleaf_stem" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"birch_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"birch_planks" => phf_map! {
	},
	"birch_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"birch_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"birch_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"black_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"black_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"black_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"black_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"black_carpet" => phf_map! {
	},
	"black_concrete" => phf_map! {
	},
	"black_concrete_powder" => phf_map! {
	},
	"black_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"black_stained_glass" => phf_map! {
	},
	"black_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"black_terracotta" => phf_map! {
	},
	"black_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"black_wool" => phf_map! {
	},
	"blackstone" => phf_map! {
	},
	"blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"blast_furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"blue_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"blue_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"blue_carpet" => phf_map! {
	},
	"blue_concrete" => phf_map! {
	},
	"blue_concrete_powder" => phf_map! {
	},
	"blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_ice" => phf_map! {
	},
	"blue_orchid" => phf_map! {
	},
	"blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"blue_stained_glass" => phf_map! {
	},
	"blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"blue_terracotta" => phf_map! {
	},
	"blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"blue_wool" => phf_map! {
	},
	"bone_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"bookshelf" => phf_map! {
	},
	"brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_block" => phf_map! {
	},
	"brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brewing_stand" => phf_map! {
		"has_bottle_0" => BlockStatePropertyType::Boolean,
		"has_bottle_1" => BlockStatePropertyType::Boolean,
		"has_bottle_2" => BlockStatePropertyType::Boolean,
	},
	"brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"bricks" => phf_map! {
	},
	"brown_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"brown_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"brown_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"brown_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"brown_carpet" => phf_map! {
	},
	"brown_concrete" => phf_map! {
	},
	"brown_concrete_powder" => phf_map! {
	},
	"brown_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_mushroom" => phf_map! {
	},
	"brown_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"brown_stained_glass" => phf_map! {
	},
	"brown_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"brown_terracotta" => phf_map! {
	},
	"brown_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"brown_wool" => phf_map! {
	},
	"bubble_column" => phf_map! {
		"drag" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_block" => phf_map! {
	},
	"bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"budding_amethyst" => phf_map! {
	},
	"cactus" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cake" => phf_map! {
		"bites" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(6) } },
	},
	"calcite" => phf_map! {
	},
	"campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"carrots" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"cartography_table" => phf_map! {
	},
	"carved_pumpkin" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cauldron" => phf_map! {
	},
	"cave_air" => phf_map! {
	},
	"cave_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
		"berries" => BlockStatePropertyType::Boolean,
	},
	"cave_vines_plant" => phf_map! {
		"berries" => BlockStatePropertyType::Boolean,
	},
	"chain" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chain_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"chipped_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"chiseled_deepslate" => phf_map! {
	},
	"chiseled_nether_bricks" => phf_map! {
	},
	"chiseled_polished_blackstone" => phf_map! {
	},
	"chiseled_quartz_block" => phf_map! {
	},
	"chiseled_red_sandstone" => phf_map! {
	},
	"chiseled_sandstone" => phf_map! {
	},
	"chiseled_stone_bricks" => phf_map! {
	},
	"chorus_flower" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(5) } },
	},
	"chorus_plant" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"clay" => phf_map! {
	},
	"coal_block" => phf_map! {
	},
	"coal_ore" => phf_map! {
	},
	"coarse_dirt" => phf_map! {
	},
	"cobbled_deepslate" => phf_map! {
	},
	"cobbled_deepslate_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobbled_deepslate_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobbled_deepslate_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"cobblestone" => phf_map! {
	},
	"cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"cobweb" => phf_map! {
	},
	"cocoa" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"comparator" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"mode" => BlockStatePropertyType::Enum { values: &[ "compare", "subtract" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"composter" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"conduit" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"copper_block" => phf_map! {
	},
	"copper_ore" => phf_map! {
	},
	"cornflower" => phf_map! {
	},
	"cracked_deepslate_bricks" => phf_map! {
	},
	"cracked_deepslate_tiles" => phf_map! {
	},
	"cracked_nether_bricks" => phf_map! {
	},
	"cracked_polished_blackstone_bricks" => phf_map! {
	},
	"cracked_stone_bricks" => phf_map! {
	},
	"crafting_table" => phf_map! {
	},
	"creeper_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"creeper_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"crimson_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"crimson_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_fungus" => phf_map! {
	},
	"crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_nylium" => phf_map! {
	},
	"crimson_planks" => phf_map! {
	},
	"crimson_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"crimson_roots" => phf_map! {
	},
	"crimson_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"crimson_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crimson_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"crying_obsidian" => phf_map! {
	},
	"cut_copper" => phf_map! {
	},
	"cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_red_sandstone" => phf_map! {
	},
	"cut_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cut_sandstone" => phf_map! {
	},
	"cut_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"cyan_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"cyan_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"cyan_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"cyan_carpet" => phf_map! {
	},
	"cyan_concrete" => phf_map! {
	},
	"cyan_concrete_powder" => phf_map! {
	},
	"cyan_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"cyan_stained_glass" => phf_map! {
	},
	"cyan_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"cyan_terracotta" => phf_map! {
	},
	"cyan_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"cyan_wool" => phf_map! {
	},
	"damaged_anvil" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dandelion" => phf_map! {
	},
	"dark_oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_oak_planks" => phf_map! {
	},
	"dark_oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"dark_oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"dark_prismarine" => phf_map! {
	},
	"dark_prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dark_prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"daylight_detector" => phf_map! {
		"inverted" => BlockStatePropertyType::Boolean,
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dead_brain_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_block" => phf_map! {
	},
	"dead_brain_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_brain_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_block" => phf_map! {
	},
	"dead_bubble_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bubble_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_bush" => phf_map! {
	},
	"dead_fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_block" => phf_map! {
	},
	"dead_fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_block" => phf_map! {
	},
	"dead_horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_block" => phf_map! {
	},
	"dead_tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"dead_tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"deepslate_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"deepslate_bricks" => phf_map! {
	},
	"deepslate_coal_ore" => phf_map! {
	},
	"deepslate_copper_ore" => phf_map! {
	},
	"deepslate_diamond_ore" => phf_map! {
	},
	"deepslate_emerald_ore" => phf_map! {
	},
	"deepslate_gold_ore" => phf_map! {
	},
	"deepslate_iron_ore" => phf_map! {
	},
	"deepslate_lapis_ore" => phf_map! {
	},
	"deepslate_redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"deepslate_tile_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"deepslate_tiles" => phf_map! {
	},
	"detector_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diamond_block" => phf_map! {
	},
	"diamond_ore" => phf_map! {
	},
	"diorite" => phf_map! {
	},
	"diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"diorite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"dirt" => phf_map! {
	},
	"dirt_path" => phf_map! {
	},
	"dispenser" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"dragon_egg" => phf_map! {
	},
	"dragon_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"dragon_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"dried_kelp_block" => phf_map! {
	},
	"dripstone_block" => phf_map! {
	},
	"dropper" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"triggered" => BlockStatePropertyType::Boolean,
	},
	"emerald_block" => phf_map! {
	},
	"emerald_ore" => phf_map! {
	},
	"enchanting_table" => phf_map! {
	},
	"end_gateway" => phf_map! {
	},
	"end_portal" => phf_map! {
	},
	"end_portal_frame" => phf_map! {
		"eye" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"end_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"end_stone" => phf_map! {
	},
	"end_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"end_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"end_stone_bricks" => phf_map! {
	},
	"ender_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"exposed_copper" => phf_map! {
	},
	"exposed_cut_copper" => phf_map! {
	},
	"exposed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"exposed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"farmland" => phf_map! {
		"moisture" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"fern" => phf_map! {
	},
	"fire" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"fire_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_block" => phf_map! {
	},
	"fire_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fire_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"fletching_table" => phf_map! {
	},
	"flower_pot" => phf_map! {
	},
	"flowering_azalea" => phf_map! {
	},
	"flowering_azalea_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"frogspawn" => phf_map! {
	},
	"frosted_ice" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"furnace" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"gilded_blackstone" => phf_map! {
	},
	"glass" => phf_map! {
	},
	"glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glow_item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"glow_lichen" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"glowstone" => phf_map! {
	},
	"gold_block" => phf_map! {
	},
	"gold_ore" => phf_map! {
	},
	"granite" => phf_map! {
	},
	"granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"granite_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"grass" => phf_map! {
	},
	"grass_block" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"gravel" => phf_map! {
	},
	"gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"gray_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"gray_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"gray_carpet" => phf_map! {
	},
	"gray_concrete" => phf_map! {
	},
	"gray_concrete_powder" => phf_map! {
	},
	"gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"gray_stained_glass" => phf_map! {
	},
	"gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"gray_terracotta" => phf_map! {
	},
	"gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"gray_wool" => phf_map! {
	},
	"green_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"green_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"green_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"green_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"green_carpet" => phf_map! {
	},
	"green_concrete" => phf_map! {
	},
	"green_concrete_powder" => phf_map! {
	},
	"green_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"green_stained_glass" => phf_map! {
	},
	"green_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"green_terracotta" => phf_map! {
	},
	"green_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"green_wool" => phf_map! {
	},
	"grindstone" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"hanging_roots" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"hay_block" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"heavy_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"honey_block" => phf_map! {
	},
	"honeycomb_block" => phf_map! {
	},
	"hopper" => phf_map! {
		"enabled" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "down", "north", "south", "west", "east" ] },
	},
	"horn_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_block" => phf_map! {
	},
	"horn_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"horn_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"ice" => phf_map! {
	},
	"infested_chiseled_stone_bricks" => phf_map! {
	},
	"infested_cobblestone" => phf_map! {
	},
	"infested_cracked_stone_bricks" => phf_map! {
	},
	"infested_deepslate" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"infested_mossy_stone_bricks" => phf_map! {
	},
	"infested_stone" => phf_map! {
	},
	"infested_stone_bricks" => phf_map! {
	},
	"iron_bars" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"iron_block" => phf_map! {
	},
	"iron_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"iron_ore" => phf_map! {
	},
	"iron_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"item_frame" => phf_map! {
		"map" => BlockStatePropertyType::Boolean,
	},
	"jack_o_lantern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"jigsaw" => phf_map! {
		"orientation" => BlockStatePropertyType::Enum { values: &[ "down_east", "down_north", "down_south", "down_west", "up_east", "up_north", "up_south", "up_west", "west_up", "east_up", "north_up", "south_up" ] },
	},
	"jukebox" => phf_map! {
		"has_record" => BlockStatePropertyType::Boolean,
	},
	"jungle_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"jungle_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"jungle_planks" => phf_map! {
	},
	"jungle_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"jungle_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"jungle_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"kelp" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"kelp_plant" => phf_map! {
	},
	"ladder" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lapis_block" => phf_map! {
	},
	"lapis_ore" => phf_map! {
	},
	"large_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"large_fern" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lava" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lava_cauldron" => phf_map! {
	},
	"lectern" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"has_book" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"lever" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"light" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_blue_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_blue_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_blue_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_blue_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"light_blue_carpet" => phf_map! {
	},
	"light_blue_concrete" => phf_map! {
	},
	"light_blue_concrete_powder" => phf_map! {
	},
	"light_blue_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_blue_stained_glass" => phf_map! {
	},
	"light_blue_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_blue_terracotta" => phf_map! {
	},
	"light_blue_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_blue_wool" => phf_map! {
	},
	"light_gray_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"light_gray_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"light_gray_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"light_gray_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"light_gray_carpet" => phf_map! {
	},
	"light_gray_concrete" => phf_map! {
	},
	"light_gray_concrete_powder" => phf_map! {
	},
	"light_gray_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"light_gray_stained_glass" => phf_map! {
	},
	"light_gray_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"light_gray_terracotta" => phf_map! {
	},
	"light_gray_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"light_gray_wool" => phf_map! {
	},
	"light_weighted_pressure_plate" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lightning_rod" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lilac" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"lily_of_the_valley" => phf_map! {
	},
	"lily_pad" => phf_map! {
	},
	"lime_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"lime_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"lime_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"lime_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"lime_carpet" => phf_map! {
	},
	"lime_concrete" => phf_map! {
	},
	"lime_concrete_powder" => phf_map! {
	},
	"lime_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"lime_stained_glass" => phf_map! {
	},
	"lime_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"lime_terracotta" => phf_map! {
	},
	"lime_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"lime_wool" => phf_map! {
	},
	"lodestone" => phf_map! {
	},
	"loom" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"magenta_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"magenta_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"magenta_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"magenta_carpet" => phf_map! {
	},
	"magenta_concrete" => phf_map! {
	},
	"magenta_concrete_powder" => phf_map! {
	},
	"magenta_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"magenta_stained_glass" => phf_map! {
	},
	"magenta_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"magenta_terracotta" => phf_map! {
	},
	"magenta_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"magenta_wool" => phf_map! {
	},
	"magma_block" => phf_map! {
	},
	"mangrove_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"mangrove_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"mangrove_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mangrove_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"mangrove_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mangrove_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"mangrove_planks" => phf_map! {
	},
	"mangrove_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"mangrove_propagule" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"hanging" => BlockStatePropertyType::Boolean,
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mangrove_roots" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mangrove_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mangrove_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mangrove_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mangrove_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mangrove_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mangrove_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"medium_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"melon" => phf_map! {
	},
	"melon_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"moss_block" => phf_map! {
	},
	"moss_carpet" => phf_map! {
	},
	"mossy_cobblestone" => phf_map! {
	},
	"mossy_cobblestone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_cobblestone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mossy_stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mossy_stone_bricks" => phf_map! {
	},
	"moving_piston" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"mud" => phf_map! {
	},
	"mud_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mud_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"mud_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"mud_bricks" => phf_map! {
	},
	"muddy_mangrove_roots" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"mushroom_stem" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"mycelium" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"nether_bricks" => phf_map! {
	},
	"nether_gold_ore" => phf_map! {
	},
	"nether_portal" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "z" ] },
	},
	"nether_quartz_ore" => phf_map! {
	},
	"nether_sprouts" => phf_map! {
	},
	"nether_wart" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"nether_wart_block" => phf_map! {
	},
	"netherite_block" => phf_map! {
	},
	"netherrack" => phf_map! {
	},
	"note_block" => phf_map! {
		"instrument" => BlockStatePropertyType::Enum { values: &[ "harp", "basedrum", "snare", "hat", "bass", "flute", "bell", "guitar", "chime", "xylophone", "iron_xylophone", "cow_bell", "didgeridoo", "bit", "banjo", "pling" ] },
		"note" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(24) } },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"oak_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"oak_planks" => phf_map! {
	},
	"oak_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"oak_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"oak_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"observer" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"obsidian" => phf_map! {
	},
	"ochre_froglight" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"orange_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"orange_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"orange_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"orange_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"orange_carpet" => phf_map! {
	},
	"orange_concrete" => phf_map! {
	},
	"orange_concrete_powder" => phf_map! {
	},
	"orange_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"orange_stained_glass" => phf_map! {
	},
	"orange_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"orange_terracotta" => phf_map! {
	},
	"orange_tulip" => phf_map! {
	},
	"orange_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"orange_wool" => phf_map! {
	},
	"oxeye_daisy" => phf_map! {
	},
	"oxidized_copper" => phf_map! {
	},
	"oxidized_cut_copper" => phf_map! {
	},
	"oxidized_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"oxidized_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"packed_ice" => phf_map! {
	},
	"packed_mud" => phf_map! {
	},
	"pearlescent_froglight" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"peony" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"petrified_oak_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"pink_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"pink_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"pink_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"pink_carpet" => phf_map! {
	},
	"pink_concrete" => phf_map! {
	},
	"pink_concrete_powder" => phf_map! {
	},
	"pink_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"pink_stained_glass" => phf_map! {
	},
	"pink_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"pink_terracotta" => phf_map! {
	},
	"pink_tulip" => phf_map! {
	},
	"pink_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"pink_wool" => phf_map! {
	},
	"piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"piston_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"short" => BlockStatePropertyType::Boolean,
		"type" => BlockStatePropertyType::Enum { values: &[ "normal", "sticky" ] },
	},
	"player_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"player_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"podzol" => phf_map! {
		"snowy" => BlockStatePropertyType::Boolean,
	},
	"pointed_dripstone" => phf_map! {
		"thickness" => BlockStatePropertyType::Enum { values: &[ "tip_merge", "tip", "frustum", "middle", "base" ] },
		"vertical_direction" => BlockStatePropertyType::Enum { values: &[ "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite" => phf_map! {
	},
	"polished_andesite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_andesite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_basalt" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"polished_blackstone" => phf_map! {
	},
	"polished_blackstone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_blackstone_bricks" => phf_map! {
	},
	"polished_blackstone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_blackstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_deepslate" => phf_map! {
	},
	"polished_deepslate_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_deepslate_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_deepslate_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"polished_diorite" => phf_map! {
	},
	"polished_diorite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_diorite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite" => phf_map! {
	},
	"polished_granite_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"polished_granite_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"poppy" => phf_map! {
	},
	"potatoes" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"potted_acacia_sapling" => phf_map! {
	},
	"potted_allium" => phf_map! {
	},
	"potted_azalea_bush" => phf_map! {
	},
	"potted_azure_bluet" => phf_map! {
	},
	"potted_bamboo" => phf_map! {
	},
	"potted_birch_sapling" => phf_map! {
	},
	"potted_blue_orchid" => phf_map! {
	},
	"potted_brown_mushroom" => phf_map! {
	},
	"potted_cactus" => phf_map! {
	},
	"potted_cornflower" => phf_map! {
	},
	"potted_crimson_fungus" => phf_map! {
	},
	"potted_crimson_roots" => phf_map! {
	},
	"potted_dandelion" => phf_map! {
	},
	"potted_dark_oak_sapling" => phf_map! {
	},
	"potted_dead_bush" => phf_map! {
	},
	"potted_fern" => phf_map! {
	},
	"potted_flowering_azalea_bush" => phf_map! {
	},
	"potted_jungle_sapling" => phf_map! {
	},
	"potted_lily_of_the_valley" => phf_map! {
	},
	"potted_mangrove_propagule" => phf_map! {
	},
	"potted_oak_sapling" => phf_map! {
	},
	"potted_orange_tulip" => phf_map! {
	},
	"potted_oxeye_daisy" => phf_map! {
	},
	"potted_pink_tulip" => phf_map! {
	},
	"potted_poppy" => phf_map! {
	},
	"potted_red_mushroom" => phf_map! {
	},
	"potted_red_tulip" => phf_map! {
	},
	"potted_spruce_sapling" => phf_map! {
	},
	"potted_warped_fungus" => phf_map! {
	},
	"potted_warped_roots" => phf_map! {
	},
	"potted_white_tulip" => phf_map! {
	},
	"potted_wither_rose" => phf_map! {
	},
	"powder_snow" => phf_map! {
	},
	"powder_snow_cauldron" => phf_map! {
		"level" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"powered_rail" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine" => phf_map! {
	},
	"prismarine_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_bricks" => phf_map! {
	},
	"prismarine_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"prismarine_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"pumpkin" => phf_map! {
	},
	"pumpkin_stem" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"purple_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"purple_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"purple_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purple_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"purple_carpet" => phf_map! {
	},
	"purple_concrete" => phf_map! {
	},
	"purple_concrete_powder" => phf_map! {
	},
	"purple_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"purple_stained_glass" => phf_map! {
	},
	"purple_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"purple_terracotta" => phf_map! {
	},
	"purple_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"purple_wool" => phf_map! {
	},
	"purpur_block" => phf_map! {
	},
	"purpur_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"purpur_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"purpur_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_block" => phf_map! {
	},
	"quartz_bricks" => phf_map! {
	},
	"quartz_pillar" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"rail" => phf_map! {
		"shape" => BlockStatePropertyType::Enum { values: &[ "north_south", "east_west", "ascending_east", "ascending_west", "ascending_north", "ascending_south", "south_east", "south_west", "north_west", "north_east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"raw_copper_block" => phf_map! {
	},
	"raw_gold_block" => phf_map! {
	},
	"raw_iron_block" => phf_map! {
	},
	"red_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"red_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"red_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"red_carpet" => phf_map! {
	},
	"red_concrete" => phf_map! {
	},
	"red_concrete_powder" => phf_map! {
	},
	"red_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_mushroom" => phf_map! {
	},
	"red_mushroom_block" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_nether_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_nether_bricks" => phf_map! {
	},
	"red_sand" => phf_map! {
	},
	"red_sandstone" => phf_map! {
	},
	"red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"red_sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"red_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"red_stained_glass" => phf_map! {
	},
	"red_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"red_terracotta" => phf_map! {
	},
	"red_tulip" => phf_map! {
	},
	"red_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"red_wool" => phf_map! {
	},
	"redstone_block" => phf_map! {
	},
	"redstone_lamp" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_ore" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_torch" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"redstone_wire" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"south" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
		"west" => BlockStatePropertyType::Enum { values: &[ "up", "side", "none" ] },
	},
	"reinforced_deepslate" => phf_map! {
	},
	"repeater" => phf_map! {
		"delay" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"locked" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"repeating_command_block" => phf_map! {
		"conditional" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"respawn_anchor" => phf_map! {
		"charges" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
	},
	"rooted_dirt" => phf_map! {
	},
	"rose_bush" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sand" => phf_map! {
	},
	"sandstone" => phf_map! {
	},
	"sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sandstone_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"scaffolding" => phf_map! {
		"bottom" => BlockStatePropertyType::Boolean,
		"distance" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sculk" => phf_map! {
	},
	"sculk_catalyst" => phf_map! {
		"bloom" => BlockStatePropertyType::Boolean,
	},
	"sculk_sensor" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"sculk_sensor_phase" => BlockStatePropertyType::Enum { values: &[ "inactive", "active", "cooldown" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sculk_shrieker" => phf_map! {
		"can_summon" => BlockStatePropertyType::Boolean,
		"shrieking" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"sculk_vein" => phf_map! {
		"down" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"sea_lantern" => phf_map! {
	},
	"sea_pickle" => phf_map! {
		"pickles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"seagrass" => phf_map! {
	},
	"shroomlight" => phf_map! {
	},
	"shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"slime_block" => phf_map! {
	},
	"small_amethyst_bud" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"small_dripleaf" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smithing_table" => phf_map! {
	},
	"smoker" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
	},
	"smooth_basalt" => phf_map! {
	},
	"smooth_quartz" => phf_map! {
	},
	"smooth_quartz_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_quartz_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone" => phf_map! {
	},
	"smooth_red_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_red_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone" => phf_map! {
	},
	"smooth_sandstone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_sandstone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"smooth_stone" => phf_map! {
	},
	"smooth_stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"snow" => phf_map! {
		"layers" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(8) } },
	},
	"snow_block" => phf_map! {
	},
	"soul_campfire" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"lit" => BlockStatePropertyType::Boolean,
		"signal_fire" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_fire" => phf_map! {
	},
	"soul_lantern" => phf_map! {
		"hanging" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"soul_sand" => phf_map! {
	},
	"soul_soil" => phf_map! {
	},
	"soul_torch" => phf_map! {
	},
	"soul_wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"spawner" => phf_map! {
	},
	"sponge" => phf_map! {
	},
	"spore_blossom" => phf_map! {
	},
	"spruce_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"spruce_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_leaves" => phf_map! {
		"distance" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
		"persistent" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"spruce_planks" => phf_map! {
	},
	"spruce_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"spruce_sapling" => phf_map! {
		"stage" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(1) } },
	},
	"spruce_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"sticky_piston" => phf_map! {
		"extended" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"stone" => phf_map! {
	},
	"stone_brick_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_brick_wall" => phf_map! {
		"east" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"north" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"south" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
		"up" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Enum { values: &[ "none", "low", "tall" ] },
	},
	"stone_bricks" => phf_map! {
	},
	"stone_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"stone_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stone_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"stonecutter" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"stripped_acacia_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_acacia_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_birch_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_crimson_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_dark_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_jungle_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_mangrove_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_mangrove_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_oak_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_log" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_spruce_wood" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"stripped_warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"structure_block" => phf_map! {
		"mode" => BlockStatePropertyType::Enum { values: &[ "save", "load", "corner", "data" ] },
	},
	"structure_void" => phf_map! {
	},
	"sugar_cane" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"sunflower" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"sweet_berry_bush" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"tall_grass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"tall_seagrass" => phf_map! {
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
	},
	"target" => phf_map! {
		"power" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"terracotta" => phf_map! {
	},
	"tinted_glass" => phf_map! {
	},
	"tnt" => phf_map! {
		"unstable" => BlockStatePropertyType::Boolean,
	},
	"torch" => phf_map! {
	},
	"trapped_chest" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"type" => BlockStatePropertyType::Enum { values: &[ "single", "left", "right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tripwire" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"disarmed" => BlockStatePropertyType::Boolean,
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"tripwire_hook" => phf_map! {
		"attached" => BlockStatePropertyType::Boolean,
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"tube_coral" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_block" => phf_map! {
	},
	"tube_coral_fan" => phf_map! {
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tube_coral_wall_fan" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"tuff" => phf_map! {
	},
	"turtle_egg" => phf_map! {
		"eggs" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"hatch" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(2) } },
	},
	"twisting_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"twisting_vines_plant" => phf_map! {
	},
	"verdant_froglight" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"vine" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"up" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"void_air" => phf_map! {
	},
	"wall_torch" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"warped_button" => phf_map! {
		"face" => BlockStatePropertyType::Enum { values: &[ "floor", "wall", "ceiling" ] },
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_door" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "upper", "lower" ] },
		"hinge" => BlockStatePropertyType::Enum { values: &[ "left", "right" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fence" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"warped_fence_gate" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"in_wall" => BlockStatePropertyType::Boolean,
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_fungus" => phf_map! {
	},
	"warped_hyphae" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_nylium" => phf_map! {
	},
	"warped_planks" => phf_map! {
	},
	"warped_pressure_plate" => phf_map! {
		"powered" => BlockStatePropertyType::Boolean,
	},
	"warped_roots" => phf_map! {
	},
	"warped_sign" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_stem" => phf_map! {
		"axis" => BlockStatePropertyType::Enum { values: &[ "x", "y", "z" ] },
	},
	"warped_trapdoor" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"open" => BlockStatePropertyType::Boolean,
		"powered" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wall_sign" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"warped_wart_block" => phf_map! {
	},
	"water" => phf_map! {
		"level" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"water_cauldron" => phf_map! {
		"level" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(3) } },
	},
	"waxed_copper_block" => phf_map! {
	},
	"waxed_cut_copper" => phf_map! {
	},
	"waxed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_exposed_copper" => phf_map! {
	},
	"waxed_exposed_cut_copper" => phf_map! {
	},
	"waxed_exposed_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_exposed_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_oxidized_copper" => phf_map! {
	},
	"waxed_oxidized_cut_copper" => phf_map! {
	},
	"waxed_oxidized_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_oxidized_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_weathered_copper" => phf_map! {
	},
	"waxed_weathered_cut_copper" => phf_map! {
	},
	"waxed_weathered_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"waxed_weathered_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weathered_copper" => phf_map! {
	},
	"weathered_cut_copper" => phf_map! {
	},
	"weathered_cut_copper_slab" => phf_map! {
		"type" => BlockStatePropertyType::Enum { values: &[ "top", "bottom", "double" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weathered_cut_copper_stairs" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"half" => BlockStatePropertyType::Enum { values: &[ "top", "bottom" ] },
		"shape" => BlockStatePropertyType::Enum { values: &[ "straight", "inner_left", "inner_right", "outer_left", "outer_right" ] },
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"weeping_vines" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(25) } },
	},
	"weeping_vines_plant" => phf_map! {
	},
	"wet_sponge" => phf_map! {
	},
	"wheat" => phf_map! {
		"age" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(7) } },
	},
	"white_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"white_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"white_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"white_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"white_carpet" => phf_map! {
	},
	"white_concrete" => phf_map! {
	},
	"white_concrete_powder" => phf_map! {
	},
	"white_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"white_stained_glass" => phf_map! {
	},
	"white_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"white_terracotta" => phf_map! {
	},
	"white_tulip" => phf_map! {
	},
	"white_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"white_wool" => phf_map! {
	},
	"wither_rose" => phf_map! {
	},
	"wither_skeleton_skull" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"wither_skeleton_wall_skull" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_banner" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"yellow_bed" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
		"occupied" => BlockStatePropertyType::Boolean,
		"part" => BlockStatePropertyType::Enum { values: &[ "head", "foot" ] },
	},
	"yellow_candle" => phf_map! {
		"candles" => BlockStatePropertyType::StrictlyPositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(4) } },
		"lit" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
	},
	"yellow_candle_cake" => phf_map! {
		"lit" => BlockStatePropertyType::Boolean,
	},
	"yellow_carpet" => phf_map! {
	},
	"yellow_concrete" => phf_map! {
	},
	"yellow_concrete_powder" => phf_map! {
	},
	"yellow_glazed_terracotta" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_shulker_box" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "east", "south", "west", "up", "down" ] },
	},
	"yellow_stained_glass" => phf_map! {
	},
	"yellow_stained_glass_pane" => phf_map! {
		"east" => BlockStatePropertyType::Boolean,
		"north" => BlockStatePropertyType::Boolean,
		"south" => BlockStatePropertyType::Boolean,
		"waterlogged" => BlockStatePropertyType::Boolean,
		"west" => BlockStatePropertyType::Boolean,
	},
	"yellow_terracotta" => phf_map! {
	},
	"yellow_wall_banner" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
	"yellow_wool" => phf_map! {
	},
	"zombie_head" => phf_map! {
		"rotation" => BlockStatePropertyType::PositiveInteger { maximum_value: unsafe { NonZeroU8::new_unchecked(15) } },
	},
	"zombie_wall_head" => phf_map! {
		"facing" => BlockStatePropertyType::Enum { values: &[ "north", "south", "west", "east" ] },
	},
};

static BLOCKSTATE_PROPERTIES: [(
	MinecraftVersion,
	MinecraftVersion,
	bool,
	&phf::Map<&str, phf::Map<&str, BlockStatePropertyType>>
); 9] = [
	(
		minecraft_version!(1, 13),
		minecraft_version!(1, 13, 2),
		false,
		&BLOCKSTATE_PROPERTIES_1_13_TO_1_13_2_EXCLUSIVE
	),
	(
		minecraft_version!(1, 13, 2),
		minecraft_version!(1, 14),
		false,
		&BLOCKSTATE_PROPERTIES_1_13_2_TO_1_14_EXCLUSIVE
	),
	(
		minecraft_version!(1, 14),
		minecraft_version!(1, 15),
		false,
		&BLOCKSTATE_PROPERTIES_1_14_TO_1_15_EXCLUSIVE
	),
	(
		minecraft_version!(1, 15),
		minecraft_version!(1, 16),
		false,
		&BLOCKSTATE_PROPERTIES_1_15_TO_1_16_EXCLUSIVE
	),
	(
		minecraft_version!(1, 16),
		minecraft_version!(1, 16, 2),
		false,
		&BLOCKSTATE_PROPERTIES_1_16_TO_1_16_2_EXCLUSIVE
	),
	(
		minecraft_version!(1, 16, 2),
		minecraft_version!(1, 17),
		false,
		&BLOCKSTATE_PROPERTIES_1_16_2_TO_1_17_EXCLUSIVE
	),
	(
		minecraft_version!(1, 17),
		minecraft_version!(1, 18),
		false,
		&BLOCKSTATE_PROPERTIES_1_17_TO_1_18_EXCLUSIVE
	),
	(
		minecraft_version!(1, 18),
		minecraft_version!(1, 19),
		false,
		&BLOCKSTATE_PROPERTIES_1_18_TO_1_19_EXCLUSIVE
	),
	(
		minecraft_version!(1, 19),
		minecraft_version!(1, 19),
		true,
		&BLOCKSTATE_PROPERTIES_1_19_TO_1_19_INCLUSIVE
	)
];

/// Gets the map of per-block block state properties that match the specified version range,
/// i.e., are read by at least a single game version in the range. `None` is returned if
/// the version range did not match any known Minecraft version.
///
/// The keys in the returned map represent block state names. The value for each key is
/// another map of pairs of property names and their possible data types. For any given
/// Minecraft version, a property will only have a single data type, but it is technically
/// possible for different Minecraft versions to assign different data types to the same
/// properties.
pub(super) fn matching_for_version_range(
	range: &impl RangeBounds<MinecraftVersion>
) -> Option<AHashMap<&'static str, AHashMap<&'static str, TinyVec<[BlockStatePropertyType; 1]>>>> {
	let mut matching_properties = None;

	for (version_start_bound, version_end_bound, end_bound_is_inclusive, properties) in
		BLOCKSTATE_PROPERTIES
	{
		let range_matches = if end_bound_is_inclusive {
			(version_start_bound..=version_end_bound).intersects(range)
		} else {
			(version_start_bound..version_end_bound).intersects(range)
		};

		if range_matches {
			let matching_properties =
				matching_properties.get_or_insert_with(|| AHashMap::with_capacity(properties.len()));

			for (block, block_properties) in properties {
				// Ensure that every block has an entry, even if it has no properties
				let block_properties_entry = matching_properties
					.entry(*block)
					.or_insert_with(|| AHashMap::with_capacity(block_properties.len()));

				for (property, property_type) in block_properties {
					let property_types = block_properties_entry
						.entry(*property)
						.or_insert_with(|| tiny_vec!());

					// O(n) check, but in practice it should be faster due to the lower constant
					// costs offsetting the extremely small expected sizes
					if !property_types.contains(property_type) {
						property_types.push(*property_type);
					}
				}
			}
		}
	}

	matching_properties
}
