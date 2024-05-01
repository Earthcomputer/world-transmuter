use crate::types;
use world_transmuter_engine::DataWalkerObjectTypePaths;

const VERSION: u32 = 1909;

pub(crate) fn register() {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:jigsaw",
        DataWalkerObjectTypePaths::new(types::flat_block_state_ref(), "final_state"),
    );
}
