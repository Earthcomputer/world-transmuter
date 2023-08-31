use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 147;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.entity().borrow_mut().add_converter_for_id(
        "ArmorStand",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.get("Silent").and_then(|v| v.as_bool()) == Some(true)
                && data.get("Marker").and_then(|v| v.as_bool()) != Some(true)
            {
                data.remove("Silent");
            }
        }),
    );
}
