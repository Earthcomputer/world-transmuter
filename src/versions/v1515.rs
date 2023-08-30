use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::rename_block;
use crate::MinecraftTypes;
use std::sync::OnceLock;

const VERSION: u32 = 1515;

static RENAMED_BLOCK_IDS: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renamed_block_ids() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMED_BLOCK_IDS.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("tube_coral_fan", "minecraft:tube_coral_wall_fan");
        map.insert_mc("brain_coral_fan", "minecraft:brain_coral_wall_fan");
        map.insert_mc("bubble_coral_fan", "minecraft:bubble_coral_wall_fan");
        map.insert_mc("fire_coral_fan", "minecraft:fire_coral_wall_fan");
        map.insert_mc("horn_coral_fan", "minecraft:horn_coral_wall_fan");
        map
    })
}

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename_block(types, VERSION, |name| {
        renamed_block_ids().get(name).map(|&str| str.to_owned())
    });
}
