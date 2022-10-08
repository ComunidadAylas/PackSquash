use ahash::AHashMap;
use serde::{Deserialize, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Write};
use std::num::NonZeroI32;
use std::ops::{Deref, DerefMut};
use std::slice;
use thiserror::Error;
use tinyvec::{tiny_vec, TinyVec};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct BlockState<'data> {
	// TODO docs: must be present if not multipart. The game falls back silently if some variant is
	//      undefined, but prints a warning for unknown properties
	// TODO docs: variant format follows regex ^(?:|normal|[^=,]+=[^=,]+(?:,[^=,]+=[^=,]+)*)$
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	pub(super) variants: Option<AHashMap<VariantPredicate, VariantList<'data>>>,
	// TODO docs: technically both multipart and variants are accepted, but the game doesn't handle
	//      them well, and the wiki documents that they are exclusive, so error if present with
	//      variants
	#[serde(rename = "multipart")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	pub(super) multipart_variants: Option<Vec<MultipartVariantSelector<'data>>>,
	#[serde(flatten)]
	#[serde(borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, serde_json::Value>
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(try_from = "Cow<'_, str>")]
pub(super) struct VariantPredicate {
	pub(super) property_value_pairs: TinyVec<[(String, String); 2]>,
	pub(super) is_legacy_invariant: bool
}

impl TryFrom<Cow<'_, str>> for VariantPredicate {
	type Error = VariantPredicateError;

	fn try_from(property_value_pairs_string: Cow<'_, str>) -> Result<Self, Self::Error> {
		let (property_value_pairs, is_legacy_invariant) = if property_value_pairs_string == "normal" {
			// Accept the legacy (< 1.13) "normal" string as a synonym for an empty predicate
			(tiny_vec!(), true)
		} else {
			let mut property_value_pairs = tiny_vec!();

			// Empty properties are ignored by the game, so filter them out
			for property_value_pair in property_value_pairs_string
				.split(',')
				.filter(|value| !value.is_empty())
			{
				let mut pair_iter = property_value_pair.splitn(2, '=');
				let property = pair_iter.next().unwrap();
				let value = pair_iter
					.next()
					.ok_or_else(|| VariantPredicateError::MissingValue(property.into()))?;

				if !is_legal_property_name_or_value(property) {
					return Err(VariantPredicateError::InvalidPropertyName(property.into()));
				}

				if !is_legal_property_name_or_value(value) {
					return Err(VariantPredicateError::InvalidPropertyValue(property.into()));
				}

				// TODO maybe optimize this to avoid clone (can be done when Cow::Borrowed, but not owned)
				property_value_pairs.push((property.into(), value.into()));
			}

			(property_value_pairs, false)
		};

		Ok(Self {
			property_value_pairs,
			is_legacy_invariant
		})
	}
}

