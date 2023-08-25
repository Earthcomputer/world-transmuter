use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 1474;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_converter_for_id(
        "minecraft:shulker",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.get("Color").and_then(|v| v.as_i32()) == Some(10) {
                data.insert("Color", 16i8);
            }
        }),
    );
    // data hooks ensure the inputs are namespaced
    rename_block(
        types,
        VERSION,
        simple_rename("minecraft:purple_shulker_box", "minecraft:shulker_box"),
    );
    rename_item(
        types,
        VERSION,
        simple_rename("minecraft:purple_shulker_box", "minecraft:shulker_box"),
    );
}
