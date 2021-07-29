//! A Minecraft resource and data pack optimizer that aims to achieve the best possible compression,
//! which allows for efficient distribution and slightly improved load times in the game, at good speed.

#![deny(unsafe_code)]
#![feature(const_option)]
#![feature(new_uninit)]
#![feature(once_cell)]
#![feature(type_alias_impl_trait)]
#![feature(try_find)]
#![feature(doc_cfg)]
#![doc(
	html_logo_url = "https://user-images.githubusercontent.com/31966940/124388201-1f40eb80-dce2-11eb-88e8-3934d7d73c0a.png"
)]
#![deny(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

use std::borrow::Cow;
use std::convert::TryInto;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{cmp, panic};
use std::{io, path::Path, time::SystemTime};

use crate::config::AudioFileOptions;
use crate::config::FileOptionsTrait;
use crate::config::JsonFileOptions;
use crate::config::PngFileOptions;
use crate::config::ShaderFileOptions;
use crate::config::{FileOptions, SquashOptions};
use crate::pack_file::audio_file::AudioFile;
use crate::pack_file::json_file::JsonFile;
use crate::pack_file::passthrough_file::PassthroughFile;
use crate::pack_file::png_file::PngFile;
use crate::pack_file::shader_file::ShaderFile;
use crate::pack_file::PackFile;
use crate::pack_file::PackFileConstructor;
use crate::pack_file::PackFileConstructorArgs;
use crate::squash_zip::system_id;
use crate::vfs::{IteratorTraversalOptions, VirtualFileSystem};

#[cfg(feature = "optifine-support")]
use crate::config::PropertiesFileOptions;
#[cfg(feature = "optifine-support")]
use crate::pack_file::properties_file::PropertiesFile;

use futures::future;
use futures::StreamExt;
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use squash_zip::{SquashZip, SquashZipError};
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncSeek;
use tokio::io::BufReader;
use tokio::sync::mpsc::Sender;
use tokio::sync::Semaphore;
use tokio::{
	io::AsyncRead,
	runtime::{Builder, Runtime}
};

pub use crate::squash_zip::relative_path::RelativePath;

pub mod config;
mod pack_file;
mod squash_zip;

/// A struct that represents a resource or data pack optimization operation with configuration
/// parameters known beforehand, which generates an output ZIP file. This is a good starting
/// point for reading the API documentation.
///
/// Once constructed, this struct can be used to run one or several optimization operations
/// with the same configuration on any pack, in an efficient manner.
pub struct PackSquasher {
	options: SquashOptions,
	runtime: Runtime,
	file_globs: GlobSet
}

impl PackSquasher {
	/// Creates a new [`PackSquasher`] struct that will squash packs according to the specified
	/// options.
	pub fn new(options: SquashOptions) -> Result<Self, PackSquasherError> {
		let mut globset_builder = GlobSetBuilder::new();
		for glob_pattern in options.file_options.keys() {
			globset_builder.add(
				GlobBuilder::new(glob_pattern)
					.literal_separator(true)
					.backslash_escape(true)
					.build()?
			);
		}

		Ok(Self {
			runtime: Builder::new_multi_thread()
				.worker_threads(options.global_options.threads.get())
				// The actual number of blocking threads will be worker threads + 1 + this (1)
				.max_blocking_threads(1)
				.thread_name("packsquasher-worker")
				.build()
				.unwrap(),
			file_globs: globset_builder.build()?,
			options
		})
	}

