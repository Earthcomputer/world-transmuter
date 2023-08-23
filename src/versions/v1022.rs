use rust_dataconverter_engine::{convert_map_in_map, convert_map_list_in_map, convert_object_list_in_map, data_walker};
use valence_nbt::{Compound, Value};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1022;

pub(crate) fn register(types: &MinecraftTypesMut) {
    let entity_type = types.entity;
    let item_stack_type = types.item_stack;
    let recipe_type = types.recipe;
    types.player.borrow_mut().add_structure_walker(VERSION, data_walker(move |data: &mut Compound, from_version, to_version| {
        if let Some(Value::Compound(root_vehicle)) = data.get_mut("RootVehicle") {
            convert_map_in_map(entity_type, root_vehicle, "Entity", from_version, to_version);
        }

        convert_map_list_in_map(item_stack_type, data, "Inventory", from_version, to_version);
        convert_map_list_in_map(item_stack_type, data, "EnderItems", from_version, to_version);

        convert_map_in_map(entity_type, data, "ShoulderEntityLeft", from_version, to_version);
        convert_map_in_map(entity_type, data, "ShoulderEntityRight", from_version, to_version);

        if let Some(Value::Compound(recipe_book)) = data.get_mut("recipeBook") {
            convert_object_list_in_map(recipe_type, recipe_book, "recipes", from_version, to_version);
            convert_object_list_in_map(recipe_type, recipe_book, "toBeDisplayed", from_version, to_version);
        }
    }));

    let item_stack_type = types.item_stack;
    types.hotbar.borrow_mut().add_structure_walker(VERSION, data_walker(move |data, from_version, to_version| {
        let keys = data.keys().cloned().collect::<Vec<_>>();
        for key in keys {
            convert_map_list_in_map(item_stack_type, data, key.as_str(), from_version, to_version);
        }
    }));
}
