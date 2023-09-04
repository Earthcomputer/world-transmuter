use crate::types;
use world_transmuter_engine::{map_data_converter_func, DataWalkerMapListPaths};

const VERSION: u32 = 808;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:shulker",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.get("Color").map(|v| v.is_number()) != Some(true) {
                data.insert("Color", 10i8);
            }
        }),
    );

    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:shulker_box",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
}
