use crate::helpers::walkers::GameEventListenerWalker;
use crate::types;

const VERSION: u32 = 2684;

pub(crate) fn register() {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:sculk_sensor",
        GameEventListenerWalker::new(types::game_event_name_ref()),
    );
}
