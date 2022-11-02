use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::{
	Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive
};
use std::str::FromStr;

#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct MinecraftVersion {
	major: u8,
	minor: u8,
	patch: u8
}

impl MinecraftVersion {
	pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
		Self {
			major,
			minor,
			patch
		}
	}
}

impl FromStr for MinecraftVersion {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut version_components = [0; 3];
		let mut version_component_count = 0;

		for (component_str, version_component) in s.splitn(3, '.').zip(version_components.iter_mut())
		{
			// The stdlib unsigned integer parser accepts a single '+' character
			// in the beginning, and padding zeros at the left. Forbid those
			if component_str.chars().any(|c| !c.is_ascii_digit())
				|| (component_str.len() > 1 && component_str.starts_with('0'))
			{
				return Err("-".parse::<u8>().unwrap_err());
			}

			*version_component = component_str.parse::<u8>()?;
			version_component_count += 1;
		}

		if version_component_count < 2 {
			// Require both major and minor versions to be specified
			Err("".parse::<u8>().unwrap_err())
		} else {
			Ok(Self {
				major: version_components[0],
				minor: version_components[1],
				patch: version_components[2]
			})
		}
	}
}

impl Display for MinecraftVersion {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
	}
}

#[macro_export]
macro_rules! minecraft_version {
	($major:expr , $minor:expr , $patch:expr) => {
		$crate::MinecraftVersion::new($major, $minor, $patch)
	};
	($major:expr , $minor:expr) => {
		minecraft_version!($major, $minor, 0)
	};
}

#[derive(Debug, Clone)]
pub enum MinecraftVersionRange {
	Range(Range<MinecraftVersion>),
	RangeFrom(RangeFrom<MinecraftVersion>),
	RangeFull(RangeFull),
	RangeInclusive(RangeInclusive<MinecraftVersion>),
	RangeTo(RangeTo<MinecraftVersion>),
	RangeToInclusive(RangeToInclusive<MinecraftVersion>)
}

impl RangeBounds<MinecraftVersion> for MinecraftVersionRange {
	fn start_bound(&self) -> Bound<&MinecraftVersion> {
		match self {
			Self::Range(range) => range.start_bound(),
			Self::RangeFrom(range) => range.start_bound(),
			Self::RangeFull(range) => range.start_bound(),
			Self::RangeInclusive(range) => range.start_bound(),
			Self::RangeTo(range) => range.start_bound(),
			Self::RangeToInclusive(range) => range.start_bound()
		}
	}

	fn end_bound(&self) -> Bound<&MinecraftVersion> {
		match self {
			Self::Range(range) => range.end_bound(),
			Self::RangeFrom(range) => range.end_bound(),
			Self::RangeFull(range) => range.end_bound(),
			Self::RangeInclusive(range) => range.end_bound(),
			Self::RangeTo(range) => range.end_bound(),
			Self::RangeToInclusive(range) => range.end_bound()
		}
	}
}

impl From<Range<MinecraftVersion>> for MinecraftVersionRange {
	fn from(range: Range<MinecraftVersion>) -> Self {
		Self::Range(range)
	}
}

impl From<RangeFrom<MinecraftVersion>> for MinecraftVersionRange {
	fn from(range: RangeFrom<MinecraftVersion>) -> Self {
		Self::RangeFrom(range)
	}
}

impl From<RangeFull> for MinecraftVersionRange {
	fn from(range: RangeFull) -> Self {
		Self::RangeFull(range)
	}
}

impl From<RangeInclusive<MinecraftVersion>> for MinecraftVersionRange {
	fn from(range: RangeInclusive<MinecraftVersion>) -> Self {
		Self::RangeInclusive(range)
	}
}

impl From<RangeTo<MinecraftVersion>> for MinecraftVersionRange {
	fn from(range: RangeTo<MinecraftVersion>) -> Self {
		Self::RangeTo(range)
	}
}

impl From<RangeToInclusive<MinecraftVersion>> for MinecraftVersionRange {
	fn from(range: RangeToInclusive<MinecraftVersion>) -> Self {
		Self::RangeToInclusive(range)
	}
}

impl Display for MinecraftVersionRange {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Range(range) => write!(f, ">= {}, < {}", range.start, range.end),
			Self::RangeFrom(range) => write!(f, ">= {}", range.start),
			Self::RangeFull(_) => f.write_str("any"),
			Self::RangeInclusive(range) => write!(f, ">= {}, <= {}", range.start(), range.end()),
			Self::RangeTo(range) => write!(f, "< {}", range.end),
			Self::RangeToInclusive(range) => write!(f, "<= {}", range.end)
		}
	}
}
