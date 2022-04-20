use std::borrow::Cow;
use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;
use crate::helpers::gson_lenient_fix::{FixedGsonLenient, fix_gson_lenient, JsonType};

const VERSION: u32 = 101;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.tile_entity.borrow_mut().add_converter_for_id("Sign", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_line::<T>(data, "Text1");
        update_line::<T>(data, "Text2");
        update_line::<T>(data, "Text3");
        update_line::<T>(data, "Text4");
    }));
}

fn update_line<T: Types + ?Sized>(data: &mut T::Map, key: &str) {
    let str = match data.get_string(key) {
        Some(str) => str,
        None => return
    };

    let result = if (str.starts_with('"') && str.ends_with('"')) || (str.starts_with('{') && str.ends_with('}')) {
        let FixedGsonLenient { value_type, fixed_str } = fix_gson_lenient(str)
            .unwrap_or(FixedGsonLenient {
                value_type: JsonType::String,
                fixed_str: Cow::Owned(format!("\"{}\"", str.replace('\\', "\\\\").replace('"', "\\\"")))
            });
        match value_type {
            JsonType::Object | JsonType::Array => fixed_str.into_owned(),
            JsonType::String | JsonType::Number => format!("{{\"text\":{}}}", fixed_str),
            JsonType::Keyword => {
                if fixed_str == "null" {
                    "{\"text\":\"\"}".to_owned()
                } else {
                    format!("{{\"text\":\"{}\"}}", fixed_str)
                }
            }
        }
    } else {
        format!("{{\"text\":\"{}\"}}", str.replace('\\', "\\\\").replace('"', "\\\""))
    };

    data.set(key, T::Object::create_string(result));
}
