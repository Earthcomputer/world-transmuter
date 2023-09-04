use crate::MinecraftTypesMut;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 2825;

pub(crate) fn register(types: MinecraftTypesMut) {
    types
        .world_gen_settings()
        .borrow_mut()
        .add_structure_converter(
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                if !data.contains_key("has_increased_height_already") {
                    data.insert("has_increased_height_already", false);
                }
            }),
        );
}
