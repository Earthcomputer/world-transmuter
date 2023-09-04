use crate::MinecraftTypesMut;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 816;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.options().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(lang)) = data.get_mut("lang") {
                lang.make_ascii_lowercase();
            }
        }),
    );
}
