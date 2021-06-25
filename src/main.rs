//! A Minecraft resource pack optimizer that aims to achieve the best possible compression,
//! which allows for efficient distribution and slightly improved load times in the game,
//! at good speed.

#![deny(unsafe_code)]
#![feature(const_option)]
#![feature(new_uninit)]
#![feature(once_cell)]
#![feature(min_type_alias_impl_trait)]
#![feature(doc_cfg)]
#![doc(
	html_logo_url = "https://user-images.githubusercontent.com/7822554/96335786-5f403f80-107b-11eb-8aa8-d0e0b6e1aae9.png"
)]
#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::invalid_html_tags)]

mod resource_pack_file;
mod squash_zip;

fn main() -> std::io::Result<()> {
	Ok(())
}

/*use std::{convert::TryInto, time::Instant};
use std::error::Error;
use std::ffi::OsStr;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{cmp, env, fs, io, process};
use std::{convert::TryInto, time::Instant};

use enumset::EnumSet;
use indexmap::IndexMap;

use getopts::{Options, ParsingStyle};

use serde::Deserialize;

use simple_error::SimpleError;

use globset::{GlobBuilder, GlobSet, GlobSetBuilder};

use resource_pack_file::{FileSettings, InvalidSettingsForResourcePackFile, Mod, ResourcePackFile};

use micro_zip::MicroZip;
use micro_zip::ZipFileType;

use sysinfo::{RefreshKind, System, SystemExt};

lazy_static! {
	static ref EMPTY_OS_STR: &'static OsStr = OsStr::new("");
	static ref STANDARD_INPUT_STR: String = String::from("standard input");
	static ref CUSTOM_VERSION_STRING: String = {
		let cargo_package_version = env!("CARGO_PKG_VERSION");
		let mut version_string = String::with_capacity(cargo_package_version.len() + 8);

		version_string.push('v');
		version_string.push_str(cargo_package_version);
		version_string.push_str("-custom");

		version_string
	};
}

#[derive(Deserialize)]
struct AppSettings {
	resource_pack_directory: PathBuf,
	#[serde(flatten)]
	general: GeneralSettings,
	#[serde(flatten)]
	file_patterns: IndexMap<String, File>
}

#[derive(Deserialize)]
#[serde(default, deny_unknown_fields)]
struct GeneralSettings {
	skip_pack_icon: bool,
	strict_zip_spec_compliance: bool,
	compress_already_compressed_files: bool,
	ignore_system_and_hidden_files: bool,
	allowed_mods: EnumSet<Mod>,
	threads: u32,
	output_file_path: String,
	output_file_spooling_buffer_size: u64
}

impl Default for GeneralSettings {
	fn default() -> Self {
		// The "k" in "kB" here has an SI-compliant meaning (1000 and not 1024 bytes)
		let available_mem_kb =
			System::new_with_specifics(RefreshKind::new().with_memory()).get_available_memory();

		Self {
			skip_pack_icon: false,
			strict_zip_spec_compliance: true,
			compress_already_compressed_files: false,
			ignore_system_and_hidden_files: true,
			allowed_mods: EnumSet::empty(),
			threads: 0,
			output_file_path: String::from("resource_pack.zip"),
			// The buffer size is in MiB, and we want to use half the available memory
			output_file_spooling_buffer_size: available_mem_kb * 125 / 262144
		}
	}
}

#[derive(Deserialize)]
struct File {
	#[serde(flatten)]
	settings: FileSettings
}

fn main() {
	let mut options = Options::new();

	options.optflag("h", "help", "Prints information about the command line arguments accepted by this application and exits")
		.optflag("v", "version", "Prints version and copyright information of the application, then exits")
		.parsing_style(ParsingStyle::StopAtFirstFree);

	let options_parse_result = options.parse(env::args().skip(1)); // Skip program name

	let print_version_information = |long_copyright_notice| {
		println!(
			"PackSquash {} ({}) for {}",
			option_env!("VERGEN_SEMVER_LIGHTWEIGHT").unwrap_or(&CUSTOM_VERSION_STRING[..]),
			env!("BUILD_DATE"),
			option_env!("VERGEN_TARGET_TRIPLE").unwrap_or("unknown platform")
		);
		println!("{}", env!("CARGO_PKG_DESCRIPTION"));
		println!();

		if long_copyright_notice {
			println!(
				"Copyright (C) {} {}",
				env!("BUILD_YEAR"),
				env!("CARGO_PKG_AUTHORS")
			);
			println!();
			println!("This program is free software: you can redistribute it and/or modify");
			println!("it under the terms of the GNU Affero General Public License as");
			println!("published by the Free Software Foundation, either version 3 of the");
			println!("License, or (at your option) any later version.");
			println!();
			println!("This program is distributed free of charge in the hope that it will");
			println!("be useful, but WITHOUT ANY WARRANTY; without even the implied warranty");
			println!("of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the");
			println!("GNU Affero General Public License for more details.");
			println!();
			println!("You should have received a copy of the GNU Affero General Public License");
			println!("along with this program. If not, see <https://www.gnu.org/licenses/>.");
		} else {
			println!("This program comes with ABSOLUTELY NO WARRANTY.");
			println!("This is free software, and you are welcome to redistribute it");
			println!(
				"under certain conditions. Run {} -v for details.",
				env!("CARGO_BIN_NAME")
			);
		}
	};

	let show_help_hint_and_fail = || {
		eprintln!(
			"Use {} -h to see command line argument help",
			env!("CARGO_BIN_NAME")
		);
		process::exit(1);
	};

	if let Ok(option_matches) = options_parse_result {
		if option_matches.opt_present("h") {
			print_version_information(true);
			println!();
			println!("Usage:");
			print!(
				"    {} [OPTION]... [settings file path]",
				env!("CARGO_BIN_NAME")
			);
			println!("{}", options.usage(""));
		} else if option_matches.opt_present("v") {
			print_version_information(true);
		} else {
			print_version_information(false);
			println!();

			// If no free arguments were passed, or if the free argument was "-",
			// consider no path
			let settings_file_path =
				if option_matches.free.len() != 1 || option_matches.free[0] == "-" {
					None
				} else {
					Some(&option_matches.free[0])
				};

			// Now read the settings data
			println!(
				"Reading settings from {}...",
				settings_file_path.unwrap_or(&STANDARD_INPUT_STR.to_string())
			);

			let settings_data = match match settings_file_path {
				Some(path) => fs::read_to_string(path),
				None => {
					let mut buf = String::with_capacity(512);
					match io::stdin().read_to_string(&mut buf) {
						Ok(_) => Ok(buf),
						Err(error) => Err(error)
					}
				}
			} {
				Ok(settings_data) => settings_data,
				Err(io_error) => {
					eprintln!(
						"Couldn't read the settings file at {}: {}",
						settings_file_path.unwrap_or(&STANDARD_INPUT_STR.to_string()),
						io_error.to_string()
					);
					process::exit(2);
				}
			};

			// Deserialize the app settings struct contained in the data
			let app_settings = match toml::from_str::<AppSettings>(&settings_data) {
				Ok(app_settings) => app_settings,
				Err(deserialize_error) => {
					eprintln!(
						"An error occurred while parsing the settings file at {}: {}",
						settings_file_path.unwrap_or(&STANDARD_INPUT_STR.to_string()),
						deserialize_error.to_string()
					);
					process::exit(3);
				}
			};

			println!("Settings read. Starting processing...");

			// Execute the resource pack compression process, reporting any error
			if let Err(error) = execute(app_settings) {
				eprintln!(
					"An error occurred while processing the resource pack: {}",
					error.to_string()
				);
				process::exit(4);
			}
		}
	} else {
		eprintln!("{}", options_parse_result.unwrap_err().to_string());
		show_help_hint_and_fail();
	}
}

fn execute(app_settings: AppSettings) -> Result<(), Box<dyn Error>> {
	let file_count = Arc::new(AtomicUsize::new(0));
	let file_thread_pool = ThreadPool::new_named(
		String::from(env!("CARGO_PKG_NAME")),
		0,
		match app_settings.general.threads {
			0 => cmp::max(num_cpus::get().try_into().unwrap_or(u32::MAX), 1),
			_ => app_settings.general.threads
		},
		Duration::from_secs(15)
	);
	let micro_zip = Arc::new(MicroZip::new(
		16,
		app_settings.general.strict_zip_spec_compliance,
		app_settings.general.output_file_spooling_buffer_size
	));
	let app_settings = Arc::new(app_settings);

	// Build the set of globs that customize settings for the files they match
	let mut globset_builder = GlobSetBuilder::new();
	for pattern in app_settings.file_patterns.keys() {
		let mut glob_builder = GlobBuilder::new(&pattern[..]);
		glob_builder.literal_separator(true);
		glob_builder.backslash_escape(false);
		globset_builder.add(glob_builder.build()?);
	}

	let start_instant = Instant::now();

	// Process the entire resource pack directory
	process_directory(
		&app_settings.resource_pack_directory,
		&app_settings.resource_pack_directory,
		&file_count,
		&file_thread_pool,
		&app_settings,
		&Arc::new(globset_builder.build()?),
		&micro_zip
	)?;

	// Wait until all the work is done
	file_thread_pool.join();

	// Append the central directory
	println!("Finishing up resource pack ZIP file...");
	micro_zip.finish_and_write(&mut fs::File::create(
		&app_settings.general.output_file_path
	)?)?;

	let process_duration = start_instant.elapsed();
	println!(
		"{} files were stored in {} (took {}.{:03} s)",
		file_count.load(Ordering::Relaxed),
		app_settings.general.output_file_path,
		process_duration.as_secs(),
		process_duration.subsec_millis()
	);

	Ok(())
}

/// Recursively processes all the resource pack files in the given path,
/// storing the resulting processed resource pack file data in a vector.
fn process_directory(
	root_path: &Path,
	current_path: &Path,
	file_count: &Arc<AtomicUsize>,
	file_thread_pool: &ThreadPool,
	app_settings: &Arc<AppSettings>,
	file_globs: &Arc<GlobSet>,
	micro_zip: &Arc<MicroZip>
) -> Result<(), Box<dyn Error>> {
	for entry in fs::read_dir(current_path)? {
		let entry = entry?;
		let path = entry.path();
		let file_metadata = fs::metadata(&path)?;
		let is_directory = file_metadata.is_dir();

		// Check whether this is a system or dot (hidden) file, and if so skip it
		if app_settings.general.ignore_system_and_hidden_files {
			let file_name = path.file_name().unwrap_or(&EMPTY_OS_STR).to_string_lossy();
			let is_dot_file = file_name.chars().next().unwrap_or('x') == '.';

			if is_dot_file || is_system_file(&file_name, is_directory) {
				continue;
			}
		}

		if is_directory {
			process_directory(
				root_path,
				&path,
				file_count,
				&file_thread_pool,
				app_settings,
				file_globs,
				micro_zip
			)?;
		} else {
			let mut relative_path = relativize_path_for_zip_file(root_path, &path);
			let path_in_root = path.parent().unwrap() == root_path;
			let relative_path_str = match relative_path.to_str() {
				Some(path) => String::from(path),
				None => {
					return Err(Box::new(SimpleError::new(
						"A path contains invalid UTF-8 codepoints"
					)))
				}
			};

			let file_count = file_count.clone();
			let micro_zip = micro_zip.clone();
			let app_settings = app_settings.clone();
			let file_globs = file_globs.clone();

			// Now process the file in a different thread
			file_thread_pool.execute(move || {
				let resource_pack_file = || -> Result<Box<dyn ResourcePackFile>, Box<dyn Error>> {
					// Try to get the first resource pack file struct for this path
					// that can be created with the settings associated to a file pattern.
					// This will effectively ignore any other matching file patterns that
					// contain proper settings for this path
					for pattern_index in file_globs.matches(&relative_path) {
						let (_, file_data) =
							app_settings.file_patterns.get_index(pattern_index).unwrap();

						match resource_pack_file::path_to_resource_pack_file(
							&path,
							path_in_root,
							app_settings.general.skip_pack_icon,
							&app_settings.general.allowed_mods,
							Some(&file_data.settings)
						) {
							Ok(file) => return Ok(file),
							Err(file_error) => {
								// Passthrough the error only if it is a "hard" error
								if !file_error.is::<InvalidSettingsForResourcePackFile>() {
									return Err(file_error);
								}
							}
						}
					}

					// If we get here, no file pattern matches. Fallback to no settings
					// (i.e. use defaults)
					resource_pack_file::path_to_resource_pack_file(
						&path,
						path_in_root,
						app_settings.general.skip_pack_icon,
						&app_settings.general.allowed_mods,
						None
					)
				}();

				// The calls to create a resource pack file struct were successful
				// at this point, but we may still not have the struct because
				// the path is not a valid resource pack file
				if let Ok(mut resource_pack_file) = resource_pack_file {
					let result = resource_pack_file.process();

					if let Ok((processed_bytes, message)) = result {
						let success_message = format!("> {}: {}", relative_path_str, message);

						// Change the relative path with the canonical extension
						relative_path.set_extension(resource_pack_file.canonical_extension());

						// If we have data to add to the ZIP file, do it
						if let Some(processed_bytes) = processed_bytes {
							let add_result = micro_zip.add_file(
								&relative_path,
								ZipFileType::RegularFile,
								&processed_bytes,
								resource_pack_file.is_compressed()
									&& !app_settings.general.compress_already_compressed_files
							);

							if add_result.is_ok() {
								println!("{}", success_message);
								file_count.fetch_add(1, Ordering::Relaxed);
							} else {
								println!(
									"! {}: Couldn't add to the result ZIP file: {}",
									relative_path_str,
									add_result.unwrap_err()
								);
							}
						} else {
							println!("{}", success_message);
						}
					} else {
						println!(
							"! {}: Couldn't process the file: {}",
							relative_path_str,
							result.unwrap_err()
						);
					}
				} else {
					println!(
						"> {}: Couldn't open file: {}",
						relative_path_str,
						resource_pack_file.err().unwrap()
					);
				}
			});
		}
	}

	Ok(())
}

/// Relativizes the specified path from a given root path.
/// The resulting path is appropriate for using in ZIP files structures.
fn relativize_path_for_zip_file(root_path: &Path, descendant_path: &Path) -> PathBuf {
	let root_components: Vec<Component> = root_path.components().collect();
	let mut relativized_path = PathBuf::new();

	for (i, descendant_component) in descendant_path.components().enumerate() {
		if i < root_components.len() && root_components[i] == descendant_component {
			// If they are the same component, we are still in the components
			// that we share with the root
		} else {
			// This component is a descendant of the root, store it
			relativized_path.push(descendant_component);
		}
	}

	relativized_path
}

/// Checks whether the specified path likely represents a file generated for use
/// with some specific software that is not Minecraft, like VCS or operating system components.
fn is_system_file(file_name: &str, is_directory: bool) -> bool {
	(file_name == "desktop.ini" || file_name == "thumbs.db" || file_name == "README.md")
		&& !is_directory
}*/
