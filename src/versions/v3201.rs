use crate::types;
use valence_nbt::{Compound, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3201;

pub(crate) fn register() {
    types::options_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            fix_list(data, "resourcePacks");
            fix_list(data, "incompatibleResourcePacks");
        }),
    );
}

fn fix_list(data: &mut Compound, target: &str) {
    if let Some(Value::String(list)) = data.get_mut(target) {
        let new_list = list.replace("\"programer_art\"", "\"programmer_art\"");
        *list = new_list;
    }
}
