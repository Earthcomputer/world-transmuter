use std::collections::BTreeMap;
use std::sync::OnceLock;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{compound, Compound, List, Value};
use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2550;

static DEFAULTS: OnceLock<McNamespaceMap<StructureFeatureConfiguration>> = OnceLock::new();

fn defaults() -> &'static McNamespaceMap<'static, StructureFeatureConfiguration> {
    DEFAULTS.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("village", StructureFeatureConfiguration::new(32, 8, 10387312));
        map.insert_mc("desert_pyramid", StructureFeatureConfiguration::new(32, 8, 14357617));
        map.insert_mc("igloo", StructureFeatureConfiguration::new(32, 8, 14357618));
        map.insert_mc("jungle_pyramid", StructureFeatureConfiguration::new(32, 8, 14357619));
        map.insert_mc("swamp_hut", StructureFeatureConfiguration::new(32, 8, 14357620));
        map.insert_mc("pillager_outpost", StructureFeatureConfiguration::new(32, 8, 165745296));
        map.insert_mc("monument", StructureFeatureConfiguration::new(32, 5, 10387313));
        map.insert_mc("endcity", StructureFeatureConfiguration::new(20, 11, 10387313));
        map.insert_mc("mansion", StructureFeatureConfiguration::new(80, 20, 10387319));
        map
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.world_gen_settings.borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        let seed = data.get("RandomSeed").and_then(|v| v.as_i64()).unwrap_or(0);
        let generator_name = match data.get("generatorName") {
            Some(Value::String(generator_name)) => Some(generator_name.to_lowercase()),
            _ => None,
        };
        let legacy_custom_options = match data.get("legacy_custom_options") {
            Some(Value::String(legacy_custom_options)) => Some(legacy_custom_options.clone()),
            _ => if generator_name.as_deref() == Some("customized") {
                match data.get("generatorOptions") {
                    Some(Value::String(generator_options)) => Some(generator_options.clone()),
                    _ => None,
                }
            } else {
                None
            }
        };

        let mut caves = false;

        let generator = match generator_name.as_deref() {
            Some("customized") | None => default_overworld(seed),
            Some("flat") => {
                let mut generator_options = match data.get_mut("generatorOptions") {
                    Some(Value::Compound(generator_options)) => Some(generator_options),
                    _ => None,
                };
                let structures = fix_flat_structures(generator_options.as_deref());
                let layers = generator_options.as_mut().and_then(|o| o.remove("layers"))
                    .and_then(|o| match o {
                        Value::List(o) => Some(o),
                        _ => None,
                    })
                    .unwrap_or_else(|| {
                        List::Compound(vec![
                            compound! {
                                "height" => 1,
                                "block" => "minecraft:bedrock",
                            },
                            compound! {
                                "height" => 2,
                                "block" => "minecraft:dirt",
                            },
                            compound! {
                                "height" => 1,
                                "block" => "minecraft:grass_block",
                            }
                        ])
                    });
                let biome = generator_options.and_then(|v| match v.get("biome") {
                    Some(Value::String(str)) => Some(str.clone()),
                    _ => None,
                }).unwrap_or_else(|| "minecraft:plains".to_owned());

                compound! {
                    "type" => "minecraft:flat",
                    "settings" => compound! {
                        "structures" => structures,
                        "layers" => layers,
                        "biome" => biome,
                    }
                }
            }
            Some("debug_all_block_states") => compound! {
                "type" => "minecraft:debug"
            },
            Some("buffet") => {
                let generator_options = match data.get_mut("generatorOptions") {
                    Some(Value::Compound(generator_options)) => Some(generator_options),
                    _ => None,
                };
                let chunk_generator_type = generator_options.as_ref().and_then(|generator_options| match generator_options.get("chunk_generator") {
                    Some(Value::Compound(chunk_generator)) => match chunk_generator.get("type") {
                        Some(Value::String(typ)) => Some(&typ[..]),
                        _ => None,
                    }
                    _ => None,
                });
                let new_type = match chunk_generator_type {
                    Some("minecraft:caves") => {
                        caves = true;
                        "minecraft:caves"
                    }
                    Some("minecraft:floating_islands") => "minecraft:floating_islands",
                    _ => "minecraft:overworld",
                };

                let mut biome_source = generator_options.and_then(|generator_options| match generator_options.remove("biome_source") {
                    Some(Value::Compound(biome_source)) => Some(biome_source),
                    _ => None,
                }).unwrap_or_else(|| compound! {
                    "type" => "minecraft:fixed",
                });

                if matches!(biome_source.get("type"), Some(Value::String(str)) if str == "minecraft:fixed") {
                    let biome = match biome_source.remove("options") {
                        Some(Value::Compound(mut options)) => match options.remove("biomes") {
                            Some(Value::List(List::String(mut biomes))) if !biomes.is_empty() => biomes.remove(0),
                            _ => "minecraft:ocean".to_owned(),
                        }
                        _ => "minecraft:ocean".to_owned(),
                    };
                    biome_source.insert("biome", biome);
                }

                noise(seed, new_type, biome_source)
            }
            _ => {
                let default_gen = generator_name.as_deref() == Some("default");
                let default_11_gen = generator_name.as_deref() == Some("default_1_1") || (default_gen && data.get("generatorVersion").and_then(|v| v.as_i32()).unwrap_or(0) == 0);
                let amplified = generator_name.as_deref() == Some("amplified");
                let large_biomes = generator_name.as_deref() == Some("largebiomes");
                noise(seed, if amplified { "minecraft:amplified" } else { "minecraft:overworld" }, vanilla_biome_source(seed, default_11_gen, large_biomes))
            }
        };

        let map_features = data.get("MapFeatures").and_then(|v| v.as_bool()).unwrap_or(true);
        let bonus_chest = data.get("BonusChest").and_then(|v| v.as_bool()).unwrap_or(false);

        let mut ret = compound! {
            "seed" => seed,
            "generate_features" => map_features,
            "bonus_chest" => bonus_chest,
            "dimensions" => vanilla_levels(seed, generator, caves),
        };
        if let Some(legacy_custom_options) = legacy_custom_options {
            ret.insert("legacy_custom_options", legacy_custom_options);
        }

        *data = ret;
    }));
}

