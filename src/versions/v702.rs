use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 702;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("Zombie", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(zombie_type) = data.remove("ZombieType").and_then(|o| o.as_i64()) {
            match zombie_type {
                1..=5 => {
                    data.set("id", T::Object::create_string("ZombieVillager".to_owned()));
                    data.set("Profession", T::Object::create_int(zombie_type as i32 - 1));
                }
                6 => data.set("id", T::Object::create_string("Husk".to_owned())),
                _ => {}
            }
        }
    }));

    register_mob(types, "ZombieVillager");
    register_mob(types, "Husk");
}

fn register_mob<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: impl Into<String>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, id, DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
