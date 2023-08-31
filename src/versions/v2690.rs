use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::{rename_block_and_fix_jigsaw, rename_item};
use crate::MinecraftTypesMut;
use std::sync::OnceLock;

const VERSION: u32 = 2690;

static RENAMES: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renames() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMES.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("weathered_copper_block", "minecraft:oxidized_copper_block");
        map.insert_mc(
            "semi_weathered_copper_block",
            "minecraft:weathered_copper_block",
        );
        map.insert_mc(
            "lightly_weathered_copper_block",
            "minecraft:exposed_copper_block",
        );
        map.insert_mc("weathered_cut_copper", "minecraft:oxidized_cut_copper");
        map.insert_mc(
            "semi_weathered_cut_copper",
            "minecraft:weathered_cut_copper",
        );
        map.insert_mc(
            "lightly_weathered_cut_copper",
            "minecraft:exposed_cut_copper",
        );
        map.insert_mc(
            "weathered_cut_copper_stairs",
            "minecraft:oxidized_cut_copper_stairs",
        );
        map.insert_mc(
            "semi_weathered_cut_copper_stairs",
            "minecraft:weathered_cut_copper_stairs",
        );
        map.insert_mc(
            "lightly_weathered_cut_copper_stairs",
            "minecraft:exposed_cut_copper_stairs",
        );
        map.insert_mc(
            "weathered_cut_copper_slab",
            "minecraft:oxidized_cut_copper_slab",
        );
        map.insert_mc(
            "semi_weathered_cut_copper_slab",
            "minecraft:weathered_cut_copper_slab",
        );
        map.insert_mc(
            "lightly_weathered_cut_copper_slab",
            "minecraft:exposed_cut_copper_slab",
        );
        map.insert_mc(
            "waxed_semi_weathered_copper",
            "minecraft:waxed_weathered_copper",
        );
        map.insert_mc(
            "waxed_lightly_weathered_copper",
            "minecraft:waxed_exposed_copper",
        );
        map.insert_mc(
            "waxed_semi_weathered_cut_copper",
            "minecraft:waxed_weathered_cut_copper",
        );
        map.insert_mc(
            "waxed_lightly_weathered_cut_copper",
            "minecraft:waxed_exposed_cut_copper",
        );
        map.insert_mc(
            "waxed_semi_weathered_cut_copper_stairs",
            "minecraft:waxed_weathered_cut_copper_stairs",
        );
        map.insert_mc(
            "waxed_lightly_weathered_cut_copper_stairs",
            "minecraft:waxed_exposed_cut_copper_stairs",
        );
        map.insert_mc(
            "waxed_semi_weathered_cut_copper_slab",
            "minecraft:waxed_weathered_cut_copper_slab",
        );
        map.insert_mc(
            "waxed_lightly_weathered_cut_copper_slab",
            "minecraft:waxed_exposed_cut_copper_slab",
        );
        map
    })
}

pub(crate) fn register(types: MinecraftTypesMut) {
    rename_item(types, VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
    rename_block_and_fix_jigsaw(types, VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
}
