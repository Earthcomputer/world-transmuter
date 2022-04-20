use rust_dataconverter_engine::{convert_map_in_map, data_converter_func, data_walker, ListType, MapEntry, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 106;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.untagged_spawner.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        // While all converters for spawners check the id for this version, we don't because spawners exist in minecarts. ooops! Loading a chunk
        // with a minecart spawner from 1.7.10 in 1.16.5 vanilla will fail to convert! Clearly there was a mistake in how they
        // used and applied spawner converters. In anycase, do not check the id - we are not guaranteed to be a tile
        // entity. We can be a regular old minecart spawner. And we know we are a spawner because this is only called from data walkers.

        if let Some(entity_id) = data.remove("EntityId").and_then(|o| o.into_string()) {
            let spawn_data = data.entry("SpawnData").or_insert_with(|| T::Object::create_map(T::Map::create_empty()));
            if let Some(spawn_data) = spawn_data.as_map_mut() {
                spawn_data.set("id", T::Object::create_string(if entity_id.is_empty() {"Pig".to_owned()} else {entity_id}));
            }
        }

        if let Some(spawn_potentials) = data.get_list_mut("SpawnPotentials") {
            for spawn in spawn_potentials.iter_mut() {
                if let Some(spawn) = spawn.as_map_mut() {
                    // convert to standard entity format (it's not a coincidence a walker for spawners is only added
                    // in this version)
                    if let Some(typ) = spawn.remove("Type").and_then(|o| o.into_string()) {
                        let mut properties = spawn.remove("Properties").and_then(|o| o.into_map()).unwrap_or_else(|| T::Map::create_empty());
                        properties.set("id", T::Object::create_string(typ));
                        spawn.set("Entity", T::Object::create_map(properties));
                    }
                }
            }
        }
    }));

    let entity_type = types.entity;
    types.untagged_spawner.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(spawn_potentials) = data.get_list_mut("SpawnPotentials") {
            for spawn in spawn_potentials.iter_mut() {
                if let Some(spawn) = spawn.as_map_mut() {
                    convert_map_in_map::<_, T>(entity_type, spawn, "Entity", from_version, to_version);
                }
            }
        }

        convert_map_in_map::<_, T>(entity_type, data, "SpawnData", from_version, to_version);
    }));
}
