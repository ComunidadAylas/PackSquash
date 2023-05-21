use std::{borrow::Cow, fmt::Formatter, sync::Arc};

use ahash::AHashMap;
use compact_str::CompactString;
use enum_map::{Enum, EnumMap};
use serde::{
	de::{self, MapAccess, Visitor},
	Deserialize, Deserializer, Serialize
};
use strum::EnumIter;
use thiserror::Error;

use crate::{
	pack_processor::java::{
		resource_location::{ResourceLocation, ResourceLocationError},
		PackType
	},
	util::{
		cow_util::StripPrefixExt,
		enum_map_serializers::{
			enum_map_ignore_missing_values_deserializer, enum_map_skip_none_values_serializer
		},
		zero_copy_deserialize_traits::ZeroCopyDeserializable
	}
};

/// A vanilla item or block model.
///
/// References:
/// - Vanilla deserializer: `net.minecraft.client.renderer.block.model.BlockModel`
///   (used for both item and block models)
/// - `net.minecraft.client.resources.model.ModelBakery#BUILTIN_*`
///   (look at `GENERATION_MARKER` and `BLOCK_ENTITY_MARKER` usages at `ModelBakery`)
/// - https://minecraft.fandom.com/wiki/Tutorials/Models#Block_models
/// - https://minecraft.fandom.com/wiki/Tutorials/Models#Item_models
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ItemOrBlockModel<'data> {
	/// The location of the parent model of this model.
	///
	/// The following parent resource locations are special and always accepted by the game:
	/// - `builtin/generated`: generates a one unit deep, one block wide rectangular cuboid by
	///                        overlaying the textures referenced by the variables `layer0` to
	///                        `layer4`.
	/// - `builtin/missing`: a black and magenta missing model cube.
	/// - `builtin/entity`: uses a hardcoded entity model as the parent, depending on the item
	///                     that is using this model. Generates an empty model for items that do
	///                     not represent block entities and block models.
	///
	/// Block entities (i.e., chests) have block models, but their elements and parent are ignored,
	/// because the game is hardcoded to render their entities instead.
	#[serde(
		deserialize_with = "parent_model_resource_location_deserializer",
		skip_serializing_if = "Option::is_none"
	)]
	#[serde(borrow, default)]
	pub(super) parent: Option<Arc<ResourceLocation<'data>>>,
	/// A map of texture variables to texture references, to be used by elements in this, child
	/// or parent models. Texture references are serialized as either texture variable names prefixed
	/// by `#` or texture asset resource locations, although the [`TextureLocationOrReference`] type
	/// provides a abstraction over that fact.
	///
	/// See the [`ElementFace::texture`] attribute documentation for more information about how
	/// this map is used.
	#[serde(rename = "textures")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	pub(super) texture_map: Option<Arc<AHashMap<CompactString, TextureLocationOrReference<'data>>>>,
	/// Whether the model will be rendered with ambient occlusion shadow effects or not.
	/// If the model has parents, the value used by the game is always inherited from the root
	/// model. Defaults to `true`.
	///
	/// In-game ambient occlusion effect preview: https://imgur.com/a/HrZKpdN
	#[serde(rename = "ambientocclusion")]
	#[serde(skip_serializing_if = "is_true")]
	#[serde(default = "bool_true")]
	pub(super) ambient_occlusion: bool,
	/// Item model display transforms. Unspecified transforms are inherited from parent models.
	#[serde(rename = "display")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	pub(super) item_display_transforms: Option<ItemTransforms<'data>>,
	/// Item model override predicates. Not inherited from parent models. Available from 1.9
	/// onwards.
	#[serde(rename = "overrides")]
	#[serde(skip_serializing_if = "is_none_or_empty_vec")]
	#[serde(borrow)]
	pub(super) item_overrides: Option<Vec<ItemOverride<'data>>>,
	/// Item rendering style. If not specified, the value used by the game is inherited from the
	/// parents, falling back to `ItemGuiLight::Side` if the root model has no light defined.
	/// Available from 1.15.2 onwards.
	#[serde(rename = "gui_light")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(super) item_gui_light: Option<ItemGuiLight>,
	/// The model block elements. If no element list is specified or if it is empty,
	/// elements are inherited from the parent. Allegedly, Minecraft < 1.9 did not support
	/// both `parent` and `elements` to be specified at the same time.
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	pub(super) elements: Option<Vec<Element<'data>>>,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

