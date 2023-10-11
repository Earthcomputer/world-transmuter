use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 3214;

pub(crate) fn register() {
    types::options_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(ao)) = data.get_mut("ao") {
                if ao == "0" {
                    *ao = JavaString::from("false");
                } else if ao == "1" {
                    *ao = JavaString::from("true");
                }
            }
        }),
    );
}
