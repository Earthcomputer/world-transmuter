use rust_dataconverter_engine::{data_converter_func, MapEntry, MapType, ObjectType, Types};
use crate::helpers::spawn_egg_name_v105;
use crate::MinecraftTypesMut;

const VERSION: u32 = 105;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:spawn_egg", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let damage = data.get_i64("Damage").unwrap_or(0) as i16;
        if damage != 0 {
            data.set("Damage", T::Object::create_short(0));
        }
        let tag = match data.entry("tag").or_insert_with(|| T::Object::create_map(T::Map::create_empty())).as_map_mut() {
            Some(map) => map,
            None => return
        };
        let entity_tag = match tag.entry("EntityTag").or_insert_with(|| T::Object::create_map(T::Map::create_empty())).as_map_mut() {
            Some(map) => map,
            None => return
        };
        if entity_tag.get_string("id").is_none() {
            if let Some(converted) = spawn_egg_name_v105::get_spawn_name_from_id((damage & 255) as u8) {
                entity_tag.set("id", T::Object::create_string(converted.clone()));
            }
        }
    }));
}
