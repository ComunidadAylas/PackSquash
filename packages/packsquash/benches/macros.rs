//! Macro definitions for benchmarks.
#![allow(unused_macros)] // The macros are actually used, but rustc complains anyway

/// Like [`criterion::criterion_group`], but passes an additional `PackDataset`
/// parameter to the benchmark functions, groups the target functions in an actual
/// benchmark group, and allows changing the sampling mode of the benchmark group.
macro_rules! custom_criterion_group {
    (name = $name:ident; config = $config:expr; sampling_mode = $sampling_mode:expr; targets = $( $target:path ),+ $(,)*) => {
        pub fn $name(pack_dataset: &mut $crate::pack_dataset::PackDataset) {
            let mut criterion: ::criterion::Criterion<_> = $config
                .configure_from_args();

			let mut benchmark_group = criterion.benchmark_group(stringify!($name));
            benchmark_group.sampling_mode($sampling_mode);

            $(
                $target(&mut benchmark_group, pack_dataset);
            )+
        }
    };
    ($name:ident, $( $target:path ),+ $(,)*) => {
        $crate::criterion_group! {
            name = $name;
            config = ::criterion::Criterion::default();
            sampling_mode = ::criterion::SamplingMode::Auto;
            targets = $( $target ),+
        }
    }
}

/// Like [`criterion::criterion_main`], but creates and passes an additional `PackDataset`
/// parameter to the benchmark groups.
macro_rules! custom_criterion_main {
    ( $( $group:path ),+ $(,)* ) => {
        fn main() {
			let mut pack_dataset = $crate::pack_dataset::PackDataset::new();

            $(
                $group(&mut pack_dataset);
            )+

            $crate::Criterion::default()
                .configure_from_args()
                .final_summary();
        }
    }
}
