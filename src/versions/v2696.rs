use std::sync::OnceLock;
use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::{rename_block_and_fix_jigsaw, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2696;

static RENAMES: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renames() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("grimstone", "minecraft:deepslate");
        map.insert_mc("grimstone_slab", "minecraft:cobbled_deepslate_slab");
        map.insert_mc("grimstone_stairs", "minecraft:cobbled_deepslate_stairs");
        map.insert_mc("grimstone_wall", "minecraft:cobbled_deepslate_wall");
        map.insert_mc("polished_grimstone", "minecraft:polished_deepslate");
        map.insert_mc("polished_grimstone_slab", "minecraft:polished_deepslate_slab");
        map.insert_mc("polished_grimstone_stairs", "minecraft:polished_deepslate_stairs");
        map.insert_mc("polished_grimstone_wall", "minecraft:polished_deepslate_wall");
        map.insert_mc("grimstone_tiles", "minecraft:deepslate_tiles");
        map.insert_mc("grimstone_tile_slab", "minecraft:deepslate_tile_slab");
        map.insert_mc("grimstone_tile_stairs", "minecraft:deepslate_tile_stairs");
        map.insert_mc("grimstone_tile_wall", "minecraft:deepslate_tile_wall");
        map.insert_mc("grimstone_bricks", "minecraft:deepslate_bricks");
        map.insert_mc("grimstone_brick_slab", "minecraft:deepslate_brick_slab");
        map.insert_mc("grimstone_brick_stairs", "minecraft:deepslate_brick_stairs");
        map.insert_mc("grimstone_brick_wall", "minecraft:deepslate_brick_wall");
        map.insert_mc("chiseled_grimstone", "minecraft:chiseled_deepslate");
        map
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename_item(types, VERSION, |name| renames().get(name).map(|&str| str.to_owned()));
    rename_block_and_fix_jigsaw(types, VERSION, |name| renames().get(name).map(|&str| str.to_owned()));
}
