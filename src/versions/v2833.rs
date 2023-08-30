use crate::types::MinecraftTypes;
use log::error;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;

const VERSION: u32 = 2833;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.world_gen_settings.borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
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
