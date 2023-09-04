use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::types;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 1928;

pub(crate) fn register() {
    rename_entity(
        VERSION,
        simple_rename("minecraft:illager_beast", "minecraft:ravager"),
    );
    rename_item(
        VERSION,
        simple_rename(
            "minecraft:illager_beast_spawn_egg",
            "minecraft:ravager_spawn_egg",
        ),
    );

    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:ravager",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
