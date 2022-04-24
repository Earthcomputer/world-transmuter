use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 502;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.item_name.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Object, _>(|data, _from_version, _to_version| {
        if data.as_string() == Some("minecraft:cooked_fished") {
            *data = T::Object::create_string("minecraft:cooked_fish".to_owned());
        }
    }));

    types.entity.borrow_mut().add_converter_for_id("Zombie", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.remove("IsVillager").and_then(|o| o.as_bool()) != Some(true) {
            return;
        }

        if data.has_key("ZombieType") {
            return;
        }

        let mut zombie_type = data.get_i64("VillagerProfession").unwrap_or(0) as i32;
        if zombie_type < 0 || zombie_type >= 6 {
            zombie_type = 0;
        }
        data.set("ZombieType", T::Object::create_int(zombie_type));
    }));
}