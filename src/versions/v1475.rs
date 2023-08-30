use crate::helpers::rename::rename_block;
use crate::MinecraftTypes;

const VERSION: u32 = 1475;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename_block(types, VERSION, |old_name| match old_name {
        "minecraft:flowing_water" => Some("minecraft:water".to_owned()),
        "minecraft:flowing_lava" => Some("minecraft:lava".to_owned()),
        _ => None,
    });
}
