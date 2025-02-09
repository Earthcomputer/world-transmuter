use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 4057;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(mut carving_masks)) = data.remove("CarvingMasks") else {
                return;
            };
            if let Some(JValue::LongArray(air_mask)) = carving_masks.remove("AIR") {
                data.insert("carving_mask", air_mask);
            }
        }),
    );
}
