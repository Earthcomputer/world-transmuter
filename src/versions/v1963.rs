use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{List, Value};

const VERSION: u32 = 1963;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:villager", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(Value::List(List::Compound(gossips))) = data.get_mut("Gossips") {
            gossips.retain(|gossip| !matches!(gossip.get("Type"), Some(Value::String(typ)) if typ == "golem"));
        }
    }));
}
