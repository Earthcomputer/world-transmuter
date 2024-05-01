use crate::helpers::components::convert_component_from_lenient;
use crate::types;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 101;

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id(
        "Sign",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_line(data, "Text1");
            update_line(data, "Text2");
            update_line(data, "Text3");
            update_line(data, "Text4");
        }),
    );
}

fn update_line(data: &mut JCompound, key: &str) {
    let Some(JValue::String(str)) = data.get(key) else {
        return;
    };
    let result = convert_component_from_lenient(str);
    data.insert(key, result);
}
