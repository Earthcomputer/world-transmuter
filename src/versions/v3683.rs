use crate::types;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{map_data_converter_func, rename_key, DataWalkerMapTypePaths};

const VERSION: u32 = 3683;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:tnt",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            rename_key(data, "Fuse", "fuse");
            data.insert(
                "block_state",
                jcompound! {
                    "Name" => "minecraft:tnt",
                },
            );
        }),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:tnt",
        DataWalkerMapTypePaths::new(types::block_state_ref(), "block_state"),
    );
}
