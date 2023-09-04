use crate::helpers::rename::{rename_stat, simple_rename};

const VERSION: u32 = 2710;

pub(crate) fn register() {
    rename_stat(
        VERSION,
        simple_rename("minecraft:play_one_minute", "minecraft:play_time"),
    );
}