	/// Executes the configured squash operation, reading pack files from the provided virtual
	/// file system, on the specified directory, and waits for it to finish. If a pack directory
	/// is not provided, it will be read from the [`SquashOptions`] struct provided when this
	/// struct was instantiated. In that case, if the [`SquashOptions`] doesn't contain a pack
	/// directory either, an error will be returned.
	///
	/// Client code can provide an optional channel to this method in the
	/// `pack_file_status_sender` parameter. Status updates of the squash operation will be
	/// sent to this channel, which the client code can use as it deems fit.
	///
	/// If this function returns successfully, it is guaranteed that a output ZIP file has been
	/// generated. If it does not, it should be noted that the caller may get more information about
	/// errors while processing particular pack files via the status updates channel, should they
	/// happen and that information be desired.
	///
	/// Note that, due to the usage of several threads, the [`PackSquasher`] instance must be
	/// wrapped on an atomically reference counted pointer (`Arc`) to call this method.
	///
	/// # Panics
	/// Reasonable client code can assume that this method does not panic. However, it should
	/// be noted that this method may temporarily set a panic hook to handle any panics that
	/// occur internally and then forward the panic information to the current panic hook,
	/// in order to fulfill its contract under any circumstances.
	///
	/// Therefore, to guarantee that this method produces the expected results, the panic hook
	/// should not be modified in any way while this method is executing.
	pub fn run<F: VirtualFileSystem + 'static, P: AsRef<Path>>(
		self: Arc<Self>,
		vfs: F,
		pack_directory: Option<P>,
		pack_file_status_sender: Option<Sender<PackSquasherStatus>>
	) -> Result<(), PackSquasherError> {
		let pack_directory = pack_directory
			.as_ref()
			.map(|p| p.as_ref())
			.or_else(|| self.options.pack_directory.as_ref().map(|p| p.as_ref()))
			.ok_or(PackSquasherError::MissingPackDirectory)?;

		if !vfs.file_type(pack_directory)?.is_dir() {
			return Err(PackSquasherError::InvalidFileType(
				"The pack directory path must point to a directory, not a file"
			));
		}

		let vfs = Arc::new(vfs);

		self.runtime.block_on(async {
			// Get the iterator over pack files from the virtual file system
			let pack_file_iter = vfs.file_iterator(
				pack_directory,
				IteratorTraversalOptions {
					ignore_system_and_hidden_files: self
						.options
						.global_options
						.ignore_system_and_hidden_files
				}
			);

			let squashzip_settings = self.options.global_options.as_squash_zip_settings();

			// Open the previous ZIP file and buffer it, if possible. Bail out if any I/O
			// error happens, except if the file does not exist, which is a normal condition
			let previous_zip = if squashzip_settings.store_squash_time {
				match File::open(&self.options.global_options.output_file_path).await {
					Ok(file) => Some(BufReader::new(file)),
					Err(err) => {
						// The previous output file may not exist the first time we run
						if err.kind() == ErrorKind::NotFound {
							None
						} else {
							return Err(err.into());
						}
					}
				}
			} else {
				None
			};

			// Instantiate SquashZip and the list of join handles to the pack file tasks
			let squash_zip = SquashZip::new(previous_zip, squashzip_settings).await?;
			let mut pack_file_tasks = Vec::with_capacity(squash_zip.previous_file_count());
			let squash_zip = Arc::new(squash_zip);

			// Instantiate a semaphore that will help us limit the number of in-flight tasks.
			// This is needed because if we spawn those tasks faster than we finish them we
			// may end up opening a lot of files, needlessly consuming memory and exhausting
			// open file limits
			let in_flight_tasks_semaphore = Arc::new(Semaphore::new(cmp::min(
				self.options.global_options.threads.get() * 2,
				// - 2 because we open the output file and the previous file
				// - 10 because the OsFilesystem VFS may keep some files open
				// / 3 because each task may consume 3 descriptors: the pack file itself,
				// and two temporary files
				cmp::max(
					(self
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

				let packsquasher = Arc::clone(&self);
				let squash_zip = Arc::clone(&squash_zip);
				let vfs = Arc::clone(&vfs);

				let in_flight_tasks_semaphore = Arc::clone(&in_flight_tasks_semaphore);
				let pack_file_optimization_failed = Arc::clone(&pack_file_optimization_failed);
				let pack_file_status_sender = pack_file_status_sender.clone();

				// Acquire a task permit before spawning it, and send it to the task. This
				// stops iteration of the VFS if it is going too fast relative to the
				// processing speed
				let task_permit = in_flight_tasks_semaphore.acquire_owned().await.unwrap();

				pack_file_tasks.push(self.runtime.spawn(async move {
					// We will release the permit only after we have completed the task
					let _ = task_permit;

					let pack_file_data = match pack_file_data {
						Ok(data) => data,
						Err(err) => {
							if let Some(tx) = pack_file_status_sender {
								tx.send(PackSquasherStatus::PackFileProcessed(PackFileStatus {
									path: RelativePath::from_inner(Cow::Borrowed("-")),
									optimization_strategy: String::from("Pack directory scan error"),
									optimization_error: Some(err.to_string())
								}))
								.await
								.ok();
							}

							pack_file_optimization_failed.store(true, Ordering::Release);
							return;
						}
					};

					/// Macro that evaluates to a [`PackFileConstructorArgs`] instance, allowing its
					/// instantiation with less verbosity.
					macro_rules! constructor_args {
						(()) => {
							PackFileConstructorArgs {
								path: &pack_file_data.relative_path,
								optimization_settings: ()
							}
						};
						($file_options:expr) => {
							PackFileConstructorArgs {
								path: &pack_file_data.relative_path,
								optimization_settings: $file_options
									.tweak_from_global_options(&packsquasher.options.global_options)
							}
						};
					}

					/// Macro that evaluates to a call to the `process_pack_file` function,
					/// providing less verbose syntax to call it.
					macro_rules! process {
						($pack_file:expr, $pack_file_meta:expr) => {{
							// We instantiated the pack file, so we got our VFS metadata back
							let (pack_file_meta, pack_file_size) = $pack_file_meta.unwrap();

							process_pack_file(
								$pack_file,
								&pack_file_data.relative_path,
								pack_file_meta.modification_time,
								pack_file_size,
								squash_zip,
								pack_file_status_sender,
								packsquasher
									.options
									.global_options
									.recompress_compressed_files
							)
							.await
						}};
					}

					/// Big macro that opens a pack file ant tries to instantiate it with the
					/// appropriate options, handling any error that could happen. The macro
					/// will return if either the pack file is processed successfully or not or
					/// an error occurs while opening the pack file. It will not return if
					/// the pack file is not of the expected file type.
					macro_rules! process_with_options {
						($file_type:ident, $constructor_args:expr) => {
							let mut pack_file_open_error = None;
							let mut vfs_file_meta = None;

							match $file_type::new(
								|| match vfs.open(&pack_file_data.file_path) {
									Ok(vfs_file) => {
										vfs_file_meta = Some((vfs_file.metadata, vfs_file.file_size));
										Some((vfs_file.file_read, vfs_file.file_size))
									}
									Err(err) => {
										pack_file_open_error = Some(err);
										None
									}
								},
								$constructor_args
							) {
								Some(pack_file) => {
									if !process!(pack_file, vfs_file_meta) {
										pack_file_optimization_failed.store(true, Ordering::Release);
									}

									return;
								}
								None => {
									if let Some(err) = pack_file_open_error {
										if let Some(tx) = pack_file_status_sender {
											tx.send(PackSquasherStatus::PackFileProcessed(
												PackFileStatus {
													path: pack_file_data.relative_path,
													optimization_strategy: String::from(
														"Error opening pack file"
													),
													optimization_error: Some(err.to_string())
												}
											))
											.await
											.ok();
										}

										pack_file_optimization_failed.store(true, Ordering::Release);
										return;
									}
								}
							}
						};
					}

					/// Processes this pack file if and only if the provided file options variant
					/// matches the expected variant for the pack file type. If the variant doesn't
					/// match, nothing is done.
					macro_rules! process_if_match {
						($file_type:ident, $file_options_type:ident, $file_options:expr) => {
							if let FileOptions::$file_options_type(file_options) = $file_options {
								process_with_options!($file_type, constructor_args!(file_options));
							}
						};
					}

					/// Processes this pack file as if it was of the specified type, using the
					/// provided options.
					macro_rules! process_with_defaults {
						($file_type:ident) => {
							process_with_options!($file_type, constructor_args!(()));
						};
						($file_type:ident, $file_options:expr) => {
							process_with_options!($file_type, constructor_args!($file_options));
						};
					}

					// Try to match configuration-provided file settings and instantiate a pack file
					// instance with those. The first match that contains settings for this pack file
					// type "wins"
					for i in packsquasher
						.file_globs
						.matches(&*pack_file_data.relative_path)
					{
						let file_options = packsquasher.options.file_options[i];

						// Now try to instantiate the pack file type that corresponds to the options
						process_if_match!(JsonFile, JsonFileOptions, file_options);
						process_if_match!(AudioFile, AudioFileOptions, file_options);
						process_if_match!(PngFile, PngFileOptions, file_options);
						#[cfg(feature = "optifine-support")]
						process_if_match!(PropertiesFile, PropertiesFileOptions, file_options);
						process_if_match!(ShaderFile, ShaderFileOptions, file_options);
					}

					// If we get here, this pack file either did not match any file settings,
					// in which case we should try defaults, or the file settings it matched
					// were not appropriate for its type (i.e. all matches were for JSON files,
					// but this is an audio file), in which case we should try defaults too
					process_with_defaults!(JsonFile, JsonFileOptions::default());
					process_with_defaults!(AudioFile, AudioFileOptions::default());
					process_with_defaults!(PngFile, PngFileOptions::default());
					#[cfg(feature = "optifine-support")]
					process_with_defaults!(PropertiesFile, PropertiesFileOptions::default());
					process_with_defaults!(ShaderFile, ShaderFileOptions::default());
					process_with_defaults!(PassthroughFile);

					// Finally, if we get here, we did not process this pack file because
					// it really is not a pack file. Tell caller we skipped it
					if let Some(tx) = pack_file_status_sender {
						tx.send(PackSquasherStatus::PackFileProcessed(PackFileStatus {
							path: pack_file_data.relative_path,
							optimization_strategy: String::from("Skipped"),
							optimization_error: None
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
						.finish(&self.options.global_options.output_file_path)
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
	/// Thrown when a pack directory was not provided in the [`SquashOptions`]
	/// struct, neither when trying to run the squash operation.
	#[error("The pack directory was not provided")]
	MissingPackDirectory,
	/// Thrown when the pack directory path in the virtual file system contains non
	/// UTF-8 characters.
	#[error("The pack directory path contains non UTF-8 characters")]
	InvalidPackDirectoryPath,
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
	PackFileError
}

/// A entry in a virtual filesystem directory that represents a possible pack file,
/// obtained via a virtual filesystem directory file iterator.
pub struct VfsPackFileIterEntry {
	relative_path: RelativePath<'static>,
	file_path: PathBuf
}

/// An open file in a virtual filesystem, from which data can be read and
/// metadata is available.
pub struct VfsFile<R: AsyncRead + Unpin + 'static> {
	file_read: R,
	file_size: u64,
	metadata: VfsPackFileMetadata
}

/// Metadata of a virtual filesystem file.
pub struct VfsPackFileMetadata {
	modification_time: Option<SystemTime>
}

/// Warnings that [`PackSquasher`] may emit while running a squash operation
/// to notify about conditions that may require some attention, because they
/// might signal some potential problem.
///
/// Although these warnings are currently emitted just before finishing the
/// squash operation, that behavior is subject to change in the future, so
/// it shouldn't be relied upon.
#[non_exhaustive]
pub enum PackSquasherWarning {
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
	/// A condition about the squash operation that may indicate a
	/// potential problem.
	Warning(PackSquasherWarning)
}

/// Status message that represents that a pack file was processed in some way,
/// successfully or not.
pub struct PackFileStatus {
	path: RelativePath<'static>,
	optimization_strategy: String,
	optimization_error: Option<String>
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
		&self.optimization_strategy
	}

	/// Gets the error that occurred while optimizing this file. If an error
	/// did not happen, this returns `None`. Like the string returned by the
	/// `optimization_strategy` method, it is user-friendly and it may change
	/// between versions.
	pub fn optimization_error(&self) -> Option<&str> {
		self.optimization_error.as_deref()
	}
}

/// Contains virtual file systems implementations to use with [`PackSquasher`].
pub mod vfs {
	use std::{fs::FileType, io, path::Path};

	use tokio::io::AsyncRead;

	use crate::{VfsFile, VfsPackFileIterEntry};

	pub mod os_fs;

	/// Defines the contract that any virtual file system must implement.
	pub trait VirtualFileSystem: Send + Sync {
		/// The type of the byte source that this virtual file system yields
		/// when successfully opening files. It reading bytes from this source
		/// is costly, it is recommended to introduce some buffering for maximum
		/// performance.
		type FileRead: AsyncRead + Unpin + Send + 'static;
		/// The type of the iterator over the files within a path that this virtual
		/// file system yields.
		type FileIter: Iterator<Item = Result<VfsPackFileIterEntry, io::Error>>;

		/// Returns an iterator over the files that are in the filesystem subtree
		/// whose root is at `root_path`, which usually is a directory, according
		/// to the specified directory traversal options. This means that not only
		/// direct children of the `root_path` are yielded, but also grandchildren
		/// and so on. Files that don't hold any readable user data (i.e. directories)
		/// are not yielded.
		///
		/// Any I/O error that may happen is returned as an element in the iterator,
		/// so getting the iterator itself can't fail.
		fn file_iterator(
			&self,
			root_path: &Path,
			iterator_traversal_options: IteratorTraversalOptions
		) -> Self::FileIter;

		/// Opens the file at the specified virtual filesystem path for read-only access.
		fn open<P: AsRef<Path>>(&self, path: P) -> Result<VfsFile<Self::FileRead>, io::Error>;

		/// Returns the type of the file at the specified virtual filesystem path.
		fn file_type<P: AsRef<Path>>(&self, path: P) -> Result<FileType, io::Error>;
	}

	/// Contains options that tweak the operation of the [`VirtualFileSystem::file_iterator`]
	/// method.
	#[non_exhaustive]
	pub struct IteratorTraversalOptions {
		/// Whether system (i.e. clearly not part of a pack file) and hidden files
		/// (usually, those whose name begins with a dot) are yielded or not.
		pub ignore_system_and_hidden_files: bool
	}

	impl Default for IteratorTraversalOptions {
		fn default() -> Self {
			Self {
				ignore_system_and_hidden_files: false
			}
		}
	}
}

/// Processes the provided pack file, adding it to the output ZIP file as appropriate and
/// notifying client code via a channel about the result of the operation. If some error
/// occurs, the state of the output ZIP file may become invalid, and no further pack files
/// should be processed and added to it.
///
/// The return value is `true` if no error ocurred, and `false` if some error happened.
async fn process_pack_file<F: AsyncRead + AsyncSeek + Unpin>(
	pack_file: impl PackFile,
	relative_path: &RelativePath<'_>,
	edit_time: Option<SystemTime>,
	file_size: u64,
	squash_zip: Arc<SquashZip<F>>,
	pack_file_status_sender: Option<Sender<PackSquasherStatus>>,
	always_compress: bool
) -> bool {
	// We may have to change the file extension to a canonical one that's accepted by Minecraft.
	// Do that early, because we store the file with the canonical extension in the ZIP
	let pack_file_path = RelativePath::from_inner(
		relative_path
			.with_extension(pack_file.canonical_extension())
			.into_os_string()
			.into_string()
			.unwrap()
	);

	let previous_file_is_current = squash_zip.file_process_time(&pack_file_path).map_or_else(
		|| false,
		|squash_time| edit_time.is_some() && Some(squash_time) >= edit_time
	);

	let mut optimization_error = None;
	let optimization_strategy;

	if previous_file_is_current {
		optimization_error = squash_zip
			.add_previous_file(&pack_file_path)
			.await
			.err()
			.map(|err| err.to_string());

		optimization_strategy = Cow::Owned(String::from("Copied from previous run"))
	} else {
		let is_compressed = pack_file.is_compressed();

		let mut processed_pack_file_chunks = pack_file.process().peekable();

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
			.map(|chunk| chunk.unwrap().1);

		let squash_zip_error = squash_zip
			.add_file(
				&pack_file_path,
				processed_pack_file_chunks,
				!always_compress && is_compressed,
				file_size.try_into().unwrap_or(0)
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
			optimization_strategy: optimization_strategy.into_owned(),
			optimization_error
		}))
		.await
		.ok();
	}

	all_ok
}
