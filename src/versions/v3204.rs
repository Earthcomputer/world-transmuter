use crate::types;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3204;

pub(crate) fn register() {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:chiseled_bookshelf",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
}
