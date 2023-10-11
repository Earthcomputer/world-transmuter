use crate::types;
use java_string::JavaString;
use world_transmuter_engine::DataWalkerMapListPaths;

const VERSION: u32 = 501;

pub(crate) fn register() {
    register_mob("PolarBear");
}

fn register_mob(id: impl Into<JavaString>) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
