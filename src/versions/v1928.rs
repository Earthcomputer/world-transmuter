use rust_dataconverter_engine::{DataWalkerMapListPaths, Types};
use crate::helpers::rename::{rename_entity, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1928;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_entity(types, VERSION, |name| {
        if name == "minecraft:illager_beast" {
            Some("minecraft:ravager".to_owned())
        } else {
            None
        }
    });
    rename_item(types, VERSION, |name| {
        if name == "minecraft:illager_beast_spawn_egg" {
            Some("minecraft:ravager_spawn_egg".to_owned())
        } else {
            None
        }
    });

    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:ravager", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
