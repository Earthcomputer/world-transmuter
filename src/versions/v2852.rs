use crate::types;
use log::error;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 2852;

pub(crate) fn register() {
    types::world_gen_settings_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(Value::Compound(dimensions)) = data.get("dimensions") {
            for dimension in dimensions.values() {
                if let Value::Compound(dimension) = dimension {
                    if !dimension.contains_key("type") {
                        error!("Unable to load old custom worlds. Conversion may clobber the world!");
                    }
                }
            }
        }
    }));
}
