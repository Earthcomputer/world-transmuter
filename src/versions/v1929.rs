use rust_dataconverter_engine::{convert_map_in_map, convert_map_list_in_map, data_walker};
use valence_nbt::{List, Value};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1929;

pub(crate) fn register(types: &MinecraftTypesMut) {
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:wandering_trader", data_walker(move |data, from_version, to_version| {
        convert_map_list_in_map(item_stack_type, data, "Inventory", from_version, to_version);

        if let Some(Value::Compound(offers)) = data.get_mut("Offers") {
            if let Some(Value::List(List::Compound(recipes))) = offers.get_mut("Recipes") {
                for recipe in recipes {
                    convert_map_in_map(item_stack_type, recipe, "buy", from_version, to_version);
                    convert_map_in_map(item_stack_type, recipe, "buyB", from_version, to_version);
                    convert_map_in_map(item_stack_type, recipe, "sell", from_version, to_version);
                }
            }
        }

        convert_map_list_in_map(item_stack_type, data, "ArmorItems", from_version, to_version);
        convert_map_list_in_map(item_stack_type, data, "HandItems", from_version, to_version);
    }));
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:trader_llama", data_walker(move |data, from_version, to_version| {
        convert_map_in_map(item_stack_type, data, "SaddleItem", from_version, to_version);
        convert_map_in_map(item_stack_type, data, "DecorItem", from_version, to_version);

        convert_map_list_in_map(item_stack_type, data, "Items", from_version, to_version);
        convert_map_list_in_map(item_stack_type, data, "ArmorItems", from_version, to_version);
        convert_map_list_in_map(item_stack_type, data, "HandItems", from_version, to_version);
    }));
}