/// A dummy struct with no generic parameters that represents [`ItemOrBlockModel`] on
/// deserialization helper code.
pub(super) struct ItemOrBlockModelRepresentative;
impl<'data> ZeroCopyDeserializable<'data> for ItemOrBlockModelRepresentative {
	type Type = ItemOrBlockModel<'data>;
}

impl ItemOrBlockModel<'_> {
	// TODO document that a single reference is assumed for inner Arcs
	pub(super) fn into_owned(self) -> ItemOrBlockModel<'static> {
		ItemOrBlockModel {
			parent: self.parent.map(|resource_location| {
				Arc::new(Arc::into_inner(resource_location).unwrap().into_owned())
			}),
			texture_map: self.texture_map.map(|texture_map| {
				Arc::new(
					Arc::into_inner(texture_map)
						.unwrap()
						.into_iter()
						.map(|(texture_variable, texture_reference)| {
							(texture_variable, texture_reference.into_owned())
						})
						.collect()
				)
			}),
			ambient_occlusion: self.ambient_occlusion,
			item_display_transforms: self
				.item_display_transforms
				.map(|item_display_transforms| item_display_transforms.into_owned()),
			item_overrides: self.item_overrides.map(|item_overrides| {
				item_overrides
					.into_iter()
					.map(|item_override| item_override.into_owned())
					.collect()
			}),
			item_gui_light: self.item_gui_light,
			elements: self.elements.map(|elements| {
				elements
					.into_iter()
					.map(|element| element.into_owned())
					.collect()
			}),
			bloat_fields: self
				.bloat_fields
				.into_iter()
				.map(|(field_name, field_value)| (field_name.into_owned().into(), field_value))
				.collect()
		}
	}
}

fn parent_model_resource_location_deserializer<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<Option<Arc<ResourceLocation<'de>>>, D::Error> {
	let resource_location_string = <Cow<'de, str>>::deserialize(deserializer)?;

	if resource_location_string.is_empty() {
		// MC 1.19.2 interprets an empty string as no parent
		Ok(None)
	} else {
		Ok(Some(Arc::new(
			ResourceLocation::new(
				PackType::ResourcePack,
				resource_location_string,
				Some("models"),
				Some("json")
			)
			.map_err(de::Error::custom)?
		)))
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Element<'data> {
	from: ElementCuboidPoint,
	to: ElementCuboidPoint,
	#[serde(skip_serializing_if = "element_rotation_angle_is_none_or_zero")]
	#[serde(borrow)]
	rotation: Option<ElementRotation<'data>>,
	#[serde(
		deserialize_with = "enum_map_ignore_missing_values_deserializer",
		serialize_with = "enum_map_skip_none_values_serializer"
	)]
	pub(super) faces: EnumMap<ElementFaceDirection, Option<ElementFace<'data>>>,
	#[serde(default = "bool_true")]
	#[serde(skip_serializing_if = "is_true")]
	shade: bool,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

