use crate::types;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{
    convert_map_in_map, convert_map_list_in_map, convert_object_in_map, convert_object_list,
    convert_object_list_in_map, map_data_converter_func, map_data_walker, AbstractValueDataType,
    DataVersion, JCompound, JList, JValue, JValueMut,
};

const VERSION: u32 = 4059;

pub(crate) fn register() {
    // previous version: 3818.3
    types::data_components_mut().add_structure_walker(
        VERSION,
        map_data_walker(|data, from_version, to_version| {
            fn walk_block_predicates(
                predicate: &mut JCompound,
                from_version: DataVersion,
                to_version: DataVersion,
            ) {
                match predicate.get_mut("blocks") {
                    Some(JValue::String(blocks)) => types::block_name().convert(
                        &mut JValueMut::String(blocks),
                        from_version,
                        to_version,
                    ),
                    Some(JValue::List(blocks)) => convert_object_list(
                        types::block_name_ref(),
                        blocks,
                        from_version,
                        to_version,
                    ),
                    _ => {}
                }
            }

            if let Some(JValue::List(JList::Compound(bees))) = data.get_mut("minecraft:bees") {
                for bee in bees {
                    convert_map_in_map(
                        types::entity_ref(),
                        bee,
                        "entity_data",
                        from_version,
                        to_version,
                    );
                }
            }

            convert_map_in_map(
                types::tile_entity_ref(),
                data,
                "minecraft:block_entity_data",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "minecraft:bundle_contents",
                from_version,
                to_version,
            );

            for component_name in ["minecraft:can_break", "minecraft:can_place_on"] {
                if let Some(JValue::Compound(component)) = data.get_mut(component_name) {
                    if let Some(JValue::List(JList::Compound(predicates))) =
                        component.get_mut("predicates")
                    {
                        for predicate in predicates {
                            walk_block_predicates(predicate, from_version, to_version);
                        }
                    }
                    walk_block_predicates(component, from_version, to_version);
                }
            }

            convert_map_list_in_map(
                types::item_stack_ref(),
                data,
                "minecraft:charged_projectiles",
                from_version,
                to_version,
            );
            if let Some(JValue::List(JList::Compound(container))) =
                data.get_mut("minecraft:container")
            {
                for slot in container {
                    convert_map_in_map(
                        types::item_stack_ref(),
                        slot,
                        "item",
                        from_version,
                        to_version,
                    );
                }
            }
            convert_map_in_map(
                types::entity_ref(),
                data,
                "minecraft:entity_data",
                from_version,
                to_version,
            );
            convert_object_list_in_map(
                types::item_name_ref(),
                data,
                "minecraft:pot_decorations",
                from_version,
                to_version,
            );
            convert_map_in_map(
                types::item_stack_ref(),
                data,
                "minecraft:use_remainder",
                from_version,
                to_version,
            );

            if let Some(JValue::Compound(equippable)) = data.get_mut("minecraft:equippable") {
                convert_object_in_map(
                    types::entity_name_ref(),
                    equippable,
                    "allowed_entities",
                    from_version,
                    to_version,
                );
                convert_object_list_in_map(
                    types::item_name_ref(),
                    equippable,
                    "allowed_entities",
                    from_version,
                    to_version,
                );
            }
        }),
    );

    types::data_components_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(food)) = data.get_mut("minecraft:food") else {
                return;
            };

            let eat_seconds = food
                .remove("eat_seconds")
                .and_then(|v| v.as_f32())
                .unwrap_or(1.6);

            let old_effects = food
                .remove("effects")
                .and_then(|v| {
                    if let JValue::List(JList::Compound(effects)) = v {
                        Some(effects)
                    } else {
                        None
                    }
                })
                .unwrap_or_default();
            let new_effects: Vec<_> = old_effects
                .into_iter()
                .map(|mut old_effect| {
                    let mut new_effect_effects = JList::new();
                    if let Some(old_effect_effect) = old_effect.remove("effect") {
                        new_effect_effects = JList::from(old_effect_effect);
                    }
                    let probability = old_effect
                        .get("probability")
                        .and_then(|v| v.as_f32())
                        .unwrap_or(1.0);
                    jcompound! {
                        "type" => "minecraft:apply_effects",
                        "effects" => new_effect_effects,
                        "probability" => probability,
                    }
                })
                .collect();

            if let Some(converts_to) = food.remove("using_converts_to") {
                data.insert("minecraft:use_remainder", converts_to);
            }

            data.insert(
                "minecraft:consumable",
                jcompound! {
                    "consume_seconds" => eat_seconds,
                    "on_consume_effects" => JList::from(new_effects),
                },
            );
        }),
    );
}
