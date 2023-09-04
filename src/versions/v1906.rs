use crate::types;
use world_transmuter_engine::{DataWalkerMapListPaths, DataWalkerMapTypePaths};

const VERSION: u32 = 1906;

pub(crate) fn register() {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:barrel",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:smoker",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:blast_furnace",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:lectern",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Book"),
    );
}
