use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1905;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            if level.get_string("Status") == Some("postprocessed") {
                level.set("Status", T::Object::create_string("fullchunk".to_owned()));
            }
        }
    }));
}
