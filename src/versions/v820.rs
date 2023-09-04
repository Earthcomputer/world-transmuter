use crate::helpers::rename::{rename_item, simple_rename};

const VERSION: u32 = 820;

pub(crate) fn register() {
    rename_item(
        VERSION,
        simple_rename("minecraft:totem", "minecraft:totem_of_undying"),
    );
}
