use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;
use crate::helpers::json_parser;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1948;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:white_banner", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        let Some(Value::Compound(tag)) = data.get_mut("tag") else { return };
        let Some(Value::Compound(display)) = tag.get_mut("display") else { return };
        let Some(Value::String(name)) = display.get_mut("Name") else { return };
        if let Ok(mut json) = json_parser::parse_map(name) {
            if let Some(Value::String(translate)) = json.get_mut("translate") {
                *translate = "block.minecraft.ominous_banner".to_owned();
                *name = json_parser::stringify_map(json);
            }
        }
    }));
}
