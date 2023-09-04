use crate::types;
use crate::versions::v3088::ConverterAddBlendingData;

const VERSION: u32 = 3441;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(VERSION, ConverterAddBlendingData);
}
