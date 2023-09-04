use crate::types::MinecraftTypesMut;
use valence_nbt::{List, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3451;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.chunk().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.remove("isLightOn");

            if let Some(Value::List(List::Compound(sections))) = data.get_mut("sections") {
                for section in sections {
                    section.remove("BlockLight");
                    section.remove("SkyLight");
                }
            }
        }),
    );
}
