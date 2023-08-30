use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::MinecraftTypes;

const VERSION: u32 = 1483;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename_entity(
        types,
        VERSION,
        simple_rename("minecraft:puffer_fish", "minecraft:pufferfish"),
    );
    rename_item(
        types,
        VERSION,
        simple_rename(
            "minecraft:puffer_fish_spawn_egg",
            "minecraft:pufferfish_spawn_egg",
        ),
    );
    types.entity.borrow_mut().copy_walkers(
        VERSION,
        "minecraft:puffer_fish",
        "minecraft:pufferfish",
    );
}
