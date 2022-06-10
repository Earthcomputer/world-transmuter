use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2550;

static DEFAULTS: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, StructureFeatureConfiguration>> = SyncOnceCell::new();

fn defaults() -> &'static rust_dataconverter_engine::Map<&'static str, StructureFeatureConfiguration> {
    DEFAULTS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:village", StructureFeatureConfiguration::new(32, 8, 10387312));
        map.insert("minecraft:desert_pyramid", StructureFeatureConfiguration::new(32, 8, 14357617));
        map.insert("minecraft:igloo", StructureFeatureConfiguration::new(32, 8, 14357618));
        map.insert("minecraft:jungle_pyramid", StructureFeatureConfiguration::new(32, 8, 14357619));
        map.insert("minecraft:swamp_hut", StructureFeatureConfiguration::new(32, 8, 14357620));
        map.insert("minecraft:pillager_outpost", StructureFeatureConfiguration::new(32, 8, 165745296));
        map.insert("minecraft:monument", StructureFeatureConfiguration::new(32, 5, 10387313));
        map.insert("minecraft:endcity", StructureFeatureConfiguration::new(20, 11, 10387313));
        map.insert("minecraft:mansion", StructureFeatureConfiguration::new(80, 20, 10387319));
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.world_gen_settings.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let seed = data.get_i64("RandomSeed").unwrap_or(0);
        let generator_name = data.get_string("generatorName").map(|str| str.to_lowercase());
        let legacy_custom_options = data.get_string("legacy_custom_options").or_else(|| {
            if generator_name.as_deref() == Some("customized") {
                data.get_string("generatorOptions")
            } else {
                None
            }
        }).map(|str| str.to_owned());

        let mut caves = false;

        let generator = match generator_name.as_deref() {
            Some("customized") | None => default_overworld::<T>(seed),
            Some("flat") => {
                let mut generator_options = data.get_map_mut("generatorOptions");
                let structures = fix_flat_structures::<T>(generator_options.as_deref());
                let mut settings = T::Map::create_empty();
                settings.set("structures", T::Object::create_map(structures));

                let layers = generator_options.as_mut().and_then(|o| o.remove("layers"))
                    .and_then(|o| o.into_list())
                    .unwrap_or_else(|| {
                        let mut layers = T::List::create_empty();
                        for (height, block) in [
                            (1, "minecraft:bedrock"),
                            (2, "minecraft:dirt"),
                            (1, "minecraft:grass_block"),
                        ] {
                            let mut layer = T::Map::create_empty();
                            layer.set("height", T::Object::create_int(height));
                            layer.set("block", T::Object::create_string(block.to_owned()));
                            layers.add(T::Object::create_map(layer));
                        }
                        layers
                    });
                settings.set("layers", T::Object::create_list(layers));
                settings.set("biome", T::Object::create_string(generator_options.and_then(|o| o.get_string("biome")).unwrap_or("minecraft:plains").to_owned()));

                let mut generator = T::Map::create_empty();
                generator.set("type", T::Object::create_string("minecraft:flat".to_owned()));
                generator.set("settings", T::Object::create_map(settings));

                generator
            }
            Some("debug_all_block_states") => {
                let mut generator = T::Map::create_empty();
                generator.set("type", T::Object::create_string("minecraft:debug".to_owned()));
                generator
            }
            Some("buffet") => {
                let mut generator_options = data.get_map_mut("generatorOptions");
                let chunk_generator_type = generator_options.as_ref().and_then(|o| o.get_map("chunk_generator"))
                    .and_then(|o| o.get_string("type"));
                let new_type = match chunk_generator_type {
                    Some("minecraft:caves") => {
                        caves = true;
                        "minecraft:caves"
                    }
                    Some("minecraft:floating_islands") => "minecraft:floating_islands",
                    _ => "minecraft:overworld",
                };

                let mut biome_source = generator_options.as_mut().and_then(|o| o.remove("biome_source"))
                    .and_then(|o| o.into_map())
                    .unwrap_or_else(|| {
                        let mut biome_source = T::Map::create_empty();
                        biome_source.set("type", T::Object::create_string("minecraft:fixed".to_owned()));
                        biome_source
                    });

                if biome_source.get_string("type") == Some("minecraft:fixed") {
                    let biome = biome_source.get_map("options")
                        .and_then(|o| o.get_list("biomes"))
                        .and_then(|o| if o.is_empty() { None } else { o.get(0).as_string() })
                        .unwrap_or("minecraft:ocean")
                        .to_owned();
                    biome_source.remove("options");
                    biome_source.set("biome", T::Object::create_string(biome));
                }

                noise::<T>(seed, new_type, biome_source)
            }
            _ => {
                let default_gen = generator_name.as_deref() == Some("default");
                let default_11_gen = generator_name.as_deref() == Some("default_1_1") || (default_gen && data.get_i64("generatorVersion").unwrap_or(0) as i32 == 0);
                let amplified = generator_name.as_deref() == Some("amplified");
                let large_biomes = generator_name.as_deref() == Some("largebiomes");
                noise::<T>(seed, if amplified { "minecraft:amplified" } else { "minecraft:overworld" }, vanilla_biome_source::<T>(seed, default_11_gen, large_biomes))
            }
        };

        let map_features = data.get_bool("MapFeatures").unwrap_or(true);
        let bonus_chest = data.get_bool("BonusChest").unwrap_or(false);

        let mut ret = T::Map::create_empty();
        ret.set("seed", T::Object::create_long(seed));
        ret.set("generate_features", T::Object::create_bool(map_features));
        ret.set("bonus_chest", T::Object::create_bool(bonus_chest));
        ret.set("dimensions", T::Object::create_map(vanilla_levels::<T>(seed, generator, caves)));
        if let Some(legacy_custom_options) = legacy_custom_options {
            ret.set("legacy_custom_options", T::Object::create_string(legacy_custom_options));
        }

        *data = ret;
    }));
}

