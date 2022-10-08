use crate::RelativePath;
use patricia_tree::PatriciaSet;
use std::iter::Map;

pub trait PatriciaSetRelativePathIterExt<'this> {
	type Iterator: Iterator<Item = RelativePath<'static>>;

	fn relative_path_iter(&'this self) -> Self::Iterator;
}

impl<'this> PatriciaSetRelativePathIterExt<'this> for PatriciaSet {
	type Iterator =
		Map<patricia_tree::set::Iter<'this>, impl FnMut(Vec<u8>) -> RelativePath<'static>>;

	fn relative_path_iter(&'this self) -> Self::Iterator {
		self.iter().map(|raw_relative_path| {
			RelativePath::from_inner(String::from_utf8(raw_relative_path).unwrap())
		})
	}
}
