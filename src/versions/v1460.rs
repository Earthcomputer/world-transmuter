use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 1460;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:painting",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(motif)) = data.get("Motive") {
                let motif = motif.to_lowercase();
                let motif = match motif.as_str() {
                    "donkeykong" => "donkey_kong".to_owned(),
                    "burningskull" => "burning_skull".to_owned(),
                    "skullandroses" => "skull_and_roses".to_owned(),
                    _ => motif,
                };
                if let Ok(loc) = motif.parse::<ResourceLocation>() {
                    data.insert("Motive", loc.to_string());
                }
            }
        }),
    );
}
