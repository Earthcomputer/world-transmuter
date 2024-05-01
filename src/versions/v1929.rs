use crate::types;
use crate::versions::v100;
use world_transmuter_engine::{
    convert_map_in_map, convert_map_list_in_map, map_data_walker, JValue,
};

const VERSION: u32 = 1929;

pub(crate) fn register() {
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:wandering_trader",
        map_data_walker(move |data, from_version, to_version| {
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "Inventory",
                from_version,
                to_version,
            );

            if let Some(JValue::Compound(offers)) = data.get_mut("Offers") {
                convert_map_list_in_map(
                    types::villager_trade_ref(),
                    offers,
                    "Recipes",
                    from_version,
                    to_version,
                );
            }
        }),
    );
    v100::register_equipment(VERSION, "minecraft:wandering_trader");
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:trader_llama",
        map_data_walker(move |data, from_version, to_version| {
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
        }),
    );
    v100::register_equipment(VERSION, "minecraft:trader_llama");
}
