//! A Minecraft resource and data pack optimizer that aims to achieve the best possible compression,
//! which allows for efficient distribution and slightly improved load times in the game, at good speed.

#![deny(unsafe_code)]
#![feature(const_option)]
#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(new_uninit)]
#![feature(once_cell)]
#![feature(type_alias_impl_trait)]
#![feature(try_find)]
#![feature(iter_intersperse)]
#![feature(map_entry_replace)]
#![feature(doc_cfg)]
#![feature(if_let_guard)]
#![doc(
	html_logo_url = "https://user-images.githubusercontent.com/31966940/124388201-1f40eb80-dce2-11eb-88e8-3934d7d73c0a.png"
)]
#![deny(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

use enumset::EnumSet;
use std::borrow::Cow;
use std::convert::{Infallible, TryInto};
use std::hint::unreachable_unchecked;
use std::io::ErrorKind;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{cmp, panic};
use std::{io, time::SystemTime};

use futures::future;
use futures::StreamExt;
use thiserror::Error;
use tokio::io::AsyncSeek;
use tokio::io::BufReader;
use tokio::sync::mpsc::Sender;
use tokio::sync::Semaphore;
use tokio::{fs::File, io::AsyncRead, runtime::Builder};

use config::ProcessedSquashOptions;
use pack_meta::{PackMeta, PackMetaError};
use squash_zip::{SquashZip, SquashZipError};

#[cfg(feature = "audio-transcoding")]
use crate::config::AudioFileOptions;
#[cfg(feature = "optifine-support")]
use crate::config::PropertiesFileOptions;
use crate::config::{FileOptions, JsonFileOptions, PngFileOptions, ShaderFileOptions, SquashOptions};
use crate::pack_file::asset_type::{
	tweak_asset_types_mask_from_global_options, PackFileAssetTypeMatcher, PackFileAssetTypeMatches
};
use crate::pack_file::PackFileProcessData;
pub use crate::squash_zip::relative_path::RelativePath;
use crate::squash_zip::{system_id, PreviousZipParseError};
use crate::vfs::{IteratorTraversalOptions, VfsPackFileIterEntry, VirtualFileSystem};

pub mod config;
pub mod vfs;

mod pack_file;
mod pack_meta;
mod squash_zip;
mod zopfli_iterations_time_model;

/// A struct that represents a resource or data pack optimization operation with configuration
/// parameters known beforehand, which generates an output ZIP file. This is a good starting
/// point for reading the API documentation.
///
/// Once constructed, this struct can be used to run one or several optimization operations
/// with the same configuration on any pack, in an efficient manner.
pub struct PackSquasher;

impl PackSquasher {
	/// Creates a new [`PackSquasher`] struct that will squash packs.
	#[allow(clippy::new_without_default)] // It does not make much sense to have a default value
	pub fn new() -> Self {
		Self
	}

