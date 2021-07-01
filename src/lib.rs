//! A Minecraft resource and data pack optimizer that aims to achieve the best possible compression,
//! which allows for efficient distribution and slightly improved load times in the game, at good speed.
//!
//! TODO: example to get started, talk about important data types, explain essential things about how
//! PackSquash works

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
#![deny(rustdoc::invalid_html_tags)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

use std::borrow::Cow;
use std::cmp;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
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
use crate::vfs::{DirectoryTraversalOptions, VirtualFileSystem};

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
use tokio::{
	io::AsyncRead,
	runtime::{Builder, Runtime}
};

pub use crate::squash_zip::relative_path::RelativePath;

pub mod config;
mod pack_file;
mod squash_zip;

/// A struct that represents a resource or data pack optimization operation with configuration
/// parameters known beforehand. This is a good starting point for reading the API documentation.
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
		let mut globset_builder = GlobSetBuilder::with_capacity(options.file_options.len());
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
				.worker_threads(options.global_options.threads.into())
				.max_blocking_threads(options.global_options.threads.into())
				.thread_name("packsquasher-worker")
				.build()
				.unwrap(),
			file_globs: globset_builder.build()?,
			options
		})
	}

	/// TODO
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
				DirectoryTraversalOptions {
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

			// In the current thread, dispatch a task for each pack file, that may execute
			// in any thread of the Tokio runtime
			for pack_file_data in pack_file_iter {
				let packsquasher = Arc::clone(&self);
				let squash_zip = Arc::clone(&squash_zip);
				let vfs = Arc::clone(&vfs);

				let pack_file_status_sender = pack_file_status_sender.clone();

				pack_file_tasks.push(self.runtime.spawn(async move {
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

							return;
						}
					};

					/// TODO
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

					/// TODO
					macro_rules! process {
						($pack_file:expr, $pack_file_meta:expr) => {
							// We instantiated the pack file, so we got our VFS metadata back
							let pack_file_meta = $pack_file_meta.unwrap();

							process_pack_file(
								$pack_file,
								&pack_file_data.relative_path,
								// The file edit time is the most recent of the creation and
								// modification times, provided we have a modification time
								// available, so in case of incongruence or non-availability
								// of the modification time we do the safest thing. Also keep
								// in mind that Some(...) > None
								pack_file_meta
									.modification_time
									.and_then(|modification_time| {
										cmp::max(
											pack_file_meta.creation_time,
											Some(modification_time)
										)
									}),
								squash_zip,
								pack_file_status_sender
							)
							.await
						};
					}

					/// TODO
					macro_rules! process_with_options {
						($file_type:ident, $constructor_args:expr) => {
							let mut pack_file_open_error = None;
							let mut vfs_pack_file_meta = None;

							match $file_type::new(
								|| match vfs.open(&pack_file_data.file_path) {
									Ok(vfs_pack_file) => {
										vfs_pack_file_meta = Some(vfs_pack_file.metadata);
										Some((vfs_pack_file.file_read, vfs_pack_file.file_size))
									}
									Err(err) => {
										pack_file_open_error = Some(err);
										None
									}
								},
								$constructor_args
							) {
								Some(pack_file) => {
									process!(pack_file, vfs_pack_file_meta);
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

											return;
										}
									}
								}
							}
						};
					}

					/// TODO
					macro_rules! process_if_match {
						($file_type:ident, $file_options_type:ident, $file_options:expr) => {
							if let FileOptions::$file_options_type(file_options) = $file_options {
								process_with_options!($file_type, constructor_args!(file_options));
							}
						};
					}

					/// TODO
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

			// Now wait for every pack file task to finish, so the ZIP file is complete
			for join_handle in pack_file_tasks {
				join_handle.await.ok();
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

			// Finally, send warnings about relevant onditions
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

/// TODO
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
	SquashZip(#[from] SquashZipError)
}

/// TODO
pub struct VfsPackFileIterEntry {
	relative_path: RelativePath<'static>,
	file_path: PathBuf
}

/// TODO
pub struct VfsPackFile<R: AsyncRead + Unpin + 'static> {
	file_read: R,
	file_size: u64,
	metadata: VfsPackFileMetadata
}

/// TODO
pub struct VfsPackFileMetadata {
	creation_time: Option<SystemTime>,
	modification_time: Option<SystemTime>
}

/// TODO
#[non_exhaustive]
pub enum PackSquasherWarning {
	/// TODO
	LowEntropySystemId,
	/// TODO
	VolatileSystemId
}

/// TODO
#[non_exhaustive]
pub enum PackSquasherStatus {
	/// TODO
	PackFileProcessed(PackFileStatus),
	/// TODO
	ZipFinish,
	/// TODO
	Warning(PackSquasherWarning)
}

/// TODO
pub struct PackFileStatus {
	path: RelativePath<'static>,
	optimization_strategy: String,
	optimization_error: Option<String>
}

impl PackFileStatus {
	/// TODO
	pub fn path(&self) -> &RelativePath<'static> {
		&self.path
	}

	/// TODO
	pub fn optimization_strategy(&self) -> &str {
		&self.optimization_strategy
	}

	/// TODO
	pub fn optimization_error(&self) -> Option<&str> {
		self.optimization_error.as_deref()
	}
}

