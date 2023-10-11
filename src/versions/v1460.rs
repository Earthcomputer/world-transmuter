use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1460;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:painting",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(motif)) = data.get("Motive") {
                let motif = motif.to_lowercase();
                let motif = match motif.as_bytes() {
                    b"donkeykong" => JavaString::from("donkey_kong"),
                    b"burningskull" => JavaString::from("burning_skull"),
                    b"skullandroses" => JavaString::from("skull_and_roses"),
                    _ => motif,
                };
                if let Ok(loc) = ResourceLocation::parse(&motif) {
                    data.insert("Motive", loc.to_java_string());
                }
            }
        }),
    );
}
