use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use java_string::JavaString;

const VERSION: u32 = 1490;

pub(crate) fn register() {
    rename_block(
        VERSION,
        simple_rename("minecraft:melon_block", "minecraft:melon"),
    );
    rename_item(VERSION, |name| match name.as_bytes() {
        b"minecraft:melon_block" => Some(JavaString::from("minecraft:melon")),
        b"minecraft:melon" => Some(JavaString::from("minecraft:melon_slice")),
        b"minecraft:speckled_melon" => Some(JavaString::from("minecraft:glistering_melon_slice")),
        _ => None,
    });
}
