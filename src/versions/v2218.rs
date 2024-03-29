use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 2218;

pub(crate) fn register() {
    types::poi_chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(sections)) = data.get_mut("Sections") {
                for section in sections.values_mut() {
                    if let JValue::Compound(section) = section {
                        section.remove("Valid");
                    }
                }
            }
        }),
    );
}
