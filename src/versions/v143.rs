use crate::helpers::rename;
use crate::helpers::rename::simple_rename;

const VERSION: u32 = 143;

pub(crate) fn register() {
    rename::rename_entity(VERSION, simple_rename("TippedArrow", "Arrow"));
}
