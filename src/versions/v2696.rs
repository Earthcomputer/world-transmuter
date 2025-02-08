use crate::helpers::rename::{rename_block, rename_item};
use crate::static_string_mc_map;

const VERSION: u32 = 2696;

static_string_mc_map! {
    renames = {
        "grimstone" => "minecraft:deepslate",
        "grimstone_slab" => "minecraft:cobbled_deepslate_slab",
        "grimstone_stairs" => "minecraft:cobbled_deepslate_stairs",
        "grimstone_wall" => "minecraft:cobbled_deepslate_wall",
        "polished_grimstone" => "minecraft:polished_deepslate",
        "polished_grimstone_slab" => "minecraft:polished_deepslate_slab",
        "polished_grimstone_stairs" => "minecraft:polished_deepslate_stairs",
        "polished_grimstone_wall" => "minecraft:polished_deepslate_wall",
        "grimstone_tiles" => "minecraft:deepslate_tiles",
        "grimstone_tile_slab" => "minecraft:deepslate_tile_slab",
        "grimstone_tile_stairs" => "minecraft:deepslate_tile_stairs",
        "grimstone_tile_wall" => "minecraft:deepslate_tile_wall",
        "grimstone_bricks" => "minecraft:deepslate_bricks",
        "grimstone_brick_slab" => "minecraft:deepslate_brick_slab",
        "grimstone_brick_stairs" => "minecraft:deepslate_brick_stairs",
        "grimstone_brick_wall" => "minecraft:deepslate_brick_wall",
        "chiseled_grimstone" => "minecraft:chiseled_deepslate",
    }
}

pub(crate) fn register() {
    rename_item(VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
    rename_block(VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
}
