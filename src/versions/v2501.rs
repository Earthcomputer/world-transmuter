use crate::helpers::rename::rename_keys_in_map;
use crate::types;
use world_transmuter_engine::{
    convert_map_list_in_map, map_data_converter_func, map_data_walker, JCompound, JValue,
};

const VERSION: u32 = 2501;

pub(crate) fn register() {
    register_furnace("minecraft:furnace");
    register_furnace("minecraft:blast_furnace");
    register_furnace("minecraft:smoker");
}

fn register_furnace(id: &str) {
    types::tile_entity_mut().add_converter_for_id(
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

            let mut recipes_used = JCompound::new();
            for i in 0..recipes_used_size {
                if let (Some(JValue::String(recipe_key)), Some(recipe_amount)) = (
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

    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        id,
        map_data_walker(move |data, from_version, to_version| {
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "Items",
                from_version,
                to_version,
            );
            rename_keys_in_map(
                types::recipe_ref(),
                data,
                "RecipesUsed",
                from_version,
                to_version,
            );
        }),
    );
}
