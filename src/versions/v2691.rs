use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block_and_fix_jigsaw, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2691;

static RENAMES: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renames() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:waxed_copper", "minecraft:waxed_copper_block");
        map.insert("minecraft:oxidized_copper_block", "minecraft:oxidized_copper");
        map.insert("minecraft:weathered_copper_block", "minecraft:weathered_copper");
        map.insert("minecraft:exposed_copper_block", "minecraft:exposed_copper");
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item(types, VERSION, |name| renames().get(name).map(|&str| str.to_owned()));
    rename_block_and_fix_jigsaw(types, VERSION, |name| renames().get(name).map(|&str| str.to_owned()));
}
