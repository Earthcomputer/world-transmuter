use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 3108;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(context)) = data.get("__context") else {
                return;
            };
            let Some(JValue::String(dimension)) = context.get("dimension") else {
                return;
            };
            if dimension != "minecraft:overworld" {
                return;
            }
            data.remove("blending_data");
        }),
    );
}
