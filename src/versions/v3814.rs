use crate::helpers::rename::{rename_attribute_old, simple_rename};

const VERSION: u32 = 3814;

pub(crate) fn register() {
    rename_attribute_old(
        VERSION,
        simple_rename(
            "minecraft:horse.jump_strength",
            "minecraft:generic.jump_strength",
        ),
    );
}
