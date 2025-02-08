use crate::{static_string_mc_map, types};
use world_transmuter_engine::{value_data_converter_func, JValueMut};

const VERSION: u32 = 2838;

static_string_mc_map! {
    biome_update = {
        "badlands_plateau" => "minecraft:badlands",
        "bamboo_jungle_hills" => "minecraft:bamboo_jungle",
        "birch_forest_hills" => "minecraft:birch_forest",
        "dark_forest_hills" => "minecraft:dark_forest",
        "desert_hills" => "minecraft:desert",
        "desert_lakes" => "minecraft:desert",
        "giant_spruce_taiga_hills" => "minecraft:old_growth_spruce_taiga",
        "giant_spruce_taiga" => "minecraft:old_growth_spruce_taiga",
        "giant_tree_taiga_hills" => "minecraft:old_growth_pine_taiga",
        "giant_tree_taiga" => "minecraft:old_growth_pine_taiga",
        "gravelly_mountains" => "minecraft:windswept_gravelly_hills",
        "jungle_edge" => "minecraft:sparse_jungle",
        "jungle_hills" => "minecraft:jungle",
        "modified_badlands_plateau" => "minecraft:badlands",
        "modified_gravelly_mountains" => "minecraft:windswept_gravelly_hills",
        "modified_jungle_edge" => "minecraft:sparse_jungle",
        "modified_jungle" => "minecraft:jungle",
        "modified_wooded_badlands_plateau" => "minecraft:wooded_badlands",
        "mountain_edge" => "minecraft:windswept_hills",
        "mountains" => "minecraft:windswept_hills",
        "mushroom_field_shore" => "minecraft:mushroom_fields",
        "shattered_savanna" => "minecraft:windswept_savanna",
        "shattered_savanna_plateau" => "minecraft:windswept_savanna",
        "snowy_mountains" => "minecraft:snowy_plains",
        "snowy_taiga_hills" => "minecraft:snowy_taiga",
        "snowy_taiga_mountains" => "minecraft:snowy_taiga",
        "snowy_tundra" => "minecraft:snowy_plains",
        "stone_shore" => "minecraft:stony_shore",
        "swamp_hills" => "minecraft:swamp",
        "taiga_hills" => "minecraft:taiga",
        "taiga_mountains" => "minecraft:taiga",
        "tall_birch_forest" => "minecraft:old_growth_birch_forest",
        "tall_birch_hills" => "minecraft:old_growth_birch_forest",
        "wooded_badlands_plateau" => "minecraft:wooded_badlands",
        "wooded_hills" => "minecraft:forest",
        "wooded_mountains" => "minecraft:windswept_forest",
        "lofty_peaks" => "minecraft:jagged_peaks",
        "snowcapped_peaks" => "minecraft:frozen_peaks",
    }
}

pub(crate) fn register() {
    types::biome_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let JValueMut::String(data) = data {
                if let Some(new_name) = biome_update().get(&data[..]).copied() {
                    **data = new_name.to_owned();
                }
            }
        }),
    );
}
