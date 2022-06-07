use rust_dataconverter_engine::{data_converter_func, MapType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1961;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            level.remove("isLightOn");
        }
    }));
}
