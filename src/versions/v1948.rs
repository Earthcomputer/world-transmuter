use crate::helpers::json_parser;
use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1948;

pub(crate) fn register() {
    types::item_stack_mut().add_converter_for_id(
        "minecraft:white_banner",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            let Some(JValue::Compound(display)) = tag.get_mut("display") else {
                return;
            };
            let Some(JValue::String(name)) = display.get_mut("Name") else {
                return;
            };
            if let Ok(mut json) = json_parser::parse_compound(name, true) {
                if let Some(JValue::String(translate)) = json.get_mut("translate") {
                    *translate = JavaString::from("block.minecraft.ominous_banner");
                    *name = json_parser::stringify_compound(json, true, false);
                }
            }
        }),
    );
}
