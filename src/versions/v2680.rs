use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2680;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item(types, VERSION, simple_rename("minecraft:grass_path", "minecraft:dirt_path"));
    rename_block(types, VERSION, simple_rename("minecraft:grass_path", "minecraft:dirt_path"));
}
