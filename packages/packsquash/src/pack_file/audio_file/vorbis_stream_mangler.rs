//! Implements an OptiVorbis stream mangler to validate Ogg Vorbis files against
//! Minecraft limitations and potentially obfuscate them.
//
// The obfuscation done here is not rocket science. You've won. Please consider
// whether circumventing it is ethical and worth your time. Please be civil and
// do not circumvent it publicly just because you hold a grudge against someone
// or something. Please use your knowledge to do good things and help shape a
// better world, not just for bragging, ripping off the work of others or other
// questionable means.

use optivorbis::remuxer::ogg_to_ogg::OggVorbisStreamMangler;

pub struct ValidatingAndObfuscatingOggVorbisStreamMangler<'flag> {
	obfuscate: bool,
	too_long_for_minecraft: &'flag mut bool
}

impl<'flag> ValidatingAndObfuscatingOggVorbisStreamMangler<'flag> {
	pub fn new(obfuscate: bool, too_long_for_minecraft: &'flag mut bool) -> Self {
		Self {
			obfuscate,
			too_long_for_minecraft
		}
	}
}

impl OggVorbisStreamMangler for ValidatingAndObfuscatingOggVorbisStreamMangler<'_> {
	fn mangle_bitrates(
		&mut self,
		minimum_bitrate: i32,
		nominal_bitrate: i32,
		maximum_bitrate: i32
	) -> (i32, i32, i32) {
		if self.obfuscate {
			(-1, -1, -1)
		} else {
			(minimum_bitrate, nominal_bitrate, maximum_bitrate)
		}
	}

	fn mangle_packet_stream_serial(
		&mut self,
		stream_serial: u32,
		packet_number: usize,
		_is_last_stream_packet: bool
	) -> u32 {
		if self.obfuscate && packet_number == 0 {
			stream_serial.wrapping_add(1)
		} else {
			stream_serial
		}
	}

	fn mangle_granule_position(
		&mut self,
		calculated_granule_position: i64,
		_packet_number: usize,
		_is_header_packet: bool,
		_is_last_stream_packet: bool
	) -> i64 {
		if calculated_granule_position > i32::MAX as i64 {
			*self.too_long_for_minecraft = true;
		}

		if self.obfuscate {
			i64::MIN + calculated_granule_position
		} else {
			calculated_granule_position
		}
	}
}
