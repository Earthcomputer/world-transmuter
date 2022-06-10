use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2679;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.block_state.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("Name") == Some("minecraft:cauldron") {
            if let Some(properties) = data.get_map("Properties") {
                if let Some("0") | None = properties.get_string("level") {
                    data.remove("Properties");
                } else {
                    data.set("Name", T::Object::create_string("minecraft:water_cauldron".to_owned()));
                }
            }
        }
    }));
}