impl Element<'_> {
	/// Computes the default face texture UV coordinates for an element face, given its direction.
	///
	/// The algorithm implemented by this function was deduced from static analysis of the Minecraft
	/// 1.19.2 `net.minecraft.client.renderer.block.model.BlockElement#uvsByFace` method.
	pub(super) fn default_face_uv(&self, direction: ElementFaceDirection) -> (f32, f32, f32, f32) {
		match direction {
			ElementFaceDirection::Down => {
				(self.from.0, 16.0 - self.to.2, self.to.0, 16.0 - self.from.2)
			}
			ElementFaceDirection::Up => (self.from.0, self.from.2, self.to.0, self.to.2),
			ElementFaceDirection::North => (
				16.0 - self.to.0,
				16.0 - self.to.1,
				16.0 - self.from.0,
				16.0 - self.from.1
			),
			ElementFaceDirection::South => {
				(self.from.0, 16.0 - self.to.1, self.to.0, 16.0 - self.from.1)
			}
			ElementFaceDirection::West => {
				(self.from.2, 16.0 - self.to.1, self.to.2, 16.0 - self.from.1)
			}
			ElementFaceDirection::East => (
				16.0 - self.to.2,
				16.0 - self.to.1,
				16.0 - self.from.2,
				16.0 - self.from.1
			)
		}
	}

	fn into_owned(self) -> Element<'static> {
		Element {
			from: self.from,
			to: self.to,
			rotation: self.rotation.map(|rotation| rotation.into_owned()),
			faces: self
				.faces
				.into_iter()
				.map(|(face_direction, face)| (face_direction, face.map(|face| face.into_owned())))
				.collect(),
			shade: self.shade,
			bloat_fields: self
				.bloat_fields
				.into_iter()
				.map(|(field_name, field_value)| (field_name.into_owned().into(), field_value))
				.collect()
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ItemTransforms<'data> {
	/// Transform to use when the item is held in the left hand from a third person view.
	/// Only valid in 1.9 and above. If not specified, the game defaults to the
	/// `third_person_right_hand` transform.
	#[serde(rename = "thirdperson_lefthand")]
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) third_person_left_hand: Option<ItemTransform<'data>>,
	/// Transform to use when the item is held in the right hand from a third person view.
	/// Only valid in 1.9 and above. If not specified, the game defaults to the corresponding
	/// parent transform, or an identity transform if the root model has no transform.
	#[serde(rename = "thirdperson_righthand")]
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) third_person_right_hand: Option<ItemTransform<'data>>,
	/// Transform to use when the item is held in the main hand from a third person view.
	/// Only valid in 1.8.9 and below.
	#[serde(rename = "thirdperson")]
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) third_person: Option<ItemTransform<'data>>,
	/// Transform to use when the item is held in the left hand from a first person view.
	/// Only valid in 1.9 and above. If not specified, the game defaults to the
	/// `first_person_right_hand` transform.
	#[serde(rename = "firstperson_lefthand")]
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) first_person_left_hand: Option<ItemTransform<'data>>,
	/// Transform to use when the item is held in the right hand from a first person view.
	/// Only valid in 1.9 and above. If not specified, the game defaults to the corresponding
	/// parent transform, or an identity transform if the root model has no transform.
	#[serde(rename = "firstperson_righthand")]
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) first_person_right_hand: Option<ItemTransform<'data>>,
	/// Transform to use when the item is held in the main hand from a first person view.
	/// Only valid in 1.8.9 and below. If not specified, the game defaults to the corresponding
	/// parent transform, or an identity transform if the root model has no transform.
	#[serde(rename = "firstperson")]
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) first_person: Option<ItemTransform<'data>>,
	/// Transform to use when the item is held in a living entity head. If not specified, the game
	/// defaults to the corresponding parent transform, or an identity transform if the root model
	/// has no transform.
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) head: Option<ItemTransform<'data>>,
	/// Transform to use when the item is drawn on the GUI (inventory, advancement toast...). If not
	/// specified, the game defaults to the corresponding parent transform, or an identity transform
	/// if the root model has no transform.
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) gui: Option<ItemTransform<'data>>,
	/// Transform to use when the item is laying on the ground as an entity. If not specified, the game
	/// defaults to the corresponding parent transform, or an identity transform if the root model
	/// has no transform.
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) ground: Option<ItemTransform<'data>>,
	/// Transform to use when the item is hung in some holder block entity (i.e., item frame). If not
	/// specified, the game defaults to the corresponding parent transform, or an identity transform
	/// if the root model has no transform.
	#[serde(skip_serializing_if = "is_none_or_identity_transform")]
	#[serde(borrow)]
	pub(super) fixed: Option<ItemTransform<'data>>,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

