use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;

const VERSION: u32 = 2702;

pub(crate) fn register(types: &MinecraftTypesMut) {
    for id in [
        "minecraft:arrow",
        "minecraft:spectral_arrow",
        "minecraft:trident",
    ] {
        types.entity.borrow_mut().add_converter_for_id(
            id,
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                if data.contains_key("pickup") {
                    return;
                }
                let player = data
                    .remove("player")
                    .and_then(|o| o.as_bool())
                    .unwrap_or(true);
                data.insert("pickup", player);
            }),
        );
    }
}
