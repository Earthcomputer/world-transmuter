use rust_dataconverter_engine::{convert_map_in_map, data_walker, map_data_converter_func};
use valence_nbt::{Compound, List, Value};
use crate::MinecraftTypesMut;

const VERSION: u32 = 106;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.untagged_spawner.borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        // While all converters for spawners check the id for this version, we don't because spawners exist in minecarts. ooops! Loading a chunk
        // with a minecart spawner from 1.7.10 in 1.16.5 vanilla will fail to convert! Clearly there was a mistake in how they
        // used and applied spawner converters. In anycase, do not check the id - we are not guaranteed to be a tile
        // entity. We can be a regular old minecart spawner. And we know we are a spawner because this is only called from data walkers.

        if let Some(Value::String(entity_id)) = data.remove("EntityId") {
            let spawn_data = data.entry("SpawnData").or_insert_with(Compound::new);
            if let Value::Compound(spawn_data) = spawn_data {
                spawn_data.insert("id", if entity_id.is_empty() {"Pig".to_owned()} else {entity_id});
            }
        }

        if let Some(Value::List(List::Compound(spawn_potentials))) = data.get_mut("SpawnPotentials") {
            for spawn in spawn_potentials {
                // convert to standard entity format (it's not a coincidence a walker for spawners is only added
                // in this version)
                if let Some(Value::String(typ)) = spawn.remove("Type") {
                    let mut properties = match spawn.remove("Properties") {
                        Some(Value::Compound(properties)) => properties,
                        _ => Compound::new(),
                    };
                    properties.insert("id", typ);
                    spawn.insert("Entity", properties);
                }
            }
        }
    }));

    let entity_type = types.entity;
    types.untagged_spawner.borrow_mut().add_structure_walker(VERSION, data_walker(move |data: &mut Compound, from_version, to_version| {
        if let Some(Value::List(List::Compound(spawn_potentials))) = data.get_mut("SpawnPotentials") {
            for spawn in spawn_potentials {
                convert_map_in_map(entity_type, spawn, "Entity", from_version, to_version);
            }
        }

        convert_map_in_map(entity_type, data, "SpawnData", from_version, to_version);
    }));
}
