use crate::helpers::rename::rename_block;
use java_string::JavaString;

const VERSION: u32 = 2700;

pub(crate) fn register() {
    rename_block(VERSION, |name| match name.as_bytes() {
        b"minecraft:cave_vines_head" => Some(JavaString::from("minecraft:cave_vines")),
        b"minecraft:cave_vines_body" => Some(JavaString::from("minecraft:cave_vines_plant")),
        _ => None,
    });
}
