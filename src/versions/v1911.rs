use crate::{static_string_map, types};
use java_string::JavaStr;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1911;

static_string_map! {
    CHUNK_STATUS_REMAP, chunk_status_remap, {
        "structure_references" => "empty",
        "biomes" => "empty",
        "base" => "surface",
        "carved" => "carvers",
        "liquid_carved" => "liquid_carvers",
        "decorated" => "features",
        "lighted" => "light",
        "mobs_spawned" => "spawn",
        "finalized" => "heightmaps",
        "fullchunk" => "full",
    }
}

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(level)) = data.get_mut("Level") {
                let status = match level.get("Status") {
                    Some(JValue::String(status)) => &status[..],
                    _ => JavaStr::from_str("empty"),
                };
                let new_status = chunk_status_remap()
                    .get(status)
                    .copied()
                    .unwrap_or(JavaStr::from_str("empty"));
                level.insert("Status", new_status);
            }
        }),
    );
}
