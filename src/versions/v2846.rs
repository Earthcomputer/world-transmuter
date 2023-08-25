use crate::helpers::rename::rename_advancement;
use crate::types::MinecraftTypesMut;

const VERSION: u32 = 2846;

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename_advancement(types, VERSION, |name| match name {
        "minecraft:husbandry/play_jukebox_in_meadows" => {
            Some("minecraft:adventure/play_jukebox_in_meadows".to_owned())
        }
        "minecraft:adventure/caves_and_cliff" => {
            Some("minecraft:adventure/fall_from_world_height".to_owned())
        }
        "minecraft:adventure/ride_strider_in_overworld_lava" => {
            Some("minecraft:nether/ride_strider_in_overworld_lava".to_owned())
        }
        _ => None,
    });
}
