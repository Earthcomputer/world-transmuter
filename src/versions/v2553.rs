use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2553;

static BIOME_RENAMES: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn biome_renames() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    BIOME_RENAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:extreme_hills", "minecraft:mountains");
        map.insert("minecraft:swampland", "minecraft:swamp");
        map.insert("minecraft:hell", "minecraft:nether_wastes");
        map.insert("minecraft:sky", "minecraft:the_end");
        map.insert("minecraft:ice_flats", "minecraft:snowy_tundra");
        map.insert("minecraft:ice_mountains", "minecraft:snowy_mountains");
        map.insert("minecraft:mushroom_island", "minecraft:mushroom_fields");
        map.insert("minecraft:mushroom_island_shore", "minecraft:mushroom_field_shore");
        map.insert("minecraft:beaches", "minecraft:beach");
        map.insert("minecraft:forest_hills", "minecraft:wooded_hills");
        map.insert("minecraft:smaller_extreme_hills", "minecraft:mountain_edge");
        map.insert("minecraft:stone_beach", "minecraft:stone_shore");
        map.insert("minecraft:cold_beach", "minecraft:snowy_beach");
        map.insert("minecraft:roofed_forest", "minecraft:dark_forest");
        map.insert("minecraft:taiga_cold", "minecraft:snowy_taiga");
        map.insert("minecraft:taiga_cold_hills", "minecraft:snowy_taiga_hills");
        map.insert("minecraft:redwood_taiga", "minecraft:giant_tree_taiga");
        map.insert("minecraft:redwood_taiga_hills", "minecraft:giant_tree_taiga_hills");
        map.insert("minecraft:extreme_hills_with_trees", "minecraft:wooded_mountains");
        map.insert("minecraft:savanna_rock", "minecraft:savanna_plateau");
        map.insert("minecraft:mesa", "minecraft:badlands");
        map.insert("minecraft:mesa_rock", "minecraft:wooded_badlands_plateau");
        map.insert("minecraft:mesa_clear_rock", "minecraft:badlands_plateau");
        map.insert("minecraft:sky_island_low", "minecraft:small_end_islands");
        map.insert("minecraft:sky_island_medium", "minecraft:end_midlands");
        map.insert("minecraft:sky_island_high", "minecraft:end_highlands");
        map.insert("minecraft:sky_island_barren", "minecraft:end_barrens");
        map.insert("minecraft:void", "minecraft:the_void");
        map.insert("minecraft:mutated_plains", "minecraft:sunflower_plains");
        map.insert("minecraft:mutated_desert", "minecraft:desert_lakes");
        map.insert("minecraft:mutated_extreme_hills", "minecraft:gravelly_mountains");
        map.insert("minecraft:mutated_forest", "minecraft:flower_forest");
        map.insert("minecraft:mutated_taiga", "minecraft:taiga_mountains");
        map.insert("minecraft:mutated_swampland", "minecraft:swamp_hills");
        map.insert("minecraft:mutated_ice_flats", "minecraft:ice_spikes");
        map.insert("minecraft:mutated_jungle", "minecraft:modified_jungle");
        map.insert("minecraft:mutated_jungle_edge", "minecraft:modified_jungle_edge");
        map.insert("minecraft:mutated_birch_forest", "minecraft:tall_birch_forest");
        map.insert("minecraft:mutated_birch_forest_hills", "minecraft:tall_birch_hills");
        map.insert("minecraft:mutated_roofed_forest", "minecraft:dark_forest_hills");
        map.insert("minecraft:mutated_taiga_cold", "minecraft:snowy_taiga_mountains");
        map.insert("minecraft:mutated_redwood_taiga", "minecraft:giant_spruce_taiga");
        map.insert("minecraft:mutated_redwood_taiga_hills", "minecraft:giant_spruce_taiga_hills");
        map.insert("minecraft:mutated_extreme_hills_with_trees", "minecraft:modified_gravelly_mountains");
        map.insert("minecraft:mutated_savanna", "minecraft:shattered_savanna");
        map.insert("minecraft:mutated_savanna_rock", "minecraft:shattered_savanna_plateau");
        map.insert("minecraft:mutated_mesa", "minecraft:eroded_badlands");
        map.insert("minecraft:mutated_mesa_rock", "minecraft:modified_wooded_badlands_plateau");
        map.insert("minecraft:mutated_mesa_clear_rock", "minecraft:modified_badlands_plateau");
        map.insert("minecraft:warm_deep_ocean", "minecraft:deep_warm_ocean");
        map.insert("minecraft:lukewarm_deep_ocean", "minecraft:deep_lukewarm_ocean");
        map.insert("minecraft:cold_deep_ocean", "minecraft:deep_cold_ocean");
        map.insert("minecraft:frozen_deep_ocean", "minecraft:deep_frozen_ocean");
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.biome.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Object, _>(|data, _from_version, _to_version| {
        if let Some(&to_biome) = data.as_string().and_then(|from_biome| biome_renames().get(from_biome)) {
            *data = T::Object::create_string(to_biome.to_owned());
        }
    }));
}
