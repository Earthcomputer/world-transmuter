use crate::helpers::rename::{rename_enchantment, simple_rename};

const VERSION: u32 = 3803;

pub(crate) fn register() {
    rename_enchantment(
        VERSION,
        simple_rename("minecraft:sweeping", "minecraft:sweeping_edge"),
    );
}
