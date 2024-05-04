use crate::helpers::rename::{rename_advancement, rename_recipe, simple_rename};
use crate::types;
use crate::versions::v100;
use world_transmuter_engine::{convert_map_in_map, map_data_walker, JList, JValue};

const VERSION: u32 = 2100;

fn register_mob(id: &str) {
    v100::register_equipment(VERSION, id);
}

pub(crate) fn register() {
    rename_recipe(
        VERSION,
        simple_rename("minecraft:sugar", "minecraft:sugar_from_sugar_cane"),
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
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::List(JList::Compound(bees))) = data.get_mut("Bees") {
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
