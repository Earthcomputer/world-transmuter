use crate::helpers::rename::{rename_option, simple_rename};
use crate::types;
use crate::versions::v2550;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 2558;

pub(crate) fn register() {
    rename_option(
        VERSION,
        simple_rename("key_key.swapHands", "key_key.swapOffhand"),
    );

    types::world_gen_settings_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let has_empty_dimensions = match data.get("dimensions") {
                Some(JValue::Compound(dimensions)) => dimensions.is_empty(),
                _ => true,
            };
            if has_empty_dimensions {
                let new_dimensions = recreate_settings(data);
                data.insert("dimensions", new_dimensions);
            }
        }),
    );
}

fn recreate_settings(data: &JCompound) -> JCompound {
    let seed = data.get("seed").and_then(|v| v.as_i64()).unwrap_or(0);
    v2550::vanilla_levels(seed, v2550::default_overworld(seed), false)
}
