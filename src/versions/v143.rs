use rust_dataconverter_engine::Types;
use crate::helpers::rename;
use crate::helpers::rename::simple_rename;
use crate::MinecraftTypesMut;

const VERSION: u32 = 143;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename::rename_entity(types, VERSION, simple_rename("TippedArrow", "Arrow"));
}
