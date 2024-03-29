use crate::types;
use world_transmuter_engine::{map_data_converter_func, rename_key, DataWalkerObjectListPaths};

const VERSION: u32 = 3448;

pub(crate) fn register() {
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:decorated_pot",
        DataWalkerObjectListPaths::new_multi(
            types::item_name_ref(),
            vec!["item".into(), "sherds".into()],
        ),
    );
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:decorated_pot",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            rename_key(data, "shards", "sherds");
        }),
    )
}
