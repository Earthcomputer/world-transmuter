use rust_dataconverter_engine::{Types, data_converter_func, MapType, ObjectType};

use crate::MinecraftTypesMut;


const VERSION: u32 = 1500;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.tile_entity.borrow_mut().add_converter_for_id("DUMMY", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        data.set("keepPacked", T::Object::create_bool(true));
    }));
}
