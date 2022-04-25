use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::helpers::resource_location::ResourceLocation;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1460;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:painting", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(motif) = data.get_string("Motive") {
            let motif = motif.to_lowercase();
            let motif = match motif.as_str() {
                "donkeykong" => "donkey_kong".to_owned(),
                "burningskull" => "burning_skull".to_owned(),
                "skullandroses" => "skull_and_roses".to_owned(),
                _ => motif
            };
            if let Ok(loc) = motif.parse::<ResourceLocation>() {
                data.set("Motive", T::Object::create_string(loc.to_string()));
            }
        }
    }));
}
