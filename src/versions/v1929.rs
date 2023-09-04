use crate::types;
use valence_nbt::{List, Value};
use world_transmuter_engine::{convert_map_in_map, convert_map_list_in_map, data_walker};

const VERSION: u32 = 1929;

pub(crate) fn register() {
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:wandering_trader",
        data_walker(move |data, from_version, to_version| {
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "Inventory",
                from_version,
                to_version,
            );

            if let Some(Value::Compound(offers)) = data.get_mut("Offers") {
                if let Some(Value::List(List::Compound(recipes))) = offers.get_mut("Recipes") {
                    for recipe in recipes {
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "buy",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "buyB",
                            from_version,
                            to_version,
                        );
                        convert_map_in_map(
                            types::item_stack_ref(),
                            recipe,
                            "sell",
                            from_version,
                            to_version,
                        );
                    }
                }
            }

            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "ArmorItems",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "HandItems",
                from_version,
                to_version,
            );
        }),
    );
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:trader_llama",
        data_walker(move |data, from_version, to_version| {
            convert_map_in_map(
                types::item_stack_ref(),
                data,
                "SaddleItem",
                from_version,
                to_version,
            );
            convert_map_in_map(
                types::item_stack_ref(),
                data,
                "DecorItem",
                from_version,
                to_version,
            );

            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "Items",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "ArmorItems",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "HandItems",
                from_version,
                to_version,
            );
        }),
    );
}
