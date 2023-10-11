use crate::helpers::rename::{rename_block, rename_item, rename_poi};
use java_string::{JavaStr, JavaString};

const VERSION: u32 = 2209;

fn renamer(name: &JavaStr) -> Option<JavaString> {
    if name == "minecraft:bee_hive" {
        Some(JavaString::from("minecraft:beehive"))
    } else {
        None
    }
}

pub(crate) fn register() {
    rename_block(VERSION, renamer);
    rename_item(VERSION, renamer);
    rename_poi(VERSION, renamer);
}
