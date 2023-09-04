use crate::types;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 2679;

pub(crate) fn register() {
    types::block_state_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if matches!(data.get("Name"), Some(Value::String(str)) if str == "minecraft:cauldron") {
                if let Some(Value::Compound(properties)) = data.get("Properties") {
                    let is_empty_cauldron = match properties.get("level") {
                        Some(Value::String(str)) if str == "0" => true,
                        None => true,
                        _ => false,
                    };
                    if is_empty_cauldron {
                        data.remove("Properties");
                    } else {
                        data.insert("Name", "minecraft:water_cauldron");
                    }
                }
            }
        }),
    );
}
