use crate::helpers::rename::{rename_item, simple_rename};
use crate::types;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 502;

pub(crate) fn register() {
    rename_item(
        VERSION,
        simple_rename("minecraft:cooked_fished", "minecraft:cooked_fish"),
    );

    types::entity_mut().add_converter_for_id(
        "Zombie",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.remove("IsVillager").and_then(|o| o.as_bool()) != Some(true) {
                return;
            }

            if data.contains_key("ZombieType") {
                return;
            }

            let mut zombie_type = data
                .get("VillagerProfession")
                .and_then(|v| v.as_i32())
                .unwrap_or(0);
            if !(0..6).contains(&zombie_type) {
                zombie_type = 0;
            }
            data.insert("ZombieType", zombie_type);
        }),
    );
}
