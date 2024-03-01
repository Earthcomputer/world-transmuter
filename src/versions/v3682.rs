use crate::types;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3682;

pub(crate) fn register() {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:crafter",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
}
