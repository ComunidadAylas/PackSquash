use std::{
	borrow::Cow,
	env,
	fmt::Display,
	fs,
	io::{self, IsTerminal, Read, Stderr},
	process,
	time::{Duration, Instant}
};

use env_logger::fmt::Color;
use env_logger::{Builder, Target, WriteStyle};
use getopts::{Options, ParsingStyle};
use is_terminal::IsTerminal;
use log::{debug, error, info, trace, warn, Level, LevelFilter};
use tokio::{runtime, select, sync::mpsc::channel, time::sleep};

use packsquash::{
	config::SquashOptions, vfs::os_fs::OsFilesystem, PackSquasher, PackSquasherError,
	PackSquasherStatus, PackSquasherWarning
};
use terminal_style::{environment_allows_color, environment_allows_emoji};
use terminal_title_controller::TerminalTitleController;

mod terminal_style;
mod terminal_title_controller;
mod terminal_title_setter;

/// The log target where status messages will be sent to.
const LOG_TARGET: Target = Target::Stderr;
/// A producer of the [`IsTerminal`] implementation that matches the [`LOG_TARGET`] constant.
const LOG_TARGET_STREAM: fn() -> Stderr = io::stderr;

fn main() {
	process::exit(run(TerminalTitleController::new()));
}

/// Runs PackSquash, parsing the command line parameters and deciding what options file
/// to read to process a pack.
fn run(title_controller: Option<TerminalTitleController>) -> i32 {
	// Show initial title
	if let Some(title_controller) = &title_controller {
		title_controller.show();
	}

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

				0
			} else if option_matches.opt_present("v") {
				print_version_information(true);

				0
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

				init_logger(enable_emoji, enable_color);

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
			init_logger(enable_emoji_default, enable_color_default);

			error!(
				"{}\nRun {} -h to see command line argument help",
				parse_err,
				env!("CARGO_BIN_NAME")
			);

			1
		}
	}
}

/// Reads an options file and launches a squash operation to optimize it with the
/// read options.
fn read_options_file_and_squash(
	options_file_path: Option<&String>,
	title_controller: Option<TerminalTitleController>
) -> i32 {
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
				user_friendly_options_path, err,
			);

			return 2;
		}
	};

	// Deserialize the options struct contained in the string
	let squash_options = match serde_path_to_error::deserialize::<_, SquashOptions>(
		toml::de::Deserializer::new(&options_string)
	) {
		Ok(squash_options) => squash_options,
		Err(deserialize_error) => {
			error!(
				"An error occurred while parsing the options file from {}: {}",
				user_friendly_options_path,
				PrettyPathDeserializeErrorDisplay::from(deserialize_error)
			);

			return 3;
		}
	};

	info!("Options read. Processing pack...");

	let output_file_path = squash_options.global_options.output_file_path.clone();
	let start_instant = Instant::now();

	squash(squash_options, title_controller).map_or_else(
		|err| {
			error!(
				"Pack processing error: {}{}\n\
				These troubleshooting instructions might be useful: \
				<https://packsquash.page.link/Troubleshooting-pack-processing-errors>",
				err,
				// We print both informational and error pack file status updates.
				// If the error was in one of those, hint the user at the status
				// update that contains the most information about the error
				if matches!(err, PackSquasherError::PackFileError) {
					"\nAnother error message with more information was emitted before. \
					You might need to scroll up to see it."
				} else {
					""
				}
			);

			128
		},
		|file_counts| {
			let process_time = start_instant.elapsed();

			debug!(
				"{} ({} pack files, {} pack files stored, {}.{:03} s)",
				output_file_path.metadata().ok().map_or_else(
					|| Cow::Borrowed("Pack processed"),
					|metadata| Cow::Owned(format!(
						"{} generated, {:.3} MiB",
						output_file_path.as_os_str().to_string_lossy(),
						metadata.len() as f64 / (1024.0 * 1024.0)
					))
				),
				file_counts.map_or_else(
					|| Cow::Borrowed("unknown"),
					|(total_file_count, _)| Cow::Owned(format!("{total_file_count}"))
				),
				file_counts.map_or_else(
					|| Cow::Borrowed("unknown"),
					|(_, processed_file_count)| Cow::Owned(format!("{processed_file_count}"))
				),
				process_time.as_secs(),
				process_time.subsec_millis()
			);

			0
		}
	)
}

