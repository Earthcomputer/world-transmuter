use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3082;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:chest_boat",
        DataWalkerMapListPaths::new(types.item_stack, "Items"),
    );
}
