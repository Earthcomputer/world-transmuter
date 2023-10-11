use crate::helpers::rename::{rename_block_and_fix_jigsaw, rename_item};
use crate::static_string_mc_map;

const VERSION: u32 = 2691;

static_string_mc_map! {
    RENAMES, renames, {
        "waxed_copper" => "minecraft:waxed_copper_block",
        "oxidized_copper_block" => "minecraft:oxidized_copper",
        "weathered_copper_block" => "minecraft:weathered_copper",
        "exposed_copper_block" => "minecraft:exposed_copper",
    }
}

pub(crate) fn register() {
    rename_item(VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
    rename_block_and_fix_jigsaw(VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
}