fn noise(seed: i64, world_type: &str, biome_source: Compound) -> Compound {
    compound! {
        "type" => "minecraft:noise",
        "biome_source" => biome_source,
        "seed" => seed,
        "settings" => world_type,
    }
}

fn vanilla_biome_source(seed: i64, default_11_gen: bool, large_biomes: bool) -> Compound {
    let mut ret = compound! {
        "type" => "minecraft:vanilla_layered",
        "seed" => seed,
        "large_biomes" => large_biomes,
    };
    if default_11_gen {
        ret.insert("legacy_biome_init_layer", true);
    }
    ret
}

fn fix_flat_structures(generator_options: Option<&Compound>) -> Compound {
    let mut distance = 32;
    let mut spread = 3;
    let mut count = 128;
    let mut stronghold = false;
    let mut new_structures = BTreeMap::new();

    if generator_options.is_none() {
        stronghold = true;
        new_structures.insert("minecraft:village", defaults().get("minecraft:village").unwrap().clone());
    }

    if let Some(Value::Compound(old_structures)) = generator_options.and_then(|options| options.get("structures")) {
        for (structure_name, structure_values) in old_structures {
            let Value::Compound(structure_values) = structure_values else { continue };
            match &structure_name[..] {
                "stronghold" => {
                    stronghold = true;
                    if let Some(Value::String(d)) = structure_values.get("distance") {
                        if let Ok(d) = d.parse::<i32>() {
                            distance = d.max(1);
                        }
                    }
                    if let Some(Value::String(s)) = structure_values.get("spread") {
                        if let Ok(s) = s.parse::<i32>() {
                            spread = s.max(1);
                        }
                    }
                    if let Some(Value::String(c)) = structure_values.get("count") {
                        if let Ok(c) = c.parse::<i32>() {
                            count = c.max(1);
                        }
                    }
                }
                "village" => {
                    set_spacing(&mut new_structures, "minecraft:village", structure_values.get("distance"), 9);
                }
                "biome_1" => {
                    set_spacing(&mut new_structures, "minecraft:desert_pyramid", structure_values.get("distance"), 9);
                    set_spacing(&mut new_structures, "minecraft:igloo", structure_values.get("distance"), 9);
                    set_spacing(&mut new_structures, "minecraft:jungle_pyramid", structure_values.get("distance"), 9);
                    set_spacing(&mut new_structures, "minecraft:swamp_hut", structure_values.get("distance"), 9);
                    set_spacing(&mut new_structures, "minecraft:pillager_outpost", structure_values.get("distance"), 9);
                }
                "endcity" => {
                    set_spacing(&mut new_structures, "minecraft:endcity", structure_values.get("distance"), 1);
                }
                "mansion" => {
                    set_spacing(&mut new_structures, "minecraft:mansion", structure_values.get("distance"), 1);
                }
                "oceanmonument" => {
                    // Vanilla and Paper overwrite the spacing with the separation here, we set the separation to the separation
                    let structure = new_structures.get("minecraft:monument").unwrap_or_else(|| defaults().get("minecraft:monument").expect("Missing minecraft:monument in defaults"));
                    let new_spacing = match structure_values.get("spacing") {
                        Some(Value::String(spacing)) => spacing.parse().ok(),
                        _ => None,
                    }.unwrap_or(structure.spacing).max(1);
                    let new_separation = match structure_values.get("separation") {
                        Some(Value::String(separation)) => separation.parse().ok(),
                        _ => None,
                    }.unwrap_or(structure.separation).max(1);
                    let new_structure = StructureFeatureConfiguration::new(new_spacing, new_separation, structure.salt);
                    new_structures.insert("minecraft:monument", new_structure);
                }
                _ => {}
            }
        }
    }

    let structures_serialized: Compound = new_structures.into_iter().map(|(key, value)| (key.to_owned(), Value::Compound(value.serialize()))).collect();
    let mut ret = compound! {
        "structures" => structures_serialized,
    };

    if stronghold {
        let stronghold_data = compound! {
            "distance" => distance,
            "spread" => spread,
            "count" => count,
        };
        ret.insert("stronghold", stronghold_data);
    }

    ret
}

