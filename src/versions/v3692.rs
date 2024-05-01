use crate::helpers::rename::{rename_block, rename_item};
use java_string::{JavaStr, JavaString};

const VERSION: u32 = 3692;

fn grass_renamer(name: &JavaStr) -> Option<JavaString> {
    if name == "minecraft:grass" {
        Some("minecraft:short_grass".into())
    } else {
        None
    }
}

pub(crate) fn register() {
    rename_block(VERSION, grass_renamer);
    rename_item(VERSION, grass_renamer);
}
