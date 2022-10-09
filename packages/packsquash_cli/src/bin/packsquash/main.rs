#![feature(cstr_from_bytes_until_nul)]

use std::borrow::Cow;
use std::fmt::Formatter;
use std::{
	env,
	fmt::Display,
	fs,
	io::{self, ErrorKind, Read},
	process::ExitCode,
	sync::atomic::{AtomicU64, AtomicUsize, Ordering},
	time::Instant
};

use env_logger::{fmt::Color, Target, WriteStyle};
use getopts::{Options, ParsingStyle};
use log::{debug, error, info, Level, LevelFilter};

use crate::util::{IoWriteToFmtWriteAdapter, PrettyPathDeserializeErrorDisplay};
use packsquash::{
	config::SquashOptions, PackSquashAssetProcessingStrategy, PackSquashStatus, VirtualFileSystemType
};
use terminal_style::{environment_allows_color, environment_allows_emoji};
use terminal_title_controller::TerminalTitleController;

mod terminal_style;
mod terminal_title_controller;
mod terminal_title_setter;
mod util;

/// The log target where status messages will be sent to.
const LOG_TARGET: Target = Target::Stderr;
/// A producer of the [`IsTerminal`] implementation that matches the [`LOG_TARGET`] constant.
const LOG_TARGET_STREAM: fn() -> Stderr = io::stderr;

static PROCESSED_FILE_COUNT: AtomicU64 = AtomicU64::new(0);
static TOTAL_FILE_COUNT: AtomicUsize = AtomicUsize::new(0);
static ASSET_WARNING_COUNT: AtomicU64 = AtomicU64::new(0);

/// Runs PackSquash, parsing the command line parameters and deciding what options file
/// to read to process a pack.
fn main() -> ExitCode {
	let title_controller = TerminalTitleController::new()
		.map(|title_controller| Box::leak(Box::new(title_controller)) as &TerminalTitleController);

	let log_target_is_tty = LOG_TARGET_STREAM().is_terminal();
	let enable_emoji_default = environment_allows_emoji(log_target_is_tty);
	let enable_color_default = environment_allows_color(log_target_is_tty);

	let mut options = Options::new();

	options.optflag("h", "help", "Prints information about the command line arguments accepted by this application and exits")
		.optflag("v", "version", "Prints version and copyright information of the application, then exits")
		.optflag(
			"",
			"emoji",
			"Always enable emojis in status messages. \
			This is equivalent to setting the PACKSQUASH_EMOJI or EMOJI environment variables to \"show\""
		)
		.optflag(
			"",
			"no-emoji",
			"Always disable emojis status messages. \
			 This is equivalent to defining the NO_EMOJI environment variable, or setting PACKSQUASH_EMOJI or EMOJI to something else than \"show\""
		)
		.optflag(
			"",
			"color",
			"Always enable color in messages. \
			This is equivalent to setting the PACKSQUASH_COLOR or COLOR environment variables to \"show\""
		)
		.optflag(
			"",
			"no-color",
			"Always disable color in messages. \
			This is equivalent to defining the NO_COLOR environment variable, or setting PACKSQUASH_COLOR or COLOR to something else than \"show\""
		)
		.parsing_style(ParsingStyle::StopAtFirstFree);

	match options.parse(env::args().skip(1)) {
		Ok(option_matches) => {
			if option_matches.opt_present("h") {
				print_version_information(true);
				println!();
				println!("Usage:");
				print!(
					"    {} [OPTION]... [options file path]",
					env!("CARGO_BIN_NAME")
				);
				println!("{}", options.usage(""));

				ExitCode::SUCCESS
			} else if option_matches.opt_present("v") {
				print_version_information(true);

				ExitCode::SUCCESS
			} else {
				let enable_emoji = if enable_emoji_default {
					!option_matches.opt_present("no-emoji")
				} else {
					option_matches.opt_present("emoji")
				};

				let enable_color = if enable_color_default {
					!option_matches.opt_present("no-color")
				} else {
					option_matches.opt_present("color")
				};

				init_logging(enable_emoji, enable_color, title_controller);

				print_version_information(false);
				println!();
				read_options_file_and_squash(
					option_matches.free.first().filter(|path| {
						// Let "-" behave as if no path was provided
						path != &"-"
					}),
					title_controller
				)
			}
		}
		Err(parse_err) => {
			init_logging(enable_emoji_default, enable_color_default, title_controller);

			error!(
				"{}\nRun {} -h to see command line argument help",
				parse_err,
				env!("CARGO_BIN_NAME")
			);

			1.into()
		}
	}
}