pub(crate) fn vanilla_levels(seed: i64, generator: Compound, caves: bool) -> Compound {
    compound! {
        "minecraft:overworld" => compound! {
            "type" => if caves { "minecraft:overworld_caves" } else { "minecraft:overworld" },
            "generator" => generator,
        },
        "minecraft:the_nether" => compound! {
            "type" => "minecraft:the_nether",
            "generator" => noise(seed, "minecraft:nether", compound! {
                "type" => "minecraft:multi_noise",
                "seed" => seed,
                "preset" => "minecraft:nether",
            }),
        },
        "minecraft:the_end" => compound! {
            "type" => "minecraft:the_end",
            "generator" => noise(seed, "minecraft:end", compound! {
                "type" => "minecraft:the_end",
                "seed" => seed,
            }),
        },
    }
}

pub(crate) fn default_overworld(seed: i64) -> Compound {
    noise(seed, "minecraft:overworld", vanilla_biome_source(seed, false, false))
}

fn set_spacing<'a>(
    structures: &mut BTreeMap<&'a str, StructureFeatureConfiguration>,
    structure_name: &'a str,
    structure_value: Option<&Value>,
    min_val: i32
) {
    let structure = structures.get(structure_name).unwrap_or_else(|| defaults().get(structure_name).expect("Unknown structure given to set_spacing"));
    let new_spacing = match structure_value {
        Some(Value::String(str)) => str.parse().ok(),
        _ => None,
    }.unwrap_or(structure.spacing).max(min_val);
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

    fn serialize(&self) -> Compound {
        compound! {
            "spacing" => self.spacing,
            "separation" => self.separation,
            "salt" => self.salt,
        }
    }
}
