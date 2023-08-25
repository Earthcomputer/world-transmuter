use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 1928;

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename_entity(
        types,
        VERSION,
        simple_rename("minecraft:illager_beast", "minecraft:ravager"),
    );
    rename_item(
        types,
        VERSION,
        simple_rename(
            "minecraft:illager_beast_spawn_egg",
            "minecraft:ravager_spawn_egg",
        ),
    );

    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:ravager",
        DataWalkerMapListPaths::new_multi(
            types.item_stack,
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