/// TODO
pub mod vfs {
	use std::{fs::FileType, io, path::Path};

	use tokio::io::AsyncRead;

	use crate::{VfsPackFile, VfsPackFileIterEntry};

	pub mod os_fs;

	/// TODO
	pub trait VirtualFileSystem: Send + Sync {
		/// TODO
		type FileRead: AsyncRead + Unpin + Send + 'static;
		/// TODO
		type FileIter: Iterator<Item = Result<VfsPackFileIterEntry, io::Error>>;

		/// TODO
		fn file_iterator(
			&self,
			root_path: &Path,
			directory_traversal_options: DirectoryTraversalOptions
		) -> Self::FileIter;

		/// TODO
		fn open<P: AsRef<Path>>(&self, path: P) -> Result<VfsPackFile<Self::FileRead>, io::Error>;

		/// TODO
		fn file_type<P: AsRef<Path>>(&self, path: P) -> Result<FileType, io::Error>;
	}

	/// TODO
	#[non_exhaustive]
	pub struct DirectoryTraversalOptions {
		/// TODO
		pub ignore_system_and_hidden_files: bool
	}

	impl Default for DirectoryTraversalOptions {
		fn default() -> Self {
			Self {
				ignore_system_and_hidden_files: false
			}
		}
	}
}

/// TODO
async fn process_pack_file<F: AsyncRead + AsyncSeek + Unpin>(
	pack_file: impl PackFile,
	relative_path: &RelativePath<'_>,
	edit_time: Option<SystemTime>,
	squash_zip: Arc<SquashZip<F>>,
	pack_file_status_sender: Option<Sender<PackSquasherStatus>>
) {
	// TODO
	// We may have to change the file extension to a canonical
	// one that's accepted by Minecraft. Do that early, because
	// we store the file with the canonical extension in the ZIP
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
		optimization_error = None; /*squash_zip
						   .add_previous_file(&pack_file_path)
						   .await
						   .err()
						   .map(|err| err.to_string());*/

		optimization_strategy = Cow::Owned(String::from("Copied from previous run"))
	} else {
		let is_compressed = pack_file.is_compressed();

		let mut processed_pack_file_chunks = pack_file.process().peekable();

		// Peek the strategy string contained in the first
		// processed chunk, and use that as the optimization
		// strategy string for all the file
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

		let squash_zip_error = None; /*squash_zip
							 .add_file(&pack_file_path, processed_pack_file_chunks, is_compressed)
							 .await
							 .err()
							 .map(|err| err.to_string());*/

		optimization_error = optimization_error.or(squash_zip_error);
	}

	if let Some(tx) = pack_file_status_sender {
		tx.send(PackSquasherStatus::PackFileProcessed(PackFileStatus {
			path: pack_file_path,
			optimization_strategy: optimization_strategy.into_owned(),
			optimization_error
		}))
		.await
		.ok();
	}
}
