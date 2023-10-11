use crate::helpers::rename::{rename_entity, rename_item};
use crate::types;
use java_string::JavaString;

const VERSION: u32 = 1486;

pub(crate) fn register() {
    rename_entity(VERSION, |name| match name.as_bytes() {
        b"minecraft:salmon_mob" => Some(JavaString::from("minecraft:salmon")),
        b"minecraft:cod_mob" => Some(JavaString::from("minecraft:cod")),
        _ => None,
    });
    rename_item(VERSION, |name| match name.as_bytes() {
        b"minecraft:salmon_mob_spawn_egg" => Some(JavaString::from("minecraft:salmon_spawn_egg")),
        b"minecraft:cod_mob_spawn_egg" => Some(JavaString::from("minecraft:cod_spawn_egg")),
        _ => None,
    });

    types::entity_mut().copy_walkers(VERSION, "minecraft:salmon_mob", "minecraft:salmon");
    types::entity_mut().copy_walkers(VERSION, "minecraft:cod_mob", "minecraft:cod");
}
