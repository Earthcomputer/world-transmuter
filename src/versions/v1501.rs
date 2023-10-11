use crate::helpers::rename::rename_advancement;
use ahash::AHashMap;
use java_string::JavaStr;
use std::sync::OnceLock;

const VERSION: u32 = 1501;

static RENAMES: OnceLock<AHashMap<&'static JavaStr, &'static JavaStr>> = OnceLock::new();

fn renames() -> &'static AHashMap<&'static JavaStr, &'static JavaStr> {
    RENAMES.get_or_init(|| {
        let mut map = AHashMap::new();
        map.insert(
            JavaStr::from_str("minecraft:recipes/brewing/speckled_melon"),
            JavaStr::from_str("minecraft:recipes/brewing/glistering_melon_slice"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/black_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/black_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/blue_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/blue_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/brown_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/brown_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/cyan_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/cyan_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/gray_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/gray_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/green_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/green_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/light_blue_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/light_blue_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/light_gray_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/light_gray_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/lime_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/lime_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/magenta_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/magenta_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/orange_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/orange_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/pink_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/pink_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/purple_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/purple_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/red_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/red_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/white_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/white_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/yellow_stained_hardened_clay"),
            JavaStr::from_str("minecraft:recipes/building_blocks/yellow_terracotta"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/acacia_wooden_slab"),
            JavaStr::from_str("minecraft:recipes/building_blocks/acacia_slab"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/birch_wooden_slab"),
            JavaStr::from_str("minecraft:recipes/building_blocks/birch_slab"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/dark_oak_wooden_slab"),
            JavaStr::from_str("minecraft:recipes/building_blocks/dark_oak_slab"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/jungle_wooden_slab"),
            JavaStr::from_str("minecraft:recipes/building_blocks/jungle_slab"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/oak_wooden_slab"),
            JavaStr::from_str("minecraft:recipes/building_blocks/oak_slab"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/spruce_wooden_slab"),
            JavaStr::from_str("minecraft:recipes/building_blocks/spruce_slab"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/brick_block"),
            JavaStr::from_str("minecraft:recipes/building_blocks/bricks"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/chiseled_stonebrick"),
            JavaStr::from_str("minecraft:recipes/building_blocks/chiseled_stone_bricks"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/end_bricks"),
            JavaStr::from_str("minecraft:recipes/building_blocks/end_stone_bricks"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/lit_pumpkin"),
            JavaStr::from_str("minecraft:recipes/building_blocks/jack_o_lantern"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/magma"),
            JavaStr::from_str("minecraft:recipes/building_blocks/magma_block"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/melon_block"),
            JavaStr::from_str("minecraft:recipes/building_blocks/melon"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/mossy_stonebrick"),
            JavaStr::from_str("minecraft:recipes/building_blocks/mossy_stone_bricks"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/nether_brick"),
            JavaStr::from_str("minecraft:recipes/building_blocks/nether_bricks"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/pillar_quartz_block"),
            JavaStr::from_str("minecraft:recipes/building_blocks/quartz_pillar"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/red_nether_brick"),
            JavaStr::from_str("minecraft:recipes/building_blocks/red_nether_bricks"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/snow"),
            JavaStr::from_str("minecraft:recipes/building_blocks/snow_block"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/smooth_red_sandstone"),
            JavaStr::from_str("minecraft:recipes/building_blocks/cut_red_sandstone"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/smooth_sandstone"),
            JavaStr::from_str("minecraft:recipes/building_blocks/cut_sandstone"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/stonebrick"),
            JavaStr::from_str("minecraft:recipes/building_blocks/stone_bricks"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/stone_stairs"),
            JavaStr::from_str("minecraft:recipes/building_blocks/cobblestone_stairs"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/building_blocks/string_to_wool"),
            JavaStr::from_str("minecraft:recipes/building_blocks/white_wool_from_string"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/decorations/fence"),
            JavaStr::from_str("minecraft:recipes/decorations/oak_fence"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/decorations/purple_shulker_box"),
            JavaStr::from_str("minecraft:recipes/decorations/shulker_box"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/decorations/slime"),
            JavaStr::from_str("minecraft:recipes/decorations/slime_block"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/decorations/snow_layer"),
            JavaStr::from_str("minecraft:recipes/decorations/snow"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/misc/bone_meal_from_block"),
            JavaStr::from_str("minecraft:recipes/misc/bone_meal_from_bone_block"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/misc/bone_meal_from_bone"),
            JavaStr::from_str("minecraft:recipes/misc/bone_meal"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/misc/gold_ingot_from_block"),
            JavaStr::from_str("minecraft:recipes/misc/gold_ingot_from_gold_block"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/misc/iron_ingot_from_block"),
            JavaStr::from_str("minecraft:recipes/misc/iron_ingot_from_iron_block"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/redstone/fence_gate"),
            JavaStr::from_str("minecraft:recipes/redstone/oak_fence_gate"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/redstone/noteblock"),
            JavaStr::from_str("minecraft:recipes/redstone/note_block"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/redstone/trapdoor"),
            JavaStr::from_str("minecraft:recipes/redstone/oak_trapdoor"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/redstone/wooden_button"),
            JavaStr::from_str("minecraft:recipes/redstone/oak_button"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/redstone/wooden_door"),
            JavaStr::from_str("minecraft:recipes/redstone/oak_door"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/redstone/wooden_pressure_plate"),
            JavaStr::from_str("minecraft:recipes/redstone/oak_pressure_plate"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/transportation/boat"),
            JavaStr::from_str("minecraft:recipes/transportation/oak_boat"),
        );
        map.insert(
            JavaStr::from_str("minecraft:recipes/transportation/golden_rail"),
            JavaStr::from_str("minecraft:recipes/transportation/powered_rail"),
        );
        map
    })
}

pub(crate) fn register() {
    let renames = renames();
    rename_advancement(VERSION, move |name| {
        renames.get(name).copied().map(|str| str.to_owned())
    })
}
