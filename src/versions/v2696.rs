use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block_and_fix_jigsaw, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2696;

static RENAMES: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renames() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:grimstone", "minecraft:deepslate");
        map.insert("minecraft:grimstone_slab", "minecraft:cobbled_deepslate_slab");
        map.insert("minecraft:grimstone_stairs", "minecraft:cobbled_deepslate_stairs");
        map.insert("minecraft:grimstone_wall", "minecraft:cobbled_deepslate_wall");
        map.insert("minecraft:polished_grimstone", "minecraft:polished_deepslate");
        map.insert("minecraft:polished_grimstone_slab", "minecraft:polished_deepslate_slab");
        map.insert("minecraft:polished_grimstone_stairs", "minecraft:polished_deepslate_stairs");
        map.insert("minecraft:polished_grimstone_wall", "minecraft:polished_deepslate_wall");
        map.insert("minecraft:grimstone_tiles", "minecraft:deepslate_tiles");
        map.insert("minecraft:grimstone_tile_slab", "minecraft:deepslate_tile_slab");
        map.insert("minecraft:grimstone_tile_stairs", "minecraft:deepslate_tile_stairs");
        map.insert("minecraft:grimstone_tile_wall", "minecraft:deepslate_tile_wall");
        map.insert("minecraft:grimstone_bricks", "minecraft:deepslate_bricks");
        map.insert("minecraft:grimstone_brick_slab", "minecraft:deepslate_brick_slab");
        map.insert("minecraft:grimstone_brick_stairs", "minecraft:deepslate_brick_stairs");
        map.insert("minecraft:grimstone_brick_wall", "minecraft:deepslate_brick_wall");
        map.insert("minecraft:chiseled_grimstone", "minecraft:chiseled_deepslate");
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item(types, VERSION, |name| renames().get(name).map(|&str| str.to_owned()));
    rename_block_and_fix_jigsaw(types, VERSION, |name| renames().get(name).map(|&str| str.to_owned()));
}
