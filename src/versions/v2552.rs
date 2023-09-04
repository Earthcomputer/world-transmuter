use crate::types;
use valence_nbt::value::ValueMut;
use world_transmuter_engine::value_data_converter_func;

const VERSION: u32 = 2552;

pub(crate) fn register() {
    types::biome_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let ValueMut::String(data) = data {
                if *data == "minecraft:nether" {
                    **data = "minecraft:nether_wastes".to_owned();
                }
            }
        }),
    );
}
