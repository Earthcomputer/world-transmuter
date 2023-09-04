use crate::helpers::rename::{rename_block, rename_item, rename_poi};

const VERSION: u32 = 2209;

fn renamer(name: &str) -> Option<String> {
    if name == "minecraft:bee_hive" {
        Some("minecraft:beehive".to_owned())
    } else {
        None
    }
}

pub(crate) fn register() {
    rename_block(VERSION, renamer);
    rename_item(VERSION, renamer);
    rename_poi(VERSION, renamer);
}
