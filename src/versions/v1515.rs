use crate::helpers::rename::rename_block;
use crate::static_string_mc_map;

const VERSION: u32 = 1515;

static_string_mc_map! {
    renamed_block_ids = {
        "tube_coral_fan" => "minecraft:tube_coral_wall_fan",
        "brain_coral_fan" => "minecraft:brain_coral_wall_fan",
        "bubble_coral_fan" => "minecraft:bubble_coral_wall_fan",
        "fire_coral_fan" => "minecraft:fire_coral_wall_fan",
        "horn_coral_fan" => "minecraft:horn_coral_wall_fan",
    }
}

pub(crate) fn register() {
    rename_block(VERSION, |name| {
        renamed_block_ids().get(name).map(|&str| str.to_owned())
    });
}
