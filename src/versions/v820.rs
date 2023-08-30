use crate::helpers::rename::{rename_item, simple_rename};
use crate::MinecraftTypes;

const VERSION: u32 = 820;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename_item(
        types,
        VERSION,
        simple_rename("minecraft:totem", "minecraft:totem_of_undying"),
    );
}
