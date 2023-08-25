use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::value_data_converter_func;
use std::sync::OnceLock;
use valence_nbt::value::ValueMut;

const VERSION: u32 = 2838;

static BIOME_UPDATE: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn biome_update() -> &'static McNamespaceMap<'static, &'static str> {
    BIOME_UPDATE.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("badlands_plateau", "minecraft:badlands");
        map.insert_mc("bamboo_jungle_hills", "minecraft:bamboo_jungle");
        map.insert_mc("birch_forest_hills", "minecraft:birch_forest");
        map.insert_mc("dark_forest_hills", "minecraft:dark_forest");
        map.insert_mc("desert_hills", "minecraft:desert");
        map.insert_mc("desert_lakes", "minecraft:desert");
        map.insert_mc(
            "giant_spruce_taiga_hills",
            "minecraft:old_growth_spruce_taiga",
        );
        map.insert_mc("giant_spruce_taiga", "minecraft:old_growth_spruce_taiga");
        map.insert_mc("giant_tree_taiga_hills", "minecraft:old_growth_pine_taiga");
        map.insert_mc("giant_tree_taiga", "minecraft:old_growth_pine_taiga");
        map.insert_mc("gravelly_mountains", "minecraft:windswept_gravelly_hills");
        map.insert_mc("jungle_edge", "minecraft:sparse_jungle");
        map.insert_mc("jungle_hills", "minecraft:jungle");
        map.insert_mc("modified_badlands_plateau", "minecraft:badlands");
        map.insert_mc(
            "modified_gravelly_mountains",
            "minecraft:windswept_gravelly_hills",
        );
        map.insert_mc("modified_jungle_edge", "minecraft:sparse_jungle");
        map.insert_mc("modified_jungle", "minecraft:jungle");
        map.insert_mc(
            "modified_wooded_badlands_plateau",
            "minecraft:wooded_badlands",
        );
        map.insert_mc("mountain_edge", "minecraft:windswept_hills");
        map.insert_mc("mountains", "minecraft:windswept_hills");
        map.insert_mc("mushroom_field_shore", "minecraft:mushroom_fields");
        map.insert_mc("shattered_savanna", "minecraft:windswept_savanna");
        map.insert_mc("shattered_savanna_plateau", "minecraft:windswept_savanna");
        map.insert_mc("snowy_mountains", "minecraft:snowy_plains");
        map.insert_mc("snowy_taiga_hills", "minecraft:snowy_taiga");
        map.insert_mc("snowy_taiga_mountains", "minecraft:snowy_taiga");
        map.insert_mc("snowy_tundra", "minecraft:snowy_plains");
        map.insert_mc("stone_shore", "minecraft:stony_shore");
        map.insert_mc("swamp_hills", "minecraft:swamp");
        map.insert_mc("taiga_hills", "minecraft:taiga");
        map.insert_mc("taiga_mountains", "minecraft:taiga");
        map.insert_mc("tall_birch_forest", "minecraft:old_growth_birch_forest");
        map.insert_mc("tall_birch_hills", "minecraft:old_growth_birch_forest");
        map.insert_mc("wooded_badlands_plateau", "minecraft:wooded_badlands");
        map.insert_mc("wooded_hills", "minecraft:forest");
        map.insert_mc("wooded_mountains", "minecraft:windswept_forest");
        map.insert_mc("lofty_peaks", "minecraft:jagged_peaks");
        map.insert_mc("snowcapped_peaks", "minecraft:frozen_peaks");
        map
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.biome.borrow_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let ValueMut::String(data) = data {
                if let Some(new_name) = biome_update().get(&data[..]).copied() {
                    **data = new_name.to_owned();
                }
            }
        }),
    );
}
