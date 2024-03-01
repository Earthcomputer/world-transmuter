use crate::types;
use java_string::JavaStr;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 2538;

const MERGE_KEYS: [&JavaStr; 7] = [
    JavaStr::from_str("RandomSeed"),
    JavaStr::from_str("generatorName"),
    JavaStr::from_str("generatorOptions"),
    JavaStr::from_str("generatorVersion"),
    JavaStr::from_str("legacy_custom_options"),
    JavaStr::from_str("MapFeatures"),
    JavaStr::from_str("BonusChest"),
];

pub(crate) fn register() {
    types::level_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let mut new_world_gen_settings = match data.remove("WorldGenSettings") {
                Some(JValue::Compound(compound)) => compound,
                _ => JCompound::new(),
            };

            for key in MERGE_KEYS {
                if let Some(value) = data.remove(key) {
                    new_world_gen_settings.insert(key, value);
                }
            }

            data.insert("WorldGenSettings", new_world_gen_settings);
        }),
    );
}
