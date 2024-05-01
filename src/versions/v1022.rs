use crate::types;
use world_transmuter_engine::{
    convert_map_in_map, convert_map_list_in_map, convert_object_list_in_map, map_data_walker,
    JValue,
};

const VERSION: u32 = 1022;

pub(crate) fn register() {
    types::player_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            if let Some(JValue::Compound(root_vehicle)) = data.get_mut("RootVehicle") {
                convert_map_in_map(
                    types::entity_ref(),
                    root_vehicle,
                    "Entity",
                    from_version,
                    to_version,
                );
            }

            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "Inventory",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "EnderItems",
                from_version,
                to_version,
            );

            convert_map_in_map(
                types::entity_ref(),
                data,
                "ShoulderEntityLeft",
                from_version,
                to_version,
            );
            convert_map_in_map(
                types::entity_ref(),
                data,
                "ShoulderEntityRight",
                from_version,
                to_version,
            );

            if let Some(JValue::Compound(recipe_book)) = data.get_mut("recipeBook") {
                convert_object_list_in_map(
                    types::recipe_ref(),
                    recipe_book,
                    "recipes",
                    from_version,
                    to_version,
                );
                convert_object_list_in_map(
                    types::recipe_ref(),
                    recipe_book,
                    "toBeDisplayed",
                    from_version,
                    to_version,
                );
            }
        }),
    );

    types::hotbar_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            let keys = data.keys().cloned().collect::<Vec<_>>();
            for key in keys {
                convert_map_list_in_map(
                    types::item_stack_ref(),
                    data,
                    &key[..],
                    from_version,
                    to_version,
                );
            }
        }),
    );
}
