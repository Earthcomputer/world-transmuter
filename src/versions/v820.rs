use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_item, simple_rename};
use crate::MinecraftTypesMut;

const VERSION: u32 = 820;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item::<T>(types, VERSION, simple_rename("minecraft:totem", "minecraft:totem_of_undying"));
}
