use crate::types;
use java_string::JavaString;
use world_transmuter_engine::{
    convert_map_list_in_map, convert_object_in_map, convert_object_list_in_map,
    convert_values_in_map, data_walker, map_data_converter_func, value_data_converter_func,
    JCompound, JList, JValue, JValueMut,
};

const VERSION: u32 = 2843;

pub(crate) fn register() {
    types::biome_mut().add_structure_converter(
        VERSION,
        value_data_converter_func(|data, _from_version, _to_version| {
            if let JValueMut::String(data) = data {
                if *data == "minecraft:deep_warm_ocean" {
                    **data = JavaString::from("minecraft:warm_ocean");
                }
            }
        }),
    );

    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            fn move_out_of_bound_ticks(
                ticks_key: &str,
                chunk_root: &mut JCompound,
                chunk_x: i32,
                chunk_z: i32,
                into_key: &str,
            ) {
                let Some(JValue::List(JList::Compound(ticks))) = chunk_root.get_mut(ticks_key)
                else {
                    return;
                };

                let mut out_of_bounds = Vec::new();
                for tick in ticks {
                    let x = tick.get("x").and_then(|v| v.as_i32()).unwrap_or(0);
                    let z = tick.get("z").and_then(|v| v.as_i32()).unwrap_or(0);
                    // anything > 1 is lost, anything less than 1 stays.
                    if u32::max(chunk_x.abs_diff(x >> 4), chunk_z.abs_diff(z >> 4)) == 1 {
                        // DFU doesn't remove, so neither do we.
                        out_of_bounds.push(tick.clone());
                    }
                }

                if !out_of_bounds.is_empty() {
                    if !matches!(chunk_root.get("UpgradeData"), Some(JValue::Compound(_))) {
                        chunk_root.insert("UpgradeData", JCompound::new());
                    }
                    let Some(JValue::Compound(upgrade_data)) = chunk_root.get_mut("UpgradeData")
                    else {
                        unreachable!()
                    };
                    upgrade_data.insert(into_key, JList::Compound(out_of_bounds));
                }
            }

            // After renames, so use new names
            let x = data.get("xPos").and_then(|v| v.as_i32()).unwrap_or(0);
            let z = data.get("zPos").and_then(|v| v.as_i32()).unwrap_or(0);

            move_out_of_bound_ticks("block_ticks", data, x, z, "neighbor_block_ticks");
            move_out_of_bound_ticks("fluid_ticks", data, x, z, "neighbor_fluid_ticks");
        }),
    );

    // DFU is missing schema for UpgradeData block names
    types::chunk_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            convert_map_list_in_map(
                types::entity_ref(),
                data,
                "entities",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::tile_entity_ref(),
                data,
                "block_entities",
                from_version,
                to_version,
            );

            if let Some(JValue::List(JList::Compound(block_ticks))) = data.get_mut("block_ticks") {
                for block_tick in block_ticks {
                    convert_object_in_map(
                        types::block_name_ref(),
                        block_tick,
                        "i",
                        from_version,
                        to_version,
                    );
                }
            }

            if let Some(JValue::Compound(upgrade_data)) = data.get_mut("UpgradeData") {
                // Even though UpgradeData will retrieve the block from the World when the type no longer exists,
                // the type from the world may not match what was actually queued. So, even though it may look like we
                // can skip the walker here, we actually don't if we want to be thorough.
                if let Some(JValue::List(JList::Compound(neighbor_block_ticks))) =
                    upgrade_data.get_mut("neighbor_block_ticks")
                {
                    for block_tick in neighbor_block_ticks {
                        convert_object_in_map(
                            types::block_name_ref(),
                            block_tick,
                            "i",
                            from_version,
                            to_version,
                        );
                    }
                }
            }

            if let Some(JValue::List(JList::Compound(sections))) = data.get_mut("sections") {
                for section in sections {
                    if let Some(JValue::Compound(biomes)) = section.get_mut("biomes") {
                        convert_object_list_in_map(
                            types::biome_ref(),
                            biomes,
                            "palette",
                            from_version,
                            to_version,
                        );
                    }
                    if let Some(JValue::Compound(block_states)) = section.get_mut("block_states") {
                        convert_map_list_in_map(
                            types::block_state_ref(),
                            block_states,
                            "palette",
                            from_version,
                            to_version,
                        );
                    }
                }
            }

            if let Some(JValue::Compound(structures)) = data.get_mut("structures") {
                convert_values_in_map(
                    types::structure_feature_ref(),
                    structures,
                    "starts",
                    from_version,
                    to_version,
                );
            }
        }),
    );
}
