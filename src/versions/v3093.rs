use crate::types::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 3093;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.entity.borrow_mut().add_converter_for_id(
        "minecraft:goat",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.insert("HasLeftHorn", true);
            data.insert("HasRightHorn", true);
        }),
    );
}
