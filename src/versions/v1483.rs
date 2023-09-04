use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::types;

const VERSION: u32 = 1483;

pub(crate) fn register() {
    rename_entity(
        VERSION,
        simple_rename("minecraft:puffer_fish", "minecraft:pufferfish"),
    );
    rename_item(
        VERSION,
        simple_rename(
            "minecraft:puffer_fish_spawn_egg",
            "minecraft:pufferfish_spawn_egg",
        ),
    );
    types::entity_mut().copy_walkers(VERSION, "minecraft:puffer_fish", "minecraft:pufferfish");
}
