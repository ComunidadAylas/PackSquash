use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use packsquash::PackSquashStatus;
use tauri::{Runtime, Window};

pub struct OptimizationProgressLogger<R: Runtime> {
	window: Window<R>
}

impl<R: Runtime> OptimizationProgressLogger<R> {
	pub fn init(window: Window<R>) -> Result<(), SetLoggerError> {
		log::set_boxed_logger(Box::new(OptimizationProgressLogger { window }))
			.map(|_| log::set_max_level(LevelFilter::Trace))
	}
}

impl<R: Runtime> Log for OptimizationProgressLogger<R> {
	fn enabled(&self, metadata: &Metadata) -> bool {
		// For our purposes, the target matches the module path the message came from
		metadata.target().starts_with("packsquash::")
	}

	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let mut status_type = 0.into();
		let status_type = record
			.key_values()
			.get("status_type".into())
			.and_then(|value| {
				status_type = value; // Extend value lifetime
				status_type.downcast_ref::<PackSquashStatus>()
			});

		match status_type {
			Some(PackSquashStatus::PackFileCount { count }) => {
				self.window.emit("pack_file_count", count).ok();
			}
			Some(PackSquashStatus::DetectedPackType { pack_type }) => {
				let game_version_range = record
					.key_values()
					.get("game_version_range".into())
					.unwrap();

				self.window
					.emit(
						"detected_pack_type",
						(pack_type.to_string(), game_version_range.to_string())
					)
					.ok();
			}
			Some(PackSquashStatus::ProcessedAsset { strategy, warnings }) => {
				let asset_path = record.key_values().get("asset_path".into()).unwrap();
				let asset_path = asset_path.to_borrowed_str().unwrap();

				self.window
					.emit(
						"processed_asset",
						(asset_path, format!("{:?}", strategy), warnings)
					)
					.ok();
			}
			_ => ()
		}
	}

	fn flush(&self) {
		// Nothing to flush
	}
}
