//! A Minecraft resource and data pack optimizer that aims to achieve the best possible compression,
//! which allows for efficient distribution and slightly improved load times in the game, at good speed.

#![allow(incomplete_features)]
#![deny(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![feature(const_option)]
#![feature(doc_cfg)]
#![feature(map_entry_replace)]
#![feature(try_find)]
#![feature(type_alias_impl_trait)]
#![feature(stmt_expr_attributes)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(impl_trait_in_assoc_type)]
#![feature(concat_idents)]
#![doc(
	html_logo_url = "https://user-images.githubusercontent.com/7822554/243825966-500faa74-de2f-462a-8f76-3a122b69856e.png"
)]
// TODO review all source and document things
//#![deny(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

macro_rules! status_log {
	($level:ident, $message_type:ident $( , $key:tt = $value:expr )*) => {
		::log::$level!(status_type = ::log::kv::Value::capture_display(&$crate::PackSquashStatus::$message_type) $( , $key = $value )*; "")
	};
	($level:ident, $message_type:ident { $( $initializer_key:tt $( : $initializer_value:expr )? ),* } $( , $key:tt = $value:expr )*) => {
		::log::$level!(
			status_type = ::log::kv::Value::capture_display(&$crate::PackSquashStatus::$message_type {
				$( $initializer_key $( : $initializer_value )? ),*
			})
			$( , $key = $value )*; ""
		)
	}
}

macro_rules! status_log_macro {
	($level:ident, $macro_name:ident) => {
		status_log_macro!($level, $macro_name, $);
	};
	($level:ident, $macro_name:ident, $d:tt) => {
		macro_rules! $macro_name {
			($message_type:ident $d ( , $d key:tt = $d value:expr )*) => {
				status_log!($level, $message_type $d ( , $d key = $d value )*)
			};
			($message_type:ident { $d ( $d initializer_key:tt $d ( : $d initializer_value:expr )? ),* } $d ( , $d key:tt = $d value:expr )*) => {
				status_log!($level, $message_type { $d ( $d initializer_key $d ( : $d initializer_value )? ),* } $d ( , $d key = $d value )* )
			};
		}
	};
}

status_log_macro!(warn, status_warn);
status_log_macro!(info, status_info);
status_log_macro!(trace, status_trace);

pub mod config;
mod util;
mod vfs;

#[macro_use]
mod minecraft_version;
mod pack_processor;
mod relative_path;
mod scratch_file;
mod squash_zip;
mod squashed_pack_state;

use crate::config::SquashOptions;
use crate::pack_processor::java;
use crate::squash_zip::PreviousZipParseError;
pub use crate::vfs::VirtualFileSystemType;
use relative_path::RelativePath;
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, BufReader, ErrorKind};
use thiserror::Error;
use tinyvec::TinyVec;

#[derive(Error, Debug)]
pub enum PackSquashError {
	#[error("{0}")]
	JavaPackError(#[from] java::PackError)
}

#[derive(Debug)]
pub enum PackSquashStatus {
	PackFileCount {
		count: usize
	},
	DetectedPackType {
		pack_type: java::PackType
	},
	AssetProcessorsToRun {
		asset_processors_list: String
	},
	QuirksToWorkAround {
		quirk_list: String
	},
	UnusablePreviousZip {
		previous_zip_parse_error: Option<PreviousZipParseError>,
		io_error: Option<io::Error>
	},
	ProcessedAsset {
		strategy: PackSquashAssetProcessingStrategy,
		warnings: TinyVec<[Cow<'static, str>; 2]>
	},
	FinishingZip,
	SystemIdHasLowEntropy,
	SystemIdIsVolatile
}

impl Display for PackSquashStatus {
	fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
		// We won't and shouldn't display this enum directly to end users, but
		// our (ab)use of the log crate structured logging API requires either
		// this or Debug to be implemented able to capture an strongly typed
		// instance of this enum. What's formatted here doesn't matter, so do
		// nothing
		Ok(())
	}
}

#[derive(Debug)]
pub enum PackSquashAssetProcessingStrategy {
	ValidatedAndMinified,
	ValidatedDebloatedAndMinified,
	ValidatedAndPrettified,
	ValidatedDebloatedAndPrettified,
	// For better style, keep these generic strategies last
	Optimized
}

pub fn run(options: &SquashOptions, vfs_type: VirtualFileSystemType) -> Result<(), PackSquashError> {
	java::pack_processor::PackProcessor::new().process(
		|| match File::open(&options.global_options.output_file_path) {
			Ok(file) => Some(BufReader::new(file)),
			Err(err) if err.kind() == ErrorKind::NotFound => None,
			Err(err) => {
				// The file could not be opened, but it exists. This has potential performance
				// implications, so warn about that, but continue as if the file did not exist

				status_warn!(UnusablePreviousZip {
					previous_zip_parse_error: None,
					io_error: Some(err)
				});

				None
			}
		},
		vfs_type,
		options
	)?;

	Ok(())
}
