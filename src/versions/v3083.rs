use crate::helpers::walkers::GameEventListenerWalker;
use crate::types;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3083;

fn register_mob(id: &str) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}

pub(crate) fn register() {
    register_mob("minecraft:allay");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:allay",
        GameEventListenerWalker::new(types::game_event_name_ref()),
    );
}
