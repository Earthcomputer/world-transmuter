use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::MinecraftTypesMut;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 2509;

pub(crate) fn register(types: MinecraftTypesMut) {
    rename_item(
        types,
        VERSION,
        simple_rename(
            "minecraft:zombie_pigman_spawn_egg",
            "minecraft:zombified_piglin_spawn_egg",
        ),
    );
    rename_entity(
        types,
        VERSION,
        simple_rename("minecraft:zombie_pigman", "minecraft:zombified_piglin"),
    );
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:zombified_piglin",
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
