use crate::helpers::rename::rename_block_and_fix_jigsaw;
use java_string::JavaString;

const VERSION: u32 = 2700;

pub(crate) fn register() {
    rename_block_and_fix_jigsaw(VERSION, |name| match name.as_bytes() {
        b"minecraft:cave_vines_head" => Some(JavaString::from("minecraft:cave_vines")),
        b"minecraft:cave_vines_body" => Some(JavaString::from("minecraft:cave_vines_plant")),
        _ => None,
    });
}
