use crate::helpers::rename::{rename_block, rename_item};
use java_string::JavaString;

const VERSION: u32 = 1802;

pub(crate) fn register() {
    rename_block(VERSION, |name| match name.as_bytes() {
        b"minecraft:stone_slab" => Some(JavaString::from("minecraft:smooth_stone_slab")),
        b"minecraft:sign" => Some(JavaString::from("minecraft:oak_sign")),
        b"minecraft:wall_sign" => Some(JavaString::from("minecraft:oak_wall_sign")),
        _ => None,
    });
    rename_item(VERSION, |name| match name.as_bytes() {
        b"minecraft:stone_slab" => Some(JavaString::from("minecraft:smooth_stone_slab")),
        b"minecraft:sign" => Some(JavaString::from("minecraft:oak_sign")),
        _ => None,
    });
}
