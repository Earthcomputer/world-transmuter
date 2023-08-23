use crate::helpers::rename::rename_block;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1475;

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename_block(types, VERSION, |old_name| {
        match old_name {
            "minecraft:flowing_water" => Some("minecraft:water".to_owned()),
            "minecraft:flowing_lava" => Some("minecraft:lava".to_owned()),
            _ => None
        }
    });
}
