use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;
use crate::helpers::json_parser;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1953;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:banner", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(Value::String(custom_name)) = data.get_mut("CustomName") {
            if let Ok(mut json) = json_parser::parse_map(custom_name) {
                if let Some(Value::String(translate)) = json.get_mut("translate") {
                    if translate == "block.minecraft.illager_banner" {
                        *translate = "block.minecraft.ominous_banner".to_owned();
                        *custom_name = json_parser::stringify_map(json);
                    }
                }
            }
        }
    }));
}
