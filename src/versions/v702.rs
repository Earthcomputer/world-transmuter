use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{map_data_converter_func, DataWalkerMapListPaths};

const VERSION: u32 = 702;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_converter_for_id(
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

    register_mob(types, "ZombieVillager");
    register_mob(types, "Husk");
}

fn register_mob(types: MinecraftTypesMut, id: impl Into<String>) {
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
