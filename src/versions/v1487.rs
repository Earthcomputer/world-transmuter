use crate::helpers::rename::{rename_block, rename_item};

const VERSION: u32 = 1487;

pub(crate) fn register() {
    let renamer = |name: &str| match name {
        "minecraft:prismarine_bricks_slab" => Some("minecraft:prismarine_brick_slab".to_owned()),
        "minecraft:prismarine_bricks_stairs" => {
            Some("minecraft:prismarine_brick_stairs".to_owned())
        }
        _ => None,
    };

    rename_item(VERSION, renamer);
    rename_block(VERSION, renamer);
}