impl Serialize for VariantPredicate {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		struct PredicatePairsWrapper<'pairs>(&'pairs [(String, String)]);

		impl Display for PredicatePairsWrapper<'_> {
			fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
				self.0
					.iter()
					.try_fold(true, |is_first, pair| {
						if !is_first {
							f.write_char(',')?;
						}

						f.write_str(&pair.0)?;
						f.write_char('=')?;
						f.write_str(&pair.1)?;

						Ok(false)
					})
					.map(|_| ())
			}
		}

		if self.is_legacy_invariant {
			serializer.serialize_str("normal")
		} else {
			serializer.collect_str(&PredicatePairsWrapper(&self.property_value_pairs))
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum VariantList<'data> {
	#[serde(borrow)]
	SingleVariant(Variant<'data>),
	#[serde(borrow)]
	SeveralVariants(TinyVec<[Variant<'data>; 2]>)
}

impl<'data> Deref for VariantList<'data> {
	type Target = [Variant<'data>];

	fn deref(&self) -> &Self::Target {
		match self {
			Self::SingleVariant(variant) => slice::from_ref(variant),
			Self::SeveralVariants(variants) => variants
		}
	}
}

impl<'data> DerefMut for VariantList<'data> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		match self {
			Self::SingleVariant(variant) => slice::from_mut(variant),
			Self::SeveralVariants(variants) => variants
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Variant<'data> {
	#[serde(borrow)]
	pub(super) model: Cow<'data, str>,
	#[serde(default)]
	#[serde(rename = "x")]
	#[serde(skip_serializing_if = "is_default")]
	rotation_x: RightAngleRotation,
	#[serde(default)]
	#[serde(rename = "y")]
	#[serde(skip_serializing_if = "is_default")]
	rotation_y: RightAngleRotation,
	#[serde(default)]
	#[serde(rename = "uvlock")]
	#[serde(skip_serializing_if = "is_default")]
	lock_uv: bool,
	#[serde(default = "default_weight")]
	#[serde(skip_serializing_if = "is_default_weight")]
	pub(super) weight: NonZeroI32,
	#[serde(flatten)]
	#[serde(borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, serde_json::Value>
}

impl Default for Variant<'_> {
	fn default() -> Self {
		Self {
			model: Default::default(),
			rotation_x: Default::default(),
			rotation_y: Default::default(),
			lock_uv: false,
			weight: default_weight(),
			bloat_fields: Default::default()
		}
	}
}

#[derive(Default, Debug, Deserialize, Serialize, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
#[serde(try_from = "u16")]
struct RightAngleRotation(u16);

impl TryFrom<u16> for RightAngleRotation {
	type Error = NonRightAngleRotationError;

	fn try_from(value: u16) -> Result<Self, Self::Error> {
		if value % 90 == 0 {
			// The game normalizes the rotation to the 0-360º range, so do it ourselves
			// to serialize potentially smaller numbers and save some space
			Ok(Self(value % 360))
		} else {
			Err(NonRightAngleRotationError(value))
		}
	}
}

#[derive(Debug, Error)]
#[error("Non right angle rotation in block state variant: {0}")]
struct NonRightAngleRotationError(u16);

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct MultipartVariantSelector<'data> {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "when")]
	#[serde(borrow)]
	pub(super) condition: Option<Condition<'data>>,
	#[serde(rename = "apply")]
	#[serde(borrow)]
	pub(super) variants: VariantList<'data>,
	#[serde(flatten)]
	#[serde(borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, serde_json::Value>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum Condition<'data> {
	#[serde(borrow)]
	Composite(CompositeBooleanCondition<'data>),
	// TODO docs: key must be a recognized blockstate property, following the regex [a-z0-9_]+
	//      (c.f. net.minecraft.world.level.block.state.StateDefinition#NAME_PATTERN)
	//      value must follow the regex !?[a-z0-9_]+(?:\|[a-z0-9_]+)*
	//      Example: !hello|world
	#[serde(borrow)]
	PropertyPredicates(AHashMap<Property<'data>, PropertyPredicate>)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub(super) enum CompositeBooleanCondition<'data> {
	#[serde(borrow)]
	Or(Vec<Condition<'data>>),
	#[serde(borrow)]
	And(Vec<Condition<'data>>)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "Cow<'data, str>")]
#[repr(transparent)]
pub(super) struct Property<'data>(#[serde(borrow)] pub(super) Cow<'data, str>);

impl<'data> TryFrom<Cow<'data, str>> for Property<'data> {
	type Error = PropertyNameError<'data>;

	fn try_from(property: Cow<'data, str>) -> Result<Self, Self::Error> {
		if is_legal_property_name_or_value(&property) {
			Ok(Self(property))
		} else {
			Err(PropertyNameError(property))
		}
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "Cow<'_, str>")]
pub(super) struct PropertyPredicate {
	negate: bool,
	pub(super) values: TinyVec<[String; 4]>
}

impl TryFrom<Cow<'_, str>> for PropertyPredicate {
	type Error = PropertyPredicateError;

	fn try_from(values_string: Cow<'_, str>) -> Result<Self, Self::Error> {
		let mut negate = false;
		let mut is_first_value = true;
		let mut values = tiny_vec!();

		// Empty values are ignored by the game
		for mut value in values_string.split('|').filter(|value| !value.is_empty()) {
			if let (true, Some(value_without_negation)) = (is_first_value, value.strip_prefix('!')) {
				value = value_without_negation;
				negate = true;
			}

			if !is_legal_property_name_or_value(value) {
				return Err(PropertyPredicateError::InvalidValue(value.into()));
			}

			// TODO maybe optimize this to avoid clone (can be done when Cow::Borrowed, but not owned)
			values.push(value.into());

			is_first_value = false;
		}

		// Empty predicates are rejected by the game
		if values.is_empty() {
			return Err(PropertyPredicateError::Empty);
		}

		Ok(Self { negate, values })
	}
}

impl Serialize for PropertyPredicate {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		struct SeparatorWrapper<'vals>(&'vals [String]);

		impl Display for SeparatorWrapper<'_> {
			fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
				self.0
					.iter()
					.try_fold(true, |is_first, value| {
						if !is_first {
							f.write_char('|')?;
						}

						f.write_str(value)?;

						Ok(false)
					})
					.map(|_| ())
			}
		}

		serializer.collect_str(&format_args!(
			"{}{}",
			if self.negate { "!" } else { "" },
			SeparatorWrapper(&self.values)
		))
	}
}

fn is_legal_property_name_or_value(str: &str) -> bool {
	!str.is_empty()
		&& str
			.chars()
			.all(|char| matches!(char, 'a'..='z' | '0'..='9' | '_'))
}

#[derive(Debug, Error)]
#[error("The block state property name {0} is invalid")]
pub(super) struct PropertyNameError<'name>(Cow<'name, str>);

#[derive(Debug, Error)]
pub(super) enum PropertyPredicateError {
	#[error("Block state predicates cannot be empty")]
	Empty,
	#[error("The block state property value {0} is invalid")]
	InvalidValue(String)
}

#[derive(Debug, Error)]
pub(super) enum VariantPredicateError {
	#[error("The block state property name {0} is invalid")]
	InvalidPropertyName(String),
	#[error("Missing value for property {0} in variant predicate")]
	MissingValue(String),
	#[error("The block state property value {0} is invalid")]
	InvalidPropertyValue(String)
}

fn is_default<T: Default + PartialEq>(value: &T) -> bool {
	*value == T::default()
}

fn default_weight() -> NonZeroI32 {
	NonZeroI32::new(1).unwrap()
}

fn is_default_weight(weight: &NonZeroI32) -> bool {
	*weight == default_weight()
}
