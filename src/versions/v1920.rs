use rust_dataconverter_engine::{data_converter_func, DataWalkerMapListPaths, MapType, ObjectType, Types};
use crate::helpers::resource_location::ResourceLocation;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1920;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            if let Some(structures) = level.get_map_mut("Structures") {
                if let Some(starts) = structures.get_map_mut("Starts") {
                    if let Some(village) = starts.remove("New_Village").and_then(|o| o.into_map()) {
                        starts.set("Village", T::Object::create_map(village));
                    } else {
                        starts.remove("Village");
                    }
                }

                if let Some(references) = structures.get_map_mut("References") {
                    if let Some(village) = references.remove("New_Village").and_then(|o| o.into_map()) {
                        references.set("Village", T::Object::create_map(village));
                    } else {
                        references.remove("Village");
                    }
                }
            }
        }
    }));

    types.structure_feature.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("id")
            .and_then(|str| str.parse::<ResourceLocation>().ok())
            .map(|rl| rl.to_string())
            .as_deref() == Some("minecraft:new_village")
        {
            data.set("id", T::Object::create_string("minecraft:village".to_owned()));
        }
    }));

    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:campfire", DataWalkerMapListPaths::new(types.item_stack, "Items"));
}
