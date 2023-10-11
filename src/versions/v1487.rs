use crate::helpers::rename::{rename_block, rename_item};
use java_string::{JavaStr, JavaString};

const VERSION: u32 = 1487;

pub(crate) fn register() {
    let renamer = |name: &JavaStr| match name.as_bytes() {
        b"minecraft:prismarine_bricks_slab" => {
            Some(JavaString::from("minecraft:prismarine_brick_slab"))
        }
        b"minecraft:prismarine_bricks_stairs" => {
            Some(JavaString::from("minecraft:prismarine_brick_stairs"))
        }
        _ => None,
    };

    rename_item(VERSION, renamer);
    rename_block(VERSION, renamer);
}
