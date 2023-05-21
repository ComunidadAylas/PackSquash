use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use tinyvec::ArrayVec;

use crate::pack_processor::java::{
	resource_location::{ResourceLocation, ResourceLocationError},
	PackType
};

/// The Minecraft client builds a long integer mask with the set of feature flags,
/// so the maximum number of flags is 64.
const MAXIMUM_FEATURE_FLAGS: usize = i64::BITS as usize;

#[derive(Deserialize, Serialize, Debug)]
#[serde(try_from = "ArrayVec<[Cow<'_, str>; MAXIMUM_FEATURE_FLAGS]>")]
pub struct FeatureFlagsSection<'data> {
	#[serde(rename = "enabled")]
	#[serde(borrow)]
	pub enabled_flags: ArrayVec<[ResourceLocation<'data>; MAXIMUM_FEATURE_FLAGS]>
}

impl<'data> TryFrom<ArrayVec<[Cow<'data, str>; MAXIMUM_FEATURE_FLAGS]>>
	for FeatureFlagsSection<'data>
{
	type Error = ResourceLocationError;

	fn try_from(
		value: ArrayVec<[Cow<'data, str>; MAXIMUM_FEATURE_FLAGS]>
	) -> Result<Self, Self::Error> {
		Ok(FeatureFlagsSection {
			enabled_flags: value
				.into_iter()
				.map(|resource_location| {
					ResourceLocation::new(
						PackType::DataPack,
						resource_location,
						None::<&str>,
						None::<&str>
					)
				})
				.collect::<Result<_, Self::Error>>()?
		})
	}
}