impl ItemTransforms<'_> {
	fn into_owned(self) -> ItemTransforms<'static> {
		ItemTransforms {
			third_person_left_hand: self
				.third_person_left_hand
				.map(|third_person_left_hand| third_person_left_hand.into_owned()),
			third_person_right_hand: self
				.third_person_right_hand
				.map(|third_person_right_hand| third_person_right_hand.into_owned()),
			third_person: self
				.third_person
				.map(|third_person| third_person.into_owned()),
			first_person_left_hand: self
				.first_person_left_hand
				.map(|first_person_left_hand| first_person_left_hand.into_owned()),
			first_person_right_hand: self
				.first_person_right_hand
				.map(|first_person_right_hand| first_person_right_hand.into_owned()),
			first_person: self
				.first_person
				.map(|first_person| first_person.into_owned()),
			head: self.head.map(|head| head.into_owned()),
			gui: self.gui.map(|gui| gui.into_owned()),
			ground: self.ground.map(|ground| ground.into_owned()),
			fixed: self.fixed.map(|fixed| fixed.into_owned()),
			bloat_fields: self
				.bloat_fields
				.into_iter()
				.map(|(field_name, field_value)| (field_name.into_owned().into(), field_value))
				.collect()
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ItemOverride<'data> {
	#[serde(deserialize_with = "item_override_model_resource_location_deserializer")]
	pub(super) model: ResourceLocation<'data>,
	// TODO keys are properties, parsed as `ResourceLocation`s (see `net.minecraft.client.renderer.block.model.ItemOverride`).
	//      Predicates are resolved to generate baked models at `net.minecraft.client.renderer.block.model.ItemOverrides#resolve`.
	//      The possible vanilla item properties are registered at `net.minecraft.client.renderer.item.ItemProperties`.
	//      A predicate matches if all its properties have actual values greater than or equal to the predicate values.
	#[serde(deserialize_with = "item_override_predicates_deserializer")]
	predicate: AHashMap<ResourceLocation<'data>, f32>,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

impl ItemOverride<'_> {
	fn into_owned(self) -> ItemOverride<'static> {
		ItemOverride {
			model: self.model.into_owned(),
			predicate: self
				.predicate
				.into_iter()
				.map(|(predicate, value)| (predicate.into_owned(), value))
				.collect(),
			bloat_fields: self
				.bloat_fields
				.into_iter()
				.map(|(field_name, field_value)| (field_name.into_owned().into(), field_value))
				.collect()
		}
	}
}

fn item_override_model_resource_location_deserializer<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<ResourceLocation<'de>, D::Error> {
	let resource_location_string = <Cow<'de, str>>::deserialize(deserializer)?;

	ResourceLocation::new(
		PackType::ResourcePack,
		resource_location_string,
		Some("models"),
		Some("json")
	)
	.map_err(de::Error::custom)
}

fn item_override_predicates_deserializer<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<AHashMap<ResourceLocation<'de>, f32>, D::Error> {
	struct MapVisitor;

	impl<'de> Visitor<'de> for MapVisitor {
		type Value = AHashMap<ResourceLocation<'de>, f32>;

		fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
			write!(formatter, "a item model override predicates map")
		}

		fn visit_map<A: MapAccess<'de>>(self, mut map_access: A) -> Result<Self::Value, A::Error> {
			let mut map = AHashMap::with_capacity(map_access.size_hint().unwrap_or(0));

			while let Some((predicate, value)) = map_access.next_entry::<Cow<str>, _>()? {
				map.insert(
					ResourceLocation::new(
						PackType::ResourcePack,
						predicate,
						None::<&str>,
						None::<&str>
					)
					.map_err(de::Error::custom)?,
					value
				);
			}

			Ok(map)
		}
	}

	deserializer.deserialize_map(MapVisitor)
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged, try_from = "Cow<'_, str>")]
pub(super) enum TextureLocationOrReference<'data> {
	Location(ResourceLocation<'data>),
	Reference(CompactString)
}

impl TextureLocationOrReference<'_> {
	fn into_owned(self) -> TextureLocationOrReference<'static> {
		match self {
			Self::Location(resource_location) => {
				TextureLocationOrReference::Location(resource_location.into_owned())
			}
			Self::Reference(texture_variable) => {
				TextureLocationOrReference::Reference(texture_variable)
			}
		}
	}
}

impl<'data> TryFrom<Cow<'data, str>> for TextureLocationOrReference<'data> {
	type Error = ResourceLocationError;

	fn try_from(value: Cow<'data, str>) -> Result<Self, Self::Error> {
		match value.strip_prefix("#") {
			Ok(texture_reference) => Ok(Self::Reference(texture_reference.into())),
			Err(texture_location) => Ok(Self::Location(ResourceLocation::new(
				PackType::ResourcePack,
				texture_location,
				Some("textures"),
				Some("png")
			)?))
		}
	}
}

