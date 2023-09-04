use crate::helpers::json_parser;
use crate::types;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1953;

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:banner",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(custom_name)) = data.get_mut("CustomName") {
                if let Ok(mut json) = json_parser::parse_compound(custom_name) {
                    if let Some(Value::String(translate)) = json.get_mut("translate") {
                        if translate == "block.minecraft.illager_banner" {
                            *translate = "block.minecraft.ominous_banner".to_owned();
                            *custom_name = json_parser::stringify_compound(json, false);
                        }
                    }
                }
            }
        }),
    );
}
