use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{DataWalkerMapListPaths, Types};
use crate::helpers::rename::rename_item;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1800;

static RENAMED_ITEM_IDS: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn renamed_item_ids() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    RENAMED_ITEM_IDS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:cactus_green", "minecraft:green_dye");
        map.insert("minecraft:rose_red", "minecraft:red_dye");
        map.insert("minecraft:dandelion_yellow", "minecraft:yellow_dye");
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item(types, VERSION, |name| renamed_item_ids().get(name).map(|&str| str.to_owned()));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:panda", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:pillager", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["Inventory".to_owned(), "ArmorItems".to_owned(), "HandItems".to_owned()]));
}