/// Reads an options file and launches a squash operation to optimize it with the
/// read options.
#[inline]
fn read_options_file_and_squash(
	options_file_path: Option<&String>,
	title_controller: Option<&TerminalTitleController>
) -> ExitCode {
	let user_friendly_options_path =
		options_file_path.map_or("standard input (keyboard input or pipe)", |path| path);

	// Tell the user where are we reading the configuration from
	info!(
		"Reading options from {}...{}",
		user_friendly_options_path,
		if options_file_path.is_none() {
			// Newbies are often confused by terms such as "standard input", so try
			// to point them in the direction of what they probably want to do
			"\nIf you are not sure what this means, try using an external options file.\
			 \nPlease check out <https://packsquash.page.link/Options-files> for examples and more information."
		} else {
			""
		}
	);

	// Read the TOML configuration data from the specified source
	let options_string = match match options_file_path {
		Some(path) => fs::read_to_string(path),
		None => {
			let mut buf = String::new();
			match io::stdin().read_to_string(&mut buf) {
				Ok(_) => Ok(buf),
				Err(err) => Err(err)
			}
		}
	} {
		Ok(options_string) => options_string,
		Err(err) => {
			error!(
				"Couldn't read the options file from {}: {}",
				user_friendly_options_path, err
			);

			return 2.into();
		}
	};

	// Deserialize the options struct contained in the string
	let squash_options = match serde_path_to_error::deserialize::<_, SquashOptions>(
		toml::de::Deserializer::new(&options_string)
	) {
		Ok(squash_options) => squash_options,
		Err(deserialize_error) => {
			error!(
				"An error occurred while parsing the options file: {}",
				PrettyPathDeserializeErrorDisplay::from(deserialize_error)
			);

			return 3.into();
		}
	};

	info!("Options read. Processing pack...");

	// Move on to the "processing" title phase
	if let Some(title_controller) = title_controller {
		title_controller.next_title_phase();
	}

	let start_instant = Instant::now();

	packsquash::run(&squash_options, VirtualFileSystemType::OsFilesystem).map_or_else(
		|err| {
			error!(
				"Pack processing error: {}\n\
				These troubleshooting instructions might be useful: \
				<https://packsquash.page.link/Troubleshooting-pack-processing-errors>",
				err
			);

			128.into()
		},
		|_| {
			let process_time = start_instant.elapsed();
			let output_file_path = squash_options.global_options.output_file_path;

			let processed_file_count = PROCESSED_FILE_COUNT.load(Ordering::Relaxed);
			let total_file_count = TOTAL_FILE_COUNT.load(Ordering::Relaxed);
			let asset_warning_count = ASSET_WARNING_COUNT.load(Ordering::Relaxed);
			let process_time_secs = process_time.as_secs();
			let process_time_subsec_millis = process_time.subsec_millis();

			match output_file_path.metadata().ok() {
				Some(metadata) => debug!(
					"{} generated, {:.3} MiB ({}/{} files stored, {} asset warnings, {}.{:03} s)",
					output_file_path.as_os_str().to_string_lossy(),
					metadata.len() as f64 / (1024.0 * 1024.0),
					processed_file_count,
					total_file_count,
					asset_warning_count,
					process_time_secs,
					process_time_subsec_millis
				),
				None => debug!(
					"Pack generated ({}/{} files stored, {} asset warnings, {}.{:03} s)",
					processed_file_count,
					total_file_count,
					asset_warning_count,
					process_time_secs,
					process_time_subsec_millis
				)
			}

			if squash_options.global_options.treat_asset_warnings_as_errors && asset_warning_count > 0
			{
				error!(
					"The pack was optimized successfully, but asset warnings were emitted.\n\
					Erroring out according to the configured options"
				);

				// Attempt to clean up the generated file
				fs::remove_file(output_file_path).ok();

				129.into()
			} else {
				ExitCode::SUCCESS
			}
		}
	)
}

