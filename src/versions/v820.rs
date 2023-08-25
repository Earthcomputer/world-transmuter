use crate::helpers::rename::{rename_item, simple_rename};
use crate::MinecraftTypesMut;

const VERSION: u32 = 820;

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename_item(
        types,
        VERSION,
        simple_rename("minecraft:totem", "minecraft:totem_of_undying"),
    );
}
