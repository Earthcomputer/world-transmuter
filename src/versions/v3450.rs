use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3450;

fn apply_rename(status: &mut String) {
    if status == "minecraft:liquid_carvers" {
        *status = "minecraft:carvers".to_owned();
    } else if status == "minecraft:heightmaps" {
        *status = "minecraft:spawn".to_owned();
    }
}

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            // Note: DFU technically enforces namespace due to how they wrote their converter, so we will do the same.
            if let Some(Value::String(status)) = data.get_mut("Status") {
                let mut new_status = status
                    .parse::<ResourceLocation>()
                    .map_or_else(|_| status.clone(), |rl| rl.to_string());
                apply_rename(&mut new_status);
                *status = new_status;
            }

            if let Some(Value::Compound(below_zero_retrogen)) = data.get_mut("below_zero_retrogen")
            {
                if let Some(Value::String(status)) = below_zero_retrogen.get_mut("target_status") {
                    let mut new_status = status
                        .parse::<ResourceLocation>()
                        .map_or_else(|_| status.clone(), |rl| rl.to_string());
                    apply_rename(&mut new_status);
                    *status = new_status;
                }
            }
        }),
    );
}
