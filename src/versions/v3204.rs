use crate::types::MinecraftTypes;
use rust_dataconverter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 3204;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.tile_entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:chiseled_bookshelf",
        DataWalkerMapListPaths::new(&types.item_stack, "Items"),
    );
}
