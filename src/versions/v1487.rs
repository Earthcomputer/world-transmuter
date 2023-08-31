use crate::helpers::rename::{rename_block, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1487;

pub(crate) fn register(types: MinecraftTypesMut) {
    let renamer = |name: &str| match name {
        "minecraft:prismarine_bricks_slab" => Some("minecraft:prismarine_brick_slab".to_owned()),
        "minecraft:prismarine_bricks_stairs" => {
            Some("minecraft:prismarine_brick_stairs".to_owned())
        }
        _ => None,
    };

    rename_item(types, VERSION, renamer);
    rename_block(types, VERSION, renamer);
}
