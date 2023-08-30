use crate::types::MinecraftTypes;
use crate::versions::v3088::ConverterAddBlendingData;

const VERSION: u32 = 3441;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types
        .chunk
        .borrow_mut()
        .add_structure_converter(VERSION, ConverterAddBlendingData);
}
