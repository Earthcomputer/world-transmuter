use crate::types;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 113;

pub(crate) fn register() {
    // Removes "HandDropChances" and "ArmorDropChances" if they're empty.
    types::entity_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            check_list(data, "HandDropChances", 2);
            check_list(data, "ArmorDropChances", 4);
        }),
    );
}

fn check_list(data: &mut JCompound, id: &str, required_length: usize) {
    if let Some(JValue::List(list)) = data.get(id) {
        if list.len() == required_length {
            for float in list {
                if let Some(float) = float.as_f32() {
                    if float != 0.0 {
                        return;
                    }
                }
            }
        }
    }

    data.remove(id);
}
