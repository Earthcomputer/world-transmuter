use crate::helpers::walkers::GameEventListenerWalker;
use crate::types::MinecraftTypesMut;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3081;

fn register_mob(types: MinecraftTypesMut, id: &str) {
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}

pub(crate) fn register(types: MinecraftTypesMut) {
    register_mob(types, "minecraft:warden");
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:warden",
        GameEventListenerWalker::new(types.game_event_name()),
    );
}
