use crate::types;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1911;

static CHUNK_STATUS_REMAP: OnceLock<BTreeMap<&'static str, &'static str>> = OnceLock::new();

fn chunk_status_remap() -> &'static BTreeMap<&'static str, &'static str> {
    CHUNK_STATUS_REMAP.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert("structure_references", "empty");
        map.insert("biomes", "empty");
        map.insert("base", "surface");
        map.insert("carved", "carvers");
        map.insert("liquid_carved", "liquid_carvers");
        map.insert("decorated", "features");
        map.insert("lighted", "light");
        map.insert("mobs_spawned", "spawn");
        map.insert("finalized", "heightmaps");
        map.insert("fullchunk", "full");
        map
    })
}

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::Compound(level)) = data.get_mut("Level") {
                let status = match level.get("Status") {
                    Some(Value::String(status)) => &status[..],
                    _ => "empty",
                };
                let new_status = chunk_status_remap().get(status).copied().unwrap_or("empty");
                level.insert("Status", new_status);
            }
        }),
    );
}
