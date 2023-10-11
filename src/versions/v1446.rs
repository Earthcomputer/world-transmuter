use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1446;

pub(crate) fn register() {
    types::options_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let mut replacements = Vec::new();

            for key in data.keys() {
                if !key.starts_with("key_") {
                    continue;
                }

                if let Some(JValue::String(value)) = data.get(&key[..]) {
                    if value.starts_with("key.mouse") {
                        continue;
                    }
                    if let Some(value) = value.strip_prefix("key.") {
                        replacements.push((key.clone(), format!("key.keyboard.{}", value)));
                    }
                }
            }

            for (key, value) in replacements {
                data.insert(key, value);
            }
        }),
    );
}
