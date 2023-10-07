use crate::types;
use std::mem;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3565;

pub(crate) fn register() {
    types::saved_data_random_sequences_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(data)) = data.get_mut("data") else {
                return;
            };

            let old_data = mem::take(data);
            data.insert("sequences", old_data);
        }),
    );
}
