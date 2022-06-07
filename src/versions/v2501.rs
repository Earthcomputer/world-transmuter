use rust_dataconverter_engine::{convert_map_list_in_map, data_converter_func, data_walker, MapType, ObjectType, Types};
use crate::helpers::rename::rename_keys_in_map;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2501;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    register_furnace(types, "minecraft:furnace");
    register_furnace(types, "minecraft:blast_furnace");
    register_furnace(types, "minecraft:smoker");
}

fn register_furnace<T: Types + ?Sized>(types: &MinecraftTypesMut<T>, id: &str) {
    types.tile_entity.borrow_mut().add_converter_for_id(id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let recipes_used_size = data.remove("RecipesUsedSize").and_then(|o| o.as_i64()).unwrap_or(0) as i32;
        if recipes_used_size <= 0 {
            return;
        }

        let mut recipes_used = T::Map::create_empty();
        for i in 0..recipes_used_size {
            if let (Some(recipe_key), Some(recipe_amount)) = (data.get_string(&format!("RecipeLocation{}", i)), data.get_i64(&format!("RecipeAmount{}", i)).map(|i| i as i32)) {
                if recipe_amount > 0 {
                    recipes_used.set(recipe_key, T::Object::create_int(recipe_amount));
                }
            }
        }
        data.set("RecipesUsed", T::Object::create_map(recipes_used));
    }));

    let item_stack_type = types.item_stack;
    let recipe_type = types.recipe;
    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, id, data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        convert_map_list_in_map::<_, T>(item_stack_type, data, "Items", from_version, to_version);
        rename_keys_in_map::<T>(recipe_type, data, "RecipesUsed", from_version, to_version);
    }));
}
