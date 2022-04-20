use rust_dataconverter_engine::Types;
use crate::helpers::rename;
use crate::MinecraftTypesMut;

const VERSION: u32 = 143;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename::rename_entity(types, VERSION, |id| if id == "TippedArrow" { Some("Arrow".to_owned()) } else { None })
}
