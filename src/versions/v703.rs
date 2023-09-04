use crate::MinecraftTypesMut;
use world_transmuter_engine::{
    map_data_converter_func, DataWalkerMapListPaths, DataWalkerMapTypePaths,
};

const VERSION: u32 = 703;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_converter_for_id(
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

    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "Horse",
        DataWalkerMapTypePaths::new_multi(
            types.item_stack(),
            vec!["ArmorItem".to_owned(), "SaddleItem".to_owned()],
        ),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "Horse",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );

    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "Donkey",
        DataWalkerMapTypePaths::new(types.item_stack(), "SaddleItem"),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "Donkey",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec![
                "Items".to_owned(),
                "ArmorItems".to_owned(),
                "HandItems".to_owned(),
            ],
        ),
    );

    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "Mule",
        DataWalkerMapTypePaths::new(types.item_stack(), "SaddleItem"),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "Mule",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec![
                "Items".to_owned(),
                "ArmorItems".to_owned(),
                "HandItems".to_owned(),
            ],
        ),
    );

    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "ZombieHorse",
        DataWalkerMapTypePaths::new(types.item_stack(), "SaddleItem"),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "ZombieHorse",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );

    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "SkeletonHorse",
        DataWalkerMapTypePaths::new(types.item_stack(), "SaddleItem"),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "SkeletonHorse",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
