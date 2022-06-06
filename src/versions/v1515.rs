use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::Types;
use crate::helpers::rename::rename_block;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1515;

static RENAMED_BLOCK_IDS: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renamed_block_ids() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMED_BLOCK_IDS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:tube_coral_fan", "minecraft:tube_coral_wall_fan");
        map.insert("minecraft:brain_coral_fan", "minecraft:brain_coral_wall_fan");
        map.insert("minecraft:bubble_coral_fan", "minecraft:bubble_coral_wall_fan");
        map.insert("minecraft:fire_coral_fan", "minecraft:fire_coral_wall_fan");
        map.insert("minecraft:horn_coral_fan", "minecraft:horn_coral_wall_fan");
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_block(types, VERSION, |name| renamed_block_ids().get(name).map(|&str| str.to_owned()));
}
