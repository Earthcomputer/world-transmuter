use crate::MinecraftTypesMut;
use valence_nbt::{compound, Compound};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1946;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.poi_chunk().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let mut sections = Compound::new();
            for y in 0..16 {
                let key = y.to_string();
                if let Some(records) = data.remove(&key) {
                    let section = compound! {
                        "Records" => records,
                    };
                    sections.insert(key, section); // integer keys convert to string in DFU (at least for NBT ops)
                }
            }
            data.insert("Sections", sections);
        }),
    );
}
