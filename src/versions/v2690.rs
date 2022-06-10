use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block_and_fix_jigsaw, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2690;

static RENAMES: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renames() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:weathered_copper_block", "minecraft:oxidized_copper_block");
        map.insert("minecraft:semi_weathered_copper_block", "minecraft:weathered_copper_block");
        map.insert("minecraft:lightly_weathered_copper_block", "minecraft:exposed_copper_block");
        map.insert("minecraft:weathered_cut_copper", "minecraft:oxidized_cut_copper");
        map.insert("minecraft:semi_weathered_cut_copper", "minecraft:weathered_cut_copper");
        map.insert("minecraft:lightly_weathered_cut_copper", "minecraft:exposed_cut_copper");
        map.insert("minecraft:weathered_cut_copper_stairs", "minecraft:oxidized_cut_copper_stairs");
        map.insert("minecraft:semi_weathered_cut_copper_stairs", "minecraft:weathered_cut_copper_stairs");
        map.insert("minecraft:lightly_weathered_cut_copper_stairs", "minecraft:exposed_cut_copper_stairs");
        map.insert("minecraft:weathered_cut_copper_slab", "minecraft:oxidized_cut_copper_slab");
        map.insert("minecraft:semi_weathered_cut_copper_slab", "minecraft:weathered_cut_copper_slab");
        map.insert("minecraft:lightly_weathered_cut_copper_slab", "minecraft:exposed_cut_copper_slab");
        map.insert("minecraft:waxed_semi_weathered_copper", "minecraft:waxed_weathered_copper");
        map.insert("minecraft:waxed_lightly_weathered_copper", "minecraft:waxed_exposed_copper");
        map.insert("minecraft:waxed_semi_weathered_cut_copper", "minecraft:waxed_weathered_cut_copper");
        map.insert("minecraft:waxed_lightly_weathered_cut_copper", "minecraft:waxed_exposed_cut_copper");
        map.insert("minecraft:waxed_semi_weathered_cut_copper_stairs", "minecraft:waxed_weathered_cut_copper_stairs");
        map.insert("minecraft:waxed_lightly_weathered_cut_copper_stairs", "minecraft:waxed_exposed_cut_copper_stairs");
        map.insert("minecraft:waxed_semi_weathered_cut_copper_slab", "minecraft:waxed_weathered_cut_copper_slab");
        map.insert("minecraft:waxed_lightly_weathered_cut_copper_slab", "minecraft:waxed_exposed_cut_copper_slab");
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item(types, VERSION, |name| renames().get(name).map(|&str| str.to_owned()));
    rename_block_and_fix_jigsaw(types, VERSION, |name| renames().get(name).map(|&str| str.to_owned()));
}
