use rust_dataconverter_engine::{convert_map_in_map, data_walker, DataWalkerMapListPaths, DataWalkerObjectTypePaths, Types};
use crate::{MinecraftTypesMut};

const VERSION: u32 = 99;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    let entity = types.entity;
    entity.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data, from_version, to_version| {
        convert_map_in_map::<_, T>(entity, data, "Riding", from_version, to_version);
    }));
}

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new(types.item_stack, "Equipment"));
}

fn register_projectile<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerObjectTypePaths::new(types.block_name, "inTile"));
}

fn register_inventory<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new(types.item_stack, "Items"));
}
