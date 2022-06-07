use rust_dataconverter_engine::{data_converter_func, MapType, ObjectRef, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2202;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            if let Some(ObjectRef::IntArray(biomes)) = level.get("Biomes").map(|o| o.as_ref()) {
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

                    level.set("Biomes", T::Object::create_int_array(new_biomes));
                }
            }
        }
    }));
}