/// Defines the rendering style of an item model in the Minecraft UI.
#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub(super) enum ItemGuiLight {
	/// Render from the front (i.e., like a item, flat).
	Front,
	/// Render from a side (i.e., like a block, in isometric view).
	Side
}

const IDENTITY_ITEM_TRANSFORM_ROTATION: (f32, f32, f32) = (0.0, 0.0, 0.0);
const IDENTITY_ITEM_TRANSFORM_TRANSLATION: (f32, f32, f32) = (0.0, 0.0, 0.0);
const IDENTITY_ITEM_TRANSFORM_SCALE: (f32, f32, f32) = (1.0, 1.0, 1.0);

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub(super) struct ItemTransform<'data> {
	/// The transform rotation for each axis, in sexagesimal degrees.
	#[serde(
		deserialize_with = "clamping_rotation_deserializer",
		skip_serializing_if = "is_default"
	)]
	rotation: (f32, f32, f32),
	#[serde(
		deserialize_with = "clamping_translation_deserializer",
		skip_serializing_if = "is_default"
	)]
	translation: (f32, f32, f32),
	#[serde(
		deserialize_with = "clamping_scale_deserializer",
		skip_serializing_if = "is_identity_scale"
	)]
	scale: (f32, f32, f32),
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

impl ItemTransform<'_> {
	fn into_owned(self) -> ItemTransform<'static> {
		ItemTransform {
			rotation: self.rotation,
			translation: self.translation,
			scale: self.scale,
			bloat_fields: self
				.bloat_fields
				.into_iter()
				.map(|(field_name, field_value)| (field_name.into_owned().into(), field_value))
				.collect()
		}
	}
}

impl Default for ItemTransform<'_> {
	fn default() -> Self {
		Self {
			rotation: IDENTITY_ITEM_TRANSFORM_ROTATION,
			translation: IDENTITY_ITEM_TRANSFORM_TRANSLATION,
			scale: IDENTITY_ITEM_TRANSFORM_SCALE,
			bloat_fields: AHashMap::new()
		}
	}
}

impl PartialEq for ItemTransform<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.rotation == other.rotation
			&& self.translation == other.translation
			&& self.scale == other.scale
	}
}

fn clamping_rotation_deserializer<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<(f32, f32, f32), D::Error> {
	let rotation = <(f32, f32, f32)>::deserialize(deserializer)?;

	// Rotations higher than 360º only serve the purpose of bloating file sizes,
	// decreasing readability and increasing floating point precision errors,
	// so bring them back to their intended range
	Ok((rotation.0 % 360.0, rotation.1 % 360.0, rotation.2 % 360.0))
}

fn clamping_translation_deserializer<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<(f32, f32, f32), D::Error> {
	let translation = <(f32, f32, f32)>::deserialize(deserializer)?;

	// The Minecraft wiki says that on 1.8.2-pre5 translation coordinates were
	// limited to the [-24, 24] range, but code analysis of the 1.19.2 client
	// reveals that the coordinates are clamped to [-5, 5] after multiplying
	// the deserialized vector by 0.0625, which equals a [-5 / 0.0625, 5 / 0.0625] =
	// = [-80, 80] range. Maybe something changed since 1.8.2-pre5, but nobody
	// added it to the model format changelog. Moreover, the latest edition of the
	// model page as of 12/02/2022 documents this [-80, 80] range. Be conservative
	// with the clamping here to avoid issues
	const MAX_TRANSLATION: f32 = 80.0;
	Ok((
		translation.0.clamp(-MAX_TRANSLATION, MAX_TRANSLATION),
		translation.1.clamp(-MAX_TRANSLATION, MAX_TRANSLATION),
		translation.2.clamp(-MAX_TRANSLATION, MAX_TRANSLATION)
	))
}

fn clamping_scale_deserializer<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<(f32, f32, f32), D::Error> {
	let translation = <(f32, f32, f32)>::deserialize(deserializer)?;

	const MAX_SCALE: f32 = 4.0;
	Ok((
		translation.0.clamp(-MAX_SCALE, MAX_SCALE),
		translation.1.clamp(-MAX_SCALE, MAX_SCALE),
		translation.2.clamp(-MAX_SCALE, MAX_SCALE)
	))
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(try_from = "(f32, f32, f32)")]
pub(super) struct ElementCuboidPoint(f32, f32, f32);

