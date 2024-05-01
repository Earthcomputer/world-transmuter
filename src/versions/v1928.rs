use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::versions::v100;

const VERSION: u32 = 1928;

pub(crate) fn register() {
    rename_entity(
        VERSION,
        simple_rename("minecraft:illager_beast", "minecraft:ravager"),
    );
    rename_item(
        VERSION,
        simple_rename(
            "minecraft:illager_beast_spawn_egg",
            "minecraft:ravager_spawn_egg",
        ),
    );

    v100::register_equipment(VERSION, "minecraft:ravager");
}
