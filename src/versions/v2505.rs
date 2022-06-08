use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2505;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:villager", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(brain) = data.get_map_mut("Brain") {
            if let Some(memories) = brain.get_map_mut("memories") {
                for key in memories.keys().cloned().collect::<Vec<_>>() {
                    let memory = memories.remove(&key).unwrap();
                    let mut wrapped = T::Map::create_empty();
                    wrapped.set("value", memory);
                    memories.set(key, T::Object::create_map(wrapped));
                }
            }
        }
    }));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:piglin", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
