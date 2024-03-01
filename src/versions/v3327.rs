use crate::types;
use world_transmuter_engine::{DataWalkerMapTypePaths, DataWalkerObjectListPaths};

const VERSION: u32 = 3327;

pub(crate) fn register() {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:decorated_pot",
        DataWalkerObjectListPaths::new_multi(
            types::item_name_ref(),
            vec!["item".into(), "shards".into()],
        ),
    );
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:suspicious_sand",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "item"),
    );
}
