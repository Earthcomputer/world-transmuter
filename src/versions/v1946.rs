use crate::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{compound, Compound};

const VERSION: u32 = 1946;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.poi_chunk.borrow_mut().add_structure_converter(
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
