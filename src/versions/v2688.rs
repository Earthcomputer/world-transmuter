use crate::types;
use crate::versions::v100;
use world_transmuter_engine::DataWalkerMapTypePaths;

const VERSION: u32 = 2688;

pub(crate) fn register() {
    v100::register_equipment(VERSION, "minecraft:glow_squid");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:glow_item_frame",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Item"),
    );
}
