use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 1456;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_converter_for_id(
        "minecraft:item_frame",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            // convert direction from 2d to 3d
            let new_dir = match data.get("Facing").and_then(|v| v.as_i8()).unwrap_or(0) {
                0 => 3,
                1 => 4,
                3 => 5,
                _ => 2,
            };
            data.insert("Facing", new_dir);
        }),
    );
}
