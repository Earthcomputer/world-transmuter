use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1946;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.poi_chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let mut sections = T::Map::create_empty();
        for y in 0..16 {
            let key = y.to_string();
            if let Some(records) = data.remove(&key) {
                let mut section = T::Map::create_empty();
                section.set("Records", records);
                sections.set(key, T::Object::create_map(section)); // integer keys convert to string in DFU (at least for NBT ops)
            }
        }
        data.set("Sections", T::Object::create_map(sections));
    }));
}
