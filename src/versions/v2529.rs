use rust_dataconverter_engine::map_data_converter_func;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2529;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:strider", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if data.get("NoGravity").and_then(|o| o.as_bool()) == Some(true) {
            data.insert("NoGravity", false);
        }
    }));
}
