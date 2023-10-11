use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1905;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(level)) = data.get_mut("Level") {
                if matches!(level.get("Status"), Some(JValue::String(str)) if str == "postprocessed")
                {
                    level.insert("Status", "fullchunk");
                }
            }
        }),
    );
}
