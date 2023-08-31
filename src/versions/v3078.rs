use crate::helpers::walkers::GameEventListenerWalker;
use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3078;

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
    register_mob(types, "minecraft:frog");
    register_mob(types, "minecraft:tadpole");
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:sculk_shrieker",
        GameEventListenerWalker::new(types.game_event_name()),
    );
}
