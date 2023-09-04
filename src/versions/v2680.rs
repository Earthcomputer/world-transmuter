use crate::helpers::rename::{rename_block, rename_item, simple_rename};

const VERSION: u32 = 2680;

pub(crate) fn register() {
    rename_item(
        VERSION,
        simple_rename("minecraft:grass_path", "minecraft:dirt_path"),
    );
    rename_block(
        VERSION,
        simple_rename("minecraft:grass_path", "minecraft:dirt_path"),
    );
}
