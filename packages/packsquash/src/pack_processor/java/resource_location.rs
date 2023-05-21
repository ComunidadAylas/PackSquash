use super::PackType;
use crate::relative_path::InvalidPathError;
use crate::util::cow_util::{ReborrowExt, SplitOnceByColonWithDefaultPrefixExt};
use crate::RelativePath;
use serde::{Serialize, Serializer};
use std::borrow::Cow;
use std::path::Path;
use strum::IntoEnumIterator;
use thiserror::Error;

static DEFAULT_NAMESPACE: &str = "minecraft";

// TODO document that these are deserialized from and serialized to resource location strings
//      (namespace + relative asset path)
// TODO document that Eq and Hash do stupid field-based equality comparison
#[derive(Debug, PartialEq, Eq, Hash, Default, Clone)]
pub struct ResourceLocation<'location> {
	pack_type: PackType,
	namespace: Cow<'location, str>,
	asset_type_directory: Option<Cow<'location, str>>,
	relative_asset_path: Cow<'location, str>,
	asset_type_extension: Option<Cow<'location, str>>
}

#[derive(Error, Debug)]
pub enum ResourceLocationError {
	#[error(
		"Illegal character \"{0}\" found in namespace {1}. The allowed characters are: a-z0-9_.-"
	)]
	InvalidNamespaceChar(char, String),
	#[error(
		"The namespace .. is technically valid, but it can't be used by the game. Please rename it"
	)]
	IllegalDirectoryTraversalSequenceNamespace,
	#[error("Illegal character \"{0}\" found in path {1}. The allowed characters are: a-z0-9_.-/")]
	InvalidPathChar(char, String),
	#[error(
		"Potential illegal directory traversal sequence in path {0}. Please remove double-dots from it"
	)]
	InvalidPathDirectoryTraversalSequence(String),
	#[error(
		"The path {0} contains current or parent directory references, or trailing or multiple \
		slashes. Please remove them"
	)]
	UncleanPath(String),
	#[error("The path does not start with the components for any known pack type")]
	InvalidRootDirectory,
	#[error("The path does not contain a namespace component")]
	MissingNamespaceComponent
}

impl<'location> ResourceLocation<'location> {
	/// TODO docs:
	/// asset_type_directory and asset_type_extension are assumed to be valid.
	/// We only need to verify that the maybe user-provided path is valid, as
	/// concatenating that path with the asset type directory and extension
	/// does not alter its validity status
	pub fn new(
		pack_type: PackType,
		resource_location_string: impl Into<Cow<'location, str>>,
		asset_type_directory: Option<impl Into<Cow<'location, str>>>,
		asset_type_extension: Option<impl Into<Cow<'location, str>>>
	) -> Result<Self, ResourceLocationError> {
		let asset_type_directory =
			asset_type_directory.map(|asset_type_directory| asset_type_directory.into());
		let asset_type_extension =
			asset_type_extension.map(|asset_type_extension| asset_type_extension.into());

		let (namespace, relative_asset_path) = resource_location_string
			.into()
			.split_once_with_default_prefix(":", DEFAULT_NAMESPACE);

		verify_namespace(&namespace)?;
		verify_path(&relative_asset_path)?;

		Ok(Self {
			pack_type,
			namespace,
			asset_type_directory,
			relative_asset_path,
			asset_type_extension
		})
	}

	pub fn namespace(&self) -> &str {
		&self.namespace
	}

