use crate::MinecraftTypesMut;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 505;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.options().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.insert("useVbo", "true");
        }),
    );
}
