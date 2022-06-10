use rust_dataconverter_engine::{data_converter_func, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2552;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.biome.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Object, _>(|data, _from_version, _to_version| {
        if data.as_string() == Some("minecraft:nether") {
            *data = T::Object::create_string("minecraft:nether_wastes".to_owned());
        }
    }));
}
