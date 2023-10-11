use crate::types;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 1955;

const LEVEL_XP_THRESHOLDS: [i32; 5] = [0, 10, 50, 100, 150];

fn get_min_xp_per_level(level: i32) -> i32 {
    LEVEL_XP_THRESHOLDS[(level - 1).clamp(0, LEVEL_XP_THRESHOLDS.len() as i32 - 1) as usize]
}

fn add_level(data: &mut JCompound, level: i32) {
    if let Some(JValue::Compound(villager_data)) = data.get_mut("VillagerData") {
        villager_data.insert("level", level);
    } else {
        let villager_data = jcompound! {
            "level" => level,
        };
        data.insert("VillagerData", villager_data);
    }
}

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:villager",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let mut level = match data.get("VillagerData") {
                Some(JValue::Compound(villager_data)) => villager_data
                    .get("level")
                    .and_then(|v| v.as_i32())
                    .unwrap_or(0),
                _ => 0,
            };
            if level == 0 || level == 1 {
                // count recipes
                let recipes_count = match data.get("Offers") {
                    Some(JValue::Compound(offers)) => match offers.get("Recipes") {
                        Some(JValue::List(recipes)) => recipes.len(),
                        _ => 0,
                    },
                    _ => 0,
                };
                level = (recipes_count / 2).clamp(1, 5) as i32;
                if level > 1 {
                    add_level(data, level);
                }
            }

            if data.get("Xp").map(|v| v.is_number()) != Some(true) {
                data.insert("Xp", get_min_xp_per_level(level));
            }
        }),
    );

    types::entity_mut().add_converter_for_id(
        "minecraft:zombie_villager",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.get("Xp").map(|v| v.is_number()) != Some(true) {
                let level = match data.get("VillagerData") {
                    Some(JValue::Compound(villager_data)) => villager_data
                        .get("level")
                        .and_then(|v| v.as_i32())
                        .unwrap_or(1),
                    _ => 1,
                };
                data.insert("Xp", get_min_xp_per_level(level));
            }
        }),
    );
}
