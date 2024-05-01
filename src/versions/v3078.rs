use crate::helpers::walkers::GameEventListenerWalker;
use crate::types;
use crate::versions::v100;

const VERSION: u32 = 3078;

fn register_mob(id: &str) {
    v100::register_equipment(VERSION, id);
}

pub(crate) fn register() {
    register_mob("minecraft:frog");
    register_mob("minecraft:tadpole");
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:sculk_shrieker",
        GameEventListenerWalker::new(types::game_event_name_ref()),
    );
}
