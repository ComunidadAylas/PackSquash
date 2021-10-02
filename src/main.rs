use std::{
	borrow::Cow,
	env, fs,
	io::{self, Read},
	process,
	sync::{
		atomic::{AtomicU64, Ordering},
		Arc
	},
	thread,
	time::Instant
};

use getopts::{Options, ParsingStyle};
use packsquash::{
	config::SquashOptions, vfs::os_fs::OsFilesystem, PackSquasher, PackSquasherError,
	PackSquasherStatus, PackSquasherWarning
};
use tokio::sync::mpsc::channel;

fn main() {
	process::exit(run());
}

/// Runs PackSquash, parsing the command line parameters and deciding what options file
/// to read to process a pack.
fn run() -> i32 {
	#[cfg(feature = "color-backtrace")]
	color_backtrace::install();

	let mut options = Options::new();

	options.optflag("h", "help", "Prints information about the command line arguments accepted by this application and exits")
		.optflag("v", "version", "Prints version and copyright information of the application, then exits")
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
				print_version_information(false);
				println!();
				read_options_file_and_process(option_matches.free.first().filter(|path| {
					// Let "-" behave as if no path was provided
					path != &"-"
				}))
			}
		}
		Err(parse_err) => {
			eprintln!("{}", parse_err);
			eprintln!(
				"Run {} -h to see command line argument help",
				env!("CARGO_BIN_NAME")
			);

			1
		}
	}
}

/// Reads an options file and launches a squash operation to optimize it with the
/// read options.
fn read_options_file_and_process(options_file_path: Option<&String>) -> i32 {
	let user_friendly_options_path =
		options_file_path.map_or_else(|| "standard input (keyboard input or pipe)", |path| path);

	// Tell the user where are we reading the configuration from
	println!("Reading options from {}...", user_friendly_options_path);
	if options_file_path.is_none() {
		// Newbies are often confused by terms such as "standard input", so try
		// to point them in the direction of what they probably want to do
		println!(
			"If you are not sure what this means or what to do now, you probably \
			want to write and use an options file with PackSquash. Please check out \
			<https://packsquash.page.link/Options-files> for more information."
		);
	}
	println!();

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
			eprintln!(
				"! Couldn't read the options file from {}: {}",
				user_friendly_options_path, err
			);

			return 2;
		}
	};

	// Deserialize the options struct contained in the string
	let squash_options = match toml::from_str::<SquashOptions>(&options_string) {
		Ok(squash_options) => squash_options,
		Err(deserialize_error) => {
			eprintln!(
				"! An error occurred while parsing the options file from {}: {}",
				user_friendly_options_path, deserialize_error
			);

			return 3;
		}
	};

	println!("Options read. Processing pack...");
	println!();

	let output_file_path = squash_options.global_options.output_file_path.clone();
	let processed_file_count = Arc::new(AtomicU64::new(0));
	let start_instant = Instant::now();

	|| -> Result<(), PackSquasherError> {
		let (sender, mut receiver) = channel(64);
		let processed_file_count = processed_file_count.clone();

		// Spawn our CLI-updating thread. This decouples the pack processing from
		// the printing of standard streams messages, unless printing to them is
		// so slow that the channel buffer is filled, in which case the process
		// threads are slowed down to avoid exhausting memory
		let cli_thread = thread::spawn(move || {
			while let Some(status_update) = receiver.blocking_recv() {
				match status_update {
					PackSquasherStatus::PackFileProcessed(pack_file_status) => {
						processed_file_count.fetch_add(1, Ordering::Relaxed);

						match pack_file_status.optimization_error() {
							Some(error_description) => eprintln!(
								"! {}: {}",
								pack_file_status.path().as_ref(),
								error_description
							),
							None => eprintln!(
								"> {}: {}",
								pack_file_status.path().as_ref(),
								pack_file_status.optimization_strategy()
							)
						}
					}
					PackSquasherStatus::ZipFinish => eprintln!("- Finishing up ZIP file..."),
					PackSquasherStatus::Notice(notice) => eprintln!("- {}", notice),
					PackSquasherStatus::Warning(warning) => match warning {
						PackSquasherWarning::LowEntropySystemId => eprintln!(
							"* Used a low entropy system ID. The dates embedded in the result ZIP file, \
							which reveal when it was generated, may be easier to decrypt. For more information \
							about the topic, check out <https://packsquash.page.link/Low-entropy-system-ID-help>"),
						PackSquasherWarning::VolatileSystemId => eprintln!(
							"* Used a volatile system ID. You maybe should not reuse the result ZIP file, \
							as unexpected results can occur after you use your device as usual. For more information \
							about the topic, check out <https://packsquash.page.link/Volatile-system-ID-help>"),
						_ => unimplemented!()
					},
					_ => unimplemented!()
				}
			}
		});

		// Squash the pack! This blocks until the operation is complete
		let result = PackSquasher::new().run(OsFilesystem, squash_options, Some(sender));

		// Wait for the CLI thread to process any remaining buffered messages
		cli_thread.join().ok();

		result
	}()
	.map_or_else(
		|err| {
			eprintln!("! Pack processing error: {}", err);

			// We print both informational and error pack file status updates.
			// If the error was in one of those, hint the user at the status
			// update that contains the most information about the error
			if matches!(err, PackSquasherError::PackFileError) {
				eprintln!(
					"Another error message was emitted with more details about the error. \
					You might need to scroll up to see it."
				);
			}

			eprintln!(
				"These troubleshooting instructions might be useful: \
				<https://packsquash.page.link/Troubleshooting-pack-processing-errors>"
			);

			// Return a different status code depending on the machine-relevant cause,
			// so an external script can do whatever it deems fit to handle the error
			(128 + err.machine_relevant_cause() as i16) as i32
		},
		|_| {
			let process_time = start_instant.elapsed();

			println!(
				"{} ({} files, {}.{:03} s)",
				output_file_path.metadata().ok().map_or_else(
					|| Cow::Borrowed("Pack processed"),
					|metadata| Cow::Owned(format!(
						"{} generated, {:.3} MiB",
						output_file_path.as_os_str().to_string_lossy(),
						metadata.len() as f64 / (1024.0 * 1024.0)
					))
				),
				Arc::try_unwrap(processed_file_count).unwrap().into_inner(),
				process_time.as_secs(),
				process_time.subsec_millis()
			);

			0
		}
	)
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
