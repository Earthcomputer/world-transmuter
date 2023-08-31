use crate::helpers::rename::{rename_block, rename_item};
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{map_data_converter_func, rename_key};
use valence_nbt::Value;

const VERSION: u32 = 1484;

pub(crate) fn register(types: MinecraftTypesMut) {
    let renamer = |name: &str| match name {
        "minecraft:sea_grass" => Some("minecraft:seagrass".to_owned()),
        "minecraft:tall_sea_grass" => Some("minecraft:tall_seagrass".to_owned()),
        _ => None,
    };
    rename_item(types, VERSION, renamer);
    rename_block(types, VERSION, renamer);

    types.chunk().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            let Some(Value::Compound(heightmaps)) = level.get_mut("Heightmaps") else {
                return;
            };

            rename_key(heightmaps, "LIQUID", "WORLD_SURFACE_WG");

            if let Some(solid) = heightmaps.remove("SOLID") {
                heightmaps.insert("OCEAN_FLOOR_WG", solid.clone());
                heightmaps.insert("OCEAN_FLOOR", solid);
            }

            rename_key(heightmaps, "LIGHT", "LIGHT_BLOCKING");

            if let Some(rain) = heightmaps.remove("RAIN") {
                heightmaps.insert("MOTION_BLOCKING", rain.clone());
                heightmaps.insert("MOTION_BLOCKING_NO_LEAVES", rain);
            }
        }),
    );
}
