use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 808;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:shulker", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_i64("Color").is_none() {
            data.set("Color", T::Object::create_byte(10));
        }
    }));

    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:shulker_box", DataWalkerMapListPaths::new(types.item_stack, "Items"));
}
