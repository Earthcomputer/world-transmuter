use crate::MinecraftTypes;
use rust_dataconverter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 2671;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:goat",
        DataWalkerMapListPaths::new_multi(
            &types.item_stack,
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
