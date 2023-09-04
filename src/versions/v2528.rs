use crate::helpers::rename::{rename_block, rename_item};

const VERSION: u32 = 2528;

pub(crate) fn register() {
    rename_item(VERSION, |name| match name {
        "minecraft:soul_fire_torch" => Some("minecraft:soul_torch".to_owned()),
        "minecraft:soul_fire_lantern" => Some("minecraft:soul_lantern".to_owned()),
        _ => None,
    });
    rename_block(VERSION, |name| match name {
        "minecraft:soul_fire_torch" => Some("minecraft:soul_torch".to_owned()),
        "minecraft:soul_fire_wall_torch" => Some("minecraft:soul_wall_torch".to_owned()),
        "minecraft:soul_fire_lantern" => Some("minecraft:soul_lantern".to_owned()),
        _ => None,
    });
}
