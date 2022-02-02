use atty::Stream::Stdout;
use env_logger::fmt::Color;
use env_logger::{Builder, WriteStyle};
use std::{
	borrow::Cow,
	env, fs,
	io::{self, Read},
	process,
	time::{Duration, Instant}
};

use getopts::{Options, ParsingStyle};
use log::{debug, error, info, trace, warn, Level, LevelFilter};
use tokio::{runtime, select, sync::mpsc::channel, time::sleep};

use packsquash::{
	config::SquashOptions, vfs::os_fs::OsFilesystem, PackSquasher, PackSquasherError,
	PackSquasherStatus, PackSquasherWarning
};
use terminal_title_controller::TerminalTitleController;

mod terminal_title_controller;
mod terminal_title_setter;

fn main() {
	#[cfg(feature = "color-backtrace")]
	color_backtrace::install();

	process::exit(run(TerminalTitleController::new()));
}

/// Runs PackSquash, parsing the command line parameters and deciding what options file
/// to read to process a pack.
fn run(title_controller: Option<TerminalTitleController>) -> i32 {
	// Show initial title
	if let Some(title_controller) = &title_controller {
		title_controller.show();
	}

	let enable_emoji_default = enable_emoji_default();
	let enable_color_default = enable_color_default();

	let mut options = Options::new();

	options.optflag("h", "help", "Prints information about the command line arguments accepted by this application and exits")
		.optflag("v", "version", "Prints version and copyright information of the application, then exits")
		.optflag("", "emoji", "Enable emoji in output")
		.optflag("", "no-emoji", "Disable emoji in output")
		.optflag("", "color", "Enable color in output")
		.optflag("", "no-color", "Disable color in output")
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
		options_file_path.map_or_else(|| "standard input (keyboard input or pipe)", |path| path);

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
	let squash_options = match toml::from_str::<SquashOptions>(&options_string) {
		Ok(squash_options) => squash_options,
		Err(deserialize_error) => {
			error!(
				"An error occurred while parsing the options file from {}: {}",
				user_friendly_options_path, deserialize_error
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
					"\nAnother error message with more details about the error was emitted before. \
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
					|(total_file_count, _)| Cow::Owned(format!("{}", total_file_count))
				),
				file_counts.map_or_else(
					|| Cow::Borrowed("unknown"),
					|(_, processed_file_count)| Cow::Owned(format!("{}", processed_file_count))
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
		env!("VERGEN_GIT_SEMVER_LIGHTWEIGHT"),
		env!("VERGEN_CARGO_PROFILE"),
		env!("BUILD_DATE"),
		env!("VERGEN_CARGO_TARGET_TRIPLE")
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

fn init_logger(enable_emoji: bool, enable_color: bool) {
	let mut builder = formatted_builder(enable_emoji, enable_color);

	if let Ok(s) = ::std::env::var("RUST_LOG") {
		builder.parse_filters(&s);
	}

	builder.try_init().unwrap();
}

fn formatted_builder(enable_emoji: bool, enable_color: bool) -> Builder {
	let mut builder = Builder::new();

	builder.filter(Some("packsquash"), LevelFilter::Trace);
	builder.write_style(WriteStyle::Always);
	builder.format(move |f, record| {
		use std::io::Write;

		let mut style = f.style();
		let (color, icon) = match record.level() {
			Level::Error => (Color::Red, if enable_emoji { "❌" } else { "!" }),
			Level::Warn => (Color::Yellow, if enable_emoji { "⚡️" } else { "*" }),
			Level::Info => (Color::Cyan, if enable_emoji { "🔎" } else { "-" }),
			Level::Debug => (Color::Green, if enable_emoji { "🍀" } else { "#" }),
			Level::Trace => (Color::White, if enable_emoji { "🏁" } else { ">" })
		};
		if enable_color {
			style.set_color(color);
		}
		let message = style.value(
			format!("{} {}", icon, record.args())
				.replace("\n", if enable_emoji { "\n   " } else { "\n  " })
		);

		writeln!(f, "{}", message)
	});

	builder
}

// From terminal-supports-emoji, but false for Unix other than Mac.
// This is because even if it supports UTF8, it may not support emoji.
// https://github.com/mainrs/terminal-supports-emoji-rs/blob/1ead98a8372dd85946576e4447ed9d40b36f00db/src/lib.rs

#[cfg(windows)]
fn platform_supports_emoji() -> bool {
	std::env::var("WT_SESSION").is_ok()
}

#[cfg(target_os = "macos")]
fn platform_supports_emoji() -> bool {
	true
}

#[cfg(all(unix, not(target_os = "macos")))]
fn platform_supports_emoji() -> bool {
	false
}

#[cfg(all(not(unix), not(windows)))]
fn platform_supports_emoji() -> bool {
	false
}

fn enable_emoji_default() -> bool {
	match (
		env::var_os("EMOJI").is_some(),
		env::var_os("NO_EMOJI").is_some()
	) {
		(true, false) => true,
		(false, true) => false,
		_ => platform_supports_emoji() && atty::is(Stdout)
	}
}

// From termcolor, but ignore NO_COLOR
// https://github.com/BurntSushi/termcolor/blob/dc7e7b93830579716fc180925a17551dee5cfddc/src/lib.rs#L233-L269

#[cfg(not(windows))]
fn env_allows_color() -> bool {
	match env::var_os("TERM") {
		// If TERM isn't set, then we are in a weird environment that
		// probably doesn't support colors.
		None => return false,
		Some(k) => {
			if k == "dumb" {
				return false;
			}
		}
	}
	true
}

#[cfg(windows)]
fn env_allows_color() -> bool {
	// On Windows, if TERM isn't set, then we shouldn't automatically
	// assume that colors aren't allowed. This is unlike Unix environments
	// where TERM is more rigorously set.
	if let Some(k) = env::var_os("TERM") {
		if k == "dumb" {
			return false;
		}
	}
	true
}

fn enable_color_default() -> bool {
	match (
		env::var_os("COLOR").is_some(),
		env::var_os("NO_COLOR").is_some()
	) {
		(true, false) => true,
		(false, true) => false,
		_ => env_allows_color() && atty::is(Stdout)
	}
}
