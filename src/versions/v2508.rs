use crate::helpers::rename::{rename_block, rename_item};

const VERSION: u32 = 2508;

fn remap(name: &str) -> Option<String> {
    match name {
        "minecraft:warped_fungi" => Some("minecraft:warped_fungus".to_owned()),
        "minecraft:crimson_fungi" => Some("minecraft:crimson_fungus".to_owned()),
        _ => None,
    }
}

pub(crate) fn register() {
    rename_block(VERSION, remap);
    rename_item(VERSION, remap);
}
