use crate::helpers::gson_lenient_fix::{fix_gson_lenient, FixedGsonLenient, JsonType};
use crate::types;
use java_string::{format_java, JavaStr, JavaString};
use std::borrow::{Borrow, Cow};
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 101;

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id(
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

fn update_line(data: &mut JCompound, key: &str) {
    let Some(JValue::String(str)) = data.get(key) else {
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
            fixed_str: Cow::Owned(format_java!(
                "\"{}\"",
                str.replace('\\', "\\\\").replace('"', "\\\"")
            )),
        });
        match value_type {
            JsonType::Object | JsonType::Array => fixed_str.into_owned(),
            JsonType::String | JsonType::Number => format_java!("{{\"text\":{}}}", fixed_str),
            JsonType::Keyword => {
                if Borrow::<JavaStr>::borrow(&fixed_str) == "null" {
                    JavaString::from("{\"text\":\"\"}")
                } else {
                    format_java!("{{\"text\":\"{}\"}}", fixed_str)
                }
            }
        }
    } else {
        format_java!(
            "{{\"text\":\"{}\"}}",
            str.replace('\\', "\\\\").replace('"', "\\\"")
        )
    };

    data.insert(key, result);
}
