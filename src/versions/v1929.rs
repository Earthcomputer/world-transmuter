use rust_dataconverter_engine::{convert_map_in_map, convert_map_list_in_map, data_walker, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1929;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:wandering_trader", data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        convert_map_list_in_map::<_, T>(item_stack_type, data, "Inventory", from_version, to_version);

        if let Some(offers) = data.get_map_mut("Offers") {
            if let Some(recipes) = offers.get_list_mut("Recipes") {
                for recipe in recipes.iter_mut() {
                    if let Some(recipe) = recipe.as_map_mut() {
                        convert_map_in_map::<_, T>(item_stack_type, recipe, "buy", from_version, to_version);
                        convert_map_in_map::<_, T>(item_stack_type, recipe, "buyB", from_version, to_version);
                        convert_map_in_map::<_, T>(item_stack_type, recipe, "sell", from_version, to_version);
                    }
                }
            }
        }

        convert_map_list_in_map::<_, T>(item_stack_type, data, "ArmorItems", from_version, to_version);
        convert_map_list_in_map::<_, T>(item_stack_type, data, "HandItems", from_version, to_version);
    }));
    let item_stack_type = types.item_stack;
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:trader_llama", data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        convert_map_in_map::<_, T>(item_stack_type, data, "SaddleItem", from_version, to_version);
        convert_map_in_map::<_, T>(item_stack_type, data, "DecorItem", from_version, to_version);

        convert_map_list_in_map::<_, T>(item_stack_type, data, "Items", from_version, to_version);
        convert_map_list_in_map::<_, T>(item_stack_type, data, "ArmorItems", from_version, to_version);
        convert_map_list_in_map::<_, T>(item_stack_type, data, "HandItems", from_version, to_version);
    }));
}
