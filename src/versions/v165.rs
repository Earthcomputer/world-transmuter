use crate::helpers::components::convert_component_from_lenient;
use crate::types;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 165;

pub(crate) fn register() {
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(tag)) = data.get_mut("tag") {
                if let Some(JValue::List(JList::String(pages))) = tag.get_mut("pages") {
                    for page in pages {
                        *page = convert_component_from_lenient(page);
                    }
                }
            }
        }),
    );
}