	pub fn resolved_asset_path(&self) -> Cow<'_, str> {
		if self.asset_type_directory.is_none() && self.asset_type_extension.is_none() {
			// If there is nothing to prepend (an asset type root directory) or append
			// (an asset type extension), the relative asset path matches the resolved
			// asset path, and we can borrow it
			Cow::Borrowed(&self.relative_asset_path)
		} else {
			let mut resolved_asset_path = String::new();

			self.push_resolved_asset_path_to(&mut resolved_asset_path);

			Cow::Owned(resolved_asset_path)
		}
	}

	pub fn as_relative_path(&self) -> Result<RelativePath<'static>, InvalidPathError> {
		let root_directory = self.pack_type.root_directory();
		let root_directory = root_directory.as_str();
		let namespace = &self.namespace;

		let asset_type_directory = self
			.asset_type_directory
			.as_ref()
			.unwrap_or(&Cow::Borrowed(""));

		let relative_asset_path = &self.relative_asset_path;

		let asset_type_extension = self
			.asset_type_extension
			.as_ref()
			.unwrap_or(&Cow::Borrowed(""));

		let mut path = String::with_capacity(
			root_directory.len()
				+ namespace.len()
				+ asset_type_directory.len()
				+ relative_asset_path.len()
				+ asset_type_extension.len()
				+ 4
		);

		// A resource location is a relative path with the following format:
		// {root_directory}/{namespace}/({asset_type_directory}/)?{asset_path}(.{asset_type_extension})?
		path.push_str(root_directory);
		path.push('/');
		path.push_str(namespace);
		path.push('/');
		self.push_resolved_asset_path_to(&mut path);

		if path.len() > u16::MAX as usize {
			return Err(InvalidPathError::TooBig);
		}

		// Rust strings are valid UTF-8, the checks we do guarantee that it is a straightforward
		// relative path with no weird metacharacters, and we have just checked that the length
		// is not too big
		Ok(RelativePath::from_inner(path))
	}

	pub fn into_owned(self) -> ResourceLocation<'static> {
		ResourceLocation {
			pack_type: self.pack_type,
			namespace: self.namespace.into_owned().into(),
			asset_type_directory: self
				.asset_type_directory
				.map(|asset_type_directory| asset_type_directory.into_owned().into()),
			relative_asset_path: self.relative_asset_path.into_owned().into(),
			asset_type_extension: self
				.asset_type_extension
				.map(|asset_type_extension| asset_type_extension.into_owned().into())
		}
	}

	pub fn as_owned(&self) -> ResourceLocation<'static> {
		ResourceLocation {
			pack_type: self.pack_type,
			namespace: self.namespace.clone().into_owned().into(),
			asset_type_directory: self
				.asset_type_directory
				.as_ref()
				.map(|asset_type_directory| asset_type_directory.clone().into_owned().into()),
			relative_asset_path: self.relative_asset_path.clone().into_owned().into(),
			asset_type_extension: self
				.asset_type_extension
				.as_ref()
				.map(|asset_type_extension| asset_type_extension.clone().into_owned().into())
		}
	}

	pub fn reborrow(&self) -> ResourceLocation<'_> {
		ResourceLocation {
			pack_type: self.pack_type,
			namespace: Cow::Borrowed(&self.namespace),
			asset_type_directory: self
				.asset_type_directory
				.as_ref()
				.map(|asset_type_directory| asset_type_directory.reborrow()),
			relative_asset_path: Cow::Borrowed(&self.relative_asset_path),
			asset_type_extension: self
				.asset_type_extension
				.as_ref()
				.map(|asset_type_extension| asset_type_extension.reborrow())
		}
	}

	fn push_resolved_asset_path_to(&self, resolved_asset_path: &mut String) {
		if let Some(asset_type_directory) = &self.asset_type_directory {
			if asset_type_directory.len() > 0 {
				resolved_asset_path.push_str(asset_type_directory);
				resolved_asset_path.push('/');
			}
		}

		resolved_asset_path.push_str(&self.relative_asset_path);

		if let Some(asset_type_extension) = &self.asset_type_extension {
			if asset_type_extension.len() > 0 {
				resolved_asset_path.push('.');
				resolved_asset_path.push_str(asset_type_extension);
			}
		}
	}
}

impl<'location> TryFrom<&'location RelativePath<'_>> for ResourceLocation<'location> {
	type Error = ResourceLocationError;

