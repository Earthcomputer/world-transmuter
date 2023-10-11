use crate::types;
use java_string::format_java;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 1803;

pub(crate) fn register() {
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            let Some(JValue::Compound(display)) = tag.get_mut("display") else {
                return;
            };
            let Some(JValue::List(JList::String(lore))) = display.get_mut("Lore") else {
                return;
            };
            for lore in lore {
                let new_lore = format_java!(
                    "{{\"text\":\"{}\"}}",
                    lore.replace('\\', "\\\\").replace('"', "\\\"")
                );
                *lore = new_lore;
            }
        }),
    );
}
