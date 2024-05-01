use crate::types;
use crate::versions::v100;
use world_transmuter_engine::{
    map_data_converter_func, DataWalkerMapListPaths, DataWalkerMapTypePaths,
};

const VERSION: u32 = 703;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "EntityHorse",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            match data.remove("Type").and_then(|o| o.as_i64()) {
                Some(1) => data.insert("id", "Donkey"),
                Some(2) => data.insert("id", "Mule"),
                Some(3) => data.insert("id", "ZombieHorse"),
                Some(4) => data.insert("id", "SkeletonHorse"),
                _ => data.insert("id", "Horse"),
            };
        }),
    );

    types::entity_mut().add_walker_for_id(
        VERSION,
        "Horse",
        DataWalkerMapTypePaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()],
        ),
    );
    v100::register_equipment(VERSION, "Horse");

    types::entity_mut().add_walker_for_id(
        VERSION,
        "Donkey",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "Donkey",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    v100::register_equipment(VERSION, "Donkey");

    types::entity_mut().add_walker_for_id(
        VERSION,
        "Mule",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "Mule",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    v100::register_equipment(VERSION, "Mule");

    types::entity_mut().add_walker_for_id(
        VERSION,
        "ZombieHorse",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    v100::register_equipment(VERSION, "ZombieHorse");

    types::entity_mut().add_walker_for_id(
        VERSION,
        "SkeletonHorse",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "SaddleItem"),
    );
    v100::register_equipment(VERSION, "SkeletonHorse");
}
