use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::helpers::rename::{rename_block, rename_item};
use crate::MinecraftTypesMut;
use std::sync::OnceLock;

const VERSION: u32 = 1480;

static RENAMED_IDS: OnceLock<McNamespaceMap<&'static str>> = OnceLock::new();

fn renamed_ids() -> &'static McNamespaceMap<'static, &'static str> {
    RENAMED_IDS.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("blue_coral", "minecraft:tube_coral_block");
        map.insert_mc("pink_coral", "minecraft:brain_coral_block");
        map.insert_mc("purple_coral", "minecraft:bubble_coral_block");
        map.insert_mc("red_coral", "minecraft:fire_coral_block");
        map.insert_mc("yellow_coral", "minecraft:horn_coral_block");
        map.insert_mc("blue_coral_plant", "minecraft:tube_coral");
        map.insert_mc("pink_coral_plant", "minecraft:brain_coral");
        map.insert_mc("purple_coral_plant", "minecraft:bubble_coral");
        map.insert_mc("red_coral_plant", "minecraft:fire_coral");
        map.insert_mc("yellow_coral_plant", "minecraft:horn_coral");
        map.insert_mc("blue_coral_fan", "minecraft:tube_coral_fan");
        map.insert_mc("pink_coral_fan", "minecraft:brain_coral_fan");
        map.insert_mc("purple_coral_fan", "minecraft:bubble_coral_fan");
        map.insert_mc("red_coral_fan", "minecraft:fire_coral_fan");
        map.insert_mc("yellow_coral_fan", "minecraft:horn_coral_fan");
        map.insert_mc("blue_dead_coral", "minecraft:dead_tube_coral");
        map.insert_mc("pink_dead_coral", "minecraft:dead_brain_coral");
        map.insert_mc("purple_dead_coral", "minecraft:dead_bubble_coral");
        map.insert_mc("red_dead_coral", "minecraft:dead_fire_coral");
        map.insert_mc("yellow_dead_coral", "minecraft:dead_horn_coral");
        map
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename_block(types, VERSION, |name| {
        renamed_ids().get(name).copied().map(|str| str.to_owned())
    });
    rename_item(types, VERSION, |name| {
        renamed_ids().get(name).copied().map(|str| str.to_owned())
    });
}
