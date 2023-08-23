use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1961;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(Value::Compound(level)) = data.get_mut("Level") {
            level.remove("isLightOn");
        }
    }));
}
