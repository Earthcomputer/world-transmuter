use crate::types;
use world_transmuter_engine::DataWalkerMapTypePaths;

const VERSION: u32 = 3325;

pub(crate) fn register() {
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:item_display",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "item"),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:block_display",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "block_state"),
    );
    // text_display is a simple entity
}
