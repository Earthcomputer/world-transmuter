use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{Compound, Value};

const VERSION: u32 = 1925;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.saved_data.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if !matches!(data.get("data"), Some(Value::Compound(_))) {
                data.insert("data", Compound::new());
            }
        }),
    );
}
