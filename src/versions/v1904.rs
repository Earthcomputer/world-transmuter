use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1904;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:ocelot", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let cat_type = data.get_i64("CatType").unwrap_or(0) as i32;
        if cat_type == 0 {
            if data.get_string("Owner").map(|str| str.is_empty()) == Some(false) || data.get_string("OwnerUUID").map(|str| str.is_empty()) == Some(false) {
                data.set("Trusting", T::Object::create_bool(true));
            }
        } else if cat_type > 0 && cat_type < 4 {
            data.set("id", T::Object::create_string("minecraft:cat".to_owned()));
            if data.get_string("OwnerUUID").is_none() {
                data.set("OwnerUUID", T::Object::create_string(String::new()));
            }
        }
    }));

    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:cat", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
