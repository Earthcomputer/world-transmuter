use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1963;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:villager", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(gossips) = data.get_list_mut("Gossips") {
            for i in (0..gossips.size()).rev() {
                if let Some(gossip) = gossips.get(i).as_map() {
                    if gossip.get_string("Type") == Some("golem") {
                        gossips.remove(i);
                    }
                }
            }
        }
    }));
}
