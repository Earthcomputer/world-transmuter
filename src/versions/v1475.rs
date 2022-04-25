use rust_dataconverter_engine::Types;
use crate::helpers::rename::rename_block;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1475;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_block::<T>(types, VERSION, |old_name| {
        match old_name {
            "minecraft:flowing_water" => Some("minecraft:water".to_owned()),
            "minecraft:flowing_lava" => Some("minecraft:lava".to_owned()),
            _ => None
        }
    });
}
