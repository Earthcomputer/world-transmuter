use crate::helpers::components::EMPTY_COMPONENT;
use crate::types;
use java_string::{JavaStr, JavaString};
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{map_data_converter_func, JCompound, JList, JValue};

const VERSION: u32 = 3439;

const DEFAULT_COLOR: &JavaStr = JavaStr::from_str("black");

fn get_line_opt(root: &JCompound, key: &str) -> Option<JavaString> {
    match root.get(key) {
        Some(JValue::String(line)) => Some(line.clone()),
        _ => None,
    }
}

fn get_line(root: &JCompound, key: &str) -> JavaString {
    get_line_opt(root, key).unwrap_or_else(|| EMPTY_COMPONENT.to_owned())
}

pub(crate) fn register() {
    for sign_id in ["minecraft:sign", "minecraft:hanging_sign"] {
        types::tile_entity_mut().add_converter_for_id(sign_id, VERSION, map_data_converter_func(|data, _from_version, _to_version| {
            let mut front_text = jcompound! {
                "messages" => JList::String(vec![
                    get_line(data, "Text1"),
                    get_line(data, "Text2"),
                    get_line(data, "Text3"),
                    get_line(data, "Text4"),
                ]),
                "color" => match data.get("Color") {
                    Some(JValue::String(color)) => color.clone(),
                    _ => DEFAULT_COLOR.to_owned(),
                },
                "has_glowing_text" => data.get("GlowingText").and_then(|v| v.as_bool()).unwrap_or(false),
                "_filtered_correct" => true,
            };

            let mut need_filtered_messages = false;
            let filtered_messages = vec![
                get_line_opt(data, "FilteredText1").inspect(|_| { need_filtered_messages = true; }).unwrap_or_else(|| get_line(data, "Text1")),
                get_line_opt(data, "FilteredText2").inspect(|_| { need_filtered_messages = true; }).unwrap_or_else(|| get_line(data, "Text2")),
                get_line_opt(data, "FilteredText3").inspect(|_| { need_filtered_messages = true; }).unwrap_or_else(|| get_line(data, "Text3")),
                get_line_opt(data, "FilteredText4").inspect(|_| { need_filtered_messages = true; }).unwrap_or_else(|| get_line(data, "Text4")),
            ];
            if need_filtered_messages {
                front_text.insert("filtered_messages", JList::String(filtered_messages));
            }

            data.insert("front_text", front_text);

            let back_text = jcompound! {
                "messages" => JList::String(vec![EMPTY_COMPONENT.to_owned(); 4]),
                "filtered_messages" => JList::String(vec![EMPTY_COMPONENT.to_owned(); 4]),
                "color" => DEFAULT_COLOR,
                "has_glowing_text" => false,
            };
            data.insert("back_text", back_text);

            data.insert("is_waxed", false);
        }));
    }
}
