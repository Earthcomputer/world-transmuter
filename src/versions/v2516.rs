use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;
use crate::versions::v2514;

const VERSION: u32 = 2516;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    for id in ["minecraft:villager", "minecraft:zombie_villager"] {
        types.entity.borrow_mut().add_converter_for_id(id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            if let Some(gossips) = data.get_list_mut("Gossips") {
                for gossip in gossips.iter_mut() {
                    if let Some(gossip) = gossip.as_map_mut() {
                        v2514::replace_uuid_from_longs::<T>(gossip, "TargetLeast", "TargetMost", "Target");
                    }
                }
            }
        }));
    }
}