	/// Executes the squash operation configured by the specified options, reading pack files from
	/// the provided virtual file system, and waits for it to finish.
	///
	/// Client code can provide an optional channel to this method in the `pack_file_status_sender`
	/// parameter. Status updates of the squash operation will be sent to this channel, which the
	/// client code can use as it deems fit.
	///
	/// If this function returns successfully, it is guaranteed that a output ZIP file has been
	/// generated. If it does not, it should be noted that the caller may get more information about
	/// errors while processing particular pack files via the status updates channel, should they
	/// happen and that information be desired.
	///
	/// # Panics
	/// Reasonable client code can assume that this method does not panic. However, it should
	/// be noted that this method may temporarily set a panic hook to handle any panics that
	/// occur internally and then forward the panic information to the current panic hook,
	/// in order to fulfill its contract under any circumstances.
	///
	/// Therefore, to guarantee that this method produces the expected results, the panic hook
	/// should not be modified in any way while this method is executing.
	pub fn run<F: VirtualFileSystem + 'static, O: TryInto<ProcessedSquashOptions>>(
		&self,
		vfs: F,
		squash_options: O,
		pack_file_status_sender: Option<Sender<PackSquasherStatus>>
	) -> Result<(), PackSquasherError>
	where
		PackSquasherError: From<<O as TryInto<ProcessedSquashOptions>>::Error>
	{
		let mut options_holder = squash_options.try_into()?;

		// When reading from a pack directory that is not a directory, no files will be
		// processed. Avoid useless computation and help the user out by bailing out early
		// with a descriptive error message in that case.
		//
		// Note that program correctness cannot depend on these conditions staying true
		// during its execution. These checks are just meant to handle usage mistakes
		// promptly
		if !vfs
			.file_type(&options_holder.options.pack_directory)
			.map_or_else(|_| true, |file_type| file_type.is_dir())
		{
			return Err(PackSquasherError::InvalidFileType(
				"The pack directory path must refer to a directory, not a file"
			));
		}

		// On Windows and Linux (and probably most other POSIX OSes), writing to a directory
		// is an error, and we would try to do so after a maybe time consuming optimization
		// process. Reading from a directory, at least on those platforms, is like reading from
		// an empty file, and we would try to do that if the previous ZIP file is to be reused.
		// Again, to avoid useless computation and help the user out, bail out early with
		// a descriptive error message. We assume that the path being "not a directory" is
		// good enough, as the remaining filesystem object types (named pipes, etc.) behave
		// like regular files, not directories.
		//
		// Note that program correctness cannot depend on these conditions staying true during
		// its execution. These checks are just meant to handle usage mistakes promptly
		let output_file_path = &options_holder.options.global_options.output_file_path;
		if vfs
			.file_type(output_file_path)
			.map_or_else(|_| false, |file_type| file_type.is_dir())
		{
			return Err(PackSquasherError::InvalidFileType(
				"The output file path must refer to a file, not a directory"
			));
		}

		let runtime = Builder::new_multi_thread()
			.worker_threads(options_holder.options.global_options.threads.get())
			// The actual number of blocking threads will be worker threads + 1 + this (1)
			.max_blocking_threads(1)
			.thread_name("packsquasher-worker")
			// 2 MiB -> 4 MiB. Avoids stack overflow during Zopfli sometimes, even on Linux
			.thread_stack_size(4 * 1024 * 1024)
			.build()
			.unwrap();

		let automatic_quirk_detection = options_holder
			.options
			.global_options
			.automatic_minecraft_quirks_detection;

		let automatic_asset_type_mask_detection = options_holder
			.options
			.global_options
			.automatic_asset_types_mask_detection;

		let read_pack_meta = automatic_quirk_detection
			|| automatic_asset_type_mask_detection
			|| options_holder
				.options
				.global_options
				.validate_pack_metadata_file;

		// By default, allow every known asset type to match pack files. This will be adjusted later
		// depending on the options and automatic asset type mask detection, if enabled
		let mut asset_types_mask = EnumSet::all();

		// Transparently modify the options before doing the actual processing we want to read the
		// pack metadata, either to validate it, use automatic quirk detection or detect an asset
		// type mask
		if read_pack_meta {
			runtime.block_on(async {
				let pack_meta = PackMeta::new(&vfs, &options_holder.options.pack_directory).await?;

				if automatic_quirk_detection {
					let quirks = pack_meta.target_minecraft_versions_quirks();

					options_holder
						.options
						.global_options
						.work_around_minecraft_quirks = quirks;

					if let Some(pack_file_status_sender) = &pack_file_status_sender {
						if !quirks.is_empty() {
							let notice_message = format!(
								"Working around automatically detected Minecraft quirks: {}",
								quirks
									.iter()
									.map(|quirk| quirk.as_str())
									.intersperse(", ")
									.collect::<String>()
							);

							pack_file_status_sender
								.send(PackSquasherStatus::Notice(Cow::Owned(notice_message)))
								.await
								.ok();
						}
					}
				}

				if automatic_asset_type_mask_detection {
					asset_types_mask = pack_meta.target_minecraft_version_asset_type_mask();
				}

				Ok::<_, PackSquasherError>(())
			})?;
		}

		let vfs = Arc::new(vfs);
		let asset_type_matcher = Arc::new(PackFileAssetTypeMatcher::new(
			tweak_asset_types_mask_from_global_options(
				asset_types_mask,
				&options_holder.options.global_options
			)
		));
		let options_holder = Arc::new(options_holder);

		runtime.block_on(async {
			let pack_file_iter = vfs.file_iterator(
				&options_holder.options.pack_directory,
				IteratorTraversalOptions {
					ignore_system_and_hidden_files: options_holder
						.options
						.global_options
						.ignore_system_and_hidden_files
				}
			);

			let squashzip_settings = options_holder
				.options
				.global_options
				.as_squash_zip_settings();

			// Open the previous ZIP file and buffer it, if possible. Bail out if any I/O
			// error happens, except if the file does not exist, which is a normal condition
			let previous_zip = if squashzip_settings.store_squash_time {
				match File::open(&options_holder.options.global_options.output_file_path).await {
					Ok(file) => Some(BufReader::new(file)),
					Err(err) if err.kind() == ErrorKind::NotFound => None,
					Err(err) => return Err(err.into())
				}
			} else {
				None
			};

			let squash_zip = Arc::new(
				match SquashZip::new(previous_zip, squashzip_settings).await {
					Ok(squash_zip) => squash_zip,
					Err((SquashZipError::PreviousZipParseError(err), squashzip_settings)) => {
						// Something went wrong while reading the previous ZIP. We can continue the
						// optimization process, albeit with reduced performance. Warn the user about
						// that and try again without using a previous ZIP
						if let Some(pack_file_status_sender) = &pack_file_status_sender {
							pack_file_status_sender
								.send(PackSquasherStatus::Warning(
									PackSquasherWarning::UnusablePreviousZip(err)
								))
								.await
								.ok();
						}

						SquashZip::new(None, squashzip_settings)
							.await
							.map_err(|(err, _)| err)?
					}
					Err((err, _)) => return Err(err.into())
				}
			);

			let mut pack_file_tasks = Vec::with_capacity(squash_zip.previous_file_count());

			// Instantiate a semaphore that will help us limit the number of in-flight tasks.
			// This is needed because if we spawn those tasks faster than we finish them we
			// may end up opening a lot of files, needlessly consuming memory and exhausting
			// open file limits
			let in_flight_tasks_semaphore = Arc::new(Semaphore::new(cmp::min(
				options_holder.options.global_options.threads.get() * 2,
				// - 2 because we open the output file and the previous file
				// - 10 because the OsFilesystem VFS may keep some files open
				// / 3 because each task may consume 3 descriptors: the pack file itself,
				// and two temporary files
				cmp::max(
					(options_holder
						.options
						.global_options
						.open_files_limit
						.get()
						.saturating_sub(2)
						.saturating_sub(10)) / 3,
					1
				)
			)));

			let pack_file_optimization_failed = Arc::new(AtomicBool::new(false));

			// To shield ourselves against pack file tasks that may panic, even if they shouldn't
			// do so (GStreamer, I'm looking at you, because initializing your libraries can fail),
			// install a temporary panic hook that will register the pack file optimization as
			// failed and then invoke the already registered hook. This will "leak" two Arc's
			// in case we don't get to restore the previous panic hook, but if that's the case
			// then we will propagate the panic to the caller, which will probably not care about
			// this anyway. This "leak" lasts until the hook is set again, because the reference
			// count then drops to zero
			let previous_panic_hook = Arc::new(panic::take_hook());
			panic::set_hook({
				let pack_file_optimization_failed = Arc::clone(&pack_file_optimization_failed);
				let previous_panic_hook = Arc::clone(&previous_panic_hook);

				Box::new(move |panic_info| {
					pack_file_optimization_failed.store(true, Ordering::Release);
					previous_panic_hook(panic_info);
				})
			});

			// In the current thread, dispatch a task for each pack file, that may execute
			// in any thread of the Tokio runtime
			for pack_file_data in pack_file_iter {
				// Stop iterating over pack files if something went wrong processing one of them.
				// Use an acquire ordering to force happens-before relationships which ensure that
				// any value stored by other threads is read promptly by this thread
				if pack_file_optimization_failed.load(Ordering::Acquire) {
					break;
				}

				let options_holder = Arc::clone(&options_holder);
				let asset_type_matcher = Arc::clone(&asset_type_matcher);
				let squash_zip = Arc::clone(&squash_zip);
				let vfs = Arc::clone(&vfs);

				let in_flight_tasks_semaphore = Arc::clone(&in_flight_tasks_semaphore);
				let pack_file_optimization_failed = Arc::clone(&pack_file_optimization_failed);
				let pack_file_status_sender = pack_file_status_sender.clone();

				// Acquire a task permit before spawning it, and send it to the task. This
				// stops iteration of the VFS if it is going too fast relative to the
				// processing speed
				let task_permit = in_flight_tasks_semaphore.acquire_owned().await.unwrap();

				pack_file_tasks.push(runtime.spawn(async move {
					// We will release the permit only after we have completed the task
					let _ = task_permit;

					let pack_file_data = match pack_file_data {
						Ok(data) => data,
						Err(err) => {
							if let Some(tx) = pack_file_status_sender {
								tx.send(PackSquasherStatus::PackFileProcessed(PackFileStatus {
									path: RelativePath::from_inner(Cow::Borrowed("-")),
									optimization_strategy: Cow::Borrowed("Pack directory scan error"),
									optimization_error: Some(err.to_string()),
									skipped: false
								}))
								.await
								.ok();
							}

							pack_file_optimization_failed.store(true, Ordering::Release);
							return;
						}
					};

					let have_default_options;
					let asset_type_matches = {
						let asset_type_matches =
							asset_type_matcher.matches_for(&pack_file_data.relative_path);

						if !asset_type_matches.is_empty() {
							// Use the found matches. Every matched asset type has default options
							// if none are specified in the options file
							have_default_options = true;
							asset_type_matches
						} else {
							// Consider a tentative match for a custom asset, which must be specified
							// in the file options. As such, there are no default options
							have_default_options = false;
							PackFileAssetTypeMatches::of_custom_asset_type()
						}
					};

					/// Ergonomic wrapper for `match_and_process_pack_file`.
					macro_rules! try_process_with_file_options {
						($file_options:expr) => {
							match_and_process_pack_file(
								&options_holder.options,
								$file_options.map(|file_options| {
									file_options.tweak_from_global_options(
										&options_holder.options.global_options
									)
								}),
								&*squash_zip,
								&*vfs,
								&asset_type_matches,
								&pack_file_data,
								&*pack_file_optimization_failed,
								pack_file_status_sender.as_ref()
							)
							.await
						};
					}

					// Try to match configuration-provided file settings and process the pack file
					// with those. The first match that contains settings for this pack file type
					// "wins"
					for i in options_holder
						.file_options_globs
						.matches(&*pack_file_data.relative_path)
					{
						let file_options = options_holder.options.file_options[i];

						if try_process_with_file_options!(Some(file_options)) {
							return;
						}
					}

					// If we get here, this pack file either did not match any file settings,
					// in which case we should try defaults, or the file settings it matched
					// were not appropriate for its type (i.e. all matches were for JSON files,
					// but this is an audio file), in which case we should try defaults too
					if have_default_options {
						for default_file_options in [
							Some(FileOptions::JsonFileOptions(JsonFileOptions::default())),
							#[cfg(feature = "audio-transcoding")]
							Some(FileOptions::AudioFileOptions(AudioFileOptions::default())),
							Some(FileOptions::PngFileOptions(PngFileOptions::default())),
							#[cfg(feature = "optifine-support")]
							Some(FileOptions::PropertiesFileOptions(
								PropertiesFileOptions::default()
							)),
							Some(FileOptions::ShaderFileOptions(ShaderFileOptions::default())),
							None
						] {
							if try_process_with_file_options!(default_file_options) {
								return;
							}
						}
					}

					// Finally, if we get here, we did not process this pack file because
					// it really is not a pack file, or we want to skip it. Tell caller we
					// skipped it
					if let Some(tx) = pack_file_status_sender {
						tx.send(PackSquasherStatus::PackFileProcessed(PackFileStatus {
							path: pack_file_data.relative_path,
							optimization_strategy: Cow::Borrowed("Skipped"),
							optimization_error: None,
							skipped: true
						}))
						.await
						.ok();
					}
				}));
			}

			// Now wait for every pack file task to finish, including those who panic,
			// so the ZIP file is complete if everything went fine, or any pending work
			// is done if not
			for join_handle in pack_file_tasks {
				join_handle.await.ok();
			}

			// Everything's done, so restore the previous panic hook
			drop(panic::take_hook());
			panic::set_hook(match Arc::try_unwrap(previous_panic_hook) {
				Ok(hook) => hook,
				Err(_) => panic!("Unexpected number of strong references to the panic hook")
			});

			// Do not try to finish the ZIP file if something went wrong. We can't rely
			// on a local variable that indicates whether the loop exited early because
			// it may be finished by the time this is set to true, so do the atomic
			// access. The ordering can't be relaxed because awaiting for a join handle
			// is not documented to guarantee any synchronization (maybe the thread that
			// ran the task is still alive in the pool)
			if pack_file_optimization_failed.load(Ordering::Acquire) {
				return Err(PackSquasherError::PackFileError);
			}

			// Notify that we are about to finish the ZIP file
			if let Some(tx) = &pack_file_status_sender {
				tx.send(PackSquasherStatus::ZipFinish).await.ok();
			}

			// At this point we have only our strong reference to squash_zip, because
			// we have just waited for the pack file tasks to conclude, and each task
			// held one strong reference
			match Arc::try_unwrap(squash_zip) {
				Ok(squash_zip) => {
					squash_zip
						.finish(&options_holder.options.global_options.output_file_path)
						.await?
				}
				Err(_) => panic!("Unexpected number of strong references to SquashZip")
			};

			// Finally, send warnings about relevant conditions
			if let Some(tx) = pack_file_status_sender {
				if let Some(system_id) = system_id::get_system_id() {
					if system_id.has_low_entropy {
						tx.send(PackSquasherStatus::Warning(
							PackSquasherWarning::LowEntropySystemId
						))
						.await
						.ok();
					}

					if system_id.is_volatile {
						tx.send(PackSquasherStatus::Warning(
							PackSquasherWarning::VolatileSystemId
						))
						.await
						.ok();
					}
				}
			}

			Ok(())
		})
	}
}

