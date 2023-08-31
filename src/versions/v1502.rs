use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::rename_recipe;
use crate::MinecraftTypesMut;
use std::sync::OnceLock;

const VERSION: u32 = 1502;

static RECIPES_UPDATES: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn recipes_updates() -> &'static McNamespaceMap<'static, &'static str> {
    RECIPES_UPDATES.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("acacia_wooden_slab", "minecraft:acacia_slab");
        map.insert_mc("birch_wooden_slab", "minecraft:birch_slab");
        map.insert_mc("black_stained_hardened_clay", "minecraft:black_terracotta");
        map.insert_mc("blue_stained_hardened_clay", "minecraft:blue_terracotta");
        map.insert_mc("boat", "minecraft:oak_boat");
        map.insert_mc(
            "bone_meal_from_block",
            "minecraft:bone_meal_from_bone_block",
        );
        map.insert_mc("bone_meal_from_bone", "minecraft:bone_meal");
        map.insert_mc("brick_block", "minecraft:bricks");
        map.insert_mc("brown_stained_hardened_clay", "minecraft:brown_terracotta");
        map.insert_mc("chiseled_stonebrick", "minecraft:chiseled_stone_bricks");
        map.insert_mc("cyan_stained_hardened_clay", "minecraft:cyan_terracotta");
        map.insert_mc("dark_oak_wooden_slab", "minecraft:dark_oak_slab");
        map.insert_mc("end_bricks", "minecraft:end_stone_bricks");
        map.insert_mc("fence_gate", "minecraft:oak_fence_gate");
        map.insert_mc("fence", "minecraft:oak_fence");
        map.insert_mc("golden_rail", "minecraft:powered_rail");
        map.insert_mc(
            "gold_ingot_from_block",
            "minecraft:gold_ingot_from_gold_block",
        );
        map.insert_mc("gray_stained_hardened_clay", "minecraft:gray_terracotta");
        map.insert_mc("green_stained_hardened_clay", "minecraft:green_terracotta");
        map.insert_mc(
            "iron_ingot_from_block",
            "minecraft:iron_ingot_from_iron_block",
        );
        map.insert_mc("jungle_wooden_slab", "minecraft:jungle_slab");
        map.insert_mc(
            "light_blue_stained_hardened_clay",
            "minecraft:light_blue_terracotta",
        );
        map.insert_mc(
            "light_gray_stained_hardened_clay",
            "minecraft:light_gray_terracotta",
        );
        map.insert_mc("lime_stained_hardened_clay", "minecraft:lime_terracotta");
        map.insert_mc("lit_pumpkin", "minecraft:jack_o_lantern");
        map.insert_mc(
            "magenta_stained_hardened_clay",
            "minecraft:magenta_terracotta",
        );
        map.insert_mc("magma", "minecraft:magma_block");
        map.insert_mc("melon_block", "minecraft:melon");
        map.insert_mc("mossy_stonebrick", "minecraft:mossy_stone_bricks");
        map.insert_mc("noteblock", "minecraft:note_block");
        map.insert_mc("oak_wooden_slab", "minecraft:oak_slab");
        map.insert_mc(
            "orange_stained_hardened_clay",
            "minecraft:orange_terracotta",
        );
        map.insert_mc("pillar_quartz_block", "minecraft:quartz_pillar");
        map.insert_mc("pink_stained_hardened_clay", "minecraft:pink_terracotta");
        map.insert_mc("purple_shulker_box", "minecraft:shulker_box");
        map.insert_mc(
            "purple_stained_hardened_clay",
            "minecraft:purple_terracotta",
        );
        map.insert_mc("red_nether_brick", "minecraft:red_nether_bricks");
        map.insert_mc("red_stained_hardened_clay", "minecraft:red_terracotta");
        map.insert_mc("slime", "minecraft:slime_block");
        map.insert_mc("smooth_red_sandstone", "minecraft:cut_red_sandstone");
        map.insert_mc("smooth_sandstone", "minecraft:cut_sandstone");
        map.insert_mc("snow_layer", "minecraft:snow");
        map.insert_mc("snow", "minecraft:snow_block");
        map.insert_mc("speckled_melon", "minecraft:glistering_melon_slice");
        map.insert_mc("spruce_wooden_slab", "minecraft:spruce_slab");
        map.insert_mc("stonebrick", "minecraft:stone_bricks");
        map.insert_mc("stone_stairs", "minecraft:cobblestone_stairs");
        map.insert_mc("string_to_wool", "minecraft:white_wool_from_string");
        map.insert_mc("trapdoor", "minecraft:oak_trapdoor");
        map.insert_mc("white_stained_hardened_clay", "minecraft:white_terracotta");
        map.insert_mc("wooden_button", "minecraft:oak_button");
        map.insert_mc("wooden_door", "minecraft:oak_door");
        map.insert_mc("wooden_pressure_plate", "minecraft:oak_pressure_plate");
        map.insert_mc(
            "yellow_stained_hardened_clay",
            "minecraft:yellow_terracotta",
        );
        map
    })
}

pub(crate) fn register(types: MinecraftTypesMut) {
    let recipes_updates = recipes_updates();
    rename_recipe(types, VERSION, move |name| {
        recipes_updates.get(name).copied().map(|str| str.to_owned())
    });
}
