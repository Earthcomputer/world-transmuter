use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;
use crate::helpers::resource_location::ResourceLocation;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1460;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:painting", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(Value::String(motif)) = data.get("Motive") {
            let motif = motif.to_lowercase();
            let motif = match motif.as_str() {
                "donkeykong" => "donkey_kong".to_owned(),
                "burningskull" => "burning_skull".to_owned(),
                "skullandroses" => "skull_and_roses".to_owned(),
                _ => motif
            };
            if let Ok(loc) = motif.parse::<ResourceLocation>() {
                data.insert("Motive", loc.to_string());
            }
        }
    }));
}
