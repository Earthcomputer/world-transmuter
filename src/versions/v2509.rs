use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::versions::v100;

const VERSION: u32 = 2509;

pub(crate) fn register() {
    rename_item(
        VERSION,
        simple_rename(
            "minecraft:zombie_pigman_spawn_egg",
            "minecraft:zombified_piglin_spawn_egg",
        ),
    );
    rename_entity(
        VERSION,
        simple_rename("minecraft:zombie_pigman", "minecraft:zombified_piglin"),
    );
    v100::register_equipment(VERSION, "minecraft:zombified_piglin");
}
