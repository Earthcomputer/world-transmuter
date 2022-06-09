use rust_dataconverter_engine::{DataWalkerMapListPaths, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2519;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:strider", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
