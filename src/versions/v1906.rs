use rust_dataconverter_engine::{DataWalkerMapListPaths, DataWalkerMapTypePaths, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1906;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:barrel", DataWalkerMapListPaths::new(types.item_stack, "Items"));
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:smoker", DataWalkerMapListPaths::new(types.item_stack, "Items"));
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:blast_furnace", DataWalkerMapListPaths::new(types.item_stack, "Items"));
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:lectern", DataWalkerMapTypePaths::new(types.item_stack, "Book"));
}
