use crate::helpers::rename::rename_keys_in_map;
use crate::MinecraftTypes;
use rust_dataconverter_engine::{convert_map_list_in_map, data_walker, map_data_converter_func};
use valence_nbt::{Compound, Value};

const VERSION: u32 = 2501;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    register_furnace(types, "minecraft:furnace");
    register_furnace(types, "minecraft:blast_furnace");
    register_furnace(types, "minecraft:smoker");
}

fn register_furnace<'a>(types: &'a MinecraftTypes<'a>, id: &str) {
    types.tile_entity.borrow_mut().add_converter_for_id(
        id,
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let recipes_used_size = data
                .remove("RecipesUsedSize")
                .and_then(|o| o.as_i32())
                .unwrap_or(0);
            if recipes_used_size <= 0 {
                return;
            }

            let mut recipes_used = Compound::new();
            for i in 0..recipes_used_size {
                if let (Some(Value::String(recipe_key)), Some(recipe_amount)) = (
                    data.get(&format!("RecipeLocation{}", i)),
                    data.get(&format!("RecipeAmount{}", i))
                        .and_then(|v| v.as_i32()),
                ) {
                    if recipe_amount > 0 {
                        recipes_used.insert(recipe_key, recipe_amount);
                    }
                }
            }
            data.insert("RecipesUsed", recipes_used);
        }),
    );

    let item_stack_type = &types.item_stack;
    let recipe_type = &types.recipe;
    types.tile_entity.borrow_mut().add_walker_for_id(
        VERSION,
        id,
        data_walker(move |data, from_version, to_version| {
            convert_map_list_in_map(item_stack_type, data, "Items", from_version, to_version);
            rename_keys_in_map(recipe_type, data, "RecipesUsed", from_version, to_version);
        }),
    );
}
