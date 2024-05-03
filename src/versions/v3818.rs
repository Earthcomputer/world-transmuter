use crate::helpers::{item_stack_to_data_components_fix, particle_to_nbt_fix};
use crate::{static_string_map, types};
use java_string::JavaStr;
use world_transmuter_engine::{
    convert_map_in_map, convert_map_list_in_map, convert_object_in_map, convert_object_list,
    convert_object_list_in_map, dynamic_data_converter_func, dynamic_data_walker,
    map_data_converter_func, map_data_walker, rename_key, AbstractValueDataType, DataVersion,
    JCompound, JList, JValue, JValueMut,
};

const VERSION: u32 = 3818;

const BANNER_COLORS: [&JavaStr; 16] = [
    JavaStr::from_str("white"),
    JavaStr::from_str("orange"),
    JavaStr::from_str("magenta"),
    JavaStr::from_str("light_blue"),
    JavaStr::from_str("yellow"),
    JavaStr::from_str("lime"),
    JavaStr::from_str("pink"),
    JavaStr::from_str("gray"),
    JavaStr::from_str("light_gray"),
    JavaStr::from_str("cyan"),
    JavaStr::from_str("purple"),
    JavaStr::from_str("blue"),
    JavaStr::from_str("brown"),
    JavaStr::from_str("green"),
    JavaStr::from_str("red"),
    JavaStr::from_str("black"),
];

pub(crate) fn get_banner_color(id: i32) -> &'static JavaStr {
    BANNER_COLORS
        .get(id as usize)
        .copied()
        .unwrap_or_else(|| BANNER_COLORS[0])
}

static_string_map! {
    PATTERN_UPDATE, pattern_update, {
        "b" => "minecraft:base",
        "bl" => "minecraft:square_bottom_left",
        "br" => "minecraft:square_bottom_right",
        "tl" => "minecraft:square_top_left",
        "tr" => "minecraft:square_top_right",
        "bs" => "minecraft:stripe_bottom",
        "ts" => "minecraft:stripe_top",
        "ls" => "minecraft:stripe_left",
        "rs" => "minecraft:stripe_right",
        "cs" => "minecraft:stripe_center",
        "ms" => "minecraft:stripe_middle",
        "drs" => "minecraft:stripe_downright",
        "dls" => "minecraft:stripe_downleft",
        "ss" => "minecraft:small_stripes",
        "cr" => "minecraft:cross",
        "sc" => "minecraft:straight_cross",
        "bt" => "minecraft:triangle_bottom",
        "tt" => "minecraft:triangle_top",
        "bts" => "minecraft:triangles_bottom",
        "tts" => "minecraft:triangles_top",
        "ld" => "minecraft:diagonal_left",
        "rd" => "minecraft:diagonal_up_right",
        "lud" => "minecraft:diagonal_up_left",
        "rud" => "minecraft:diagonal_right",
        "mc" => "minecraft:circle",
        "mr" => "minecraft:rhombus",
        "vh" => "minecraft:half_vertical",
        "hh" => "minecraft:half_horizontal",
        "vhr" => "minecraft:half_vertical_right",
        "hhb" => "minecraft:half_horizontal_bottom",
        "bo" => "minecraft:border",
        "cbo" => "minecraft:curly_border",
        "gra" => "minecraft:gradient",
        "gru" => "minecraft:gradient_up",
        "bri" => "minecraft:bricks",
        "glb" => "minecraft:globe",
        "cre" => "minecraft:creeper",
        "sku" => "minecraft:skull",
        "flo" => "minecraft:flower",
        "moj" => "minecraft:mojang",
        "pig" => "minecraft:piglin",
    }
}

