use crate::types;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1917;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:cat",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.get("CatType").and_then(|v| v.as_i32()) == Some(9) {
                data.insert("CatType", 10);
            }
        }),
    );
}
