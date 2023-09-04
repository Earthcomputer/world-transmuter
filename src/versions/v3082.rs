use crate::types;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3082;

pub(crate) fn register() {
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:chest_boat",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
}
