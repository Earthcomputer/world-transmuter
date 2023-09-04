use crate::helpers::json_parser;
use crate::types;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1948;

pub(crate) fn register() {
    types::item_stack_mut().add_converter_for_id(
        "minecraft:white_banner",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            let Some(Value::Compound(display)) = tag.get_mut("display") else {
                return;
            };
            let Some(Value::String(name)) = display.get_mut("Name") else {
                return;
            };
            if let Ok(mut json) = json_parser::parse_compound(name) {
                if let Some(Value::String(translate)) = json.get_mut("translate") {
                    *translate = "block.minecraft.ominous_banner".to_owned();
                    *name = json_parser::stringify_compound(json, false);
                }
            }
        }),
    );
}
