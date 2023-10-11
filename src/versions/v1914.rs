use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1914;

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:chest",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(loot_table)) = data.get_mut("LootTable") {
                if loot_table == "minecraft:chests/village_blacksmith" {
                    *loot_table = JavaString::from("minecraft:chests/village/village_weaponsmith");
                }
            }
        }),
    );
}
