use crate::helpers::rename::{rename_advancement, rename_recipe, simple_rename};
use crate::types;
use valence_nbt::{List, Value};
use world_transmuter_engine::{convert_map_in_map, data_walker, DataWalkerMapListPaths};

const VERSION: u32 = 2100;

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
    rename_recipe(
        VERSION,
        simple_rename("minecraft:sugar", "sugar_from_sugar_cane"),
    );
    rename_advancement(
        VERSION,
        simple_rename(
            "minecraft:recipes/misc/sugar",
            "minecraft:recipes/misc/sugar_from_sugar_cane",
        ),
    );
    register_mob("minecraft:bee");
    register_mob("minecraft:bee_stinger");
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:beehive",
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::List(List::Compound(bees))) = data.get_mut("Bees") {
                for bee in bees {
                    convert_map_in_map(
                        types::entity_ref(),
                        bee,
                        "EntityData",
                        from_version,
                        to_version,
                    );
                }
            }
        }),
    );
}
