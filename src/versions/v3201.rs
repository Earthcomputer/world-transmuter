use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{Compound, Value};

const VERSION: u32 = 3201;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.options().borrow_mut().add_structure_converter(
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