fn squash(
	squash_options: SquashOptions,
	mut title_controller: Option<TerminalTitleController>
) -> Result<Option<(u64, u64)>, PackSquasherError> {
	let (sender, mut receiver) = channel(64);

	// Move on to the "processing" title phase
	if let Some(title_controller) = &mut title_controller {
		title_controller.next_title_phase();
		title_controller.show();
	}

	// Build the runtime we're going to use to concurrently wait for status messages and update
	// the title on a single thread: updating the display
	let runtime = runtime::Builder::new_current_thread()
		.enable_time()
		.build()
		.unwrap();

	let cli_update_task = runtime.spawn(async move {
		/// The maximum interval of time between two progress ticks of the title. Used to assure
		/// the user that progress is being made even when something takes a while to optimize.
		const PROGRESS_TICK_INTERVAL: Duration = Duration::from_secs(1);

		let mut total_file_count = 0;
		let mut processed_file_count = 0;
		let progress_tick_timer = sleep(PROGRESS_TICK_INTERVAL);

		tokio::pin!(progress_tick_timer);

		loop {
			select! {
				// Give priority to status updates
				biased;

				status_update_message = receiver.recv() => match status_update_message {
					Some(status_update) => match status_update {
						PackSquasherStatus::PackFileProcessed(pack_file_status) => {
							total_file_count += 1;
							processed_file_count += 1 - pack_file_status.skipped() as u64;

							match pack_file_status.optimization_error() {
								Some(error_description) => error!(
									"{}: {}",
									pack_file_status.path().as_str(),
									error_description
								),
								None => {
									if pack_file_status.skipped() {
										warn!(
											"{}: {}",
											pack_file_status.path().as_str(),
											pack_file_status.optimization_strategy()
										)
									} else {
										trace!(
											"{}: {}",
											pack_file_status.path().as_str(),
											pack_file_status.optimization_strategy()
										)
									};
								}
							};

							if let Some(title_controller) = &mut title_controller {
								title_controller.advance_and_show();

								// Prevent the forceful title progress tick from running too soon after this
								progress_tick_timer
									.as_mut()
									.reset(tokio::time::Instant::now() + PROGRESS_TICK_INTERVAL);
							}
						}
						PackSquasherStatus::ZipFinish => {
							info!("Finishing up ZIP file...");

							// Move on to the "finishing" title phase
							if let Some(title_controller) = &mut title_controller {
								title_controller.next_title_phase();
								title_controller.show();
							}
						}
						PackSquasherStatus::Notice(notice) => info!("{}", notice),
						PackSquasherStatus::Warning(warning) => match warning {
							PackSquasherWarning::UnusablePreviousZip(err) => warn!(
								"The previous ZIP file could not be read. It will not be used to speed up processing. \
									Was the file last modified by PackSquash? Cause: {}",
								err
							),
							PackSquasherWarning::LowEntropySystemId => warn!(
								"Used a low entropy system ID. The dates embedded in the result ZIP file, \
									which reveal when it was generated, may be easier to decrypt. For more information \
									about the topic, check out <https://packsquash.page.link/Low-entropy-system-ID-help>"
							),
							PackSquasherWarning::VolatileSystemId => warn!(
								"Used a volatile system ID. You maybe should not reuse the result ZIP file, \
									as unexpected results can occur after you use your device as usual. For more information \
									about the topic, check out <https://packsquash.page.link/Volatile-system-ID-help>"
							),
							_ => unimplemented!()
						},
						_ => unimplemented!()
					}
					None => {
						// PackSquasher has finished its work, and it will not send any other messages
						break
					}
				},
				_ = &mut progress_tick_timer => {
					// We have not yet received any message from the PackSquasher. Change the title
					// so that we give the user the illusion of some progress, and then schedule
					// another progress tick
					if let Some(title_controller) = &mut title_controller {
						title_controller.advance_and_show();

						progress_tick_timer
							.as_mut()
							.reset(tokio::time::Instant::now() + PROGRESS_TICK_INTERVAL);
					}
				}
			}
		}

		(total_file_count, processed_file_count)
	});

	// Squash the pack! This blocks until the operation is complete, so we can't run it in this thread
	let packsquasher = runtime
		.spawn_blocking(|| PackSquasher::new().run(OsFilesystem, squash_options, Some(sender)));

	runtime.block_on(async {
		// Wait for completion. Unwrap the handle because any panic in the thread is fatal anyway,
		// and we should propagate it
		match packsquasher.await.unwrap() {
			Ok(_) => Ok(cli_update_task.await.ok()),
			Err(err) => Err(err)
		}
	})
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

/// Initializes the logging of the application, responsible of showing to the user relevant
/// application operation information.
fn init_logger(enable_emoji: bool, enable_colors: bool) {
	let mut logger_builder = Builder::new();

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
			use std::io::Write;

			let (level_color, level_icon, level_bold_style) = match record.level() {
				Level::Error => (Color::Red, if enable_emoji { '❌' } else { '!' }, true),
				Level::Warn => (Color::Yellow, if enable_emoji { '⚡' } else { '*' }, false),
				Level::Info => (Color::Cyan, if enable_emoji { '🔷' } else { '-' }, false),
				Level::Debug => (Color::Green, if enable_emoji { '🍀' } else { '#' }, false),
				Level::Trace => (Color::White, if enable_emoji { '🏁' } else { '>' }, false)
			};

			let mut style = f.style();
			style.set_color(level_color).set_bold(level_bold_style);

			let message = style.value(
				format!("{} {}", level_icon, record.args())
					// Emojis are usually displayed with a non-monospaced font that is wider
					// than other characters. Output an extra space as a hack to look pretty
					.replace('\n', if enable_emoji { "\n   " } else { "\n  " })
			);

			writeln!(f, "{message}")
		});

	if let Ok(log_filters) = env::var("PACKSQUASH_LOG").or_else(|_| env::var("RUST_LOG")) {
		logger_builder.parse_filters(&log_filters);
	}

	logger_builder.init();
}

/// Newtype wrapper struct to prettify how serde errors with path information
/// are displayed.
#[repr(transparent)]
struct PrettyPathDeserializeErrorDisplay<E>(serde_path_to_error::Error<E>);

impl<E: Display> Display for PrettyPathDeserializeErrorDisplay<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let path = self.0.path();
		let inner = self.0.inner();

		// When the path is empty, its Display implementation writes a dot ("."),
		// which looks ugly and is somewhat confusing. Show the inner error
		// directly instead
		if path.iter().next().is_some() {
			write!(f, "{path}: {inner}")
		} else {
			inner.fmt(f)
		}
	}
}

impl<E> From<serde_path_to_error::Error<E>> for PrettyPathDeserializeErrorDisplay<E> {
	fn from(value: serde_path_to_error::Error<E>) -> Self {
		Self(value)
	}
}
