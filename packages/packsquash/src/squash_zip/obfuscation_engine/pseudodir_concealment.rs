use crate::{config::compile_pack_file_glob_pattern, RelativePath};
use globset::{GlobSet, GlobSetBuilder};

static KNOWN_LISTED_RESOURCE_PATTERNS: &[&str] = &[
	"assets/realms/textures/gui/images/**/*.png",
	"assets/*/models/**/*.json",
	"assets/*/blockstates/**/*.json",
	"assets/*/sounds/**/*.ogg",
	"assets/*/font/**/*.json",
	"assets/*/shaders/**/*.{json,fsh,vsh,glsl}",
	"assets/*/particles/**/*.json",
	// Valid precise globs as of 1.20.2. Use more generic glob for forwards compatibility.
	// The following precise globs match most of the possible datapack contents:
	// "data/*/dimension_type/**/*.json",
	// "data/*/chat_type/**/*.json",
	// "data/*/trim_pattern/**/*.json",
	// "data/*/trim_material/**/*.json",
	// "data/*/damage_type/**/*.json",
	// "data/*/dimension/**/*.json",
	// "data/*/worldgen/biome/**/*.json",
	// "data/*/worldgen/configured_carver/**/*.json",
	// "data/*/worldgen/configured_feature/**/*.json",
	// "data/*/worldgen/placed_feature/**/*.json",
	// "data/*/worldgen/structure/**/*.json",
	// "data/*/worldgen/structure_set/**/*.json",
	// "data/*/worldgen/processor_list/**/*.json",
	// "data/*/worldgen/template_pool/**/*.json",
	// "data/*/worldgen/noise_settings/**/*.json",
	// "data/*/worldgen/noise/**/*.json",
	// "data/*/worldgen/density_function/**/*.json",
	// "data/*/worldgen/world_preset/**/*.json",
	// "data/*/worldgen/flat_level_generator_preset/**/*.json",
	// "data/*/worldgen/multi_noise_biome_source_parameter_list/**/*.json",
	// "data/*/worldgen/**/*.json",
	// "data/*/predicates/*/**.json",
	// "data/*/item_modifiers/*/**.json",
	// "data/*/loot_tables/*/**.json",
	// "data/*/structures/**/*.nbt",
	// "data/*/recipes/**/*.json",
	// "data/*/advancements/**/*.json",
	// "data/*/tags/**/*.json"
	"data/*/**/*.json",
	"data/*/functions/**/*.mcfunction"
];

#[derive(Copy, Clone)]
pub struct FileListingCircumstances {
	pub is_atlas_texture: bool
}

pub struct PseudodirConcealer {
	known_listed_resources: GlobSet
}

impl PseudodirConcealer {
	pub fn new() -> Self {
		let mut globset_builder = GlobSetBuilder::new();
		for pattern in KNOWN_LISTED_RESOURCE_PATTERNS {
			globset_builder.add(compile_pack_file_glob_pattern(pattern).unwrap());
		}
		Self {
			known_listed_resources: globset_builder.build().unwrap()
		}
	}

	pub fn conceal(&self, path: &mut RelativePath, circumstances: FileListingCircumstances) {
		if self.is_concealable(path, circumstances) {
			path.as_inner_mut().to_mut().push('/');
		}
	}

	fn is_concealable(&self, path: &RelativePath, circumstances: FileListingCircumstances) -> bool {
		!circumstances.is_atlas_texture && !self.known_listed_resources.is_match(path)
	}
}
