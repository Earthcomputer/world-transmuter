use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{
    convert_map_in_map, map_data_converter_func, map_data_walker, JCompound, JList, JValue,
};

const VERSION: u32 = 106;

pub(crate) fn register() {
    types::untagged_spawner_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            // While all converters for spawners check the id for this version, we don't because spawners exist in minecarts. ooops! Loading a chunk
            // with a minecart spawner from 1.7.10 in 1.16.5 vanilla will fail to convert! Clearly there was a mistake in how they
            // used and applied spawner converters. In anycase, do not check the id - we are not guaranteed to be a tile
            // entity. We can be a regular old minecart spawner. And we know we are a spawner because this is only called from data walkers.

            if let Some(JValue::String(entity_id)) = data.remove("EntityId") {
                let spawn_data = data.entry("SpawnData").or_insert_with(JCompound::new);
                if let JValue::Compound(spawn_data) = spawn_data {
                    spawn_data.insert(
                        "id",
                        if entity_id.is_empty() {
                            JavaString::from("Pig")
                        } else {
                            entity_id
                        },
                    );
                }
            }

            if let Some(JValue::List(JList::Compound(spawn_potentials))) =
                data.get_mut("SpawnPotentials")
            {
                for spawn in spawn_potentials {
                    // convert to standard entity format (it's not a coincidence a walker for spawners is only added
                    // in this version)
                    if let Some(JValue::String(typ)) = spawn.remove("Type") {
                        let mut properties = match spawn.remove("Properties") {
                            Some(JValue::Compound(properties)) => properties,
                            _ => JCompound::new(),
                        };
                        properties.insert("id", typ);
                        spawn.insert("Entity", properties);
                    }
                }
            }
        }),
    );

    types::untagged_spawner_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::List(JList::Compound(spawn_potentials))) =
                data.get_mut("SpawnPotentials")
            {
                for spawn in spawn_potentials {
                    convert_map_in_map(
                        types::entity_ref(),
                        spawn,
                        "Entity",
                        from_version,
                        to_version,
                    );
                }
            }

            convert_map_in_map(
                types::entity_ref(),
                data,
                "SpawnData",
                from_version,
                to_version,
            );
        }),
    );
}
