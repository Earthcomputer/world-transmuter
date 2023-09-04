use crate::helpers::rename::{rename_block, rename_item, simple_rename};

const VERSION: u32 = 2717;

pub(crate) fn register() {
    rename_item(
        VERSION,
        simple_rename(
            "minecraft:azalea_leaves_flowers",
            "minecraft:flowering_azalea_leaves",
        ),
    );
    rename_block(
        VERSION,
        simple_rename(
            "minecraft:azalea_leaves_flowers",
            "minecraft:flowering_azalea_leaves",
        ),
    );
}
