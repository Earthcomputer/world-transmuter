use crate::types;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 2202;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            let Some(Value::IntArray(biomes)) = level.get_mut("Biomes") else {
                return;
            };
            if biomes.len() == 256 {
                let mut new_biomes = vec![0; 1024];

                for new_z in 0..4 {
                    for new_x in 0..4 {
                        let old_x = (new_x << 2) + 2;
                        let old_z = (new_z << 2) + 2;
                        let old_index = old_z << 4 | old_x;
                        new_biomes[new_z << 2 | new_x] = biomes[old_index];
                    }
                }

                for n in 1..64 {
                    new_biomes.copy_within(0..16, n * 16);
                }

                *biomes = new_biomes;
            }
        }),
    );
}
