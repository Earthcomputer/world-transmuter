use crate::types::MinecraftTypesMut;
use world_transmuter_engine::{DataWalkerMapTypePaths, DataWalkerObjectListPaths};

const VERSION: u32 = 3327;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:decorated_pot",
        DataWalkerObjectListPaths::new(types.item_name(), "shards"),
    );
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:suspicious_sand",
        DataWalkerMapTypePaths::new(types.item_stack(), "item"),
    );
}