pub(crate) fn register() {
    // Step 0
    types::hotbar_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            for item_list in data.values_mut() {
                if let JValue::List(JList::Compound(item_list)) = item_list {
                    for item in item_list {
                        let id = match item.get("id") {
                            Some(JValue::String(id)) => Some(&id[..]),
                            _ => None,
                        };
                        let count = item.get("Count").and_then(|o| o.as_i32()).unwrap_or(0);
                        if id == Some(JavaStr::from_str("minecraft:air")) || count <= 0 {
                            item.clear();
                        }
                    }
                }
            }
        }),
    );

    types::tile_entity_mut().add_converter_for_id(
        "minecraft:beehive",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            rename_key(data, "Bees", "bees");
            if let Some(JValue::List(JList::Compound(bees))) = data.get_mut("bees") {
                for bee in bees {
                    rename_key(bee, "EntityData", "entity_data");
                    rename_key(bee, "TicksInHive", "ticks_in_hive");
                    rename_key(bee, "MinOccupationTicks", "min_ticks_in_hive");
                }
            }
        }),
    );
    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:beehive",
        map_data_walker(|data, from_version, to_version| {
            if let Some(JValue::List(JList::Compound(bees))) = data.get_mut("bees") {
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
        }),
    );

    // Step 1
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:banner",
        DataVersion::new(VERSION, 1),
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::List(JList::Compound(patterns))) = data.get_mut("Patterns") {
                for pattern in patterns {
                    if let Some(JValue::String(pattern_name)) = pattern.get_mut("Pattern") {
                        if let Some(&new_pattern_name) = pattern_update().get(&pattern_name[..]) {
                            *pattern_name = new_pattern_name.to_owned();
                        }
                    }
                    rename_key(pattern, "Pattern", "pattern");

                    if let Some(banner_color) = pattern.get_mut("Color") {
                        let banner_color_value = banner_color.as_i32().unwrap_or(0);
                        *banner_color =
                            JValue::String(get_banner_color(banner_color_value).to_owned());
                    }
                    rename_key(pattern, "Color", "color");
                }
            }

            rename_key(data, "Patterns", "patterns");
        }),
    );

    // Step 2
    types::entity_mut().add_converter_for_id(
        "minecraft:arrow",
        DataVersion::new(VERSION, 2),
        map_data_converter_func(|data, _from_version, _to_version| {
            let potion = data.remove("Potion");
            let custom_potion_effects = data.remove("custom_potion_effects");
            let color = data.remove("Color");

            if potion.is_none() && custom_potion_effects.is_none() && color.is_none() {
                return;
            }

            let Some(JValue::Compound(item)) = data.get_mut("item") else {
                return;
            };

            let tag = match item.get_mut("tag") {
                Some(JValue::Compound(tag)) => tag,
                _ => {
                    item.insert("tag", JCompound::new());
                    let Some(JValue::Compound(tag)) = item.get_mut("tag") else {
                        unreachable!()
                    };
                    tag
                }
            };

            if let Some(potion) = potion {
                tag.insert("Potion", potion);
            }
            if let Some(custom_potion_effects) = custom_potion_effects {
                tag.insert("custom_potion_effects", custom_potion_effects);
            }
            if let Some(color) = color {
                tag.insert("CustomPotionColor", color);
            }
        }),
    );

    // Step 3
    types::data_components_mut().add_structure_walker(
        DataVersion::new(VERSION, 3),
        map_data_walker(|data, from_version, to_version| {
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
                    }
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
        }),
    );

    // Step 4
    types::particle_mut().add_structure_converter(
        DataVersion::new(VERSION, 4),
        dynamic_data_converter_func(|data, _from_version, _to_version| {
            let JValue::String(flat) = data else {
                return;
            };
            *data = JValue::Compound(particle_to_nbt_fix::convert(flat));
        }),
    );
    types::particle_mut().add_structure_walker(
        DataVersion::new(VERSION, 4),
        dynamic_data_walker(|data, from_version, to_version| {
            let JValue::Compound(data) = data else {
                return;
            };

            convert_map_in_map(
                types::item_stack_ref(),
                data,
                "item",
                from_version,
                to_version,
            );
            convert_map_in_map(
                types::block_state_ref(),
                data,
                "block_state",
                from_version,
                to_version,
            );
        }),
    );

    // Step 5
    // Note: needs breakpoint, reads nested tile entity data
    types::item_stack_mut().add_structure_converter(
        DataVersion::new(VERSION, 5),
        map_data_converter_func(|data, _from_version, _to_version| {
            *data = item_stack_to_data_components_fix::convert_item(std::mem::take(data))
        }),
    );
    types::item_stack_mut().add_structure_walker(
        DataVersion::new(VERSION, 5),
        map_data_walker(|data, from_version, to_version| {
            convert_object_in_map(types::item_name_ref(), data, "id", from_version, to_version);
            convert_map_in_map(
                types::data_components_ref(),
                data,
                "components",
                from_version,
                to_version,
            );
        }),
    );

    // Step 6
    types::entity_mut().add_converter_for_id(
        "minecraft:area_effect_cloud",
        DataVersion::new(VERSION, 6),
        map_data_converter_func(|data, _from_version, _to_version| {
            let color = data.remove("Color");
            let effects = data.remove("effects");
            let potion = data.remove("Potion");

            if color.is_none() && effects.is_none() && potion.is_none() {
                return;
            }

            let mut potion_contents = JCompound::new();
            if let Some(color) = color {
                potion_contents.insert("custom_color", color);
            }
            if let Some(effects) = effects {
                potion_contents.insert("custom_effects", effects);
            }
            if let Some(potion) = potion {
                potion_contents.insert("potion", potion);
            }

            data.insert("potion_contents", potion_contents);
        }),
    );
}
