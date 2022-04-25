use rust_dataconverter_engine::{DataWalkerMapListPaths, DataWalkerMapTypePaths, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1470;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    register_mob(types, "minecraft:turtle");
    register_mob(types, "minecraft:cod_mob");
    register_mob(types, "minecraft:tropical_fish");
    register_mob(types, "minecraft:salmon_mob");
    register_mob(types, "minecraft:puffer_fish");
    register_mob(types, "minecraft:phantom");
    register_mob(types, "minecraft:dolphin");
    register_mob(types, "minecraft:drowned");

    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:trident", DataWalkerMapTypePaths::new(types.block_state, "inBlockState"));
}

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
