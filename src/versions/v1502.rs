use crate::helpers::rename::rename_recipe;
use crate::static_string_mc_map;

const VERSION: u32 = 1502;

static_string_mc_map! {
    recipes_updates = {
        "acacia_wooden_slab" => "minecraft:acacia_slab",
        "birch_wooden_slab" => "minecraft:birch_slab",
        "black_stained_hardened_clay" => "minecraft:black_terracotta",
        "blue_stained_hardened_clay" => "minecraft:blue_terracotta",
        "boat" => "minecraft:oak_boat",
        "bone_meal_from_block" => "minecraft:bone_meal_from_bone_block",
        "bone_meal_from_bone" => "minecraft:bone_meal",
        "brick_block" => "minecraft:bricks",
        "brown_stained_hardened_clay" => "minecraft:brown_terracotta",
        "chiseled_stonebrick" => "minecraft:chiseled_stone_bricks",
        "cyan_stained_hardened_clay" => "minecraft:cyan_terracotta",
        "dark_oak_wooden_slab" => "minecraft:dark_oak_slab",
        "end_bricks" => "minecraft:end_stone_bricks",
        "fence_gate" => "minecraft:oak_fence_gate",
        "fence" => "minecraft:oak_fence",
        "golden_rail" => "minecraft:powered_rail",
        "gold_ingot_from_block" => "minecraft:gold_ingot_from_gold_block",
        "gray_stained_hardened_clay" => "minecraft:gray_terracotta",
        "green_stained_hardened_clay" => "minecraft:green_terracotta",
        "iron_ingot_from_block" => "minecraft:iron_ingot_from_iron_block",
        "jungle_wooden_slab" => "minecraft:jungle_slab",
        "light_blue_stained_hardened_clay" => "minecraft:light_blue_terracotta",
        "light_gray_stained_hardened_clay" => "minecraft:light_gray_terracotta",
        "lime_stained_hardened_clay" => "minecraft:lime_terracotta",
        "lit_pumpkin" => "minecraft:jack_o_lantern",
        "magenta_stained_hardened_clay" => "minecraft:magenta_terracotta",
        "magma" => "minecraft:magma_block",
        "melon_block" => "minecraft:melon",
        "mossy_stonebrick" => "minecraft:mossy_stone_bricks",
        "noteblock" => "minecraft:note_block",
        "oak_wooden_slab" => "minecraft:oak_slab",
        "orange_stained_hardened_clay" => "minecraft:orange_terracotta",
        "pillar_quartz_block" => "minecraft:quartz_pillar",
        "pink_stained_hardened_clay" => "minecraft:pink_terracotta",
        "purple_shulker_box" => "minecraft:shulker_box",
        "purple_stained_hardened_clay" => "minecraft:purple_terracotta",
        "red_nether_brick" => "minecraft:red_nether_bricks",
        "red_stained_hardened_clay" => "minecraft:red_terracotta",
        "slime" => "minecraft:slime_block",
        "smooth_red_sandstone" => "minecraft:cut_red_sandstone",
        "smooth_sandstone" => "minecraft:cut_sandstone",
        "snow_layer" => "minecraft:snow",
        "snow" => "minecraft:snow_block",
        "speckled_melon" => "minecraft:glistering_melon_slice",
        "spruce_wooden_slab" => "minecraft:spruce_slab",
        "stonebrick" => "minecraft:stone_bricks",
        "stone_stairs" => "minecraft:cobblestone_stairs",
        "string_to_wool" => "minecraft:white_wool_from_string",
        "trapdoor" => "minecraft:oak_trapdoor",
        "white_stained_hardened_clay" => "minecraft:white_terracotta",
        "wooden_button" => "minecraft:oak_button",
        "wooden_door" => "minecraft:oak_door",
        "wooden_pressure_plate" => "minecraft:oak_pressure_plate",
        "yellow_stained_hardened_clay" => "minecraft:yellow_terracotta",
    }
}

pub(crate) fn register() {
    let recipes_updates = recipes_updates();
    rename_recipe(VERSION, move |name| {
        recipes_updates.get(name).copied().map(|str| str.to_owned())
    });
}
