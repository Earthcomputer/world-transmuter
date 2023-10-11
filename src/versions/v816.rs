use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 816;

pub(crate) fn register() {
    types::options_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(lang)) = data.get_mut("lang") {
                lang.make_ascii_lowercase();
            }
        }),
    );
}