/// An error that may occur during a pack squashing operation.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PackSquasherError {
	/// Thrown when an invalid file glob pattern was found in the file options.
	#[error("Invalid file glob pattern: {0}")]
	InvalidFileGlobPattern(#[from] globset::Error),
	/// Thrown when a I/O error occurs during the operation.
	#[error("I/O error: {0}")]
	IoError(#[from] io::Error),
	/// Thrown when the pack directory is not a directory, or the output file
	/// path is a directory.
	#[error("Invalid file type: {0}")]
	InvalidFileType(&'static str),
	/// Thrown when some error occurs in a ZIP file operation.
	#[error("Error while performing a ZIP file operation: {0}")]
	SquashZip(#[from] SquashZipError),
	/// Thrown when some error occurs while processing a pack file. More detailed
	/// information about it can be obtained by reading the corresponding status
	/// update.
	#[error("An error occurred while processing a pack file")]
	PackFileError,
	/// Thrown when an error happened while parsing the pack metadata file,
	/// which defines some basic characteristics of a pack.
	#[error("Pack metadata file error: {0}")]
	PackMetaError(#[from] PackMetaError)
}

impl From<Infallible> for PackSquasherError {
	fn from(_: Infallible) -> Self {
		// SAFETY: by construction, Infallible cannot be instantiated, so this
		// code can't ever be executed
		#[allow(unsafe_code)]
		unsafe {
			unreachable_unchecked()
		}
	}
}

/// Warnings that [`PackSquasher`] may emit while running a squash operation
/// to notify about conditions that may require some attention, because they
/// might signal some potential problem.
///
/// Although these warnings are currently emitted just before starting and
/// finishing the squash operation, that behavior is subject to change in the
/// future, so it shouldn't be relied upon.
#[non_exhaustive]
pub enum PackSquasherWarning {
	/// The previously generated ZIP file will not be used to speed up pack
	/// processing, because some error occurred while parsing it. These errors
	/// are usually caused due to the system identifier changing, a previously
	/// failed optimization process, or using different PackSquash versions.
	UnusablePreviousZip(PreviousZipParseError),
	/// A system identifier with low entropy was used to encrypt data, which
	/// may render that data easier to decrypt.
	LowEntropySystemId,
	/// A system identifier that may change even if no targeted action by the
	/// user to explicitly change it was done was used.
	VolatileSystemId
}

/// A status message concerning an in-progress squash operation.
#[non_exhaustive]
pub enum PackSquasherStatus {
	/// A pack file was processed in some way, either successfully or not.
	PackFileProcessed(PackFileStatus),
	/// Every pack file was processed, and the output ZIP file is being
	/// finished up.
	ZipFinish,
	/// An informational message that does not indicate a potential problem.
	Notice(Cow<'static, str>),
	/// A condition about the squash operation that may indicate a potential
	/// problem.
	Warning(PackSquasherWarning)
}

/// Status message that represents that a pack file was processed in some way,
/// successfully or not.
pub struct PackFileStatus {
	path: RelativePath<'static>,
	optimization_strategy: Cow<'static, str>,
	optimization_error: Option<String>,
	skipped: bool
}

impl PackFileStatus {
	/// Gets the relative path of the pack file that was processed.
	pub const fn path(&self) -> &RelativePath<'static> {
		&self.path
	}

	/// Gets the optimization strategy that was applied to this file. The string
	/// returned by this method is guaranteed to be user-friendly, but it is not
	/// advised to match patterns against it, because these strings may change
	/// between releases.
	pub fn optimization_strategy(&self) -> &str {
		self.optimization_strategy.as_ref()
	}

	/// Gets the error that occurred while optimizing this file. If an error
	/// did not happen, this returns `None`. Like the string returned by the
	/// `optimization_strategy` method, it is user-friendly and it may change
	/// between versions.
	pub fn optimization_error(&self) -> Option<&str> {
		self.optimization_error.as_deref()
	}

	/// Checks whether this file was processed successfully, but not included in
	/// the generated ZIP file either because it was deemed to be unnecessary or
	/// PackSquash did not recognize it.
	pub const fn skipped(&self) -> bool {
		self.skipped
	}
}

/// Processes the given pack file according to the provided file options and the asset types that
/// matched it. Any error condition will be handled by sending status updates and changing the
/// value held in `pack_file_optimization_failed` accordingly.
///
/// A return value of `false` signals that the pack file was not processed, but an error did not
/// occur, so the caller should try again with other file options. A return value of `true` means
/// that either the file was processed or an error occurred, so there's nothing else to do with
/// this file for now.
#[allow(clippy::too_many_arguments)] // Alternatives are not really more readable
async fn match_and_process_pack_file<R: AsyncRead + AsyncSeek + Unpin>(
	squash_options: &SquashOptions,
	file_options: Option<FileOptions>,
	squash_zip: &SquashZip<R>,
	vfs: &impl VirtualFileSystem,
	asset_type_matches: &PackFileAssetTypeMatches,
	pack_file_data: &VfsPackFileIterEntry,
	pack_file_optimization_failed: &AtomicBool,
	pack_file_status_sender: Option<&Sender<PackSquasherStatus>>
) -> bool {
	let mut pack_file_open_error = None;
	let mut vfs_file_meta = None;

	let process_data =
		asset_type_matches.process_data(file_options, || match vfs.open(&pack_file_data.file_path) {
			Ok(vfs_file) => {
				vfs_file_meta = Some((vfs_file.metadata, vfs_file.file_size_hint));
				Some((vfs_file.file_read, vfs_file.file_size_hint))
			}
			Err(err) => {
				pack_file_open_error = Some(err);
				None
			}
		});

	let pack_file_process_failed;
	let have_process_data;
	if let Some(process_data) = process_data {
		let (vfs_file_meta, pack_file_size_hint) = vfs_file_meta.unwrap();

		pack_file_process_failed = !process_pack_file(
			process_data,
			pack_file_data.relative_path.as_owned(),
			vfs_file_meta.modification_time,
			pack_file_size_hint,
			squash_zip,
			pack_file_status_sender,
			squash_options.global_options.recompress_compressed_files
		)
		.await;
		have_process_data = true;
	} else {
		pack_file_process_failed = false;
		have_process_data = false;
	}

	if let Some(err) = &pack_file_open_error {
		if let Some(tx) = pack_file_status_sender {
			tx.send(PackSquasherStatus::PackFileProcessed(PackFileStatus {
				path: pack_file_data.relative_path.as_owned(),
				optimization_strategy: Cow::Borrowed("Error opening pack file"),
				optimization_error: Some(err.to_string()),
				skipped: false
			}))
			.await
			.ok();
		}

		pack_file_optimization_failed.store(true, Ordering::Release);
	} else if pack_file_process_failed {
		pack_file_optimization_failed.store(true, Ordering::Release);
	}

	have_process_data || pack_file_open_error.is_some() || pack_file_process_failed
}

/// Processes the provided pack file, adding it to the output ZIP file as appropriate and
/// notifying client code via a channel about the result of the operation. If some error
/// occurs, the state of the output ZIP file may become invalid, and no further pack files
/// should be processed and added to it.
///
/// The return value is `true` if no error occurred, and `false` if some error happened.
async fn process_pack_file<F: AsyncRead + AsyncSeek + Unpin>(
	pack_file_process_data: PackFileProcessData,
	relative_path: RelativePath<'static>,
	edit_time: Option<SystemTime>,
	file_size_hint: u64,
	squash_zip: &SquashZip<F>,
	pack_file_status_sender: Option<&Sender<PackSquasherStatus>>,
	compress_already_compressed: bool
) -> bool {
	// We may have to change the file extension to a canonical one that's accepted by Minecraft.
	// Do that early, because we store the file with the canonical extension in the ZIP
	let pack_file_path = match pack_file_process_data.canonical_extension {
		Some(canonical_extension) => RelativePath::from_inner(
			relative_path
				.with_extension(canonical_extension)
				.into_os_string()
				.into_string()
				.unwrap()
		),
		None => relative_path
	};

	let copy_previous_file = squash_zip.file_process_time(&pack_file_path).map_or_else(
		|| false,
		|squash_time| edit_time.is_some() && Some(squash_time) >= edit_time
	);

	let mut optimization_error = None;
	let optimization_strategy;

	if copy_previous_file {
		optimization_error = squash_zip
			.add_previous_file(&pack_file_path)
			.await
			.err()
			.map(|err| err.to_string());

		optimization_strategy = Cow::Borrowed("Copied from previous run");
	} else {
		let mut processed_pack_file_chunks = pack_file_process_data
			.optimized_byte_chunks_stream
			.peekable();

		// Peek the strategy string contained in the first processed chunk, and use that
		// as the optimization strategy string for all the file
		optimization_strategy = {
			let first_chunk = Pin::new(&mut processed_pack_file_chunks).peek().await;

			first_chunk.map_or_else(
				|| &Cow::Borrowed("Copied (empty file)"),
				|chunk_result| {
					chunk_result
						.as_ref()
						.map_or_else(|_| &Cow::Borrowed("Error"), |chunk| &chunk.0)
				}
			)
		}
		.clone();

		// Stop taking chunks of processed data if some error happens, and store
		// the error that happened. After that, unwrap the successful chunks
		// so that they only contain the data
		let processed_pack_file_chunks = processed_pack_file_chunks
			.take_while(|chunk| {
				future::ready(if let Err(err) = chunk {
					optimization_error = Some(err.to_string());

					false
				} else {
					true
				})
			})
			.map(|chunk| BoxedDynAsByteSliceRef(chunk.unwrap().1));

		let squash_zip_error = squash_zip
			.add_file(
				&pack_file_path,
				processed_pack_file_chunks,
				!compress_already_compressed && pack_file_process_data.is_compressed,
				file_size_hint.try_into().unwrap_or(0)
			)
			.await
			.err()
			.map(|err| err.to_string());

		optimization_error = optimization_error.or(squash_zip_error);
	}

	let all_ok = optimization_error.is_none();

	if let Some(tx) = pack_file_status_sender {
		tx.send(PackSquasherStatus::PackFileProcessed(PackFileStatus {
			path: pack_file_path,
			optimization_strategy,
			optimization_error,
			skipped: false
		}))
		.await
		.ok();
	}

	all_ok
}

/// Helper newtype that consumes a boxed trait object that implements `AsRef<[u8]> + Send` and
/// delegates both traits on the wrapped object.
#[repr(transparent)]
struct BoxedDynAsByteSliceRef(Box<dyn AsRef<[u8]> + Send>);

impl AsRef<[u8]> for BoxedDynAsByteSliceRef {
	fn as_ref(&self) -> &[u8] {
		(&*self.0).as_ref()
	}
}
