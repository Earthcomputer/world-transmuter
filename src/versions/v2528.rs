use crate::helpers::rename::{rename_block, rename_item};
use java_string::JavaString;

const VERSION: u32 = 2528;

pub(crate) fn register() {
    rename_item(VERSION, |name| match name.as_bytes() {
        b"minecraft:soul_fire_torch" => Some(JavaString::from("minecraft:soul_torch")),
        b"minecraft:soul_fire_lantern" => Some(JavaString::from("minecraft:soul_lantern")),
        _ => None,
    });
    rename_block(VERSION, |name| match name.as_bytes() {
        b"minecraft:soul_fire_torch" => Some(JavaString::from("minecraft:soul_torch")),
        b"minecraft:soul_fire_wall_torch" => Some(JavaString::from("minecraft:soul_wall_torch")),
        b"minecraft:soul_fire_lantern" => Some(JavaString::from("minecraft:soul_lantern")),
        _ => None,
    });
}
