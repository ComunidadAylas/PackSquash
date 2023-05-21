use crate::java::resource_location::ResourceLocation;
use ahash::AHashMap;
use onig::{Regex, RegexOptions, Syntax};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;
use tinyvec::TinyVec;

#[derive(Deserialize, Serialize, Debug)]
pub struct ResourceFilterSection<'data> {
	#[serde(rename = "filter")]
	pub block_filters: ResourceFilterBlockList<'data>,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResourceFilterBlockList<'data> {
	#[serde(borrow)]
	block: TinyVec<[ResourceBlockFilter<'data>; 2]>,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

impl<'data> Deref for ResourceFilterBlockList<'data> {
	type Target = [ResourceBlockFilter<'data>];

	fn deref(&self) -> &Self::Target {
		&self.block
	}
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(try_from = "ResourceBlockFilterObject<'_>")]
pub struct ResourceBlockFilter<'data> {
	#[serde(skip_serializing)]
	namespace_regex: Option<Regex>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	namespace: Option<Cow<'data, str>>,
	#[serde(skip_serializing)]
	path_regex: Option<Regex>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	path: Option<Cow<'data, str>>
}

impl ResourceBlockFilter<'_> {
	pub fn matches_resource_location(&self, resource_location: &ResourceLocation<'_>) -> bool {
		// The Minecraft code builds a predicate whose truth result is the regex match result,
		// then AND's them together. Missing regexes fall back to a tautology.
		// See net.minecraft.server.packs.resources.ResourceFilterSection#ResourceLocationPattern
		self.namespace_regex.as_ref().map_or_else(
			|| true,
			|namespace_regex| namespace_regex.is_match(resource_location.namespace())
		) && self.path_regex.as_ref().map_or_else(
			|| true,
			|path_regex| path_regex.is_match(&resource_location.resolved_asset_path())
		)
	}
}

#[derive(Deserialize, Debug)]
struct ResourceBlockFilterObject<'data> {
	#[serde(borrow)]
	namespace: Option<Cow<'data, str>>,
	#[serde(borrow)]
	path: Option<Cow<'data, str>>
}

impl<'data> TryFrom<ResourceBlockFilterObject<'data>> for ResourceBlockFilter<'data> {
	type Error = onig::Error;

	fn try_from(value: ResourceBlockFilterObject<'data>) -> Result<Self, Self::Error> {
		// The Minecraft code calls Pattern.compile on the regexes without any options to build
		// the matcher predicates. See net.minecraft.util.ExtraCodecs#PATTERN and its usage by
		// net.minecraft.server.packs.resources.ResourceFilterSection#ResourceLocationPattern

		let namespace_regex = value
			.namespace
			.as_ref()
			.map(|namespace| {
				Regex::with_options(namespace, RegexOptions::REGEX_OPTION_NONE, Syntax::java())
			})
			.transpose()?;

		let path_regex = value
			.path
			.as_ref()
			.map(|path| Regex::with_options(path, RegexOptions::REGEX_OPTION_NONE, Syntax::java()))
			.transpose()?;

		Ok(Self {
			namespace_regex,
			namespace: value.namespace,
			path_regex,
			path: value.path
		})
	}
}
