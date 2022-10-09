use crate::java::resource_location::ResourceLocation;
use onig::{Regex, RegexOptions, Syntax};
use serde::Deserialize;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::ops::Deref;
use tinyvec::TinyVec;

#[derive(Deserialize, Debug)]
pub struct ResourceFilterSection<'data> {
	#[serde(borrow)]
	#[serde(rename = "filter")]
	pub block_filters: ResourceFilterBlockList<'data>
}

#[derive(Deserialize, Debug)]
pub struct ResourceFilterBlockList<'data> {
	#[serde(borrow)]
	block: TinyVec<[ResourceBlockFilter<'data>; 2]>
}

impl<'data> Deref for ResourceFilterBlockList<'data> {
	type Target = [ResourceBlockFilter<'data>];

	fn deref(&self) -> &Self::Target {
		&self.block
	}
}

#[derive(Deserialize, Debug, Default)]
#[serde(try_from = "RawResourceBlockFilter<'data>")]
pub struct ResourceBlockFilter<'data> {
	namespace_regex: Option<Regex>,
	path_regex: Option<Regex>,
	#[serde(borrow)]
	try_from_lifetime_marker: PhantomData<&'data ()>
}

impl ResourceBlockFilter<'_> {
	pub fn matches_resource_location(&self, resource_location: &ResourceLocation<'_>) -> bool {
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
struct RawResourceBlockFilter<'data> {
	#[serde(borrow)]
	namespace: Option<Cow<'data, str>>,
	#[serde(borrow)]
	path: Option<Cow<'data, str>>
}

impl<'data> TryFrom<RawResourceBlockFilter<'data>> for ResourceBlockFilter<'data> {
	type Error = onig::Error;

	fn try_from(value: RawResourceBlockFilter<'data>) -> Result<Self, Self::Error> {
		let namespace_regex = value
			.namespace
			.map(|namespace| {
				Regex::with_options(&namespace, RegexOptions::REGEX_OPTION_NONE, Syntax::java())
			})
			.transpose()?;

		let path_regex = value
			.path
			.map(|path| Regex::with_options(&path, RegexOptions::REGEX_OPTION_NONE, Syntax::java()))
			.transpose()?;

		Ok(Self {
			namespace_regex,
			path_regex,
			..Default::default()
		})
	}
}
