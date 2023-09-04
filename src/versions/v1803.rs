use crate::types;
use valence_nbt::{List, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1803;

pub(crate) fn register() {
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(tag)) = data.get_mut("tag") else {
                return;
            };
            let Some(Value::Compound(display)) = tag.get_mut("display") else {
                return;
            };
            let Some(Value::List(List::String(lore))) = display.get_mut("Lore") else {
                return;
            };
            for lore in lore {
                let new_lore = format!(
                    "{{\"text\":\"{}\"}}",
                    lore.replace('\\', "\\\\").replace('"', "\\\"")
                );
                *lore = new_lore;
            }
        }),
    );
}
