use log::warn;
use rust_dataconverter_engine::{data_converter_func, MapEntry, MapType, ObjectType, Types};
use crate::helpers::item_name_v102;
use crate::MinecraftTypesMut;

const VERSION: u32 = 102;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    // V102 schema only modifies ITEM_STACK to have only a string ID, but our ITEM_NAME is generic (int or String) so we don't
    // actually need to update the walker

    types.item_name.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Object, _>(|data, _from_version, _to_version| {
        if let Some(id) = data.as_i64() {
            let id = id as i32;
            let name = item_name_v102::get_name_from_id(id).unwrap_or_else(|| {
                warn!("Unknown legacy integer id (V102): {}", id);
                item_name_v102::get_name_from_id(0).unwrap()
            });
            *data = T::Object::create_string(name.to_owned());
        }
    }));

    types.item_stack.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(id) = data.get_i64("id") {
            let id = id as i32;
            let name = item_name_v102::get_name_from_id(id).unwrap_or_else(|| {
                warn!("Unknown legacy integer id (V102): {}", id);
                item_name_v102::get_name_from_id(0).unwrap()
            });
            data.set("id", T::Object::create_string(name.to_owned()));
        }
    }));

    types.item_stack.borrow_mut().add_converter_for_id("minecraft:potion", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(damage) = data.get_i64("Damage") {
            let damage = damage as i16;
            if damage != 0 {
                data.set("Damage", T::Object::create_short(0));
            }
            let tag = data.entry("tag").or_insert_with(|| T::Object::create_map(T::Map::create_empty()));
            if let Some(tag) = tag.as_map_mut() {
                if tag.get_string("Potion").is_none() {
                    let converted = item_name_v102::get_potion_name_from_id(damage as i32).unwrap_or("minecraft:water");
                    tag.set("Potion", T::Object::create_string(converted.to_owned()));
                    if (damage & 16384) == 16384 {
                        data.set("id", T::Object::create_string("minecraft:splash_potion".to_owned()));
                    }
                }
            }
        }
    }));
}
