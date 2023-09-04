use crate::MinecraftTypesMut;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1500;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.tile_entity().borrow_mut().add_converter_for_id(
        "DUMMY",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.insert("keepPacked", true);
        }),
    );
}
