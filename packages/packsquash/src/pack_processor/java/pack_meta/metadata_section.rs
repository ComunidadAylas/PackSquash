use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Deserialize, Serialize, Debug)]
pub struct MetadataSection<'data> {
	#[serde(rename = "pack_format")]
	pub pack_format_version: i32,
	// This is interpreted as a Minecraft text component, parsed by the
	// static class Serializer at net.minecraft.network.chat.Component
	#[serde(borrow)]
	pub description: ChatComponent<'data>,
	#[serde(flatten, borrow)]
	pub(super) bloat_fields: AHashMap<Cow<'data, str>, ijson::IValue>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ChatComponent<'data> {
	#[serde(borrow)]
	Literal(Cow<'data, str>),
	FormattedText(ijson::IValue)
}
