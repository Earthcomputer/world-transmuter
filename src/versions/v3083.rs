use crate::helpers::walkers::GameEventListenerWalker;
use crate::types;
use crate::versions::v100;

const VERSION: u32 = 3083;

fn register_mob(id: &str) {
    v100::register_equipment(VERSION, id);
}

pub(crate) fn register() {
    register_mob("minecraft:allay");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:allay",
        GameEventListenerWalker::new(types::game_event_name_ref()),
    );
}
