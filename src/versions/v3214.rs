use crate::types::MinecraftTypesMut;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3214;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.options().borrow_mut().add_structure_converter(
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
