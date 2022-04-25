use rust_dataconverter_engine::{data_converter_func, MapType, Types};
use crate::helpers::rename::{rename_block, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1484;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    let renamer = |name: &str| {
        match name {
            "minecraft:sea_grass" => Some("minecraft:seagrass".to_owned()),
            "minecraft:tall_sea_grass" => Some("minecraft:tall_seagrass".to_owned()),
            _ => None
        }
    };
    rename_item::<T>(types, VERSION, renamer);
    rename_block::<T>(types, VERSION, renamer);

    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let level = match data.get_map_mut("Level") {
            Some(level) => level,
            None => return
        };

        let heightmaps = match level.get_map_mut("Heightmaps") {
            Some(heightmaps) => heightmaps,
            None => return
        };

        heightmaps.rename_key("LIQUID", "WORLD_SURFACE_WG");

        if let Some(solid) = heightmaps.remove("SOLID") {
            heightmaps.set("OCEAN_FLOOR_WG", solid.clone());
            heightmaps.set("OCEAN_FLOOR", solid);
        }

        heightmaps.rename_key("LIGHT", "LIGHT_BLOCKING");

        if let Some(rain) = heightmaps.remove("RAIN") {
            heightmaps.set("MOTION_BLOCKING", rain.clone());
            heightmaps.set("MOTION_BLOCKING_NO_LEAVES", rain);
        }
    }));
}
