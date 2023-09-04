use crate::types;
use valence_nbt::{compound, Compound, List, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3439;

const BLANK_TEXT_LINE: &str = "{\"text\":\"\"}";
const DEFAULT_COLOR: &str = "black";

fn get_line(root: &Compound, key: &str) -> String {
    match root.get(key) {
        Some(Value::String(line)) => line.clone(),
        _ => BLANK_TEXT_LINE.to_owned(),
    }
}

pub(crate) fn register() {
    for sign_id in ["minecraft:sign", "minecraft:hanging_sign"] {
        types::tile_entity_mut().add_converter_for_id(sign_id, VERSION, map_data_converter_func(|data, _from_version, _to_version| {
            macro_rules! migrate_to_list {
                ($root:expr, $prefix:literal) => {
                    List::String(vec![
                        get_line($root, concat!($prefix, "1")),
                        get_line($root, concat!($prefix, "2")),
                        get_line($root, concat!($prefix, "3")),
                        get_line($root, concat!($prefix, "4")),
                    ])
                }
            }

            let front_text = compound! {
                "messages" => migrate_to_list!(data, "Text"),
                "filtered_messages" => migrate_to_list!(data, "FilteredText"),
                "color" => match data.get("Color") {
                    Some(Value::String(color)) => color.clone(),
                    _ => DEFAULT_COLOR.to_owned(),
                },
                "has_glowing_text" => data.get("GlowingText").and_then(|v| v.as_bool()).unwrap_or(false),
            };
            data.insert("front_text", front_text);

            let back_text = compound! {
                "messages" => List::String(vec![BLANK_TEXT_LINE.to_owned(); 4]),
                "filtered_messages" => List::String(vec![BLANK_TEXT_LINE.to_owned(); 4]),
                "color" => DEFAULT_COLOR,
                "has_glowing_text" => false,
            };
            data.insert("back_text", back_text);
        }));
    }
}
