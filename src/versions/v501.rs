use rust_dataconverter_engine::{DataWalkerMapListPaths, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 501;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    register_mob(types, "PolarBear");
}

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
