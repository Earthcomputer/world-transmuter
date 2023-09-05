use crate::helpers::gson_lenient_fix::{fix_gson_lenient, FixedGsonLenient, JsonType};
use crate::helpers::json_parser;
use crate::types;
use ahash::AHashMap;
use log::warn;
use std::sync::OnceLock;
use valence_nbt::{compound, Compound, List, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1506;

static BIOME_MAP: OnceLock<AHashMap<&'static str, &'static str>> = OnceLock::new();

fn biome_map() -> &'static AHashMap<&'static str, &'static str> {
    BIOME_MAP.get_or_init(|| {
        let mut map = AHashMap::new();
        map.insert("0", "minecraft:ocean");
        map.insert("1", "minecraft:plains");
        map.insert("2", "minecraft:desert");
        map.insert("3", "minecraft:mountains");
        map.insert("4", "minecraft:forest");
        map.insert("5", "minecraft:taiga");
        map.insert("6", "minecraft:swamp");
        map.insert("7", "minecraft:river");
        map.insert("8", "minecraft:nether");
        map.insert("9", "minecraft:the_end");
        map.insert("10", "minecraft:frozen_ocean");
        map.insert("11", "minecraft:frozen_river");
        map.insert("12", "minecraft:snowy_tundra");
        map.insert("13", "minecraft:snowy_mountains");
        map.insert("14", "minecraft:mushroom_fields");
        map.insert("15", "minecraft:mushroom_field_shore");
        map.insert("16", "minecraft:beach");
        map.insert("17", "minecraft:desert_hills");
        map.insert("18", "minecraft:wooded_hills");
        map.insert("19", "minecraft:taiga_hills");
        map.insert("20", "minecraft:mountain_edge");
        map.insert("21", "minecraft:jungle");
        map.insert("22", "minecraft:jungle_hills");
        map.insert("23", "minecraft:jungle_edge");
        map.insert("24", "minecraft:deep_ocean");
        map.insert("25", "minecraft:stone_shore");
        map.insert("26", "minecraft:snowy_beach");
        map.insert("27", "minecraft:birch_forest");
        map.insert("28", "minecraft:birch_forest_hills");
        map.insert("29", "minecraft:dark_forest");
        map.insert("30", "minecraft:snowy_taiga");
        map.insert("31", "minecraft:snowy_taiga_hills");
        map.insert("32", "minecraft:giant_tree_taiga");
        map.insert("33", "minecraft:giant_tree_taiga_hills");
        map.insert("34", "minecraft:wooded_mountains");
        map.insert("35", "minecraft:savanna");
        map.insert("36", "minecraft:savanna_plateau");
        map.insert("37", "minecraft:badlands");
        map.insert("38", "minecraft:wooded_badlands_plateau");
        map.insert("39", "minecraft:badlands_plateau");
        map.insert("40", "minecraft:small_end_islands");
        map.insert("41", "minecraft:end_midlands");
        map.insert("42", "minecraft:end_highlands");
        map.insert("43", "minecraft:end_barrens");
        map.insert("44", "minecraft:warm_ocean");
        map.insert("45", "minecraft:lukewarm_ocean");
        map.insert("46", "minecraft:cold_ocean");
        map.insert("47", "minecraft:deep_warm_ocean");
        map.insert("48", "minecraft:deep_lukewarm_ocean");
        map.insert("49", "minecraft:deep_cold_ocean");
        map.insert("50", "minecraft:deep_frozen_ocean");
        map.insert("127", "minecraft:the_void");
        map.insert("129", "minecraft:sunflower_plains");
        map.insert("130", "minecraft:desert_lakes");
        map.insert("131", "minecraft:gravelly_mountains");
        map.insert("132", "minecraft:flower_forest");
        map.insert("133", "minecraft:taiga_mountains");
        map.insert("134", "minecraft:swamp_hills");
        map.insert("140", "minecraft:ice_spikes");
        map.insert("149", "minecraft:modified_jungle");
        map.insert("151", "minecraft:modified_jungle_edge");
        map.insert("155", "minecraft:tall_birch_forest");
        map.insert("156", "minecraft:tall_birch_hills");
        map.insert("157", "minecraft:dark_forest_hills");
        map.insert("158", "minecraft:snowy_taiga_mountains");
        map.insert("160", "minecraft:giant_spruce_taiga");
        map.insert("161", "minecraft:giant_spruce_taiga_hills");
        map.insert("162", "minecraft:modified_gravelly_mountains");
        map.insert("163", "minecraft:shattered_savanna");
        map.insert("164", "minecraft:shattered_savanna_plateau");
        map.insert("165", "minecraft:eroded_badlands");
        map.insert("166", "minecraft:modified_wooded_badlands_plateau");
        map.insert("167", "minecraft:modified_badlands_plateau");
        map
    })
}

