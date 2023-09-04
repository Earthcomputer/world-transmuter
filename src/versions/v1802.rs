use crate::helpers::rename::{rename_block, rename_item};

const VERSION: u32 = 1802;

pub(crate) fn register() {
    rename_block(VERSION, |name| match name {
        "minecraft:stone_slab" => Some("minecraft:smooth_stone_slab".to_owned()),
        "minecraft:sign" => Some("minecraft:oak_sign".to_owned()),
        "minecraft:wall_sign" => Some("minecraft:oak_wall_sign".to_owned()),
        _ => None,
    });
    rename_item(VERSION, |name| match name {
        "minecraft:stone_slab" => Some("minecraft:smooth_stone_slab".to_owned()),
        "minecraft:sign" => Some("minecraft:oak_sign".to_owned()),
        _ => None,
    });
}
