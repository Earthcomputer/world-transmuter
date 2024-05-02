use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 3450;

fn apply_rename(status: &mut JavaString) {
    if status == "minecraft:liquid_carvers" {
        *status = JavaString::from("minecraft:carvers");
    } else if status == "minecraft:heightmaps" {
        *status = JavaString::from("minecraft:spawn");
    }
}

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            // Note: DFU technically enforces namespace due to how they wrote their converter, so we will do the same.
            if let Some(JValue::String(status)) = data.get_mut("Status") {
                let mut new_status = ResourceLocation::make_correct(status);
                apply_rename(&mut new_status);
                *status = new_status;
            }

            if let Some(JValue::Compound(below_zero_retrogen)) = data.get_mut("below_zero_retrogen")
            {
                if let Some(JValue::String(status)) = below_zero_retrogen.get_mut("target_status") {
                    let mut new_status = ResourceLocation::make_correct(status);
                    apply_rename(&mut new_status);
                    *status = new_status;
                }
            }
        }),
    );
}
