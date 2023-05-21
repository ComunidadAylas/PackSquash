use std::{
	io::{Read, Seek, Write},
	sync::RwLock,
	time::SystemTime
};

use patricia_tree::PatriciaSet;

use crate::{
	scratch_file::ScratchFilesBudget,
	squash_zip::{SquashZip, SquashZipError, SquashZipSettings},
	RelativePath
};

pub struct SquashedPackState<'settings, 'budget, F: Read + Seek + Send> {
	squash_zip: SquashZip<'settings, 'budget, F>,
	recompress_compressed_files: bool,
	scratch_files_budget: &'budget ScratchFilesBudget,
	processed_files: RwLock<PatriciaSet>
}

impl<'settings, 'budget, F: Read + Seek + Send> SquashedPackState<'settings, 'budget, F> {
	pub fn new(
		previous_zip: impl FnOnce() -> Option<F>,
		scratch_files_budget: &'budget ScratchFilesBudget,
		recompress_compressed_files: bool,
		squash_zip_settings: &'settings SquashZipSettings
	) -> Result<Self, SquashZipError> {
		Ok(Self {
			squash_zip: match SquashZip::new(
				squash_zip_settings
					.store_squash_time
					.then(previous_zip)
					.flatten(),
				squash_zip_settings,
				scratch_files_budget
			) {
				Ok(squash_zip) => squash_zip,
				Err(SquashZipError::PreviousZipParseError(err)) => {
					// We can recover from this error by not using the previous ZIP file,
					// although that has negative performance implications. Warn about it

					status_warn!(UnusablePreviousZip {
						previous_zip_parse_error: Some(err),
						io_error: None
					});

					SquashZip::new(None, squash_zip_settings, scratch_files_budget)?
				}
				Err(err) => return Err(err)
			},
			recompress_compressed_files,
			scratch_files_budget,
			processed_files: RwLock::new(PatriciaSet::new())
		})
	}

	pub fn add_file_to_zip(
		&self,
		path: &RelativePath,
		data: impl Read + Seek,
		is_compressed: bool
	) -> Result<(), SquashZipError> {
		self.squash_zip.add_file(
			path,
			data,
			!self.recompress_compressed_files && is_compressed
		)
	}

	pub fn add_previous_zip_file(&self, path: &RelativePath) -> Result<(), SquashZipError> {
		self.squash_zip.add_previous_file(path)
	}

	pub fn is_previous_zip_file_fresh(
		&self,
		file_path: &RelativePath,
		modification_time: Option<SystemTime>
	) -> bool {
		self.squash_zip.file_process_time(file_path).map_or_else(
			|| false,
			|squash_time| modification_time.is_some() && Some(squash_time) >= modification_time
		)
	}

	pub fn scratch_files_budget(&self) -> &ScratchFilesBudget {
		self.scratch_files_budget
	}

	pub fn mark_file_as_processed(&self, file_path: &RelativePath) -> bool {
		self.processed_files
			.write()
			.unwrap()
			.insert(file_path.as_str())
	}

	pub fn finish<W: Write>(
		self,
		write_provider: impl FnOnce() -> Result<W, SquashZipError>
	) -> Result<(), SquashZipError> {
		self.squash_zip.finish(write_provider)
	}
}
