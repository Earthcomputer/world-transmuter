use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::{map_data_converter_func, rename_key};

const VERSION: u32 = 3090;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_converter_for_id(
        "minecraft:painting",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            rename_key(data, "Motive", "variant");
            rename_key(data, "Facing", "facing");
        }),
    );
}
