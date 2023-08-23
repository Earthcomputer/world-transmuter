use crate::helpers::rename;
use crate::helpers::rename::simple_rename;
use crate::MinecraftTypesMut;

const VERSION: u32 = 143;

pub(crate) fn register(types: &MinecraftTypesMut) {
    rename::rename_entity(types, VERSION, simple_rename("TippedArrow", "Arrow"));
}
