use crate::helpers::gson_lenient_fix::{fix_gson_lenient, FixedGsonLenient, JsonType};
use crate::helpers::json_parser;
use crate::{static_string_map, types};
use java_string::{JavaStr, JavaString};
use tracing::warn;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{map_data_converter_func, JCompound, JList, JValue};

const VERSION: u32 = 1506;

static_string_map! {
    biome_map = {
        "0" => "minecraft:ocean",
        "1" => "minecraft:plains",
        "2" => "minecraft:desert",
        "3" => "minecraft:mountains",
        "4" => "minecraft:forest",
        "5" => "minecraft:taiga",
        "6" => "minecraft:swamp",
        "7" => "minecraft:river",
        "8" => "minecraft:nether",
        "9" => "minecraft:the_end",
        "10" => "minecraft:frozen_ocean",
        "11" => "minecraft:frozen_river",
        "12" => "minecraft:snowy_tundra",
        "13" => "minecraft:snowy_mountains",
        "14" => "minecraft:mushroom_fields",
        "15" => "minecraft:mushroom_field_shore",
        "16" => "minecraft:beach",
        "17" => "minecraft:desert_hills",
        "18" => "minecraft:wooded_hills",
        "19" => "minecraft:taiga_hills",
        "20" => "minecraft:mountain_edge",
        "21" => "minecraft:jungle",
        "22" => "minecraft:jungle_hills",
        "23" => "minecraft:jungle_edge",
        "24" => "minecraft:deep_ocean",
        "25" => "minecraft:stone_shore",
        "26" => "minecraft:snowy_beach",
        "27" => "minecraft:birch_forest",
        "28" => "minecraft:birch_forest_hills",
        "29" => "minecraft:dark_forest",
        "30" => "minecraft:snowy_taiga",
        "31" => "minecraft:snowy_taiga_hills",
        "32" => "minecraft:giant_tree_taiga",
        "33" => "minecraft:giant_tree_taiga_hills",
        "34" => "minecraft:wooded_mountains",
        "35" => "minecraft:savanna",
        "36" => "minecraft:savanna_plateau",
        "37" => "minecraft:badlands",
        "38" => "minecraft:wooded_badlands_plateau",
        "39" => "minecraft:badlands_plateau",
        "40" => "minecraft:small_end_islands",
        "41" => "minecraft:end_midlands",
        "42" => "minecraft:end_highlands",
        "43" => "minecraft:end_barrens",
        "44" => "minecraft:warm_ocean",
        "45" => "minecraft:lukewarm_ocean",
        "46" => "minecraft:cold_ocean",
        "47" => "minecraft:deep_warm_ocean",
        "48" => "minecraft:deep_lukewarm_ocean",
        "49" => "minecraft:deep_cold_ocean",
        "50" => "minecraft:deep_frozen_ocean",
        "127" => "minecraft:the_void",
        "129" => "minecraft:sunflower_plains",
        "130" => "minecraft:desert_lakes",
        "131" => "minecraft:gravelly_mountains",
        "132" => "minecraft:flower_forest",
        "133" => "minecraft:taiga_mountains",
        "134" => "minecraft:swamp_hills",
        "140" => "minecraft:ice_spikes",
        "149" => "minecraft:modified_jungle",
        "151" => "minecraft:modified_jungle_edge",
        "155" => "minecraft:tall_birch_forest",
        "156" => "minecraft:tall_birch_hills",
        "157" => "minecraft:dark_forest_hills",
        "158" => "minecraft:snowy_taiga_mountains",
        "160" => "minecraft:giant_spruce_taiga",
        "161" => "minecraft:giant_spruce_taiga_hills",
        "162" => "minecraft:modified_gravelly_mountains",
        "163" => "minecraft:shattered_savanna",
        "164" => "minecraft:shattered_savanna_plateau",
        "165" => "minecraft:eroded_badlands",
        "166" => "minecraft:modified_wooded_badlands_plateau",
        "167" => "minecraft:modified_badlands_plateau",
    }
}

pub(crate) fn register() {
    types::level_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let generator_options = match data.get("generatorOptions") {
                Some(JValue::String(str)) => Some(&str[..]),
                _ => None,
            };
            let generator_name = match data.get("generatorName") {
                Some(JValue::String(str)) => Some(&str[..]),
                _ => None,
            };
            match (generator_name.map(JavaStr::as_bytes), generator_options) {
                (Some(b"flat"), _) => {
                    data.insert(
                        "generatorOptions",
                        convert(generator_options.unwrap_or(JavaStr::from_str(""))),
                    );
                }
                (Some(b"buffet"), Some(generator_options)) => {
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

fn convert(generator_settings: &JavaStr) -> JCompound {
    let mut split_settings = generator_settings.split(';');
    let mut biome = JavaStr::from_str("minecraft:plains");
    let mut structures = JCompound::new();
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
                    .unwrap_or(JavaStr::from_str("minecraft:plains"));
            }

            // structures is next
            if let Some(structures_str) = split_settings.next() {
                for structure_str in structures_str.to_lowercase().split(',') {
                    let (structure_name, structure_values) = match structure_str.find('(') {
                        Some(paren_index) => structure_str.split_at(paren_index),
                        None => (structure_str, JavaStr::from_str("")),
                    };
                    structures.insert(structure_name, JCompound::new());
                    if structure_values.ends_with(')') && structure_values.len() > 2 {
                        for kv in structure_values[1..structure_values.len() - 1].split(' ') {
                            if let Some(eq_index) = kv.find('=') {
                                let Some(JValue::Compound(structure)) =
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
                structures.insert("village", JCompound::new());
            }
        }
        layers
    } else {
        structures.insert("village", JCompound::new());
        vec![
            (1, JavaString::from("minecraft:bedrock")),
            (2, JavaString::from("minecraft:dirt")),
            (1, JavaString::from("minecraft:grass_block")),
        ]
    };

    let layer_tag: Vec<_> = layers
        .into_iter()
        .map(|(height, block)| {
            jcompound! {
                "height" => height,
                "block" => block,
            }
        })
        .collect();

    jcompound! {
        "layers" => JList::Compound(layer_tag),
        "biome" => biome,
        "structures" => structures,
    }
}

fn layer_info_from_string(layer_string: &JavaStr) -> Option<(i32, JavaString)> {
    match layer_string.find('*') {
        Some(star_index) => Some((
            layer_string[..star_index].parse().ok()?,
            layer_string[star_index + 1..].to_owned(),
        )),
        None => Some((1, layer_string.to_owned())),
    }
}

fn layers_info_from_string(layers_string: &JavaStr) -> Vec<(i32, JavaString)> {
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
