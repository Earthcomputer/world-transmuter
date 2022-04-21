use rust_dataconverter_engine::{data_converter_func, MapType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 816;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.options.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(lang) = data.get_string_mut("lang") {
            lang.make_ascii_lowercase();
        }
    }));
}
