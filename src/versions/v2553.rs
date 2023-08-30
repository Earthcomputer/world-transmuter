use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::MinecraftTypes;
use rust_dataconverter_engine::value_data_converter_func;
use std::sync::OnceLock;
use valence_nbt::value::ValueMut;

const VERSION: u32 = 2553;

static BIOME_RENAMES: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn biome_renames() -> &'static McNamespaceMap<'static, &'static str> {
    BIOME_RENAMES.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("extreme_hills", "minecraft:mountains");
        map.insert_mc("swampland", "minecraft:swamp");
        map.insert_mc("hell", "minecraft:nether_wastes");
        map.insert_mc("sky", "minecraft:the_end");
        map.insert_mc("ice_flats", "minecraft:snowy_tundra");
        map.insert_mc("ice_mountains", "minecraft:snowy_mountains");
        map.insert_mc("mushroom_island", "minecraft:mushroom_fields");
        map.insert_mc("mushroom_island_shore", "minecraft:mushroom_field_shore");
        map.insert_mc("beaches", "minecraft:beach");
        map.insert_mc("forest_hills", "minecraft:wooded_hills");
        map.insert_mc("smaller_extreme_hills", "minecraft:mountain_edge");
        map.insert_mc("stone_beach", "minecraft:stone_shore");
        map.insert_mc("cold_beach", "minecraft:snowy_beach");
        map.insert_mc("roofed_forest", "minecraft:dark_forest");
        map.insert_mc("taiga_cold", "minecraft:snowy_taiga");
        map.insert_mc("taiga_cold_hills", "minecraft:snowy_taiga_hills");
        map.insert_mc("redwood_taiga", "minecraft:giant_tree_taiga");
        map.insert_mc("redwood_taiga_hills", "minecraft:giant_tree_taiga_hills");
        map.insert_mc("extreme_hills_with_trees", "minecraft:wooded_mountains");
        map.insert_mc("savanna_rock", "minecraft:savanna_plateau");
        map.insert_mc("mesa", "minecraft:badlands");
        map.insert_mc("mesa_rock", "minecraft:wooded_badlands_plateau");
        map.insert_mc("mesa_clear_rock", "minecraft:badlands_plateau");
        map.insert_mc("sky_island_low", "minecraft:small_end_islands");
        map.insert_mc("sky_island_medium", "minecraft:end_midlands");
        map.insert_mc("sky_island_high", "minecraft:end_highlands");
        map.insert_mc("sky_island_barren", "minecraft:end_barrens");
        map.insert_mc("void", "minecraft:the_void");
        map.insert_mc("mutated_plains", "minecraft:sunflower_plains");
        map.insert_mc("mutated_desert", "minecraft:desert_lakes");
        map.insert_mc("mutated_extreme_hills", "minecraft:gravelly_mountains");
        map.insert_mc("mutated_forest", "minecraft:flower_forest");
        map.insert_mc("mutated_taiga", "minecraft:taiga_mountains");
        map.insert_mc("mutated_swampland", "minecraft:swamp_hills");
        map.insert_mc("mutated_ice_flats", "minecraft:ice_spikes");
        map.insert_mc("mutated_jungle", "minecraft:modified_jungle");
        map.insert_mc("mutated_jungle_edge", "minecraft:modified_jungle_edge");
        map.insert_mc("mutated_birch_forest", "minecraft:tall_birch_forest");
        map.insert_mc("mutated_birch_forest_hills", "minecraft:tall_birch_hills");
        map.insert_mc("mutated_roofed_forest", "minecraft:dark_forest_hills");
        map.insert_mc("mutated_taiga_cold", "minecraft:snowy_taiga_mountains");
        map.insert_mc("mutated_redwood_taiga", "minecraft:giant_spruce_taiga");
        map.insert_mc(
            "mutated_redwood_taiga_hills",
            "minecraft:giant_spruce_taiga_hills",
        );
        map.insert_mc(
            "mutated_extreme_hills_with_trees",
            "minecraft:modified_gravelly_mountains",
        );
        map.insert_mc("mutated_savanna", "minecraft:shattered_savanna");
        map.insert_mc(
            "mutated_savanna_rock",
            "minecraft:shattered_savanna_plateau",
        );
        map.insert_mc("mutated_mesa", "minecraft:eroded_badlands");
        map.insert_mc(
            "mutated_mesa_rock",
            "minecraft:modified_wooded_badlands_plateau",
        );
        map.insert_mc(
            "mutated_mesa_clear_rock",
            "minecraft:modified_badlands_plateau",
        );
        map.insert_mc("warm_deep_ocean", "minecraft:deep_warm_ocean");
        map.insert_mc("lukewarm_deep_ocean", "minecraft:deep_lukewarm_ocean");
        map.insert_mc("cold_deep_ocean", "minecraft:deep_cold_ocean");
        map.insert_mc("frozen_deep_ocean", "minecraft:deep_frozen_ocean");
        map
    })
}

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.biome.borrow_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let ValueMut::String(data) = data {
                if let Some(new_name) = biome_renames().get(&data[..]).copied() {
                    **data = new_name.to_owned();
                }
            }
        }),
    );
}
