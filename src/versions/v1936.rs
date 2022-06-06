use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1936;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.options.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(chat_opacity) = data.get_string("chatOpacity") {
            // Vanilla uses createDouble here, but options is always string -> string. I presume they made
            // a mistake with this converter.
            let background = calculate_background(chat_opacity);
            data.set("textBackgroundOpacity", T::Object::create_string(background));
        }
    }));
}

fn calculate_background(opacity: &str) -> String {
    opacity.parse::<f64>().map(|d| (0.9 * d + 0.1) / 2.0).unwrap_or(0.5).to_string()
}
