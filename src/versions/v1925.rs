use crate::types;
use valence_nbt::{Compound, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1925;

pub(crate) fn register() {
    types::saved_data_map_data_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if !matches!(data.get("data"), Some(Value::Compound(_))) {
                data.insert("data", Compound::new());
            }
        }),
    );
}
