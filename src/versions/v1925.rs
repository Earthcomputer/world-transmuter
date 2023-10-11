use crate::types;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 1925;

pub(crate) fn register() {
    types::saved_data_map_data_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if !matches!(data.get("data"), Some(JValue::Compound(_))) {
                data.insert("data", JCompound::new());
            }
        }),
    );
}
