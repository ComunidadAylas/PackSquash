use super::pack_meta::PackMeta;
use super::resource_pack_processor::ResourcePackProcessor;
use super::{PackError, PackType};
use crate::scratch_file::ScratchFilesBudget;
use crate::squash_zip;
use crate::squash_zip::SquashZipSettings;
use crate::squashed_pack_state::SquashedPackState;
use crate::vfs::{os_fs::OsFilesystem, VirtualFileSystem, VirtualFileSystemType};
use itertools::Itertools;
use packsquash_options::{MinecraftQuirk, SquashOptions, ZipSpecConformanceLevel};
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::io::{Read, Seek};

// TODO
pub struct PackProcessor;

impl PackProcessor {
	pub fn new() -> Self {
		Self
	}

	pub fn process<F: Read + Seek + Send>(
		self,
		previous_zip: impl FnOnce() -> Option<F>,
		vfs_type: VirtualFileSystemType,
		options: &SquashOptions
	) -> Result<(), PackError> {
		let global_options = &options.global_options;

		let scratch_files_budget =
			ScratchFilesBudget::new(global_options.maximum_scratch_files_buffers_size);

		let os_vfs;
		let vfs = match vfs_type {
			VirtualFileSystemType::OsFilesystem => {
				os_vfs = OsFilesystem::new(options.pack_directory.as_ref(), &scratch_files_budget);
				&os_vfs
			}
		};

		let pack_meta = PackMeta::new(
			vfs,
			global_options.target_minecraft_version,
			global_options.always_allow_json_comments
		)?;

		let file_options = &options.file_options;

		let squash_zip_settings = SquashZipSettings {
			zopfli_iterations: global_options.zip_compression_iterations,
			store_squash_time: !global_options.never_store_squash_times
				&& !matches!(
					global_options.zip_spec_conformance_level,
					ZipSpecConformanceLevel::Pedantic
				),
			enable_obfuscation: matches!(
				global_options.zip_spec_conformance_level,
				ZipSpecConformanceLevel::Disregard
			),
			enable_deduplication: matches!(
				global_options.zip_spec_conformance_level,
				ZipSpecConformanceLevel::Balanced | ZipSpecConformanceLevel::Disregard
			),
			enable_size_increasing_obfuscation: global_options.size_increasing_zip_obfuscation,
			percentage_of_records_tuned_for_obfuscation_discretion: global_options
				.percentage_of_zip_structures_tuned_for_obfuscation_discretion,
			workaround_old_java_obfuscation_quirks: global_options
				.work_around_minecraft_quirks
				.unwrap_or_else(|| pack_meta.game_version_quirks())
				.contains(MinecraftQuirk::Java8ZipParsing)
		};

		let squashed_pack_state = SquashedPackState::new(
			previous_zip,
			&scratch_files_budget,
			global_options.recompress_compressed_files,
			&squash_zip_settings
		)?;

		let pack_type = pack_meta.pack_type();

		let pack_files = vfs.file_set()?;

		// Let consumers know the pack file count, the pack type we detected, and the quirks
		// we're working around
		status_info!(PackFileCount {
			count: pack_files.len()
		});

		status_info!(
			DetectedPackType { pack_type },
			game_version_range = log::kv::Value::capture_display(pack_meta.game_version_range())
		);

		status_info!(QuirksToWorkAround {
			quirk_list: pack_meta.game_version_quirks().iter().join(", ")
		});

		ThreadPoolBuilder::new()
			.num_threads(global_options.threads.get())
			.thread_name(|i| format!("packsquash-worker-{}", i))
			.build()?
			.install(|| match pack_type {
				PackType::ResourcePack => ResourcePackProcessor::new().process(
					vfs,
					pack_meta,
					pack_files,
					global_options,
					file_options,
					&squashed_pack_state
				),
				PackType::DataPack => todo!()
			})
			.and_then(|_| {
				// Let interested parties know that we are about to generate the result ZIP file
				status_info!(FinishingZip);

				// Generate the result ZIP file
				squashed_pack_state.finish(|| Ok(File::create(&global_options.output_file_path)?))?;

				// Warn about relevant system ID soft failure modes, if we needed to get a system ID
				if let Some(system_id) = squash_zip::system_id::get_system_id() {
					if system_id.has_low_entropy {
						status_warn!(SystemIdHasLowEntropy);
					}

					if system_id.is_volatile {
						status_warn!(SystemIdIsVolatile);
					}
				}

				Ok(())
			})
	}
}
