use crate::types::MinecraftTypesMut;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3082;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:chest_boat",
        DataWalkerMapListPaths::new(types.item_stack(), "Items"),
    );
}
