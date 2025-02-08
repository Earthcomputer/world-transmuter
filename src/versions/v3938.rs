use crate::types;
use world_transmuter_engine::DataWalkerMapTypePaths;

const VERSION: u32 = 3938;

pub(crate) fn register() {
    register_arrow("minecraft:spectral_arrow");
    register_arrow("minecraft:arrow");
}

fn register_arrow(id: &str) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapTypePaths::new(types::block_state_ref(), "inBlockState"),
    );
    // new: weapon
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapTypePaths::new_multi(
            types::item_stack_ref(),
            vec!["item".to_owned(), "weapon".to_owned()],
        ),
    );
}
