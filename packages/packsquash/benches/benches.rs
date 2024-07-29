//! Automated benchmarks for the `PackSquash` library, using the Criterion framework.

use std::path::Path;
use std::time::Duration;

use criterion::{measurement::Measurement, BatchSize, BenchmarkGroup, Criterion, SamplingMode};
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
use criterion_perf_events::Perf;
use enumset::EnumSet;
use indexmap::IndexMap;
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
use perfcnt::linux::{
	HardwareEventType, PerfCounterBuilderLinux as PerfCounterBuilder, SoftwareEventType
};
use tempfile::NamedTempFile;

use pack_dataset::PackDataset;
use packsquash::{
	config::{FileOptions, GlobalOptions, ProcessedSquashOptions, SquashOptions},
	vfs::os_fs::OsFilesystem,
	PackSquasher
};

mod pack_dataset;

#[macro_use]
mod macros;

/// Benchmarks the optimization of an empty pack. This is primarily useful to measure
/// pack-independent initialization overhead.
fn empty_pack<M: Measurement + 'static>(
	benchmark_group: &mut BenchmarkGroup<'_, M>,
	pack_dataset: &mut PackDataset
) {
	benchmark_group.bench_function("empty_pack", |b| {
		b.iter_batched(
			|| {
				squash_options(
					pack_dataset,
					"empty.tar.xz",
					GlobalOptions::default(),
					IndexMap::default()
				)
			},
			squash_pack,
			BatchSize::SmallInput
		);
	});
}

/// Benchmarks the optimization of a resource pack with ~15 files that has lots of custom
/// item models, textures and sounds.
fn aylas_khron_micro_pack<M: Measurement + 'static>(
	benchmark_group: &mut BenchmarkGroup<'_, M>,
	pack_dataset: &mut PackDataset
) {
	benchmark_group.bench_function("aylas_khron_micro_pack", |b| {
		b.iter_batched(
			|| {
				squash_options(
					pack_dataset,
					"private/aylas_khron_micro.tar.xz",
					GlobalOptions::default(),
					IndexMap::default()
				)
			},
			squash_pack,
			BatchSize::SmallInput
		);
	});
}

/// Benchmarks the optimization of a resource pack with ~15 files that has lots of models
/// and textures, and is meant to be used with mods.
fn jilchu_chronos_micro_pack<M: Measurement + 'static>(
	benchmark_group: &mut BenchmarkGroup<'_, M>,
	pack_dataset: &mut PackDataset
) {
	benchmark_group.bench_function("jilchu_chronos_micro_pack", |b| {
		b.iter_batched(
			|| {
				squash_options(
					pack_dataset,
					"private/jilchu_chronos_micro.tar.xz",
					{
						let mut global_options = GlobalOptions::default();

						global_options.allow_mods = EnumSet::all();

						global_options
					},
					IndexMap::default()
				)
			},
			squash_pack,
			BatchSize::SmallInput
		);
	});
}

/// Benchmarks the optimization of a resource pack with ~15 files that has lots of assets
/// of every type, meant to be used with quite a bit of mods.
fn aiamded_breadstick_micro_pack<M: Measurement + 'static>(
	benchmark_group: &mut BenchmarkGroup<'_, M>,
	pack_dataset: &mut PackDataset
) {
	benchmark_group.bench_function("aiamded_breadstick_micro_pack", |b| {
		b.iter_batched(
			|| {
				squash_options(
					pack_dataset,
					"private/aiamded_breadstick_micro.tar.xz",
					{
						let mut global_options = GlobalOptions::default();

						global_options.allow_mods = EnumSet::all();

						global_options
					},
					IndexMap::default()
				)
			},
			squash_pack,
			BatchSize::SmallInput
		);
	});
}

