use crate::types;
use world_transmuter_engine::{DataWalkerMapListPaths, DataWalkerMapTypePaths};

const VERSION: u32 = 2688;

pub(crate) fn register() {
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:glow_squid",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:glow_item_frame",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Item"),
    );
}
