use std::{borrow::Cow, ops::Deref};

use ahash::AHashMap;
use compact_str::format_compact;
use serde::{Deserialize, Serialize};

use crate::pack_processor::java::{resource_location::ResourceLocation, PackType};

#[derive(Deserialize, Serialize, Debug)]
pub struct LanguageSection<'data> {
	#[serde(flatten, borrow)]
	pub language_info_map: AHashMap<LanguageCode<'data>, LanguageInfo<'data>>,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Hash, Debug)]
#[serde(try_from = "Cow<'_, str>")]
pub struct LanguageCode<'data>(Cow<'data, str>);

impl<'data> TryFrom<Cow<'data, str>> for LanguageCode<'data> {
	type Error = &'static str;

	fn try_from(value: Cow<'data, str>) -> Result<Self, Self::Error> {
		// Prefix the path with an empty resource location namespace to prevent the path
		// from being considered valid when it contains :
		// The 16 characters limit is imposed by Minecraft
		if value.len() <= 16
			&& ResourceLocation::new(
				PackType::ResourcePack,
				format_compact!(":{value}"),
				None::<&str>,
				None::<&str>
			)
			.is_ok()
		{
			Ok(LanguageCode(value))
		} else {
			Err("A language code must be a valid resource location path and less than 16 characters long")
		}
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LanguageInfo<'data> {
	#[serde(borrow)]
	pub name: NonEmptyString<'data>,
	#[serde(borrow)]
	pub region: NonEmptyString<'data>,
	#[serde(rename = "bidirectional")]
	#[serde(default)]
	pub rtl: bool,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(try_from = "Cow<'_, str>")]
#[repr(transparent)]
pub struct NonEmptyString<'data>(Cow<'data, str>);

impl<'data> TryFrom<Cow<'data, str>> for NonEmptyString<'data> {
	type Error = &'static str;

	fn try_from(value: Cow<'data, str>) -> Result<Self, Self::Error> {
		if !value.is_empty() {
			Ok(NonEmptyString(value))
		} else {
			Err("Must be a non-empty string")
		}
	}
}

impl<'data> Deref for NonEmptyString<'data> {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
