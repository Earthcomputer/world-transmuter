use crate::helpers::walkers::GameEventListenerWalker;
use crate::types;
use crate::versions::v100;

const VERSION: u32 = 3081;

fn register_mob(id: &str) {
    v100::register_equipment(VERSION, id);
}

pub(crate) fn register() {
    register_mob("minecraft:warden");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:warden",
        GameEventListenerWalker::new(types::game_event_name_ref()),
    );
}
