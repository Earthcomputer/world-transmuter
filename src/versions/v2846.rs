use crate::helpers::rename::rename_advancement;
use java_string::JavaString;

const VERSION: u32 = 2846;

pub(crate) fn register() {
    rename_advancement(VERSION, |name| match name.as_bytes() {
        b"minecraft:husbandry/play_jukebox_in_meadows" => Some(JavaString::from(
            "minecraft:adventure/play_jukebox_in_meadows",
        )),
        b"minecraft:adventure/caves_and_cliff" => Some(JavaString::from(
            "minecraft:adventure/fall_from_world_height",
        )),
        b"minecraft:adventure/ride_strider_in_overworld_lava" => Some(JavaString::from(
            "minecraft:nether/ride_strider_in_overworld_lava",
        )),
        _ => None,
    });
}
