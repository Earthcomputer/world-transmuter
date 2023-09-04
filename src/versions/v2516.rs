use crate::versions::v2514;
use crate::MinecraftTypesMut;
use valence_nbt::{List, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 2516;

pub(crate) fn register(types: MinecraftTypesMut) {
    for id in ["minecraft:villager", "minecraft:zombie_villager"] {
        types.entity().borrow_mut().add_converter_for_id(
            id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                if let Some(Value::List(List::Compound(gossips))) = data.get_mut("Gossips") {
                    for gossip in gossips {
                        v2514::replace_uuid_from_longs(
                            gossip,
                            "TargetLeast",
                            "TargetMost",
                            "Target",
                        );
                    }
                }
            }),
        );
    }
}
