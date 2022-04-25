use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1480;

static RENAMED_IDS: SyncOnceCell<rust_dataconverter_engine::Map<String, String>> = SyncOnceCell::new();

fn renamed_ids() -> &'static rust_dataconverter_engine::Map<String, String> {
    RENAMED_IDS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:blue_coral".to_owned(), "minecraft:tube_coral_block".to_owned());
        map.insert("minecraft:pink_coral".to_owned(), "minecraft:brain_coral_block".to_owned());
        map.insert("minecraft:purple_coral".to_owned(), "minecraft:bubble_coral_block".to_owned());
        map.insert("minecraft:red_coral".to_owned(), "minecraft:fire_coral_block".to_owned());
        map.insert("minecraft:yellow_coral".to_owned(), "minecraft:horn_coral_block".to_owned());
        map.insert("minecraft:blue_coral_plant".to_owned(), "minecraft:tube_coral".to_owned());
        map.insert("minecraft:pink_coral_plant".to_owned(), "minecraft:brain_coral".to_owned());
        map.insert("minecraft:purple_coral_plant".to_owned(), "minecraft:bubble_coral".to_owned());
        map.insert("minecraft:red_coral_plant".to_owned(), "minecraft:fire_coral".to_owned());
        map.insert("minecraft:yellow_coral_plant".to_owned(), "minecraft:horn_coral".to_owned());
        map.insert("minecraft:blue_coral_fan".to_owned(), "minecraft:tube_coral_fan".to_owned());
        map.insert("minecraft:pink_coral_fan".to_owned(), "minecraft:brain_coral_fan".to_owned());
        map.insert("minecraft:purple_coral_fan".to_owned(), "minecraft:bubble_coral_fan".to_owned());
        map.insert("minecraft:red_coral_fan".to_owned(), "minecraft:fire_coral_fan".to_owned());
        map.insert("minecraft:yellow_coral_fan".to_owned(), "minecraft:horn_coral_fan".to_owned());
        map.insert("minecraft:blue_dead_coral".to_owned(), "minecraft:dead_tube_coral".to_owned());
        map.insert("minecraft:pink_dead_coral".to_owned(), "minecraft:dead_brain_coral".to_owned());
        map.insert("minecraft:purple_dead_coral".to_owned(), "minecraft:dead_bubble_coral".to_owned());
        map.insert("minecraft:red_dead_coral".to_owned(), "minecraft:dead_fire_coral".to_owned());
        map.insert("minecraft:yellow_dead_coral".to_owned(), "minecraft:dead_horn_coral".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_block::<T>(types, VERSION, |name| renamed_ids().get(name).cloned());
    rename_item::<T>(types, VERSION, |name| renamed_ids().get(name).cloned());
}
