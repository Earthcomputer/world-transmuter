use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1911;

static CHUNK_STATUS_REMAP: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, &'static str>> = SyncOnceCell::new();

fn chunk_status_remap() -> &'static rust_dataconverter_engine::Map<&'static str, &'static str> {
    CHUNK_STATUS_REMAP.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
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

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            let new_status = chunk_status_remap().get(level.get_string("Status").unwrap_or("empty")).copied().unwrap_or("empty");
            level.set("Status", T::Object::create_string(new_status.to_owned()));
        }
    }));
}
