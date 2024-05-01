use crate::types;
use crate::versions::v100;
use world_transmuter_engine::{convert_map_in_map, map_data_walker, JList, JValue};

const VERSION: u32 = 3689;

fn register_mob(id: &str) {
    v100::register_equipment(VERSION, id);
}

pub(crate) fn register() {
    register_mob("minecraft:breeze");

    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:trial_spawner",
        map_data_walker(|data, from_version, to_version| {
            if let Some(JValue::List(JList::Compound(spawn_potentials))) =
                data.get_mut("spawn_potentials")
            {
                for potential in spawn_potentials {
                    if let Some(JValue::Compound(data)) = potential.get_mut("data") {
                        convert_map_in_map(
                            types::entity_ref(),
                            data,
                            "entity",
                            from_version,
                            to_version,
                        );
                    }
                }

                convert_map_in_map(
                    types::entity_ref(),
                    data,
                    "spawn_data",
                    from_version,
                    to_version,
                );
            }
        }),
    );
}
