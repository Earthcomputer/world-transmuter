use rust_dataconverter_engine::{DataWalkerMapListPaths, Types};
use crate::helpers::rename::{rename_entity, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2509;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item(types, VERSION, |name| {
        if name == "minecraft:zombie_pigman_spawn_egg" {
            Some("minecraft:zombified_piglin_spawn_egg".to_owned())
        } else {
            None
        }
    });
    rename_entity(types, VERSION, |name| {
        if name == "minecraft:zombie_pigman" {
            Some("minecraft:zombified_piglin".to_owned())
        } else {
            None
        }
    });
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:zombified_piglin", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
