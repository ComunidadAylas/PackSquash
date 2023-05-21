//! Deserializer and serializer functions for `EnumMap`s of `Option<V>` values
//! that slightly tweak the default deserialization and serialization of these
//! `EnumMap`s.
//!
//! Inspired by the upstream source code at
//! https://github.com/xfix/enum-map/blob/085679ed8bf279e4d773f58b48a12ceb7e76bf72/enum-map/src/serde.rs

use std::{fmt, marker::PhantomData};

use enum_map::{EnumArray, EnumMap};
use serde::{de, de::MapAccess, Deserialize, Deserializer, Serialize, Serializer};

/// Serializer function for `EnumMap`s of `Option<V>` values, usable with Serde's
/// `serialize_with` field attribute, that does not serialize entries with a `None`
/// value.
pub fn enum_map_skip_none_values_serializer<
	S: Serializer,
	K: EnumArray<Option<V>> + Serialize,
	V: Serialize
>(
	enum_map: &EnumMap<K, Option<V>>,
	serializer: S
) -> Result<S::Ok, S::Error> {
	serializer.collect_map(enum_map.iter().filter(|(_, v)| v.is_some()))
}

/// Deserializer function for `EnumMap`s of `Option<V>` values, usable with Serde's
/// `deserialize_with` field attribute, that leaves missing entries set to their
/// default value, `None`.
pub fn enum_map_ignore_missing_values_deserializer<
	'de,
	D: Deserializer<'de>,
	K: EnumArray<Option<V>> + Deserialize<'de>,
	V: Deserialize<'de>
>(
	deserializer: D
) -> Result<EnumMap<K, Option<V>>, D::Error> {
	struct MapVisitor<K, V>(PhantomData<(K, V)>);

	impl<'de, K: EnumArray<Option<V>> + Deserialize<'de>, V: Deserialize<'de>> de::Visitor<'de>
		for MapVisitor<K, V>
	{
		type Value = EnumMap<K, Option<V>>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			write!(formatter, "a map")
		}

		fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
			// Initialize the map to None values
			let mut map = EnumMap::default();

			while let Some((key, value)) = access.next_entry()? {
				// This accepts None as a valid value
				map[key] = value;
			}

			Ok(map)
		}
	}

	deserializer.deserialize_map(MapVisitor(PhantomData))
}
