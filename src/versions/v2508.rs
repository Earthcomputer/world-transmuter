use crate::helpers::rename::{rename_block, rename_item};
use java_string::{JavaStr, JavaString};

const VERSION: u32 = 2508;

fn remap(name: &JavaStr) -> Option<JavaString> {
    match name.as_bytes() {
        b"minecraft:warped_fungi" => Some(JavaString::from("minecraft:warped_fungus")),
        b"minecraft:crimson_fungi" => Some(JavaString::from("minecraft:crimson_fungus")),
        _ => None,
    }
}

pub(crate) fn register() {
    rename_block(VERSION, remap);
    rename_item(VERSION, remap);
}
