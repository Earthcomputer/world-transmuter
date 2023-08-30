use crate::helpers::rename;
use crate::helpers::rename::simple_rename;
use crate::MinecraftTypes;

const VERSION: u32 = 143;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename::rename_entity(types, VERSION, simple_rename("TippedArrow", "Arrow"));
}
