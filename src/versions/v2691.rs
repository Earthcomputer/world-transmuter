use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::{rename_block_and_fix_jigsaw, rename_item};
use crate::MinecraftTypes;
use std::sync::OnceLock;

const VERSION: u32 = 2691;

static RENAMES: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renames() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("waxed_copper", "minecraft:waxed_copper_block");
        map.insert_mc("oxidized_copper_block", "minecraft:oxidized_copper");
        map.insert_mc("weathered_copper_block", "minecraft:weathered_copper");
        map.insert_mc("exposed_copper_block", "minecraft:exposed_copper");
        map
    })
}

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename_item(types, VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
    rename_block_and_fix_jigsaw(types, VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
}
