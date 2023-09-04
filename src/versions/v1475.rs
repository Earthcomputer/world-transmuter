use crate::helpers::rename::rename_block;

const VERSION: u32 = 1475;

pub(crate) fn register() {
    rename_block(VERSION, |old_name| match old_name {
        "minecraft:flowing_water" => Some("minecraft:water".to_owned()),
        "minecraft:flowing_lava" => Some("minecraft:lava".to_owned()),
        _ => None,
    });
}
