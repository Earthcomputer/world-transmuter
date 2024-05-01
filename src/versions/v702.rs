use crate::types;
use crate::versions::v100;
use java_string::JavaString;
use world_transmuter_engine::{
    convert_map_list_in_map, map_data_converter_func, map_data_walker, JValue,
};

const VERSION: u32 = 702;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "Zombie",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(zombie_type) = data.remove("ZombieType").and_then(|o| o.as_i64()) {
                match zombie_type {
                    1..=5 => {
                        data.insert("id", "ZombieVillager");
                        data.insert("Profession", zombie_type as i32 - 1);
                    }
                    6 => {
                        data.insert("id", "Husk");
                    }
                    _ => {}
                }
            }
        }),
    );

    register_mob("ZombieVillager");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "ZombieVillager",
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::Compound(offers)) = data.get_mut("Offers") {
                convert_map_list_in_map(
                    types::villager_trade_ref(),
                    offers,
                    "Recipes",
                    from_version,
                    to_version,
                );
            }
        }),
    );
    register_mob("Husk");
}

fn register_mob(id: impl Into<JavaString>) {
    v100::register_equipment(VERSION, id);
}
