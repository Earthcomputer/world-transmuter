use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::Types;
use crate::helpers::rename::rename_advancement;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1501;

static RENAMES: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn renames() -> &'static rust_dataconverter_engine::Map<String, String> {
    RENAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:recipes/brewing/speckled_melon".to_owned(), "minecraft:recipes/brewing/glistering_melon_slice".to_owned());
        map.insert("minecraft:recipes/building_blocks/black_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/black_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/blue_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/blue_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/brown_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/brown_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/cyan_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/cyan_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/gray_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/gray_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/green_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/green_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/light_blue_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/light_blue_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/light_gray_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/light_gray_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/lime_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/lime_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/magenta_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/magenta_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/orange_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/orange_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/pink_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/pink_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/purple_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/purple_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/red_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/red_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/white_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/white_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/yellow_stained_hardened_clay".to_owned(), "minecraft:recipes/building_blocks/yellow_terracotta".to_owned());
        map.insert("minecraft:recipes/building_blocks/acacia_wooden_slab".to_owned(), "minecraft:recipes/building_blocks/acacia_slab".to_owned());
        map.insert("minecraft:recipes/building_blocks/birch_wooden_slab".to_owned(), "minecraft:recipes/building_blocks/birch_slab".to_owned());
        map.insert("minecraft:recipes/building_blocks/dark_oak_wooden_slab".to_owned(), "minecraft:recipes/building_blocks/dark_oak_slab".to_owned());
        map.insert("minecraft:recipes/building_blocks/jungle_wooden_slab".to_owned(), "minecraft:recipes/building_blocks/jungle_slab".to_owned());
        map.insert("minecraft:recipes/building_blocks/oak_wooden_slab".to_owned(), "minecraft:recipes/building_blocks/oak_slab".to_owned());
        map.insert("minecraft:recipes/building_blocks/spruce_wooden_slab".to_owned(), "minecraft:recipes/building_blocks/spruce_slab".to_owned());
        map.insert("minecraft:recipes/building_blocks/brick_block".to_owned(), "minecraft:recipes/building_blocks/bricks".to_owned());
        map.insert("minecraft:recipes/building_blocks/chiseled_stonebrick".to_owned(), "minecraft:recipes/building_blocks/chiseled_stone_bricks".to_owned());
        map.insert("minecraft:recipes/building_blocks/end_bricks".to_owned(), "minecraft:recipes/building_blocks/end_stone_bricks".to_owned());
        map.insert("minecraft:recipes/building_blocks/lit_pumpkin".to_owned(), "minecraft:recipes/building_blocks/jack_o_lantern".to_owned());
        map.insert("minecraft:recipes/building_blocks/magma".to_owned(), "minecraft:recipes/building_blocks/magma_block".to_owned());
        map.insert("minecraft:recipes/building_blocks/melon_block".to_owned(), "minecraft:recipes/building_blocks/melon".to_owned());
        map.insert("minecraft:recipes/building_blocks/mossy_stonebrick".to_owned(), "minecraft:recipes/building_blocks/mossy_stone_bricks".to_owned());
        map.insert("minecraft:recipes/building_blocks/nether_brick".to_owned(), "minecraft:recipes/building_blocks/nether_bricks".to_owned());
        map.insert("minecraft:recipes/building_blocks/pillar_quartz_block".to_owned(), "minecraft:recipes/building_blocks/quartz_pillar".to_owned());
        map.insert("minecraft:recipes/building_blocks/red_nether_brick".to_owned(), "minecraft:recipes/building_blocks/red_nether_bricks".to_owned());
        map.insert("minecraft:recipes/building_blocks/snow".to_owned(), "minecraft:recipes/building_blocks/snow_block".to_owned());
        map.insert("minecraft:recipes/building_blocks/smooth_red_sandstone".to_owned(), "minecraft:recipes/building_blocks/cut_red_sandstone".to_owned());
        map.insert("minecraft:recipes/building_blocks/smooth_sandstone".to_owned(), "minecraft:recipes/building_blocks/cut_sandstone".to_owned());
        map.insert("minecraft:recipes/building_blocks/stonebrick".to_owned(), "minecraft:recipes/building_blocks/stone_bricks".to_owned());
        map.insert("minecraft:recipes/building_blocks/stone_stairs".to_owned(), "minecraft:recipes/building_blocks/cobblestone_stairs".to_owned());
        map.insert("minecraft:recipes/building_blocks/string_to_wool".to_owned(), "minecraft:recipes/building_blocks/white_wool_from_string".to_owned());
        map.insert("minecraft:recipes/decorations/fence".to_owned(), "minecraft:recipes/decorations/oak_fence".to_owned());
        map.insert("minecraft:recipes/decorations/purple_shulker_box".to_owned(), "minecraft:recipes/decorations/shulker_box".to_owned());
        map.insert("minecraft:recipes/decorations/slime".to_owned(), "minecraft:recipes/decorations/slime_block".to_owned());
        map.insert("minecraft:recipes/decorations/snow_layer".to_owned(), "minecraft:recipes/decorations/snow".to_owned());
        map.insert("minecraft:recipes/misc/bone_meal_from_block".to_owned(), "minecraft:recipes/misc/bone_meal_from_bone_block".to_owned());
        map.insert("minecraft:recipes/misc/bone_meal_from_bone".to_owned(), "minecraft:recipes/misc/bone_meal".to_owned());
        map.insert("minecraft:recipes/misc/gold_ingot_from_block".to_owned(), "minecraft:recipes/misc/gold_ingot_from_gold_block".to_owned());
        map.insert("minecraft:recipes/misc/iron_ingot_from_block".to_owned(), "minecraft:recipes/misc/iron_ingot_from_iron_block".to_owned());
        map.insert("minecraft:recipes/redstone/fence_gate".to_owned(), "minecraft:recipes/redstone/oak_fence_gate".to_owned());
        map.insert("minecraft:recipes/redstone/noteblock".to_owned(), "minecraft:recipes/redstone/note_block".to_owned());
        map.insert("minecraft:recipes/redstone/trapdoor".to_owned(), "minecraft:recipes/redstone/oak_trapdoor".to_owned());
        map.insert("minecraft:recipes/redstone/wooden_button".to_owned(), "minecraft:recipes/redstone/oak_button".to_owned());
        map.insert("minecraft:recipes/redstone/wooden_door".to_owned(), "minecraft:recipes/redstone/oak_door".to_owned());
        map.insert("minecraft:recipes/redstone/wooden_pressure_plate".to_owned(), "minecraft:recipes/redstone/oak_pressure_plate".to_owned());
        map.insert("minecraft:recipes/transportation/boat".to_owned(), "minecraft:recipes/transportation/oak_boat".to_owned());
        map.insert("minecraft:recipes/transportation/golden_rail".to_owned(), "minecraft:recipes/transportation/powered_rail".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    let renames = renames();
    rename_advancement(types, VERSION, move |name| renames.get(name).cloned())
}
