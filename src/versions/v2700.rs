use crate::helpers::rename::rename_block_and_fix_jigsaw;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2700;

pub(crate) fn register(types: MinecraftTypesMut) {
    rename_block_and_fix_jigsaw(types, VERSION, |name| match name {
        "minecraft:cave_vines_head" => Some("minecraft:cave_vines".to_owned()),
        "minecraft:cave_vines_body" => Some("minecraft:cave_vines_plant".to_owned()),
        _ => None,
    });
}