/// Prints PackSquash version information to the standard output stream.
fn print_version_information(verbose: bool) {
	println!(
		"PackSquash {} ({}, {}) for {}",
		env!("BUILD_VERSION"),
		env!("CARGO_PROFILE"),
		env!("BUILD_DATE"),
		env!("CARGO_TARGET_TRIPLE")
	);
	println!("{}", env!("CARGO_PKG_DESCRIPTION"));
	println!();

	if verbose {
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
		println!("under certain conditions. Use the -v command line switch for");
		println!("more details about these conditions.");
	}
}

/// Initializes the logging of the application, responsible of showing the user
/// relevant operation information.
fn init_logging(
	enable_emoji: bool,
	enable_colors: bool,
	title_controller: Option<&'static TerminalTitleController>
) {
	let mut logger_builder = env_logger::Builder::new();

	let error_icon = if enable_emoji { '❌' } else { '!' };
	let warn_icon = if enable_emoji { '⚡' } else { '*' };
	let info_icon = if enable_emoji { '🔷' } else { '-' };
	let debug_icon = if enable_emoji { '🍀' } else { '#' };
	let trace_icon = if enable_emoji { '🏁' } else { '>' };

	logger_builder
		.target(LOG_TARGET)
		.write_style(if enable_colors {
			WriteStyle::Always
		} else {
			WriteStyle::Never
		})
		// Hide log messages from libraries by default
		.filter(Some("packsquash"), LevelFilter::max())
		.format(move |f, record| {
			use env_logger::fmt::Style;
			use std::fmt::Write;

			let mut status_type = 0.into();
			let status_type = record
				.key_values()
				.get("status_type".into())
				.and_then(|value| {
					status_type = value; // Extend value lifetime
					status_type.downcast_ref::<PackSquashStatus>()
				});

			const WARN_COLOR: Color = Color::Yellow;
			const WARN_TEXT_IS_BOLD: bool = false;
			let (level_color, level_icon, level_text_is_bold) = match record.level() {
				Level::Error => (Color::Red, error_icon, true),
				Level::Warn => (WARN_COLOR, warn_icon, WARN_TEXT_IS_BOLD),
				Level::Info => (Color::Cyan, info_icon, false),
				Level::Debug => (Color::Green, debug_icon, false),
				Level::Trace => (Color::White, trace_icon, false)
			};

			let mut level_style = f.style();
			level_style
				.set_color(level_color)
				.set_bold(level_text_is_bold);

			let mut warn_style = f.style();
			warn_style.set_color(WARN_COLOR).set_bold(WARN_TEXT_IS_BOLD);

			let write_message = |message: &dyn Display| {
				let mut f = IoWriteToFmtWriteAdapter(f);

				writeln!(
					// Our messages are prefixed by an icon, which may be an emoji, and a
					// space. Some messages may span several lines, so for the second and
					// next lines to look nice we must indent them by two characters. This
					// indenter does that efficiently, without heap allocations
					indenter::indented(&mut f).with_format(indenter::Format::Custom {
						inserter: &mut move |line_number, f| {
							if line_number == 0 {
								// First line, with the message icon. Do not indent
								Ok(())
							} else if enable_emoji {
								// Emojis are usually displayed with a non-monospaced font
								// that is wider than other monospaced characters. Output
								// an extra space as a hack to look pretty
								write!(f, "   ")
							} else {
								write!(f, "  ")
							}
						}
					}),
					"{}",
					level_style.value(format_args!("{} {}", level_icon, message))
				)
				.map_err(|err| io::Error::new(ErrorKind::Other, err))
			};

			match status_type {
				Some(PackSquashStatus::PackFileCount { count }) => {
					// A relaxed memory ordering is enough because we will read the value of this
					// counter after PackSquash finishes running and its thread stop (external
					// synchronization)
					TOTAL_FILE_COUNT.store(*count, Ordering::Relaxed);

					Ok(())
				}
				Some(PackSquashStatus::DetectedPackType { pack_type }) => {
					let game_version_range = record
						.key_values()
						.get("game_version_range".into())
						.unwrap();

					write_message(&format_args!(
						"Pack metadata read. Minecraft {} {} detected",
						game_version_range, pack_type
					))
				}
				Some(PackSquashStatus::QuirksToWorkAround { quirk_list }) => {
					if quirk_list.is_empty() {
						write_message(&"No Minecraft quirks to work around")
					} else {
						write_message(&format_args!(
							"Working around Minecraft quirks: {}",
							quirk_list
						))
					}
				}
				Some(PackSquashStatus::UnusablePreviousZip {
					previous_zip_parse_error,
					io_error
				}) => {
					if let Some(parse_error) = previous_zip_parse_error {
						write_message(&format_args!(
							"The previous ZIP file could not be read. \
							It will not be used to speed up processing. \
							Was the file last modified by PackSquash? Cause: {}",
							parse_error
						))
					} else if let Some(io_error) = io_error {
						write_message(&format_args!(
							"The previous ZIP file could not be read. \
							It will not be used to speed up processing. \
							Cause: {}",
							io_error
						))
					} else {
						Ok(())
					}
				}
				Some(PackSquashStatus::AssetProcessorsToRun {
					asset_processors_list
				}) => write_message(&format_args!(
					"Using asset processors: {}",
					asset_processors_list
				)),
				Some(PackSquashStatus::ProcessedAsset { strategy, warnings }) => {
					let asset_path = record.key_values().get("asset_path".into()).unwrap();
					let asset_path = asset_path.to_borrowed_str().unwrap();

					// A relaxed memory ordering is enough because we will read the values of these
					// counters after PackSquash finishes running and its thread stop (external
					// synchronization)
					PROCESSED_FILE_COUNT.fetch_add(1, Ordering::Relaxed);
					ASSET_WARNING_COUNT.fetch_add(warnings.len() as u64, Ordering::Relaxed);

					// Update the title text to signal that progress was made
					if let Some(title_controller) = title_controller {
						title_controller.advance_and_show();
					}

					let strategy_message = match strategy {
						PackSquashAssetProcessingStrategy::ValidatedAndMinified => {
							"validated and minified"
						}
						PackSquashAssetProcessingStrategy::ValidatedDebloatedAndMinified => {
							"validated, debloated and minified"
						}
						PackSquashAssetProcessingStrategy::ValidatedAndPrettified => {
							"validated and prettified"
						}
						PackSquashAssetProcessingStrategy::ValidatedDebloatedAndPrettified => {
							"validated, debloated and prettified"
						}
						PackSquashAssetProcessingStrategy::Optimized => "optimized"
					};

					struct WarningDisplayWrapper<T: AsRef<[Cow<'static, str>]>> {
						warnings: T,
						warn_style: Style,
						warn_icon: char
					}

					impl<T: AsRef<[Cow<'static, str>]>> Display for WarningDisplayWrapper<T> {
						fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
							for warning in self.warnings.as_ref() {
								f.write_fmt(format_args!(
									"\n{}",
									self.warn_style
										.value(format_args!("{} {}", self.warn_icon, warning))
								))?;
							}

							Ok(())
						}
					}

					write_message(&format_args!(
						"{}: {}{}",
						asset_path,
						strategy_message,
						WarningDisplayWrapper {
							warnings,
							warn_style,
							warn_icon
						}
					))
				}
				Some(PackSquashStatus::FinishingZip) => {
					if let Some(title_controller) = title_controller {
						title_controller.next_title_phase();
					}

					write_message(&"Finishing up ZIP file...")
				}
				Some(PackSquashStatus::SystemIdHasLowEntropy) => write_message(
					&"Used a low entropy system ID. The dates embedded in the result \
					ZIP file, which reveal when it was generated, may be easier to \
					decrypt. For more information about the topic, check out \
					<https://packsquash.page.link/Low-entropy-system-ID-help>"
				),
				Some(PackSquashStatus::SystemIdIsVolatile) => write_message(
					&"Used a volatile system ID. You maybe should not reuse the result \
					ZIP file, as unexpected results can occur after you use your device as \
					usual. For more information about the topic, check out \
					<https://packsquash.page.link/Volatile-system-ID-help>"
				),
				None => {
					// The record is not a status update from PackSquash. Write it out verbatim
					write_message(record.args())
				}
			}
		});

	// Read log filtering configuration from user-controlled environment variables, if defined
	if let Ok(log_filters) = env::var("PACKSQUASH_LOG").or_else(|_| env::var("RUST_LOG")) {
		logger_builder.parse_filters(&log_filters);
	}

	logger_builder.init();
}
