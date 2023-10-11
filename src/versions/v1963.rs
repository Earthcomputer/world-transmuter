use crate::types;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 1963;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id("minecraft:villager", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(JValue::List(JList::Compound(gossips))) = data.get_mut("Gossips") {
            gossips.retain(|gossip| !matches!(gossip.get("Type"), Some(JValue::String(typ)) if typ == "golem"));
        }
    }));
}