custom_criterion_group! {
	name = tiny_benches_wall_time;
	config = Criterion::default()
		.warm_up_time(Duration::from_secs(1));
	sampling_mode = SamplingMode::Auto;
	targets = empty_pack
}

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
custom_criterion_group! {
	name = tiny_benches_instruction_count;
	config = Criterion::default()
		.warm_up_time(Duration::from_secs(1))
		.sample_size(10)
		.with_measurement(Perf::new(PerfCounterBuilder::from_hardware_event(HardwareEventType::Instructions)));
	// This perf counter is highly deterministic and little sensitive to external noise, so fewer samples are okay
	sampling_mode = SamplingMode::Flat;
	targets = empty_pack
}

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
custom_criterion_group! {
	name = tiny_benches_context_switches;
	config = Criterion::default()
		.warm_up_time(Duration::from_secs(1))
		.sample_size(10)
		.with_measurement(Perf::new(PerfCounterBuilder::from_software_event(SoftwareEventType::ContextSwitches)));
	// This perf counter is highly deterministic and little sensitive to external noise, so fewer samples are okay
	sampling_mode = SamplingMode::Flat;
	targets = empty_pack
}

custom_criterion_group! {
	name = small_benches_wall_time;
	config = Criterion::default()
		.warm_up_time(Duration::from_secs(15))
		.measurement_time(Duration::from_secs(45))
		.sample_size(15);
	sampling_mode = SamplingMode::Auto;
	targets = aylas_khron_micro_pack, jilchu_chronos_micro_pack, aiamded_breadstick_micro_pack
}

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
custom_criterion_group! {
	name = small_benches_instruction_count;
	config = Criterion::default()
		.warm_up_time(Duration::from_secs(15))
		.measurement_time(Duration::from_secs(45))
		.sample_size(10)
		.with_measurement(Perf::new(PerfCounterBuilder::from_hardware_event(HardwareEventType::Instructions)));
	// This perf counter is highly deterministic and little sensitive to external noise, so fewer samples are okay
	sampling_mode = SamplingMode::Flat;
	targets = aylas_khron_micro_pack, jilchu_chronos_micro_pack, aiamded_breadstick_micro_pack
}

#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
custom_criterion_group! {
	name = small_benches_context_switches;
	config = Criterion::default()
		.warm_up_time(Duration::from_secs(15))
		.measurement_time(Duration::from_secs(45))
		.sample_size(10)
		.with_measurement(Perf::new(PerfCounterBuilder::from_software_event(SoftwareEventType::ContextSwitches)));
	// This perf counter is highly deterministic and little sensitive to external noise, so fewer samples are okay
	sampling_mode = SamplingMode::Flat;
	targets = aylas_khron_micro_pack, jilchu_chronos_micro_pack, aiamded_breadstick_micro_pack
}

#[cfg(not(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64"))))]
custom_criterion_main!(tiny_benches_wall_time, small_benches_wall_time);
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
custom_criterion_main!(
	tiny_benches_wall_time,
	small_benches_wall_time,
	// Linux-specific benchmarks here
	tiny_benches_instruction_count,
	tiny_benches_context_switches,
	small_benches_instruction_count,
	small_benches_context_switches
);

/// Optimizes a pack, as configured in the specified options. This function is meant to be
/// benchmarked directly.
#[inline(always)]
fn squash_pack(squash_options: ProcessedSquashOptions) {
	PackSquasher::new()
		.run(OsFilesystem, squash_options, None)
		.expect("The optimization should be successful");
}

/// Returns the [`SquashOptions`] to use for optimizing a pack identified by its path,
/// initialized using the specified global and file options. The output file path will
/// be set to point to the temporary files directory.
fn squash_options<'dataset, P: AsRef<Path> + ?Sized>(
	pack_dataset: &mut PackDataset<'dataset>,
	relative_pack_path: &'dataset P,
	mut global_options: GlobalOptions,
	file_options: IndexMap<String, FileOptions>
) -> ProcessedSquashOptions {
	let (_, temporary_output_path) = NamedTempFile::new().unwrap().keep().unwrap();
	global_options.output_file_path = temporary_output_path;

	ProcessedSquashOptions::try_from(SquashOptions {
		pack_directory: pack_dataset.get(relative_pack_path).unwrap().path().into(),
		global_options,
		file_options
	})
	.unwrap()
}
