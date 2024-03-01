use crate::types;
use world_transmuter_engine::{
    convert_map_in_map, data_walker, DataWalkerMapListPaths, JList, JValue,
};

const VERSION: u32 = 3689;

fn register_mob(id: &str) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types::item_stack_ref(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}

pub(crate) fn register() {
    register_mob("minecraft:breeze");

    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:trial_spawner",
        data_walker(|data, from_version, to_version| {
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
