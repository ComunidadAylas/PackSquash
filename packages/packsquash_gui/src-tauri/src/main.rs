#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod optimization_progress_logger;

use crate::optimization_progress_logger::OptimizationProgressLogger;
use packsquash::VirtualFileSystemType;
use packsquash_options::SquashOptions;
use path_absolutize::Absolutize;
use std::borrow::Cow;
use std::path::Path;
use std::time::Duration;
use std::{env, fs};
use tauri::{Runtime, Window};
use tauri_plugin_store::StoreBuilder;

fn main() {
	tauri::Builder::default()
		.plugin(
			tauri_plugin_store::PluginBuilder::default()
				.store(StoreBuilder::new("settings.json".into()).build())
				.build()
		)
		.setup(|app| {
			tauri::updater::builder(app.handle()).timeout(Duration::from_secs(30));

			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
			app_build_version,
			app_build_date,
			app_build_target_triple,
			app_build_profile,
			is_plausible_pack_directory,
			init_optimization_progress_logger,
			run_packsquash,
			absolutize_path,
			get_parent_path,
			set_working_directory
		])
		.run(tauri::generate_context!())
		.expect("Could not launch application")
}

#[tauri::command]
fn app_build_version() -> &'static str {
	packsquash::BUILD_VERSION
}

#[tauri::command]
fn app_build_date() -> &'static str {
	packsquash::BUILD_DATE
}

#[tauri::command]
fn app_build_target_triple() -> &'static str {
	packsquash::CARGO_TARGET_TRIPLE
}

#[tauri::command]
fn app_build_profile() -> &'static str {
	packsquash::CARGO_PROFILE
}

#[tauri::command]
fn is_plausible_pack_directory(path: &Path) -> bool {
	fs::metadata(path.join("pack.mcmeta")).map_or_else(|_| false, |metadata| !metadata.is_dir())
}

#[tauri::command]
fn init_optimization_progress_logger<R: Runtime>(window: Window<R>) {
	OptimizationProgressLogger::init(window).ok();
}

#[tauri::command(async)]
fn run_packsquash(options: SquashOptions) -> Result<(), String> {
	packsquash::run(&options, VirtualFileSystemType::OsFilesystem).map_err(|err| err.to_string())
}

#[tauri::command]
fn absolutize_path(path: &Path) -> Cow<'_, Path> {
	path.absolutize().unwrap_or(Cow::Borrowed(path))
}

#[tauri::command]
fn get_parent_path(path: &Path) -> &Path {
	path.parent().unwrap_or(path)
}

#[tauri::command]
fn set_working_directory(path: &Path) {
	env::set_current_dir(path).ok();
}