impl TryFrom<(f32, f32, f32)> for ElementCuboidPoint {
	type Error = CuboidPointCoordinateOutOfBounds;

	fn try_from(value: (f32, f32, f32)) -> Result<Self, Self::Error> {
		fn check_coordinate_range(
			value: f32,
			axis: DirectionAxis
		) -> Result<f32, CuboidPointCoordinateOutOfBounds> {
			if (-16.0..=32.0).contains(&value) {
				Ok(value)
			} else {
				Err(CuboidPointCoordinateOutOfBounds { axis, value })
			}
		}

		Ok(Self(
			check_coordinate_range(value.0, DirectionAxis::X)?,
			check_coordinate_range(value.1, DirectionAxis::Y)?,
			check_coordinate_range(value.2, DirectionAxis::Z)?
		))
	}
}

#[derive(Debug, Error)]
#[error("The cuboid point {axis:?} coordinate is not in the [-16, 32] range: {value}")]
pub(super) struct CuboidPointCoordinateOutOfBounds {
	axis: DirectionAxis,
	value: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ElementRotation<'data> {
	// This vector is multiplied by 0.0625 when it is deserialized by the game
	origin: (f32, f32, f32),
	axis: DirectionAxis,
	angle: ElementRotationAngle,
	#[serde(default)]
	#[serde(skip_serializing_if = "is_default")]
	rescale: bool,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

impl ElementRotation<'_> {
	fn into_owned(self) -> ElementRotation<'static> {
		ElementRotation {
			origin: self.origin,
			axis: self.axis,
			angle: self.angle,
			rescale: self.rescale,
			bloat_fields: self
				.bloat_fields
				.into_iter()
				.map(|(field_name, field_value)| (field_name.into_owned().into(), field_value))
				.collect()
		}
	}
}

// TODO document that the game accepts both uppercase and lowercase, but serializing
//      to lowercase is better because the surrounding identifiers are lowercase, so
//      compressibility is increased
#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all(serialize = "lowercase"))]
pub(super) enum DirectionAxis {
	#[serde(alias = "x")]
	X,
	#[serde(alias = "y")]
	Y,
	#[serde(alias = "z")]
	Z
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialOrd, PartialEq)]
#[repr(transparent)]
#[serde(try_from = "f32")]
pub(super) struct ElementRotationAngle(f32);

impl TryFrom<f32> for ElementRotationAngle {
	type Error = InvalidElementRotationAngleError;

	fn try_from(value: f32) -> Result<Self, Self::Error> {
		let abs_value = value.abs();
		if value == 0.0 || abs_value == 22.5 || abs_value == 45.0 {
			Ok(ElementRotationAngle(value))
		} else {
			Err(InvalidElementRotationAngleError(value))
		}
	}
}

#[derive(Debug, Error)]
#[error("The element rotation angle must be one of -45, -22.5, 0, 22.5 or 45, but it was {0}")]
pub(super) struct InvalidElementRotationAngleError(f32);

