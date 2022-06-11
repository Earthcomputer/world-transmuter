use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_stat, simple_rename};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2710;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_stat(types, VERSION, simple_rename("minecraft:play_one_minute", "minecraft:play_time"));
}
