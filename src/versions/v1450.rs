use rust_dataconverter_engine::{data_converter_func, Types};
use crate::helpers::block_flattening_v1450;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1450;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.block_state.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(new_data) = block_flattening_v1450::flatten_nbt::<T>(data) {
            *data = new_data;
        }
    }));
}
