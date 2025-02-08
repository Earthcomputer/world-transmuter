use crate::helpers::rename::{rename_block, rename_item};
use crate::static_string_mc_map;

const VERSION: u32 = 2690;

static_string_mc_map! {
    renames = {
        "weathered_copper_block" => "minecraft:oxidized_copper_block",
        "semi_weathered_copper_block" => "minecraft:weathered_copper_block",
        "lightly_weathered_copper_block" => "minecraft:exposed_copper_block",
        "weathered_cut_copper" => "minecraft:oxidized_cut_copper",
        "semi_weathered_cut_copper" => "minecraft:weathered_cut_copper",
        "lightly_weathered_cut_copper" => "minecraft:exposed_cut_copper",
        "weathered_cut_copper_stairs" => "minecraft:oxidized_cut_copper_stairs",
        "semi_weathered_cut_copper_stairs" => "minecraft:weathered_cut_copper_stairs",
        "lightly_weathered_cut_copper_stairs" => "minecraft:exposed_cut_copper_stairs",
        "weathered_cut_copper_slab" => "minecraft:oxidized_cut_copper_slab",
        "semi_weathered_cut_copper_slab" => "minecraft:weathered_cut_copper_slab",
        "lightly_weathered_cut_copper_slab" => "minecraft:exposed_cut_copper_slab",
        "waxed_semi_weathered_copper" => "minecraft:waxed_weathered_copper",
        "waxed_lightly_weathered_copper" => "minecraft:waxed_exposed_copper",
        "waxed_semi_weathered_cut_copper" => "minecraft:waxed_weathered_cut_copper",
        "waxed_lightly_weathered_cut_copper" => "minecraft:waxed_exposed_cut_copper",
        "waxed_semi_weathered_cut_copper_stairs" => "minecraft:waxed_weathered_cut_copper_stairs",
        "waxed_lightly_weathered_cut_copper_stairs" => "minecraft:waxed_exposed_cut_copper_stairs",
        "waxed_semi_weathered_cut_copper_slab" => "minecraft:waxed_weathered_cut_copper_slab",
        "waxed_lightly_weathered_cut_copper_slab" => "minecraft:waxed_exposed_cut_copper_slab",
    }
}

pub(crate) fn register() {
    rename_item(VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
    rename_block(VERSION, |name| {
        renames().get(name).map(|&str| str.to_owned())
    });
}
