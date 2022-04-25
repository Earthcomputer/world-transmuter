use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1483;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_entity::<T>(types, VERSION, simple_rename("minecraft:puffer_fish", "minecraft:pufferfish"));
    rename_item::<T>(types, VERSION, simple_rename("minecraft:puffer_fish_spawn_egg", "minecraft:pufferfish_spawn_egg"));
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:puffer_fish", "minecraft:pufferfish");
}
