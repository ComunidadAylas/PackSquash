use std::ops::{Bound, RangeBounds};

pub trait RangeBoundsOnlyExt<T: ?Sized> {
	fn only(&self) -> Option<&T>;
}

// TODO remove this util file if it is not used after all
impl<T: RangeBounds<U>, U: ?Sized + PartialEq> RangeBoundsOnlyExt<U> for T {
	fn only(&self) -> Option<&U> {
		match (self.start_bound(), self.end_bound()) {
			(Bound::Included(start), Bound::Included(end)) => (start == end).then_some(start),
			// The end bound may be the successor in the partial order to the start bound, and
			// thus the range may effectively contain a single item. However, there's no good
			// way to check if the end bound is the successor of the start bound (the Step trait
			// is unstable and does not work well for our purposes), so consider that never happens
			(Bound::Included(_), Bound::Excluded(_)) => None,
			(Bound::Unbounded, _) | (_, Bound::Unbounded) => None,
			// Start bounds of Rust ranges are either unbounded or inclusive, never exclusive
			(Bound::Excluded(_), _) => unimplemented!()
		}
	}
}