fn noise<T: Types + ?Sized>(seed: i64, world_type: &str, biome_source: T::Map) -> T::Map {
    let mut ret = T::Map::create_empty();
    ret.set("type", T::Object::create_string("minecraft:noise".to_owned()));
    ret.set("biome_source", T::Object::create_map(biome_source));
    ret.set("seed", T::Object::create_long(seed));
    ret.set("settings", T::Object::create_string(world_type.to_owned()));
    ret
}

fn vanilla_biome_source<T: Types + ?Sized>(seed: i64, default_11_gen: bool, large_biomes: bool) -> T::Map {
    let mut ret = T::Map::create_empty();
    ret.set("type", T::Object::create_string("minecraft:vanilla_layered".to_owned()));
    ret.set("seed", T::Object::create_long(seed));
    ret.set("large_biomes", T::Object::create_bool(large_biomes));
    if default_11_gen {
        ret.set("legacy_biome_init_layer", T::Object::create_bool(true));
    }
    ret
}

fn fix_flat_structures<T: Types + ?Sized>(generator_options: Option<&T::Map>) -> T::Map {
    let mut distance = 32;
    let mut spread = 3;
    let mut count = 128;
    let mut stronghold = false;
    let mut new_structures = rust_dataconverter_engine::Map::new();

    if generator_options.is_none() {
        stronghold = true;
        new_structures.insert("minecraft:village", defaults().get("minecraft:village").unwrap().clone());
    }

    if let Some(old_structures) = generator_options.and_then(|options| options.get_map("structures")) {
        for structure_name in old_structures.keys() {
            let structure_name: &str = structure_name;
            if let Some(structure_values) = old_structures.get_map(structure_name) {
                match structure_name {
                    "stronghold" => {
                        stronghold = true;
                        distance = structure_values.get_string("distance").and_then(|str| str.parse().ok()).unwrap_or(distance).max(1);
                        spread = structure_values.get_string("spread").and_then(|str| str.parse().ok()).unwrap_or(spread).max(1);
                        count = structure_values.get_string("count").and_then(|str| str.parse().ok()).unwrap_or(count).max(1);
                    }
                    "village" => {
                        set_spacing(&mut new_structures, "minecraft:village", structure_values.get_string("distance"), 9);
                    }
                    "biome_1" => {
                        set_spacing(&mut new_structures, "minecraft:desert_pyramid", structure_values.get_string("distance"), 9);
                        set_spacing(&mut new_structures, "minecraft:igloo", structure_values.get_string("distance"), 9);
                        set_spacing(&mut new_structures, "minecraft:jungle_pyramid", structure_values.get_string("distance"), 9);
                        set_spacing(&mut new_structures, "minecraft:swamp_hut", structure_values.get_string("distance"), 9);
                        set_spacing(&mut new_structures, "minecraft:pillager_outpost", structure_values.get_string("distance"), 9);
                    }
                    "endcity" => {
                        set_spacing(&mut new_structures, "minecraft:endcity", structure_values.get_string("distance"), 1);
                    }
                    "mansion" => {
                        set_spacing(&mut new_structures, "minecraft:mansion", structure_values.get_string("distance"), 1);
                    }
                    "oceanmonument" => {
                        // Vanilla and Paper overwrite the spacing with the separation here, we set the separation to the separation
                        let structure = new_structures.get("minecraft:monument").unwrap_or_else(|| defaults().get("minecraft:monument").expect("Missing minecraft:monument in defaults"));
                        let new_spacing = structure_values.get_string("spacing").and_then(|str| str.parse().ok()).unwrap_or(structure.spacing).max(1);
                        let new_separation = structure_values.get_string("separation").and_then(|str| str.parse().ok()).unwrap_or(structure.separation).max(1);
                        let new_structure = StructureFeatureConfiguration::new(new_spacing, new_separation, structure.salt);
                        new_structures.insert("minecraft:monument", new_structure);
                    }
                    _ => {}
                }
            }
        }
    }

    let mut ret = T::Map::create_empty();
    let mut structures_serialized = T::Map::create_empty();
    for (key, value) in new_structures {
        structures_serialized.set(key, T::Object::create_map(value.serialize::<T>()));
    }
    ret.set("structures", T::Object::create_map(structures_serialized));

    if stronghold {
        let mut stronghold_data = T::Map::create_empty();
        stronghold_data.set("distance", T::Object::create_int(distance));
        stronghold_data.set("spread", T::Object::create_int(spread));
        stronghold_data.set("count", T::Object::create_int(count));
        ret.set("stronghold", T::Object::create_map(stronghold_data));
    }

    ret
}

