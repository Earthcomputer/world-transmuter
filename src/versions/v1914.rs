use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1914;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:chest", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("LootTable") == Some("minecraft:chests/village_blacksmith") {
            data.set("LootTable", T::Object::create_string("minecraft:chests/village/village_weaponsmith".to_owned()));
        }
    }));
}
