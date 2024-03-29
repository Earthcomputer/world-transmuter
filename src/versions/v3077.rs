use crate::types;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 3077;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.get("isLightOn").and_then(|v| v.as_bool()) == Some(true) {
                return;
            }

            let Some(JValue::List(JList::Compound(sections))) = data.get_mut("sections") else {
                return;
            };
            for section in sections {
                section.remove("BlockLight");
                section.remove("SkyLight");
            }
        }),
    );
}
