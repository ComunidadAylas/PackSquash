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

pub trait PatriciaSetContainsRelativePathExt {
	fn has_relative_path(&self, relative_path: &RelativePath) -> bool;
}

impl PatriciaSetContainsRelativePathExt for PatriciaSet {
	fn has_relative_path(&self, relative_path: &RelativePath) -> bool {
		self.contains(relative_path.as_str())
	}
}
