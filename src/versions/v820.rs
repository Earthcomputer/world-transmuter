use rust_dataconverter_engine::Types;
use crate::helpers::rename::rename_item;
use crate::MinecraftTypesMut;

const VERSION: u32 = 820;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item::<T>(types, VERSION, |name| if name == "minecraft:totem" { Some("minecraft:totem_of_undying".to_owned()) } else { None });
}
