use crate::helpers::rename::{rename_block, rename_item};
use crate::static_string_mc_map;

const VERSION: u32 = 1480;

static_string_mc_map! {
    RENAMED_IDS, renamed_ids, {
        "blue_coral" => "minecraft:tube_coral_block",
        "pink_coral" => "minecraft:brain_coral_block",
        "purple_coral" => "minecraft:bubble_coral_block",
        "red_coral" => "minecraft:fire_coral_block",
        "yellow_coral" => "minecraft:horn_coral_block",
        "blue_coral_plant" => "minecraft:tube_coral",
        "pink_coral_plant" => "minecraft:brain_coral",
        "purple_coral_plant" => "minecraft:bubble_coral",
        "red_coral_plant" => "minecraft:fire_coral",
        "yellow_coral_plant" => "minecraft:horn_coral",
        "blue_coral_fan" => "minecraft:tube_coral_fan",
        "pink_coral_fan" => "minecraft:brain_coral_fan",
        "purple_coral_fan" => "minecraft:bubble_coral_fan",
        "red_coral_fan" => "minecraft:fire_coral_fan",
        "yellow_coral_fan" => "minecraft:horn_coral_fan",
        "blue_dead_coral" => "minecraft:dead_tube_coral",
        "pink_dead_coral" => "minecraft:dead_brain_coral",
        "purple_dead_coral" => "minecraft:dead_bubble_coral",
        "red_dead_coral" => "minecraft:dead_fire_coral",
        "yellow_dead_coral" => "minecraft:dead_horn_coral",
    }
}

pub(crate) fn register() {
    rename_block(VERSION, |name| {
        renamed_ids().get(name).copied().map(|str| str.to_owned())
    });
    rename_item(VERSION, |name| {
        renamed_ids().get(name).copied().map(|str| str.to_owned())
    });
}
