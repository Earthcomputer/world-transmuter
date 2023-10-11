use crate::types;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 2533;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id("minecraft:villager", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(JValue::List(JList::Compound(attributes))) = data.get_mut("Attributes") {
            for attribute in attributes {
                if matches!(attribute.get("Name"), Some(JValue::String(str)) if str == "generic.follow_range") && attribute.get("Base").and_then(|v| v.as_f64()) == Some(16.0) {
                    attribute.insert("Base", 48.0);
                }
            }
        }
    }));
}
