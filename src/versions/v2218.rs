use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2218;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.poi_chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(sections) = data.get_map_mut("Sections") {
            for section in sections.values_mut() {
                if let Some(section) = section.as_map_mut() {
                    section.remove("Valid");
                }
            }
        }
    }));
}
