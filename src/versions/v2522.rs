use rust_dataconverter_engine::DataWalkerMapListPaths;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2522;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:zoglin", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
