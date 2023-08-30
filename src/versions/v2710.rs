use crate::helpers::rename::{rename_stat, simple_rename};
use crate::MinecraftTypes;

const VERSION: u32 = 2710;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename_stat(
        types,
        VERSION,
        simple_rename("minecraft:play_one_minute", "minecraft:play_time"),
    );
}
