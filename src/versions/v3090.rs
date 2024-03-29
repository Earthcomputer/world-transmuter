use crate::types;
use world_transmuter_engine::{map_data_converter_func, rename_key};

const VERSION: u32 = 3090;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:painting",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            rename_key(data, "Motive", "variant");
            rename_key(data, "Facing", "facing");
        }),
    );
}
