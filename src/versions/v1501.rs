use std::sync::OnceLock;
use ahash::AHashMap;
use crate::helpers::rename::rename_advancement;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1501;

static RENAMES: OnceLock<AHashMap<&'static str, &'static str>> = OnceLock::new();

fn renames() -> &'static AHashMap<&'static str, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = AHashMap::new();
        map.insert("minecraft:recipes/brewing/speckled_melon", "minecraft:recipes/brewing/glistering_melon_slice");
        map.insert("minecraft:recipes/building_blocks/black_stained_hardened_clay", "minecraft:recipes/building_blocks/black_terracotta");
        map.insert("minecraft:recipes/building_blocks/blue_stained_hardened_clay", "minecraft:recipes/building_blocks/blue_terracotta");
        map.insert("minecraft:recipes/building_blocks/brown_stained_hardened_clay", "minecraft:recipes/building_blocks/brown_terracotta");
        map.insert("minecraft:recipes/building_blocks/cyan_stained_hardened_clay", "minecraft:recipes/building_blocks/cyan_terracotta");
        map.insert("minecraft:recipes/building_blocks/gray_stained_hardened_clay", "minecraft:recipes/building_blocks/gray_terracotta");
        map.insert("minecraft:recipes/building_blocks/green_stained_hardened_clay", "minecraft:recipes/building_blocks/green_terracotta");
        map.insert("minecraft:recipes/building_blocks/light_blue_stained_hardened_clay", "minecraft:recipes/building_blocks/light_blue_terracotta");
        map.insert("minecraft:recipes/building_blocks/light_gray_stained_hardened_clay", "minecraft:recipes/building_blocks/light_gray_terracotta");
        map.insert("minecraft:recipes/building_blocks/lime_stained_hardened_clay", "minecraft:recipes/building_blocks/lime_terracotta");
        map.insert("minecraft:recipes/building_blocks/magenta_stained_hardened_clay", "minecraft:recipes/building_blocks/magenta_terracotta");
        map.insert("minecraft:recipes/building_blocks/orange_stained_hardened_clay", "minecraft:recipes/building_blocks/orange_terracotta");
        map.insert("minecraft:recipes/building_blocks/pink_stained_hardened_clay", "minecraft:recipes/building_blocks/pink_terracotta");
        map.insert("minecraft:recipes/building_blocks/purple_stained_hardened_clay", "minecraft:recipes/building_blocks/purple_terracotta");
        map.insert("minecraft:recipes/building_blocks/red_stained_hardened_clay", "minecraft:recipes/building_blocks/red_terracotta");
        map.insert("minecraft:recipes/building_blocks/white_stained_hardened_clay", "minecraft:recipes/building_blocks/white_terracotta");
        map.insert("minecraft:recipes/building_blocks/yellow_stained_hardened_clay", "minecraft:recipes/building_blocks/yellow_terracotta");
        map.insert("minecraft:recipes/building_blocks/acacia_wooden_slab", "minecraft:recipes/building_blocks/acacia_slab");
        map.insert("minecraft:recipes/building_blocks/birch_wooden_slab", "minecraft:recipes/building_blocks/birch_slab");
        map.insert("minecraft:recipes/building_blocks/dark_oak_wooden_slab", "minecraft:recipes/building_blocks/dark_oak_slab");
        map.insert("minecraft:recipes/building_blocks/jungle_wooden_slab", "minecraft:recipes/building_blocks/jungle_slab");
        map.insert("minecraft:recipes/building_blocks/oak_wooden_slab", "minecraft:recipes/building_blocks/oak_slab");
        map.insert("minecraft:recipes/building_blocks/spruce_wooden_slab", "minecraft:recipes/building_blocks/spruce_slab");
        map.insert("minecraft:recipes/building_blocks/brick_block", "minecraft:recipes/building_blocks/bricks");
        map.insert("minecraft:recipes/building_blocks/chiseled_stonebrick", "minecraft:recipes/building_blocks/chiseled_stone_bricks");
        map.insert("minecraft:recipes/building_blocks/end_bricks", "minecraft:recipes/building_blocks/end_stone_bricks");
        map.insert("minecraft:recipes/building_blocks/lit_pumpkin", "minecraft:recipes/building_blocks/jack_o_lantern");
        map.insert("minecraft:recipes/building_blocks/magma", "minecraft:recipes/building_blocks/magma_block");
        map.insert("minecraft:recipes/building_blocks/melon_block", "minecraft:recipes/building_blocks/melon");
        map.insert("minecraft:recipes/building_blocks/mossy_stonebrick", "minecraft:recipes/building_blocks/mossy_stone_bricks");
        map.insert("minecraft:recipes/building_blocks/nether_brick", "minecraft:recipes/building_blocks/nether_bricks");
        map.insert("minecraft:recipes/building_blocks/pillar_quartz_block", "minecraft:recipes/building_blocks/quartz_pillar");
        map.insert("minecraft:recipes/building_blocks/red_nether_brick", "minecraft:recipes/building_blocks/red_nether_bricks");
        map.insert("minecraft:recipes/building_blocks/snow", "minecraft:recipes/building_blocks/snow_block");
        map.insert("minecraft:recipes/building_blocks/smooth_red_sandstone", "minecraft:recipes/building_blocks/cut_red_sandstone");
        map.insert("minecraft:recipes/building_blocks/smooth_sandstone", "minecraft:recipes/building_blocks/cut_sandstone");
        map.insert("minecraft:recipes/building_blocks/stonebrick", "minecraft:recipes/building_blocks/stone_bricks");
        map.insert("minecraft:recipes/building_blocks/stone_stairs", "minecraft:recipes/building_blocks/cobblestone_stairs");
        map.insert("minecraft:recipes/building_blocks/string_to_wool", "minecraft:recipes/building_blocks/white_wool_from_string");
        map.insert("minecraft:recipes/decorations/fence", "minecraft:recipes/decorations/oak_fence");
        map.insert("minecraft:recipes/decorations/purple_shulker_box", "minecraft:recipes/decorations/shulker_box");
        map.insert("minecraft:recipes/decorations/slime", "minecraft:recipes/decorations/slime_block");
        map.insert("minecraft:recipes/decorations/snow_layer", "minecraft:recipes/decorations/snow");
        map.insert("minecraft:recipes/misc/bone_meal_from_block", "minecraft:recipes/misc/bone_meal_from_bone_block");
        map.insert("minecraft:recipes/misc/bone_meal_from_bone", "minecraft:recipes/misc/bone_meal");
        map.insert("minecraft:recipes/misc/gold_ingot_from_block", "minecraft:recipes/misc/gold_ingot_from_gold_block");
        map.insert("minecraft:recipes/misc/iron_ingot_from_block", "minecraft:recipes/misc/iron_ingot_from_iron_block");
        map.insert("minecraft:recipes/redstone/fence_gate", "minecraft:recipes/redstone/oak_fence_gate");
        map.insert("minecraft:recipes/redstone/noteblock", "minecraft:recipes/redstone/note_block");
        map.insert("minecraft:recipes/redstone/trapdoor", "minecraft:recipes/redstone/oak_trapdoor");
        map.insert("minecraft:recipes/redstone/wooden_button", "minecraft:recipes/redstone/oak_button");
        map.insert("minecraft:recipes/redstone/wooden_door", "minecraft:recipes/redstone/oak_door");
        map.insert("minecraft:recipes/redstone/wooden_pressure_plate", "minecraft:recipes/redstone/oak_pressure_plate");
        map.insert("minecraft:recipes/transportation/boat", "minecraft:recipes/transportation/oak_boat");
        map.insert("minecraft:recipes/transportation/golden_rail", "minecraft:recipes/transportation/powered_rail");
        map
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
    let renames = renames();
    rename_advancement(types, VERSION, move |name| renames.get(name).copied().map(|str| str.to_owned()))
}
