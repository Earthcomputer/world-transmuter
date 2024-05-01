use crate::types;
use crate::versions::v100;
use java_string::JavaString;
use world_transmuter_engine::DataWalkerMapTypePaths;

const VERSION: u32 = 1470;

pub(crate) fn register() {
    register_mob("minecraft:turtle");
    register_mob("minecraft:cod_mob");
    register_mob("minecraft:tropical_fish");
    register_mob("minecraft:salmon_mob");
    register_mob("minecraft:puffer_fish");
    register_mob("minecraft:phantom");
    register_mob("minecraft:dolphin");
    register_mob("minecraft:drowned");

    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:trident",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "inBlockState"),
    );
}

fn register_mob(id: impl Into<JavaString>) {
    v100::register_equipment(VERSION, id);
}