#[derive(Debug, Deserialize, Serialize, Copy, Clone, Enum, EnumIter)]
#[serde(rename_all = "lowercase")]
pub(super) enum ElementFaceDirection {
	Down,
	Up,
	North,
	South,
	West,
	East
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct ElementFace<'data> {
	/// A texture map entry key (i.e., texture reference) that is used by the game to get a material (i.e.,
	/// texture) for this cuboid face, deserialized by us without the leading # prefix.
	///
	/// The game resolves it to a material as follows:
	///
	/// 1. If it starts with # (the character that marks a texture reference), it strips the # prefix.
	/// 2. It uses the key to address the texture map, whose values are either a material or a string (entry key).
	///    If the current model texture map does not contain the key, parents are looked up, falling back to
	///    a missing texture material value if the root model does not contain the key. If a loop is detected (i.e.,
	///    the same entry key is visited twice), an exception is thrown.
	/// 3. If the retrieved texture map value is not a material, go back to 2 using the new resolved
	///    string (entry key).
	///
	/// The values for a texture map are deserialized for a `ItemOrBlockModel` as either a material or a
	/// texture reference. Texture references are stripped their prefix and converted to a plain string
	/// (entry key). Materials are deserialized by parsing their values as a `ResourceLocation` and
	/// creating a `Material` from that.
	///
	/// Some observations:
	///
	/// - It is not necessary to use texture reference syntax (i.e., prefix texture references with #)
	///   to refer to texture variables in element faces.
	#[serde(deserialize_with = "texture_reference_deserializer")]
	#[serde(borrow)]
	pub(super) texture: Cow<'data, str>,
	/// UV coordinates in [x1, y1, x2, y2] format. When missing, the game calculates them as if
	/// [`Element::default_face_uv`] was run for this face direction, using the `from` and `to`
	/// points of the element. On < 1.9, this is a required attribute.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(super) uv: Option<(f32, f32, f32, f32)>,
	#[serde(default)]
	#[serde(skip_serializing_if = "is_default")]
	rotation: ElementFaceRotationAngle,
	#[serde(rename = "tintindex")]
	#[serde(default = "minus_one")]
	#[serde(skip_serializing_if = "is_minus_one")]
	tint_index: i32,
	#[serde(rename = "cullface")]
	#[serde(skip_serializing_if = "Option::is_none")]
	cull_face: Option<ElementFaceDirection>,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

impl ElementFace<'_> {
	fn into_owned(self) -> ElementFace<'static> {
		ElementFace {
			texture: self.texture.into_owned().into(),
			uv: self.uv,
			rotation: self.rotation,
			tint_index: self.tint_index,
			cull_face: self.cull_face,
			bloat_fields: self
				.bloat_fields
				.into_iter()
				.map(|(field_name, field_value)| (field_name.into_owned().into(), field_value))
				.collect()
		}
	}
}

fn texture_reference_deserializer<'de, D: Deserializer<'de>>(
	deserializer: D
) -> Result<Cow<'de, str>, D::Error> {
	Ok(<Cow<'de, str>>::deserialize(deserializer)?
		.strip_prefix("#")
		.map_or_else(
			|texture_reference| texture_reference,
			|texture_reference| texture_reference
		))
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(transparent)]
#[serde(try_from = "u16")]
pub(super) struct ElementFaceRotationAngle(u16);

impl TryFrom<u16> for ElementFaceRotationAngle {
	type Error = InvalidElementFaceRotationAngleError;

	fn try_from(value: u16) -> Result<Self, Self::Error> {
		if value % 90 == 0 && value / 90 < 4 {
			Ok(ElementFaceRotationAngle(value))
		} else {
			Err(InvalidElementFaceRotationAngleError(value))
		}
	}
}

#[derive(Debug, Error)]
#[error("The element rotation angle must be one of 0, 90, 180 or 270, but it was {0}")]
pub(super) struct InvalidElementFaceRotationAngleError(u16);

fn element_rotation_angle_is_none_or_zero(value: &Option<ElementRotation>) -> bool {
	if let Some(element_rotation) = value {
		element_rotation.angle.0 == 0.0
	} else {
		// The game treats this case as no rotation
		true
	}
}

const fn bool_true() -> bool {
	true
}

const fn is_true(value: &bool) -> bool {
	*value
}

const fn minus_one() -> i32 {
	-1
}

const fn is_minus_one(value: &i32) -> bool {
	*value == -1
}

fn is_none_or_identity_transform(value: &Option<ItemTransform<'_>>) -> bool {
	if let Some(transform) = value {
		is_identity_transform(transform)
	} else {
		true
	}
}

fn is_identity_transform(value: &ItemTransform<'_>) -> bool {
	value.rotation == IDENTITY_ITEM_TRANSFORM_ROTATION
		&& value.translation == IDENTITY_ITEM_TRANSFORM_TRANSLATION
		&& is_identity_scale(&value.scale)
}

fn is_identity_scale(value: &(f32, f32, f32)) -> bool {
	*value == IDENTITY_ITEM_TRANSFORM_SCALE
}

fn is_none_or_empty_vec<T>(value: &Option<Vec<T>>) -> bool {
	if let Some(vec) = value {
		vec.is_empty()
	} else {
		true
	}
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
	*value == T::default()
}
