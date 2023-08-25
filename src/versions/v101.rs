use crate::helpers::gson_lenient_fix::{fix_gson_lenient, FixedGsonLenient, JsonType};
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use std::borrow::Cow;
use valence_nbt::{Compound, Value};

const VERSION: u32 = 101;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.tile_entity.borrow_mut().add_converter_for_id(
        "Sign",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_line(data, "Text1");
            update_line(data, "Text2");
            update_line(data, "Text3");
            update_line(data, "Text4");
        }),
    );
}

fn update_line(data: &mut Compound, key: &str) {
    let Some(Value::String(str)) = data.get(key) else {
        return;
    };

    let result = if (str.starts_with('"') && str.ends_with('"'))
        || (str.starts_with('{') && str.ends_with('}'))
    {
        let FixedGsonLenient {
            value_type,
            fixed_str,
        } = fix_gson_lenient(str).unwrap_or(FixedGsonLenient {
            value_type: JsonType::String,
            fixed_str: Cow::Owned(format!(
                "\"{}\"",
                str.replace('\\', "\\\\").replace('"', "\\\"")
            )),
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
        format!(
            "{{\"text\":\"{}\"}}",
            str.replace('\\', "\\\\").replace('"', "\\\"")
        )
    };

    data.insert(key, result);
}
