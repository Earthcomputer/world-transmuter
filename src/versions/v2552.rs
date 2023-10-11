use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{value_data_converter_func, JValueMut};

const VERSION: u32 = 2552;

pub(crate) fn register() {
    types::biome_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let JValueMut::String(data) = data {
                if *data == "minecraft:nether" {
                    **data = JavaString::from("minecraft:nether_wastes");
                }
            }
        }),
    );
}
