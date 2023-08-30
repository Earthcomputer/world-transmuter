use crate::types::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;

const VERSION: u32 = 3214;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.options.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(ao)) = data.get_mut("ao") {
                if ao == "0" {
                    *ao = "false".to_owned();
                } else if ao == "1" {
                    *ao = "true".to_owned();
                }
            }
        }),
    );
}
