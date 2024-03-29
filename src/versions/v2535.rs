use crate::types;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 2535;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:shulker",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            // Mojang uses doubles for whatever reason... rotation is in FLOAT. by using double here
            // the entity load will just ignore rotation and set it to 0...
            if let Some(JValue::List(rotation)) = data.get_mut("Rotation") {
                let mut new_rotation: Vec<_> = rotation.iter().filter_map(|v| v.as_f32()).collect();
                if !new_rotation.is_empty() {
                    new_rotation[0] -= 180.0;
                }
                *rotation = JList::Float(new_rotation);
            }
        }),
    );
}
