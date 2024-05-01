use crate::types;
use crate::versions::v100;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 2505;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:villager",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(brain)) = data.get_mut("Brain") {
                if let Some(JValue::Compound(memories)) = brain.get_mut("memories") {
                    for value in memories.values_mut() {
                        let temp = std::mem::replace(value, JValue::Compound(JCompound::new()));
                        let JValue::Compound(value_compound) = value else {
                            unreachable!()
                        };
                        value_compound.insert("value", temp);
                    }
                }
            }
        }),
    );

    v100::register_equipment(VERSION, "minecraft:piglin");
}
