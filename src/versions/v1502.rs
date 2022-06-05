use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::Types;
use crate::helpers::rename::rename_recipe;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1502;

static RECIPES_UPDATES: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn recipes_updates() -> &'static rust_dataconverter_engine::Map<String, String> {
    RECIPES_UPDATES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:acacia_wooden_slab".to_owned(), "minecraft:acacia_slab".to_owned());
        map.insert("minecraft:birch_wooden_slab".to_owned(), "minecraft:birch_slab".to_owned());
        map.insert("minecraft:black_stained_hardened_clay".to_owned(), "minecraft:black_terracotta".to_owned());
        map.insert("minecraft:blue_stained_hardened_clay".to_owned(), "minecraft:blue_terracotta".to_owned());
        map.insert("minecraft:boat".to_owned(), "minecraft:oak_boat".to_owned());
        map.insert("minecraft:bone_meal_from_block".to_owned(), "minecraft:bone_meal_from_bone_block".to_owned());
        map.insert("minecraft:bone_meal_from_bone".to_owned(), "minecraft:bone_meal".to_owned());
        map.insert("minecraft:brick_block".to_owned(), "minecraft:bricks".to_owned());
        map.insert("minecraft:brown_stained_hardened_clay".to_owned(), "minecraft:brown_terracotta".to_owned());
        map.insert("minecraft:chiseled_stonebrick".to_owned(), "minecraft:chiseled_stone_bricks".to_owned());
        map.insert("minecraft:cyan_stained_hardened_clay".to_owned(), "minecraft:cyan_terracotta".to_owned());
        map.insert("minecraft:dark_oak_wooden_slab".to_owned(), "minecraft:dark_oak_slab".to_owned());
        map.insert("minecraft:end_bricks".to_owned(), "minecraft:end_stone_bricks".to_owned());
        map.insert("minecraft:fence_gate".to_owned(), "minecraft:oak_fence_gate".to_owned());
        map.insert("minecraft:fence".to_owned(), "minecraft:oak_fence".to_owned());
        map.insert("minecraft:golden_rail".to_owned(), "minecraft:powered_rail".to_owned());
        map.insert("minecraft:gold_ingot_from_block".to_owned(), "minecraft:gold_ingot_from_gold_block".to_owned());
        map.insert("minecraft:gray_stained_hardened_clay".to_owned(), "minecraft:gray_terracotta".to_owned());
        map.insert("minecraft:green_stained_hardened_clay".to_owned(), "minecraft:green_terracotta".to_owned());
        map.insert("minecraft:iron_ingot_from_block".to_owned(), "minecraft:iron_ingot_from_iron_block".to_owned());
        map.insert("minecraft:jungle_wooden_slab".to_owned(), "minecraft:jungle_slab".to_owned());
        map.insert("minecraft:light_blue_stained_hardened_clay".to_owned(), "minecraft:light_blue_terracotta".to_owned());
        map.insert("minecraft:light_gray_stained_hardened_clay".to_owned(), "minecraft:light_gray_terracotta".to_owned());
        map.insert("minecraft:lime_stained_hardened_clay".to_owned(), "minecraft:lime_terracotta".to_owned());
        map.insert("minecraft:lit_pumpkin".to_owned(), "minecraft:jack_o_lantern".to_owned());
        map.insert("minecraft:magenta_stained_hardened_clay".to_owned(), "minecraft:magenta_terracotta".to_owned());
        map.insert("minecraft:magma".to_owned(), "minecraft:magma_block".to_owned());
        map.insert("minecraft:melon_block".to_owned(), "minecraft:melon".to_owned());
        map.insert("minecraft:mossy_stonebrick".to_owned(), "minecraft:mossy_stone_bricks".to_owned());
        map.insert("minecraft:noteblock".to_owned(), "minecraft:note_block".to_owned());
        map.insert("minecraft:oak_wooden_slab".to_owned(), "minecraft:oak_slab".to_owned());
        map.insert("minecraft:orange_stained_hardened_clay".to_owned(), "minecraft:orange_terracotta".to_owned());
        map.insert("minecraft:pillar_quartz_block".to_owned(), "minecraft:quartz_pillar".to_owned());
        map.insert("minecraft:pink_stained_hardened_clay".to_owned(), "minecraft:pink_terracotta".to_owned());
        map.insert("minecraft:purple_shulker_box".to_owned(), "minecraft:shulker_box".to_owned());
        map.insert("minecraft:purple_stained_hardened_clay".to_owned(), "minecraft:purple_terracotta".to_owned());
        map.insert("minecraft:red_nether_brick".to_owned(), "minecraft:red_nether_bricks".to_owned());
        map.insert("minecraft:red_stained_hardened_clay".to_owned(), "minecraft:red_terracotta".to_owned());
        map.insert("minecraft:slime".to_owned(), "minecraft:slime_block".to_owned());
        map.insert("minecraft:smooth_red_sandstone".to_owned(), "minecraft:cut_red_sandstone".to_owned());
        map.insert("minecraft:smooth_sandstone".to_owned(), "minecraft:cut_sandstone".to_owned());
        map.insert("minecraft:snow_layer".to_owned(), "minecraft:snow".to_owned());
        map.insert("minecraft:snow".to_owned(), "minecraft:snow_block".to_owned());
        map.insert("minecraft:speckled_melon".to_owned(), "minecraft:glistering_melon_slice".to_owned());
        map.insert("minecraft:spruce_wooden_slab".to_owned(), "minecraft:spruce_slab".to_owned());
        map.insert("minecraft:stonebrick".to_owned(), "minecraft:stone_bricks".to_owned());
        map.insert("minecraft:stone_stairs".to_owned(), "minecraft:cobblestone_stairs".to_owned());
        map.insert("minecraft:string_to_wool".to_owned(), "minecraft:white_wool_from_string".to_owned());
        map.insert("minecraft:trapdoor".to_owned(), "minecraft:oak_trapdoor".to_owned());
        map.insert("minecraft:white_stained_hardened_clay".to_owned(), "minecraft:white_terracotta".to_owned());
        map.insert("minecraft:wooden_button".to_owned(), "minecraft:oak_button".to_owned());
        map.insert("minecraft:wooden_door".to_owned(), "minecraft:oak_door".to_owned());
        map.insert("minecraft:wooden_pressure_plate".to_owned(), "minecraft:oak_pressure_plate".to_owned());
        map.insert("minecraft:yellow_stained_hardened_clay".to_owned(), "minecraft:yellow_terracotta".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    let recipes_updates = recipes_updates();
    rename_recipe(types, VERSION, move |name| recipes_updates.get(name).cloned());
}
