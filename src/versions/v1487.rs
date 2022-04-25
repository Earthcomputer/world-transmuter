use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1487;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    let renamer = |name: &str| {
        match name {
            "minecraft:prismarine_bricks_slab" => Some("minecraft:prismarine_brick_slab".to_owned()),
            "minecraft:prismarine_bricks_stairs" => Some("minecraft:prismarine_brick_stairs".to_owned()),
            _ => None
        }
    };

    rename_item::<T>(types, VERSION, renamer);
    rename_block::<T>(types, VERSION, renamer);
}
