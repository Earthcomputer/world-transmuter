use crate::helpers::rename::{rename_block, rename_item};
use crate::types;
use java_string::{JavaStr, JavaString};
use world_transmuter_engine::{map_data_converter_func, rename_key, JValue};

const VERSION: u32 = 1484;

pub(crate) fn register() {
    let renamer = |name: &JavaStr| match name.as_bytes() {
        b"minecraft:sea_grass" => Some(JavaString::from("minecraft:seagrass")),
        b"minecraft:tall_sea_grass" => Some(JavaString::from("minecraft:tall_seagrass")),
        _ => None,
    };
    rename_item(VERSION, renamer);
    rename_block(VERSION, renamer);

    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            let Some(JValue::Compound(heightmaps)) = level.get_mut("Heightmaps") else {
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
