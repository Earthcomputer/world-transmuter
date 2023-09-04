use crate::types::MinecraftTypesMut;
use world_transmuter_engine::DataWalkerMapTypePaths;

const VERSION: u32 = 3325;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:item_display",
        DataWalkerMapTypePaths::new(types.item_stack(), "item"),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:block_display",
        DataWalkerMapTypePaths::new(types.block_state(), "block_state"),
    );
    // text_display is a simple entity
}
