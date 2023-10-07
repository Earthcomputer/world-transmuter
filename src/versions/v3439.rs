use crate::types;
use valence_nbt::{compound, Compound, List, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3439;

const BLANK_TEXT_LINE: &str = "{\"text\":\"\"}";
const DEFAULT_COLOR: &str = "black";

fn get_line_opt(root: &Compound, key: &str) -> Option<String> {
    match root.get(key) {
        Some(Value::String(line)) => Some(line.clone()),
        _ => None,
    }
}

fn get_line(root: &Compound, key: &str) -> String {
    get_line_opt(root, key).unwrap_or_else(|| BLANK_TEXT_LINE.to_owned())
}

pub(crate) fn register() {
    for sign_id in ["minecraft:sign", "minecraft:hanging_sign"] {
        types::tile_entity_mut().add_converter_for_id(sign_id, VERSION, map_data_converter_func(|data, _from_version, _to_version| {
            let mut front_text = compound! {
                "messages" => List::String(vec![
                    get_line(data, "Text1"),
                    get_line(data, "Text2"),
                    get_line(data, "Text3"),
                    get_line(data, "Text4"),
                ]),
                "color" => match data.get("Color") {
                    Some(Value::String(color)) => color.clone(),
                    _ => DEFAULT_COLOR.to_owned(),
                },
                "has_glowing_text" => data.get("GlowingText").and_then(|v| v.as_bool()).unwrap_or(false),
                "_filtered_correct" => true,
            };

            let mut need_filtered_messages = false;
            let filtered_messages = vec![
                get_line_opt(data, "FilteredText1").map(|s| { need_filtered_messages = true; s }).unwrap_or_else(|| get_line(data, "Text1")),
                get_line_opt(data, "FilteredText2").map(|s| { need_filtered_messages = true; s }).unwrap_or_else(|| get_line(data, "Text2")),
                get_line_opt(data, "FilteredText3").map(|s| { need_filtered_messages = true; s }).unwrap_or_else(|| get_line(data, "Text3")),
                get_line_opt(data, "FilteredText4").map(|s| { need_filtered_messages = true; s }).unwrap_or_else(|| get_line(data, "Text4")),
            ];
            if need_filtered_messages {
                front_text.insert("filtered_messages", List::String(filtered_messages));
            }

            data.insert("front_text", front_text);

            let back_text = compound! {
                "messages" => List::String(vec![BLANK_TEXT_LINE.to_owned(); 4]),
                "filtered_messages" => List::String(vec![BLANK_TEXT_LINE.to_owned(); 4]),
                "color" => DEFAULT_COLOR,
                "has_glowing_text" => false,
            };
            data.insert("back_text", back_text);

            data.insert("is_waxed", false);
        }));
    }
}
