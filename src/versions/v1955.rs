use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1955;

const LEVEL_XP_THRESHOLDS: [i32; 5] = [0, 10, 50, 100, 150];

fn get_min_xp_per_level(level: i32) -> i32 {
    LEVEL_XP_THRESHOLDS[(level - 1).clamp(0, LEVEL_XP_THRESHOLDS.len() as i32 - 1) as usize]
}

fn add_level<T: Types + ?Sized>(data: &mut T::Map, level: i32) {
    if let Some(villager_data) = data.get_map_mut("VillagerData") {
        villager_data.set("level", T::Object::create_int(level));
    } else {
        let mut villager_data = T::Map::create_empty();
        villager_data.set("level", T::Object::create_int(level));
        data.set("VillagerData", T::Object::create_map(villager_data));
    }
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:villager", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let mut level = data.get_map("VillagerData").and_then(|villager_data| villager_data.get_i64("level")).unwrap_or(0) as i32;
        if level == 0 || level == 1 {
            // count recipes
            let recipes_count = data.get_map("Offers")
                .and_then(|offers| offers.get_list("Recipes"))
                .map(|recipes| recipes.size())
                .unwrap_or(0);
            level = (recipes_count / 2).clamp(1, 5) as i32;
            if level > 1 {
                add_level::<T>(data, level);
            }
        }

        if data.get_i64("Xp").is_none() {
            data.set("Xp", T::Object::create_int(get_min_xp_per_level(level)));
        }
    }));

    types.entity.borrow_mut().add_converter_for_id("minecraft:zombie_villager", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_i64("Xp").is_none() {
            let level = data.get_map("VillagerData").and_then(|villager_data| villager_data.get_i64("level")).unwrap_or(1) as i32;
            data.set("Xp", T::Object::create_int(get_min_xp_per_level(level)));
        }
    }));
}
