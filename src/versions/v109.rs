use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 109;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(health) = data.remove("HealF").and_then(|o| o.as_f64()).or_else(|| data.get_f64("Health")) {
            data.set("Health", T::Object::create_float(health as f32));
        }
    }));
}
