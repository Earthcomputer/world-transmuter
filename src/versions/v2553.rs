use crate::{static_string_mc_map, types};
use world_transmuter_engine::{value_data_converter_func, JValueMut};

const VERSION: u32 = 2553;

static_string_mc_map! {
    biome_renames = {
        "extreme_hills" => "minecraft:mountains",
        "swampland" => "minecraft:swamp",
        "hell" => "minecraft:nether_wastes",
        "sky" => "minecraft:the_end",
        "ice_flats" => "minecraft:snowy_tundra",
        "ice_mountains" => "minecraft:snowy_mountains",
        "mushroom_island" => "minecraft:mushroom_fields",
        "mushroom_island_shore" => "minecraft:mushroom_field_shore",
        "beaches" => "minecraft:beach",
        "forest_hills" => "minecraft:wooded_hills",
        "smaller_extreme_hills" => "minecraft:mountain_edge",
        "stone_beach" => "minecraft:stone_shore",
        "cold_beach" => "minecraft:snowy_beach",
        "roofed_forest" => "minecraft:dark_forest",
        "taiga_cold" => "minecraft:snowy_taiga",
        "taiga_cold_hills" => "minecraft:snowy_taiga_hills",
        "redwood_taiga" => "minecraft:giant_tree_taiga",
        "redwood_taiga_hills" => "minecraft:giant_tree_taiga_hills",
        "extreme_hills_with_trees" => "minecraft:wooded_mountains",
        "savanna_rock" => "minecraft:savanna_plateau",
        "mesa" => "minecraft:badlands",
        "mesa_rock" => "minecraft:wooded_badlands_plateau",
        "mesa_clear_rock" => "minecraft:badlands_plateau",
        "sky_island_low" => "minecraft:small_end_islands",
        "sky_island_medium" => "minecraft:end_midlands",
        "sky_island_high" => "minecraft:end_highlands",
        "sky_island_barren" => "minecraft:end_barrens",
        "void" => "minecraft:the_void",
        "mutated_plains" => "minecraft:sunflower_plains",
        "mutated_desert" => "minecraft:desert_lakes",
        "mutated_extreme_hills" => "minecraft:gravelly_mountains",
        "mutated_forest" => "minecraft:flower_forest",
        "mutated_taiga" => "minecraft:taiga_mountains",
        "mutated_swampland" => "minecraft:swamp_hills",
        "mutated_ice_flats" => "minecraft:ice_spikes",
        "mutated_jungle" => "minecraft:modified_jungle",
        "mutated_jungle_edge" => "minecraft:modified_jungle_edge",
        "mutated_birch_forest" => "minecraft:tall_birch_forest",
        "mutated_birch_forest_hills" => "minecraft:tall_birch_hills",
        "mutated_roofed_forest" => "minecraft:dark_forest_hills",
        "mutated_taiga_cold" => "minecraft:snowy_taiga_mountains",
        "mutated_redwood_taiga" => "minecraft:giant_spruce_taiga",
        "mutated_redwood_taiga_hills" => "minecraft:giant_spruce_taiga_hills",
        "mutated_extreme_hills_with_trees" => "minecraft:modified_gravelly_mountains",
        "mutated_savanna" => "minecraft:shattered_savanna",
        "mutated_savanna_rock" => "minecraft:shattered_savanna_plateau",
        "mutated_mesa" => "minecraft:eroded_badlands",
        "mutated_mesa_rock" => "minecraft:modified_wooded_badlands_plateau",
        "mutated_mesa_clear_rock" => "minecraft:modified_badlands_plateau",
        "warm_deep_ocean" => "minecraft:deep_warm_ocean",
        "lukewarm_deep_ocean" => "minecraft:deep_lukewarm_ocean",
        "cold_deep_ocean" => "minecraft:deep_cold_ocean",
        "frozen_deep_ocean" => "minecraft:deep_frozen_ocean",
    }
}

pub(crate) fn register() {
    types::biome_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let JValueMut::String(data) = data {
                if let Some(new_name) = biome_renames().get(&data[..]).copied() {
                    **data = new_name.to_owned();
                }
            }
        }),
    );
}
