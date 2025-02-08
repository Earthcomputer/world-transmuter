use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 3943;

pub(crate) fn register() {
    types::options_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(range_mut) = data.get_mut("menuBackgroundBlurriness") {
                if let JValue::String(range) = &*range_mut {
                    if let Ok(range) = range.parse::<f64>() {
                        *range_mut = JValue::String(range.to_string().into());
                    } else {
                        *range_mut = JValue::String("5".into());
                    }
                } else {
                    *range_mut = JValue::String("5".into());
                }
            } else {
                data.insert("menuBackgroundBlurriness", "5");
            }
        }),
    );
}
