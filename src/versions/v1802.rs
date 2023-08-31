use crate::helpers::rename::{rename_block, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1802;

pub(crate) fn register(types: MinecraftTypesMut) {
    rename_block(types, VERSION, |name| match name {
        "minecraft:stone_slab" => Some("minecraft:smooth_stone_slab".to_owned()),
        "minecraft:sign" => Some("minecraft:oak_sign".to_owned()),
        "minecraft:wall_sign" => Some("minecraft:oak_wall_sign".to_owned()),
        _ => None,
    });
    rename_item(types, VERSION, |name| match name {
        "minecraft:stone_slab" => Some("minecraft:smooth_stone_slab".to_owned()),
        "minecraft:sign" => Some("minecraft:oak_sign".to_owned()),
        _ => None,
    });
}
