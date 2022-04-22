use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1446;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.options.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let mut replacements = Vec::new();

        for key in data.keys() {
            if !key.starts_with("key_") {
                continue;
            }

            if let Some(value) = data.get_string(key) {
                if value.starts_with("key.mouse") {
                    continue;
                }
                if let Some(value) = value.strip_prefix("key.") {
                    replacements.push((key.clone(), format!("key.keyboard.{}", value)));
                }
            }
        }

        for (key, value) in replacements {
            data.set(key, T::Object::create_string(value));
        }
    }));
}
