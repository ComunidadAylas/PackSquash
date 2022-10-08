use std::ops::{Bound, RangeBounds};

pub trait RangeBoundsIntersectExt<T: ?Sized> {
	fn intersects(&self, other: &impl RangeBounds<T>) -> bool;
}

impl<T: RangeBounds<U>, U: ?Sized + PartialOrd> RangeBoundsIntersectExt<U> for T {
	fn intersects(&self, other: &impl RangeBounds<U>) -> bool {
		let other_starts_before_or_at_self_end_bound = match (other.start_bound(), self.end_bound()) {
			(Bound::Included(other_start_bound), Bound::Included(self_end_bound)) => {
				other_start_bound <= self_end_bound
			}
			(Bound::Included(other_start_bound), Bound::Excluded(self_end_bound)) => {
				other_start_bound < self_end_bound
			}
			(Bound::Unbounded, _) | (_, Bound::Unbounded) => true,
			// Start bounds of Rust ranges are either unbounded or inclusive, never exclusive
			(Bound::Excluded(_), _) => unimplemented!()
		};

		let other_ends_after_or_at_self_start_bound = match (other.end_bound(), self.start_bound()) {
			(Bound::Included(other_end_bound), Bound::Included(self_start_bound)) => {
				other_end_bound >= self_start_bound
			}
			(Bound::Excluded(other_end_bound), Bound::Included(self_start_bound)) => {
				other_end_bound > self_start_bound
			}
			(Bound::Unbounded, _) | (_, Bound::Unbounded) => true,
			(_, Bound::Excluded(_)) => unimplemented!()
		};

		// Properties:
		// - Intersection is commutative: if self intersects other, other intersects self.
		// - An empty range intersects with any other range.
		// - Completely unbounded ranges intersect.
		// - If both ranges are non-empty and bounded, the other range should start before self
		//   ends, and end after it starts.
		is_range_empty(self)
			|| is_range_empty(other)
			|| (other_starts_before_or_at_self_end_bound && other_ends_after_or_at_self_start_bound)
	}
}

fn is_range_empty<T: ?Sized + PartialOrd>(range: &impl RangeBounds<T>) -> bool {
	match (range.start_bound(), range.end_bound()) {
		(Bound::Included(start_bound), Bound::Excluded(end_bound)) => start_bound >= end_bound,
		(Bound::Included(start_bound), Bound::Included(end_bound)) => start_bound > end_bound,
		_ => false
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn intersects_works() {
		for (self_range, other_range, expected_result) in [
			(1..4, 0..5, true),
			(4..20, 0..5, true),
			(-5..3, 0..5, true),
			(0..5, 5..10, false)
		] {
			assert_eq!(self_range.intersects(&other_range), expected_result);
			assert_eq!(other_range.intersects(&self_range), expected_result);
		}

		for (self_range, other_range, expected_result) in [(..20, 0..5, true), (..0, 0..5, false)] {
			assert_eq!(self_range.intersects(&other_range), expected_result);
			assert_eq!(other_range.intersects(&self_range), expected_result);
		}

		assert!((..).intersects(&(0..5)));
		assert!((0..5).intersects(&(..)));

		for (self_range, other_range, expected_result) in [(-1..=0, 0..5, true), (4..=5, 0..5, true)]
		{
			assert_eq!(self_range.intersects(&other_range), expected_result);
			assert_eq!(other_range.intersects(&self_range), expected_result);
		}

		for (self_range, other_range, expected_result) in [(..3, 0..5, true), (..0, 0..5, false)] {
			assert_eq!(self_range.intersects(&other_range), expected_result);
			assert_eq!(other_range.intersects(&self_range), expected_result);
		}

		assert!((..=0).intersects(&(0..5)));
		assert!((0..5).intersects(&(0..=0)));

		assert!((0..5).intersects(&(-1..-1)));
		assert!((-1..-1).intersects(&(0..5)));
	}
}
