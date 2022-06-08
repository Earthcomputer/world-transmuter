use rust_dataconverter_engine::{data_converter_func, DataVersion, DataWalkerMapTypePaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2511;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:egg", VERSION, data_converter_func::<T::Map, _>(throwable_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:ender_pearl", VERSION, data_converter_func::<T::Map, _>(throwable_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:experience_bottle", VERSION, data_converter_func::<T::Map, _>(throwable_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:snowball", VERSION, data_converter_func::<T::Map, _>(throwable_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:potion", VERSION, data_converter_func::<T::Map, _>(throwable_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:potion", VERSION, data_converter_func::<T::Map, _>(potion_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:llama_spit", VERSION, data_converter_func::<T::Map, _>(llama_spit_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:arrow", VERSION, data_converter_func::<T::Map, _>(arrow_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:spectral_arrow", VERSION, data_converter_func::<T::Map, _>(arrow_converter::<T>));
    types.entity.borrow_mut().add_converter_for_id("minecraft:trident", VERSION, data_converter_func::<T::Map, _>(arrow_converter::<T>));

    // Vanilla migrates the potion item but does not change the schema.
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:potion", DataWalkerMapTypePaths::new(types.item_stack, "Item"));
}

fn set_uuid<T: Types + ?Sized>(data: &mut T::Map, most: i64, least: i64) {
    // if either most or least is 0, that's an invalid uuid so don't convert
    // this checks for cases where there wasn't a uuid to start with
    if most != 0 && least != 0 {
        data.set("OwnerUUID", T::Object::create_int_array(vec![
            (most as u64 >> 32) as i32,
            most as i32,
            (least as u64 >> 32) as i32,
            least as i32,
        ]));
    }
}

fn throwable_converter<T: Types + ?Sized>(data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
    if let Some(owner) = data.remove("owner").and_then(|o| o.into_map()) {
        set_uuid::<T>(data, owner.get_i64("M").unwrap_or(0), owner.get_i64("L").unwrap_or(0));
    }
}

fn potion_converter<T: Types + ?Sized>(data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
    if let Some(potion) = data.remove("Potion").and_then(|o| o.into_map()) {
        data.set("Item", T::Object::create_map(potion));
    } else {
        data.set("Item", T::Object::create_map(T::Map::create_empty()));
    }
}

fn llama_spit_converter<T: Types + ?Sized>(data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
    if let Some(owner) = data.remove("Owner").and_then(|o| o.into_map()) {
        set_uuid::<T>(data, owner.get_i64("OwnerUUIDMost").unwrap_or(0), owner.get_i64("OwnerUUIDLeast").unwrap_or(0));
    }
}

fn arrow_converter<T: Types + ?Sized>(data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
    let most = data.remove("OwnerUUIDMost").and_then(|o| o.as_i64()).unwrap_or(0);
    let least = data.remove("OwnerUUIDLeast").and_then(|o| o.as_i64()).unwrap_or(0);
    set_uuid::<T>(data, most, least);
}
