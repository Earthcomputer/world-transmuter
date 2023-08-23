use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2218;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.poi_chunk.borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(Value::Compound(sections)) = data.get_mut("Sections") {
            for section in sections.values_mut() {
                if let Value::Compound(section) = section {
                    section.remove("Valid");
                }
            }
        }
    }));
}
