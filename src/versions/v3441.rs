use crate::types::MinecraftTypesMut;
use crate::versions::v3088::ConverterAddBlendingData;

const VERSION: u32 = 3441;

pub(crate) fn register(types: MinecraftTypesMut) {
    types
        .chunk()
        .borrow_mut()
        .add_structure_converter(VERSION, ConverterAddBlendingData);
}
