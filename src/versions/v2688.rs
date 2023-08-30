use crate::MinecraftTypes;
use rust_dataconverter_engine::{DataWalkerMapListPaths, DataWalkerMapTypePaths};

const VERSION: u32 = 2688;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:glow_squid",
        DataWalkerMapListPaths::new_multi(
            &types.item_stack,
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:glow_item_frame",
        DataWalkerMapTypePaths::new(&types.item_stack, "Item"),
    );
}
