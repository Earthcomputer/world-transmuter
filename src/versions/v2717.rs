use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use crate::MinecraftTypes;

const VERSION: u32 = 2717;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename_item(
        types,
        VERSION,
        simple_rename(
            "minecraft:azalea_leaves_flowers",
            "minecraft:flowering_azalea_leaves",
        ),
    );
    rename_block(
        types,
        VERSION,
        simple_rename(
            "minecraft:azalea_leaves_flowers",
            "minecraft:flowering_azalea_leaves",
        ),
    );
}
