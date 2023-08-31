use crate::helpers::rename::{rename_advancement, rename_recipe, simple_rename};
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{convert_map_in_map, data_walker, DataWalkerMapListPaths};
use valence_nbt::{List, Value};

const VERSION: u32 = 2100;

fn register_mob(types: MinecraftTypesMut, id: &str) {
    types.entity().borrow_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapListPaths::new_multi(
            types.item_stack(),
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}

pub(crate) fn register(types: MinecraftTypesMut) {
    rename_recipe(
        types,
        VERSION,
        simple_rename("minecraft:sugar", "sugar_from_sugar_cane"),
    );
    rename_advancement(
        types,
        VERSION,
        simple_rename(
            "minecraft:recipes/misc/sugar",
            "minecraft:recipes/misc/sugar_from_sugar_cane",
        ),
    );
    register_mob(types, "minecraft:bee");
    register_mob(types, "minecraft:bee_stinger");
    let entity_type = types.entity();
    types.tile_entity().borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:beehive",
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::List(List::Compound(bees))) = data.get_mut("Bees") {
                for bee in bees {
                    convert_map_in_map(entity_type, bee, "EntityData", from_version, to_version);
                }
            }
        }),
    );
}