pub(crate) fn register() {
    types::level_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let generator_options = match data.get("generatorOptions") {
                Some(Value::String(str)) => Some(&str[..]),
                _ => None,
            };
            let generator_name = match data.get("generatorName") {
                Some(Value::String(str)) => Some(&str[..]),
                _ => None,
            };
            match (generator_name, generator_options) {
                (Some("flat"), _) => {
                    data.insert("generatorOptions", convert(generator_options.unwrap_or("")));
                }
                (Some("buffet"), Some(generator_options)) => {
                    if let Ok(FixedGsonLenient {
                        value_type: JsonType::Object,
                        fixed_str: fixed_gson,
                    }) = fix_gson_lenient(generator_options)
                    {
                        if let Ok(result) = json_parser::parse_compound(&fixed_gson, false) {
                            data.insert("generatorOptions", result);
                            return;
                        }
                    }
                    warn!("Invalid generatorOptions syntax: {}", generator_options);
                }
                _ => {}
            };
        }),
    );
}

fn convert(generator_settings: &str) -> Compound {
    let mut split_settings = generator_settings.split(';');
    let mut biome = "minecraft:plains";
    let mut structures = Compound::new();
    let layers = if let Some(layers) = split_settings
        .next()
        .filter(|_| !generator_settings.is_empty())
    {
        let layers = layers_info_from_string(layers);
        if !layers.is_empty() {
            // biomes is next
            if let Some(biome_id) = split_settings.next() {
                biome = biome_map()
                    .get(biome_id)
                    .copied()
                    .unwrap_or("minecraft:plains");
            }

            // structures is next
            if let Some(structures_str) = split_settings.next() {
                for structure_str in structures_str.to_lowercase().split(',') {
                    let (structure_name, structure_values) = match structure_str.find('(') {
                        Some(paren_index) => structure_str.split_at(paren_index),
                        None => (structure_str, ""),
                    };
                    structures.insert(structure_name, Compound::new());
                    if structure_values.ends_with(')') && structure_values.len() > 2 {
                        for kv in structure_values[1..structure_values.len() - 1].split(' ') {
                            if let Some(eq_index) = kv.find('=') {
                                let Some(Value::Compound(structure)) =
                                    structures.get_mut(structure_name)
                                else {
                                    unreachable!()
                                };
                                structure.insert(&kv[..eq_index], &kv[eq_index + 1..]);
                            }
                        }
                    }
                }
            } else {
                structures.insert("village", Compound::new());
            }
        }
        layers
    } else {
        structures.insert("village", Compound::new());
        vec![
            (1, "minecraft:bedrock".to_owned()),
            (2, "minecraft:dirt".to_owned()),
            (1, "minecraft:grass_block".to_owned()),
        ]
    };

    let layer_tag: Vec<_> = layers
        .into_iter()
        .map(|(height, block)| {
            compound! {
                "height" => height,
                "block" => block,
            }
        })
        .collect();

    compound! {
        "layers" => List::Compound(layer_tag),
        "biome" => biome,
        "structures" => structures,
    }
}

fn layer_info_from_string(layer_string: &str) -> Option<(i32, String)> {
    match layer_string.find('*') {
        Some(star_index) => Some((
            layer_string[..star_index].parse().ok()?,
            layer_string[star_index + 1..].to_owned(),
        )),
        None => Some((1, layer_string.to_owned())),
    }
}

fn layers_info_from_string(layers_string: &str) -> Vec<(i32, String)> {
    let mut ret = Vec::new();
    for layer in layers_string.split(',') {
        if let Some(layer) = layer_info_from_string(layer) {
            ret.push(layer);
        } else {
            return Vec::new();
        }
    }
    ret
}
