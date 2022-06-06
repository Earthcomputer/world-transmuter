use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1918;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    for entity_id in ["minecraft:villager", "minecraft:zombie_villager"] {
        types.entity.borrow_mut().add_converter_for_id(entity_id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            let profession = data.remove("Profession").and_then(|obj| obj.as_i64()).unwrap_or(0) as i32;
            let career = data.remove("Career").and_then(|obj| obj.as_i64()).unwrap_or(0) as i32;
            let career_level = data.remove("CareerLevel").and_then(|obj| obj.as_i64()).unwrap_or(1) as i32;

            let mut villager_data = T::Map::create_empty();
            villager_data.set("type", T::Object::create_string("minecraft:plains".to_owned()));
            villager_data.set("profession", T::Object::create_string(get_profession_string(profession, career).to_owned()));
            villager_data.set("level", T::Object::create_int(career_level));
            data.set("VillagerData", T::Object::create_map(villager_data));
        }));
    }
}

fn get_profession_string(profession_id: i32, career_id: i32) -> &'static str {
    match (profession_id, career_id) {
        (0, 2) => "minecraft:fisherman",
        (0, 3) => "minecraft:shepherd",
        (0, 4) => "minecraft:fletcher",
        (0, _) => "minecraft:farmer",
        (1, 2) => "minecraft:cartographer",
        (1, _) => "minecraft:librarian",
        (2, _) => "minecraft:cleric",
        (3, 2) => "minecraft:weaponsmith",
        (3, 3) => "minecraft:toolsmith",
        (3, _) => "minecraft:armorer",
        (4, 2) => "minecraft:leatherworker",
        (4, _) => "minecraft:butcher",
        (5, _) => "minecraft:nitwit",
        _ => "minecraft:none",
    }
}
