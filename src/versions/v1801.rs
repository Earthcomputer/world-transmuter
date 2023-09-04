use crate::types;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 1801;

pub(crate) fn register() {
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:illager_beast",
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