	fn try_from(path: &'location RelativePath<'_>) -> Result<Self, Self::Error> {
		// Resource locations are just different syntax for a subset of relative file
		// paths. Validate that the path to be converted has:
		// - A valid pack type root directory
		// - A valid namespace
		// - A valid relative asset path
		// If that validation parses, then it is a valid resource path, relative to the
		// pack type root directory (asset_type_directory is assumed to be None)

		let (path_without_root_dir, pack_type) = PackType::iter()
			.filter_map(|pack_type| {
				path.strip_prefix(&*pack_type.root_directory())
					.ok()
					.map(|path_without_root_dir| (path_without_root_dir, pack_type))
			})
			.next()
			.ok_or(ResourceLocationError::InvalidRootDirectory)?;

		let mut namespace_and_asset_path_components = path_without_root_dir.components();
		let namespace = namespace_and_asset_path_components
			.next()
			.ok_or(ResourceLocationError::MissingNamespaceComponent)?
			.as_os_str()
			.to_str()
			.unwrap()
			.into();

		verify_namespace(&namespace)?;

		let relative_asset_path = namespace_and_asset_path_components
			.as_path()
			.as_os_str()
			.to_str()
			.unwrap()
			.into();

		verify_path(&relative_asset_path)?;

		Ok(Self {
			pack_type,
			namespace,
			asset_type_directory: None,
			relative_asset_path,
			asset_type_extension: None
		})
	}
}

impl Serialize for ResourceLocation<'_> {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		if self.namespace == DEFAULT_NAMESPACE {
			serializer.serialize_str(&self.relative_asset_path)
		} else {
			serializer.collect_str(&format_args!(
				"{}:{}",
				self.namespace, self.relative_asset_path
			))
		}
	}
}

fn verify_namespace(namespace: impl AsRef<str>) -> Result<(), ResourceLocationError> {
	let namespace = namespace.as_ref();

	// Minecraft trivially concatenates strings together, including the namespace, to
	// form paths. Even though it is valid, the ".." namespace may be interpreted as
	// going up one directory and does not work as expected in any case
	if namespace == ".." {
		return Err(ResourceLocationError::IllegalDirectoryTraversalSequenceNamespace);
	}

	namespace.chars().try_for_each(|char| {
		// Vanilla verification is at net.minecraft.resources.ResourceLocation#isValidNamespace
		matches!(char, 'a'..='z' | '0'..='9' | '_' | '-' | '.')
			.then_some(())
			.ok_or_else(|| ResourceLocationError::InvalidNamespaceChar(char, namespace.into()))
	})
}

fn verify_path(path: impl AsRef<str>) -> Result<(), ResourceLocationError> {
	let path = path.as_ref();

	// Vanilla verification is at net.minecraft.resources.ResourceLocation#isValidPath.
	// However, there are three non-obvious gotchas that are not considered by the method
	// above:
	// - net.minecraft.server.packs.resources.FallbackResourceManager refuses to yield
	//   files when their resource location contains the ".." path traversal sequence.
	//   This resource manager is used by the rest of resource managers, effectively
	//   turning any resource location that contains ".." anywhere in its path illegal.
	// - Double slashes in the path might be okay when using the file pack provider (i.e.,
	//   when the pack is not read from a ZIP), but they definitely break when they are
	//   in a ZIP, because paths in ZIP files are not stored with these double slashes,
	//   and AFAIK there is no code that normalizes them.
	// - Similarly, leading and trailing slashes may be troublesome. The Minecraft ZIP
	//   parsing code is smart enough to look up entries by their name and their name with
	//   a slash appended, but it doesn't bother removing trailing or leading slashes, so
	//   it's best to avoid them.
	// - In a similar vein, ./ sequences are troublesome too, because ZIP files store
	//   normalized paths.
	// Start our verification by ensuring that the path is clean (i.e., free from the
	// gotchas stated above)
	let cleaned_path = path_clean::clean(path);
	if cleaned_path != Path::new(path) || cleaned_path == Path::new(".") {
		return Err(ResourceLocationError::UncleanPath(path.to_string()));
	}

	// We have a clean path now. Restrict it further according to what the game expects.
	// It may contain leading parent directory traversal sequences such as ../../file,
	// but that will be rejected because both us and the game outright forbid double dots
	let mut previous_char_was_dot = false;
	for char in path.chars() {
		let is_dot = char == '.';

		if is_dot && previous_char_was_dot {
			return Err(ResourceLocationError::InvalidPathDirectoryTraversalSequence(path.into()));
		}

		previous_char_was_dot = is_dot;

		let is_valid =
			char.is_ascii_lowercase()
				|| char.is_ascii_digit()
				|| char == '_' || char == '-'
				|| char == '/' || is_dot;

		if !is_valid {
			return Err(ResourceLocationError::InvalidPathChar(char, path.into()));
		}
	}

	Ok(())
}
