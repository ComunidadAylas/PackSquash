//! Implements a linear regression model for calculating a suitable number of Zopfli compression iterations,
//! given the size of the data to be compressed and a constant target time to compress 1 MiB of such data.
//! The size of the data is adjusted using a non-linear function to get a data magnitude, whose order is that
//! of a fractional power, so while bigger data is still compressed less to meet the target time, the model is
//! a bit hesitant to do so and still overshoots the target time for bigger files, although not by much in
//! practical scenarios. This is better than using linear functions exclusively, because such functions will
//! sacrifice lots of compression to precisely meet the target time.
//!
//! Mathematically, the model is defined as follows. Let `A` be the slope of a linear regression function
//! that is meant to estimate the relative time that it takes to compress 64 KiB of somewhat difficult to
//! compress data with a single Zopfli compression iteration. Let `B` be the intercept of such function.
//! The relative time `T` it takes to compress S 64 KiB blocks of data with `I` Zopfli compression
//! iterations is `T = (A * I + B) * S`.

/// Slope of the linear regression function that estimates the Zopfli compression
/// time for a 64 KiB block of somewhat difficult to compress data.
const A: f32 = 0.004381402;
/// Intercept of the linear regression function described in [`A`].
const B: f32 = 0.035055663;

#[derive(Debug)]
pub(crate) struct ZopfliIterationsTimeModel {
	target_compression_time: f32,
	magnitude_power: f32
}

impl ZopfliIterationsTimeModel {
	/// Creates a new linear regression model to predict the relative time it will take to compress
	/// data with some number of Zopfli iterations.
	///
	/// The `one_mib_iterations` parameter specifies the number of iterations that the model will
	/// return for data of 1 MiB of magnitude, tuning the model compression-speed tradeoff. The
	/// `magnitude_power` sets the fractional power that will be used to get a magnitude from the
	/// size, so a value of 1 is equivalent to an identity function.
	pub(crate) fn new(one_mib_iterations: u8, magnitude_power: f32) -> Self {
		Self {
			target_compression_time: (A * one_mib_iterations as f32 + B) * 16.0,
			magnitude_power
		}
	}

	/// Computes an optimum number of Zopfli iterations for a data block whose size is known in
	/// advance, trying to loosely match the time it would take to compress 1 MiB of data, as
	/// configured in `new()`. The data is assumed to follow a fixed distribution, somewhere between
	/// a standard distribution (i.e. the data is random and incompressible) and a distribution that
	/// always yields zero bytes (i.e. the data is all zeroes and trivial to compress), biasing towards
	/// the standard distribution (i.e. the data is somewhat difficult to compress).
	pub(crate) fn iterations_for_data_size(
		&self,
		size: u32,
		min_iterations: u8,
		max_iterations: u8
	) -> u8 {
		let data_magnitude = (size as f64 / 65536.0).powf(self.magnitude_power as f64) as f32;

		((self.target_compression_time - B * data_magnitude) / (A * data_magnitude))
			.clamp(min_iterations as f32, max_iterations as f32)
			.round() as u8
	}
}