fn vanilla_levels<T: Types + ?Sized>(seed: i64, generator: T::Map, caves: bool) -> T::Map {
    let mut ret = T::Map::create_empty();

    let mut overworld = T::Map::create_empty();
    let mut nether = T::Map::create_empty();
    let mut end = T::Map::create_empty();

    overworld.set("type", T::Object::create_string(if caves { "minecraft:overworld_caves".to_owned() } else { "minecraft:overworld".to_owned() }));
    overworld.set("generator", T::Object::create_map(generator));

    nether.set("type", T::Object::create_string("minecraft:the_nether".to_owned()));
    let mut nether_biome_source = T::Map::create_empty();
    nether_biome_source.set("type", T::Object::create_string("minecraft:multi_noise".to_owned()));
    nether_biome_source.set("seed", T::Object::create_long(seed));
    nether_biome_source.set("preset", T::Object::create_string("minecraft:nether".to_owned()));
    nether.set("generator", T::Object::create_map(noise::<T>(seed, "minecraft:nether", nether_biome_source)));

    end.set("type", T::Object::create_string("minecraft:the_end".to_owned()));
    let mut end_biome_source = T::Map::create_empty();
    end_biome_source.set("type", T::Object::create_string("minecraft:the_end".to_owned()));
    end_biome_source.set("seed", T::Object::create_long(seed));
    end.set("generator", T::Object::create_map(noise::<T>(seed, "minecraft:end", end_biome_source)));

    ret.set("minecraft:overworld", T::Object::create_map(overworld));
    ret.set("minecraft:the_nether", T::Object::create_map(nether));
    ret.set("minecraft:the_end", T::Object::create_map(end));

    ret
}

fn default_overworld<T: Types + ?Sized>(seed: i64) -> T::Map {
    noise::<T>(seed, "minecraft:overworld", vanilla_biome_source::<T>(seed, false, false))
}

fn set_spacing<'a>(
    structures: &mut rust_dataconverter_engine::Map<&'a str, StructureFeatureConfiguration>,
    structure_name: &'a str,
    structure_value: Option<&str>,
    min_val: i32
) {
    let structure = structures.get(structure_name).unwrap_or_else(|| defaults().get(structure_name).expect("Unknown structure given to set_spacing"));
    let new_spacing = structure_value.and_then(|str| str.parse().ok()).unwrap_or(structure.spacing).max(min_val);
    let new_structure = StructureFeatureConfiguration::new(new_spacing, structure.separation, structure.salt);
    structures.insert(structure_name, new_structure);
}

#[derive(Clone, Debug)]
struct StructureFeatureConfiguration {
    spacing: i32,
    separation: i32,
    salt: i32,
}

impl StructureFeatureConfiguration {
    fn new(spacing: i32, separation: i32, salt: i32) -> Self {
        Self { spacing, separation, salt }
    }

    fn serialize<T: Types + ?Sized>(&self) -> T::Map {
        let mut map = T::Map::create_empty();
        map.set("spacing", T::Object::create_int(self.spacing));
        map.set("separation", T::Object::create_int(self.separation));
        map.set("salt", T::Object::create_int(self.salt));
        map
    }
}
