use crate::helpers::rename::{rename_item, simple_rename};

const VERSION: u32 = 3800;

pub(crate) fn register() {
    rename_item(
        VERSION,
        simple_rename("minecraft:scute", "minecraft:turtle_scute"),
    );
}
