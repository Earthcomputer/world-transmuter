use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1474;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:shulker", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_i64("Color").map(|i| i as i32) == Some(10) {
            data.set("Color", T::Object::create_byte(16));
        }
    }));
    // data hooks ensure the inputs are namespaced
    rename_block::<T>(types, VERSION, simple_rename("minecraft:purple_shulker_box", "minecraft:shulker_box"));
    rename_item::<T>(types, VERSION, simple_rename("minecraft:purple_shulker_box", "minecraft:shulker_box"));
}
