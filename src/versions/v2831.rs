use crate::types;
use valence_nbt::{compound, Compound, List, Value};
use world_transmuter_engine::{convert_map_in_map, data_walker, map_data_converter_func};

const VERSION: u32 = 2831;

pub(crate) fn register() {
    types::untagged_spawner_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::List(List::Compound(spawn_potentials))) =
                data.get_mut("SpawnPotentials")
            {
                for spawn_potential in spawn_potentials {
                    if let Some(Value::Compound(spawn_data)) = spawn_potential.get_mut("data") {
                        convert_map_in_map(
                            types::entity_ref(),
                            spawn_data,
                            "entity",
                            from_version,
                            to_version,
                        );
                    }
                }
            }

            if let Some(Value::Compound(spawn_data)) = data.get_mut("SpawnData") {
                convert_map_in_map(
                    types::entity_ref(),
                    spawn_data,
                    "entity",
                    from_version,
                    to_version,
                );
            }
        }),
    );

    types::untagged_spawner_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if matches!(data.get("SpawnData"), Some(Value::Compound(_))) {
                let Some(Value::Compound(spawn_data)) = data.remove("SpawnData") else {
                    unreachable!()
                };
                data.insert(
                    "SpawnData",
                    compound! {
                        "entity" => spawn_data,
                    },
                );
            }

            if let Some(Value::List(List::Compound(spawn_potentials))) =
                data.get_mut("SpawnPotentials")
            {
                for spawn_potential in spawn_potentials {
                    // new format of weighted list (SpawnPotentials):
                    // root.data -> data
                    // root.weight -> weight

                    let weight = spawn_potential
                        .remove("Weight")
                        .and_then(|o| o.as_i32())
                        .unwrap_or(1);
                    spawn_potential.insert("weight", weight);
                    let mut data = Compound::new();
                    if let Some(Value::Compound(entity)) = spawn_potential.remove("Entity") {
                        data.insert("entity", entity);
                    }
                    spawn_potential.insert("data", data);
                }
            }
        }),
    );
}
