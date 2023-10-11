use crate::helpers::bit_storage::{
    ceil_log2, AlignedBitStorage, LocalPos, NullSectionInitializer, Section,
};
use crate::versions::v2841;
use crate::{static_string_mc_set, static_string_set, types};
use ahash::{AHashMap, AHashSet};
use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use java_string::{JavaStr, JavaString};
use log::error;
use std::collections::BTreeSet;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{
    convert_map_list_in_map, convert_object_in_map, convert_object_list_in_map,
    convert_values_in_map, data_walker, get_mut_multi, map_data_converter_func, JCompound, JList,
    JValue,
};

const VERSION: u32 = 2832;

const BIOMES_BY_ID: [Option<&JavaStr>; 183] = {
    let mut biomes_by_id = [None; 183];
    biomes_by_id[0] = Some(JavaStr::from_str("minecraft:ocean"));
    biomes_by_id[1] = Some(JavaStr::from_str("minecraft:plains"));
    biomes_by_id[2] = Some(JavaStr::from_str("minecraft:desert"));
    biomes_by_id[3] = Some(JavaStr::from_str("minecraft:mountains"));
    biomes_by_id[4] = Some(JavaStr::from_str("minecraft:forest"));
    biomes_by_id[5] = Some(JavaStr::from_str("minecraft:taiga"));
    biomes_by_id[6] = Some(JavaStr::from_str("minecraft:swamp"));
    biomes_by_id[7] = Some(JavaStr::from_str("minecraft:river"));
    biomes_by_id[8] = Some(JavaStr::from_str("minecraft:nether_wastes"));
    biomes_by_id[9] = Some(JavaStr::from_str("minecraft:the_end"));
    biomes_by_id[10] = Some(JavaStr::from_str("minecraft:frozen_ocean"));
    biomes_by_id[11] = Some(JavaStr::from_str("minecraft:frozen_river"));
    biomes_by_id[12] = Some(JavaStr::from_str("minecraft:snowy_tundra"));
    biomes_by_id[13] = Some(JavaStr::from_str("minecraft:snowy_mountains"));
    biomes_by_id[14] = Some(JavaStr::from_str("minecraft:mushroom_fields"));
    biomes_by_id[15] = Some(JavaStr::from_str("minecraft:mushroom_field_shore"));
    biomes_by_id[16] = Some(JavaStr::from_str("minecraft:beach"));
    biomes_by_id[17] = Some(JavaStr::from_str("minecraft:desert_hills"));
    biomes_by_id[18] = Some(JavaStr::from_str("minecraft:wooded_hills"));
    biomes_by_id[19] = Some(JavaStr::from_str("minecraft:taiga_hills"));
    biomes_by_id[20] = Some(JavaStr::from_str("minecraft:mountain_edge"));
    biomes_by_id[21] = Some(JavaStr::from_str("minecraft:jungle"));
    biomes_by_id[22] = Some(JavaStr::from_str("minecraft:jungle_hills"));
    biomes_by_id[23] = Some(JavaStr::from_str("minecraft:jungle_edge"));
    biomes_by_id[24] = Some(JavaStr::from_str("minecraft:deep_ocean"));
    biomes_by_id[25] = Some(JavaStr::from_str("minecraft:stone_shore"));
    biomes_by_id[26] = Some(JavaStr::from_str("minecraft:snowy_beach"));
    biomes_by_id[27] = Some(JavaStr::from_str("minecraft:birch_forest"));
    biomes_by_id[28] = Some(JavaStr::from_str("minecraft:birch_forest_hills"));
    biomes_by_id[29] = Some(JavaStr::from_str("minecraft:dark_forest"));
    biomes_by_id[30] = Some(JavaStr::from_str("minecraft:snowy_taiga"));
    biomes_by_id[31] = Some(JavaStr::from_str("minecraft:snowy_taiga_hills"));
    biomes_by_id[32] = Some(JavaStr::from_str("minecraft:giant_tree_taiga"));
    biomes_by_id[33] = Some(JavaStr::from_str("minecraft:giant_tree_taiga_hills"));
    biomes_by_id[34] = Some(JavaStr::from_str("minecraft:wooded_mountains"));
    biomes_by_id[35] = Some(JavaStr::from_str("minecraft:savanna"));
    biomes_by_id[36] = Some(JavaStr::from_str("minecraft:savanna_plateau"));
    biomes_by_id[37] = Some(JavaStr::from_str("minecraft:badlands"));
    biomes_by_id[38] = Some(JavaStr::from_str("minecraft:wooded_badlands_plateau"));
    biomes_by_id[39] = Some(JavaStr::from_str("minecraft:badlands_plateau"));
    biomes_by_id[40] = Some(JavaStr::from_str("minecraft:small_end_islands"));
    biomes_by_id[41] = Some(JavaStr::from_str("minecraft:end_midlands"));
    biomes_by_id[42] = Some(JavaStr::from_str("minecraft:end_highlands"));
    biomes_by_id[43] = Some(JavaStr::from_str("minecraft:end_barrens"));
    biomes_by_id[44] = Some(JavaStr::from_str("minecraft:warm_ocean"));
    biomes_by_id[45] = Some(JavaStr::from_str("minecraft:lukewarm_ocean"));
    biomes_by_id[46] = Some(JavaStr::from_str("minecraft:cold_ocean"));
    biomes_by_id[47] = Some(JavaStr::from_str("minecraft:deep_warm_ocean"));
    biomes_by_id[48] = Some(JavaStr::from_str("minecraft:deep_lukewarm_ocean"));
    biomes_by_id[49] = Some(JavaStr::from_str("minecraft:deep_cold_ocean"));
    biomes_by_id[50] = Some(JavaStr::from_str("minecraft:deep_frozen_ocean"));
    biomes_by_id[127] = Some(JavaStr::from_str("minecraft:the_void"));
    biomes_by_id[129] = Some(JavaStr::from_str("minecraft:sunflower_plains"));
    biomes_by_id[130] = Some(JavaStr::from_str("minecraft:desert_lakes"));
    biomes_by_id[131] = Some(JavaStr::from_str("minecraft:gravelly_mountains"));
    biomes_by_id[132] = Some(JavaStr::from_str("minecraft:flower_forest"));
    biomes_by_id[133] = Some(JavaStr::from_str("minecraft:taiga_mountains"));
    biomes_by_id[134] = Some(JavaStr::from_str("minecraft:swamp_hills"));
    biomes_by_id[140] = Some(JavaStr::from_str("minecraft:ice_spikes"));
    biomes_by_id[149] = Some(JavaStr::from_str("minecraft:modified_jungle"));
    biomes_by_id[151] = Some(JavaStr::from_str("minecraft:modified_jungle_edge"));
    biomes_by_id[155] = Some(JavaStr::from_str("minecraft:tall_birch_forest"));
    biomes_by_id[156] = Some(JavaStr::from_str("minecraft:tall_birch_hills"));
    biomes_by_id[157] = Some(JavaStr::from_str("minecraft:dark_forest_hills"));
    biomes_by_id[158] = Some(JavaStr::from_str("minecraft:snowy_taiga_mountains"));
    biomes_by_id[160] = Some(JavaStr::from_str("minecraft:giant_spruce_taiga"));
    biomes_by_id[161] = Some(JavaStr::from_str("minecraft:giant_spruce_taiga_hills"));
    biomes_by_id[162] = Some(JavaStr::from_str("minecraft:modified_gravelly_mountains"));
    biomes_by_id[163] = Some(JavaStr::from_str("minecraft:shattered_savanna"));
    biomes_by_id[164] = Some(JavaStr::from_str("minecraft:shattered_savanna_plateau"));
    biomes_by_id[165] = Some(JavaStr::from_str("minecraft:eroded_badlands"));
    biomes_by_id[166] = Some(JavaStr::from_str(
        "minecraft:modified_wooded_badlands_plateau",
    ));
    biomes_by_id[167] = Some(JavaStr::from_str("minecraft:modified_badlands_plateau"));
    biomes_by_id[168] = Some(JavaStr::from_str("minecraft:bamboo_jungle"));
    biomes_by_id[169] = Some(JavaStr::from_str("minecraft:bamboo_jungle_hills"));
    biomes_by_id[170] = Some(JavaStr::from_str("minecraft:soul_sand_valley"));
    biomes_by_id[171] = Some(JavaStr::from_str("minecraft:crimson_forest"));
    biomes_by_id[172] = Some(JavaStr::from_str("minecraft:warped_forest"));
    biomes_by_id[173] = Some(JavaStr::from_str("minecraft:basalt_deltas"));
    biomes_by_id[174] = Some(JavaStr::from_str("minecraft:dripstone_caves"));
    biomes_by_id[175] = Some(JavaStr::from_str("minecraft:lush_caves"));
    biomes_by_id[177] = Some(JavaStr::from_str("minecraft:meadow"));
    biomes_by_id[178] = Some(JavaStr::from_str("minecraft:grove"));
    biomes_by_id[179] = Some(JavaStr::from_str("minecraft:snowy_slopes"));
    biomes_by_id[180] = Some(JavaStr::from_str("minecraft:snowcapped_peaks"));
    biomes_by_id[181] = Some(JavaStr::from_str("minecraft:lofty_peaks"));
    biomes_by_id[182] = Some(JavaStr::from_str("minecraft:stony_peaks"));
    biomes_by_id
};

fn get_biome_by_id(id: i32) -> &'static JavaStr {
    if (0..BIOMES_BY_ID.len() as i32).contains(&id) {
        if let Some(biome) = BIOMES_BY_ID[id as usize] {
            return biome;
        }
    }
    JavaStr::from_str("minecraft:plains")
}

const HEIGHTMAP_TYPES: [&str; 7] = [
    "WORLD_SURFACE_WG",
    "WORLD_SURFACE",
    "WORLD_SURFACE_IGNORE_SNOW",
    "OCEAN_FLOOR_WG",
    "OCEAN_FLOOR",
    "MOTION_BLOCKING",
    "MOTION_BLOCKING_NO_LEAVES",
];

static_string_set! {
    STATUS_IS_OR_AFTER_SURFACE, status_is_or_after_surface, {
        "surface",
        "carvers",
        "liquid_carvers",
        "features",
        "light",
        "spawn",
        "heightmaps",
        "full",
    }
}

static_string_set! {
    STATUS_IS_OR_AFTER_NOISE, status_is_or_after_noise, {
        "noise",
        "surface",
        "carvers",
        "liquid_carvers",
        "features",
        "light",
        "spawn",
        "heightmaps",
        "full",
    }
}

static_string_mc_set! {
    BLOCKS_BEFORE_FEATURE_STATUS, blocks_before_feature_status, {
        "air",
        "basalt",
        "bedrock",
        "blackstone",
        "calcite",
        "cave_air",
        "coarse_dirt",
        "crimson_nylium",
        "dirt",
        "end_stone",
        "grass_block",
        "gravel",
        "ice",
        "lava",
        "mycelium",
        "nether_wart_block",
        "netherrack",
        "orange_terracotta",
        "packed_ice",
        "podzol",
        "powder_snow",
        "red_sand",
        "red_sandstone",
        "sand",
        "sandstone",
        "snow_block",
        "soul_sand",
        "soul_soil",
        "stone",
        "terracotta",
        "warped_nylium",
        "warped_wart_block",
        "water",
        "white_terracotta",
    }
}

fn get_objects_per_value(val: &[i64]) -> u8 {
    4096usize.div_ceil(val.len()) as u8
}

fn resize(
    val: &[i64],
    old_bits_per_object: u8,
    new_bits_per_object: u8,
) -> Result<Vec<i64>, String> {
    let old_mask = (1u64 << (old_bits_per_object & 63)) - 1;
    let new_mask = (1u64 << (new_bits_per_object & 63)) - 1;
    let old_objects_per_value = 64 / old_bits_per_object;
    let new_objects_per_value = 64 / new_bits_per_object;

    let items = 4096;
    let mut ret = Vec::with_capacity(
        (items + new_objects_per_value as usize - 1) / new_objects_per_value as usize,
    );

    let expected_size =
        (items + old_objects_per_value as usize - 1) / old_objects_per_value as usize;
    if val.len() != expected_size {
        return Err(format!(
            "Expected size: {}, got: {}",
            expected_size,
            val.len()
        ));
    }

    let mut shift = 0;
    let mut new_curr = 0;

    let mut curr_item = 0;
    for &old_curr in val {
        let mut obj_idx = 0;
        while curr_item < items && obj_idx + old_bits_per_object <= 64 {
            let value = (old_curr >> obj_idx) as u64 & old_mask;

            if (value & new_mask) != value {
                return Err("Old data storage has values that cannot be moved into new palette (would erase data)!".to_owned());
            }

            new_curr |= value << shift;
            shift += new_bits_per_object;

            if shift + new_bits_per_object > 64 {
                // will next write overflow?
                // must move to next idx
                ret.push(new_curr as i64);
                shift = 0;
                new_curr = 0;
            }

            obj_idx += old_bits_per_object;
            curr_item += 1;
        }
    }

    // don't forget to write the last one
    if shift != 0 {
        ret.push(new_curr as i64);
    }

    assert_eq!(
        ret.len(),
        (items + new_objects_per_value as usize - 1) / new_objects_per_value as usize
    );

    Ok(ret)
}

fn fix_lithium_chunks(data: &mut JCompound) {
    // See https://github.com/CaffeineMC/lithium-fabric/issues/279
    let Some(JValue::Compound(level)) = data.get_mut("Level") else {
        return;
    };

    let chunk_x = level.get("xPos").and_then(|v| v.as_i32()).unwrap_or(0);
    let chunk_z = level.get("zPos").and_then(|v| v.as_i32()).unwrap_or(0);

    let Some(JValue::List(JList::Compound(sections))) = level.get_mut("Sections") else {
        return;
    };
    for section in sections {
        let section_y = section.get("Y").and_then(|v| v.as_i32()).unwrap_or(0);

        let [Some(JValue::List(JList::Compound(palette))), Some(JValue::LongArray(block_states))] =
            get_mut_multi(section, ["Palette", "BlockStates"])
        else {
            continue;
        };

        let expected_bits = 4.max(ceil_log2(palette.len() as u32));
        let got_objects_per_value = get_objects_per_value(block_states);
        let got_bits = 64 / got_objects_per_value;

        if expected_bits == got_bits {
            continue;
        }

        match resize(block_states, got_bits, expected_bits) {
            Ok(new_block_states) => *block_states = new_block_states,
            Err(message) => error!(
                "Failed to rewrite mismatched palette and data storage for section y: {} \
                for chunk [{},{}], palette entries: {}, data storage size: {}: {}",
                section_y,
                chunk_x,
                chunk_z,
                palette.len(),
                block_states.len(),
                message
            ),
        }
    }
}

pub(crate) fn register() {
    // See V2551 for the layout of world gen settings
    types::world_gen_settings_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        // converters were added to older versions note whether the world has increased height already or not
        let no_height_flag = !data.contains_key("has_increased_height_already");
        let has_increased_height = data.remove("has_increased_height_already").and_then(|v| v.as_bool()).unwrap_or(true);

        let Some(JValue::Compound(dimensions)) = data.get_mut("dimensions") else { return };

        // only care about overworld
        let Some(JValue::Compound(overworld)) = dimensions.get_mut("minecraft:overworld") else { return };

        let Some(JValue::Compound(generator)) = overworld.get_mut("generator") else { return };

        match generator.get("type") {
            Some(JValue::String(str)) if str == "minecraft:noise" => {
                let Some(JValue::Compound(biome_source)) = generator.get("biome_source") else { return };

                let mut large_biomes = false;

                if let Some(JValue::String(source_type)) = biome_source.get("type") {
                    if source_type == "minecraft:vanilla_layered" || (no_height_flag && source_type == "minecraft:multi_noise") {
                        large_biomes = biome_source.get("large_biomes").and_then(|v| v.as_bool()).unwrap_or(false);

                        let new_biome_source = jcompound! {
                            "preset" => "minecraft:overworld",
                            "type" => "minecraft:multi_noise",
                        };
                        generator.insert("biome_source", new_biome_source);
                    }
                }

                if large_biomes && matches!(generator.get("settings"), Some(JValue::String(str)) if str == "minecraft:overworld") {
                    generator.insert("settings", "minecraft:large_biomes");
                }
            }
            Some(JValue::String(str)) if str == "minecraft:flat" => {
                if !has_increased_height {
                    let Some(JValue::Compound(settings)) = generator.get_mut("settings") else { return };
                    let Some(JValue::List(layers)) = settings.get_mut("layers") else { return };
                    if !matches!(layers, JList::End | JList::Compound(_)) {
                        return;
                    }
                    update_layers(layers);
                }
            }
            _ => {
                // do nothing
            }
        }
    }));

    // It looks like DFU will only support worlds in the old height format or the new one, any custom one isn't supported
    // and by not supported I mean it will just treat it as the old format... maybe at least throw in that case?
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            // The below covers padPaletteEntries - this was written BEFORE that code was added to the datafixer -
            // and this still works, so I'm keeping it. Don't fix what isn't broken.
            fix_lithium_chunks(data); // See https://github.com/CaffeineMC/lithium-fabric/issues/279

            let [Some(JValue::Compound(level)), context] =
                get_mut_multi(data, ["Level", "__context"])
            else {
                return;
            };

            let (dimension, generator) = match context {
                Some(JValue::Compound(context)) => (
                    match context.get("dimension") {
                        Some(JValue::String(dimension)) => &dimension[..],
                        _ => JavaStr::from_str(""),
                    },
                    match context.get("generator") {
                        Some(JValue::String(generator)) => &generator[..],
                        _ => JavaStr::from_str(""),
                    },
                ),
                _ => (JavaStr::from_str(""), JavaStr::from_str("")),
            };
            let is_overworld = dimension == "minecraft:overworld";
            let min_section = if is_overworld { -4 } else { 0 };
            let mut is_already_extended = false;

            let mut new_biomes =
                create_biome_sections(level, is_overworld, &mut is_already_extended);
            let wrapped_empty_block_palette = get_empty_block_palette();

            if !matches!(
                level.get("Sections"),
                Some(JValue::List(JList::Compound(_)))
            ) {
                level.insert("Sections", JList::new());
            }
            let Some(JValue::List(sections)) = level.get_mut("Sections") else {
                unreachable!()
            };

            // must update sections for two things:
            // 1. the biomes are now stored per section, so we must insert the biomes palette into each section (and create them if they don't exist)
            // 2. each section must now have block states (or at least DFU is ensuring they do, but current code does not require)
            let mut bottom_section_idx = None;
            let mut existing_sections = AHashSet::new();
            let mut all_blocks = BTreeSet::new();

            if let JList::Compound(sections) = sections {
                for (idx, section) in sections.iter_mut().enumerate() {
                    let y = section.get("Y").and_then(|v| v.as_i32()).unwrap_or(0);
                    let section_index = y - min_section;

                    existing_sections.insert(y);

                    // add in relevant biome section
                    if (0..new_biomes.len() as i32).contains(&section_index) {
                        // exclude out of bounds sections (i.e the light sections above and below the world)
                        section.insert(
                            "biomes",
                            std::mem::take(&mut new_biomes[section_index as usize]),
                        );
                    }

                    // update palette
                    let palette = match section.remove("Palette") {
                        Some(JValue::List(JList::Compound(palette))) => Some(palette),
                        _ => None,
                    };
                    let block_states = match section.remove("BlockStates") {
                        Some(JValue::LongArray(block_states)) => Some(block_states),
                        _ => None,
                    };

                    if let Some(palette) = &palette {
                        for block in palette {
                            all_blocks.insert(v2841::get_block_id(Some(block)).to_owned());
                        }
                    }

                    if let (Some(palette), Some(block_states)) = (palette, block_states) {
                        section.insert(
                            "block_states",
                            wrap_palette_optimized(palette, block_states),
                        );
                    } else {
                        section.insert("block_states", wrapped_empty_block_palette.clone());
                    }

                    if section.get("Y").and_then(|v| v.as_i32()) == Some(0) {
                        bottom_section_idx = Some(idx);
                    }
                }
            }

            // all existing sections updated, now we must create new sections just for the biomes migration
            for (section_index, new_biomes) in new_biomes.into_iter().enumerate() {
                let section_y = section_index as i32 + min_section;
                if !existing_sections.insert(section_y) {
                    // exists already
                    continue;
                }

                let new_section = jcompound! {
                    "Y" => section_y as i8,
                    "block_states" => wrapped_empty_block_palette.clone(),
                    "biomes" => new_biomes,
                };

                let _ = sections.try_push(new_section);
            }

            // update status so interpolation can take place
            predict_chunk_status_before_surface(level, all_blocks);

            // done with sections, update the rest of the chunk
            upgrade_chunk_data(
                level,
                is_overworld,
                is_already_extended,
                generator == "minecraft:noise",
                bottom_section_idx,
            );
        }),
    );

    types::world_gen_settings_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            let Some(JValue::Compound(dimensions)) = data.get_mut("dimensions") else {
                return;
            };
            for dimension_data in dimensions.values_mut() {
                let JValue::Compound(dimension_data) = dimension_data else {
                    continue;
                };
                let Some(JValue::Compound(generator)) = dimension_data.get_mut("generator") else {
                    continue;
                };
                let Some(JValue::String(typ)) = generator.get("type") else {
                    continue;
                };

                match typ.as_bytes() {
                    b"minecraft:flat" => {
                        let Some(JValue::Compound(settings)) = generator.get_mut("settings") else {
                            continue;
                        };

                        convert_object_in_map(
                            types::biome_ref(),
                            settings,
                            "biome",
                            from_version,
                            to_version,
                        );

                        if let Some(JValue::List(JList::Compound(layers))) =
                            settings.get_mut("layers")
                        {
                            for layer in layers {
                                convert_object_in_map(
                                    types::block_name_ref(),
                                    layer,
                                    "block",
                                    from_version,
                                    to_version,
                                );
                            }
                        }
                    }
                    b"minecraft:noise" => {
                        if let Some(JValue::Compound(settings)) = generator.get_mut("settings") {
                            convert_object_in_map(
                                types::block_name_ref(),
                                settings,
                                "default_block",
                                from_version,
                                to_version,
                            );
                            convert_object_in_map(
                                types::block_name_ref(),
                                settings,
                                "default_fluid",
                                from_version,
                                to_version,
                            );
                        }

                        if let Some(JValue::Compound(biome_source)) =
                            generator.get_mut("biome_source")
                        {
                            if let Some(JValue::String(biome_source_type)) =
                                biome_source.get("type")
                            {
                                match biome_source_type.as_bytes() {
                                    b"minecraft:fixed" => {
                                        convert_object_in_map(
                                            types::biome_ref(),
                                            biome_source,
                                            "biome",
                                            from_version,
                                            to_version,
                                        );
                                    }
                                    b"minecraft:multi_noise" => {
                                        convert_object_in_map(
                                            types::multi_noise_biome_source_parameter_list_ref(),
                                            biome_source,
                                            "preset",
                                            from_version,
                                            to_version,
                                        );

                                        // Vanilla's schema is _still_ wrong. It should be DSL.fields("biomes", DSL.list(DSL.fields("biome")))
                                        // But it just contains the list part. That obviously can never be the case, because
                                        // the root object is a compound, not a list.

                                        if let Some(JValue::List(JList::Compound(biomes))) =
                                            biome_source.get_mut("biomes")
                                        {
                                            for biome in biomes {
                                                convert_object_in_map(
                                                    types::biome_ref(),
                                                    biome,
                                                    "biome",
                                                    from_version,
                                                    to_version,
                                                );
                                            }
                                        }
                                    }
                                    b"minecraft:checkerboard" => {
                                        convert_object_list_in_map(
                                            types::biome_ref(),
                                            biome_source,
                                            "biomes",
                                            from_version,
                                            to_version,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }),
    );

    types::chunk_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            let Some(JValue::Compound(level)) = data.get_mut("Level") else {
                return;
            };

            convert_map_list_in_map(
                types::entity_ref(),
                level,
                "Entities",
                from_version,
                to_version,
            );
            convert_map_list_in_map(
                types::tile_entity_ref(),
                level,
                "TileEntities",
                from_version,
                to_version,
            );

            if let Some(JValue::List(JList::Compound(tile_ticks))) = level.get_mut("TileTicks") {
                for tile_tick in tile_ticks {
                    convert_object_in_map(
                        types::block_name_ref(),
                        tile_tick,
                        "i",
                        from_version,
                        to_version,
                    );
                }
            }

            if let Some(JValue::List(JList::Compound(sections))) = level.get_mut("Sections") {
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

            if let Some(JValue::Compound(structures)) = level.get_mut("Structures") {
                convert_values_in_map(
                    types::structure_feature_ref(),
                    structures,
                    "Starts",
                    from_version,
                    to_version,
                );
            }
        }),
    );
}

fn predict_chunk_status_before_surface(
    level: &mut JCompound,
    mut chunk_blocks: BTreeSet<JavaString>,
) {
    let status = match level.get("Status") {
        Some(JValue::String(status)) => &status[..],
        _ => JavaStr::from_str("empty"),
    };
    if status_is_or_after_surface().contains(status) {
        return;
    }

    chunk_blocks.remove(JavaStr::from_str("minecraft:air"));
    let chunk_not_empty = !chunk_blocks.is_empty();
    let chunk_feature_status = chunk_blocks
        .into_iter()
        .any(|block| !blocks_before_feature_status().contains(&block[..]));

    let update = if chunk_feature_status {
        JavaString::from("liquid_carvers")
    } else if status != "noise" && !chunk_not_empty {
        if status == "biomes" {
            JavaString::from("structure_references")
        } else {
            status.to_owned()
        }
    } else {
        JavaString::from("noise")
    };

    level.insert("Status", update);
}

fn get_empty_block_palette() -> JCompound {
    wrap_palette(
        JList::Compound(vec![jcompound! {
            "Name" => "minecraft:air",
        }]),
        None,
    )
}

fn shift_upgrade_data(upgrade_data: Option<&mut JValue>, shift: i32) {
    let Some(JValue::Compound(upgrade_data)) = upgrade_data else {
        return;
    };
    let Some(JValue::Compound(indices)) = upgrade_data.get_mut("Indices") else {
        return;
    };

    world_transmuter_engine::rename_keys(indices, |input| {
        input
            .parse::<i32>()
            .ok()
            .map(|i| JavaString::from((i + shift).to_string()))
    });
}

fn upgrade_chunk_data(
    level: &mut JCompound,
    want_extended_height: bool,
    is_already_extended: bool,
    on_noise_generator: bool,
    bottom_section_idx: Option<usize>,
) {
    level.remove("Biomes");
    if !want_extended_height {
        pad_carving_masks(level, 16, 0);
        return;
    }

    if is_already_extended {
        pad_carving_masks(level, 24, 0);
        return;
    }

    offset_heightmaps(level);
    // Difference from DFU: Still convert the Lights data. Just because it's being removed in a later version doesn't mean
    // that it should be removed here.
    // Generally, converters act only on the current version to bring it to the next. This principle allows the converter
    // for the next version to assume that it acts on its current version, not some in-between of the current version
    // and some future version that did not exist at the time it was written. This allows converters to be written and tested
    // only with knowledge of the current version and the next version.
    add_empty_list_padding(level, "Lights");
    add_empty_list_padding(level, "LiquidsToBeTicked");
    add_empty_list_padding(level, "PostProcessing");
    add_empty_list_padding(level, "ToBeTicked");
    shift_upgrade_data(level.get_mut("UpgradeData"), 4); // https://bugs.mojang.com/browse/MC-238076 - fixed now, Mojang fix is identical. No change required.
    pad_carving_masks(level, 24, 4);

    if !on_noise_generator {
        return;
    }

    let Some(JValue::String(status)) = level.get("Status") else {
        return;
    };
    if status == "empty" {
        return;
    }
    let status = status.clone();

    let old_noise = status_is_or_after_noise().contains(&status[..]);
    level.insert(
        "blending_data",
        jcompound! {
            "old_noise" => old_noise,
        },
    );

    let Some(bottom_section_idx) = bottom_section_idx else {
        return;
    };
    let Some(JValue::List(JList::Compound(sections))) = level.get("Sections") else {
        unreachable!("bottom_section_idx can only get a value if there is a sections list and it is non-empty")
    };
    let bottom_section = &sections[bottom_section_idx];

    let chunk_x = level.get("xPos").and_then(|v| v.as_i32()).unwrap_or(0);
    let chunk_z = level.get("zPos").and_then(|v| v.as_i32()).unwrap_or(0);
    let Some(bottom_section) = Section::<AlignedBitStorage<&[i64]>>::wrap_2832(
        chunk_x,
        chunk_z,
        bottom_section,
        &mut NullSectionInitializer,
    ) else {
        return;
    };

    let mut missing_bedrock: BitArray<_, Lsb0> = BitArray::new([0u64; 4]);
    let mut has_bedrock = status == "noise";
    for z in 0..=15 {
        for x in 0..=15 {
            let Some(block) = bottom_section.get_block(LocalPos::new(x, 0, z)) else {
                continue;
            };
            if block.name == "minecraft:air" {
                missing_bedrock.set(((z << 4) | x) as usize, true);
            }
            has_bedrock |= block.name == "minecraft:bedrock";
        }
    }

    let missing_bedrock = missing_bedrock.into_inner();
    if has_bedrock && missing_bedrock != [u64::MAX; 4] {
        let target_status = if status == "full" {
            JavaString::from("heightmaps")
        } else {
            status
        };
        level.insert(
            "below_zero_retrogen",
            jcompound! {
                "target_status" => target_status,
                "missing_bedrock" => missing_bedrock.map(|i| i as i64).to_vec(),
            },
        );
        level.insert("Status", "empty");
    }

    level.insert("isLightOn", false);
}

fn pad_carving_masks(level: &mut JCompound, new_size: usize, offset: usize) {
    let Some(JValue::Compound(carving_masks)) = level.get_mut("CarvingMasks") else {
        // if empty, DFU still writes
        level.insert("CarvingMasks", JCompound::new());
        return;
    };

    for carving_mask in carving_masks.values_mut() {
        let JValue::ByteArray(old) = carving_mask else {
            continue;
        };
        let mut new_val = vec![0; 64 * new_size];
        i8_slice_to_i64_le(
            &old[..((64 * new_size - 64 * offset) * 8).min(old.len())],
            &mut new_val[..],
            64 * offset,
        );
        *carving_mask = JValue::LongArray(new_val);
    }
}

fn i8_slice_to_i64_le(i8_slice: &[i8], i64_slice: &mut [i64], offset: usize) {
    if i8_slice.is_empty() {
        return;
    }

    let u8_slice =
        unsafe { std::slice::from_raw_parts(i8_slice.as_ptr() as *const u8, i8_slice.len()) };
    let rem = u8_slice.len() % 8;

    // pre-access the final index in the dest slice that we will be using, to panic if the index is out of bounds
    let _ = i64_slice[offset + u8_slice.len() / 8 - (rem == 0) as usize];

    if rem != 0 {
        let mut shift = 0;
        let mut n = 0;
        for &i in &u8_slice[u8_slice.len() - rem..] {
            n |= (i as i64) << shift;
            shift += 8;
        }
        unsafe {
            // SAFETY: this is the final index of the slice that we're using, we checked it above
            *i64_slice.get_unchecked_mut(offset + u8_slice.len() / 8) = n;
        }
    }

    for i in 0..u8_slice.len() / 8 {
        unsafe {
            // SAFETY: i < u8_slice.len() / 8
            // therefore i + 1 <= u8_slice.len() / 8
            // therefore 8*i + 8 <= u8_slice.len()
            let sub_slice = std::slice::from_raw_parts(u8_slice.as_ptr().add(i * 8), 8);
            // SAFETY: i < u8_slice.len()
            // therefore offset + i < offset + u8_slice.len()
            // therefore offset + i <= offset + u8_slice.len() - 1, the minimum possible index checked above
            *i64_slice.get_unchecked_mut(offset + i) =
                i64::from_le_bytes(sub_slice.try_into().unwrap());
        }
    }
}

fn add_empty_list_padding(level: &mut JCompound, path: &str) {
    let Some(JValue::List(list)) = level.get_mut(path) else {
        // difference from DFU: Don't create the damn thing!
        return;
    };

    if list.len() == 24 {
        return;
    }

    // offset the section array to the new format
    for _ in 0..4 {
        let _ = list.try_insert(0, JList::new()); // add below
        let _ = list.try_push(JList::new()); // add above
    }
}

fn offset_heightmaps(level: &mut JCompound) {
    let Some(JValue::Compound(heightmaps)) = level.get_mut("Heightmaps") else {
        return;
    };
    for key in HEIGHTMAP_TYPES {
        if let Some(JValue::LongArray(heightmap)) = heightmaps.get_mut(key) {
            offset_heightmap(heightmap);
        }
    }
}

fn offset_heightmap(heightmap: &mut [i64]) {
    // heightmaps are configured to have 9 bits per value, with 256 total values
    // heightmaps are also relative to the lowest position

    for element in heightmap {
        let mut next = 0;
        for obj_idx in (0..=64 - 9).step_by(9) {
            let value = (*element >> obj_idx) & 511;
            if value != 0 {
                let offset = 511.min(value + 64);

                next |= offset << obj_idx;
            }
        }

        *element = next;
    }
}

fn create_biome_sections(
    level: &JCompound,
    want_extended_height: bool,
    is_already_extended: &mut bool,
) -> Vec<JCompound> {
    let mut ret = Vec::with_capacity(if want_extended_height { 24 } else { 16 });

    let biomes = match level.get("Biomes") {
        Some(JValue::IntArray(biomes)) => Some(biomes),
        _ => None,
    };

    if let Some(biomes) = biomes {
        if biomes.len() == 1536 {
            // magic value for 24 sections of biomes (24 * 4^3)
            *is_already_extended = true;
            for section_index in 0..24 {
                ret.push(create_biome_section(biomes, section_index * 64, usize::MAX));
            }
            return ret;
        }
        if biomes.len() == 1024 {
            // magic value for 16 sections of biomes (16 * 4^3)
            if want_extended_height {
                let bottom_copy = create_biome_section(biomes, 0, 15); // just want the biomes at y = 0
                for _ in 0..4 {
                    ret.push(bottom_copy.clone());
                }
            }

            for section_y in 0..16 {
                ret.push(create_biome_section(biomes, section_y * 64, usize::MAX));
            }

            if want_extended_height {
                let top_copy = create_biome_section(biomes, 1008, 15); // just want the biomes at y = 252
                for _ in 20..24 {
                    ret.push(top_copy.clone());
                }
            }

            return ret;
        }
    }

    let palette = vec![JavaString::from("minecraft:plains")];
    for _ in 0..if want_extended_height { 16 } else { 24 } {
        ret.push(wrap_palette(palette.clone(), None));
    }
    ret
}

fn create_biome_section(biomes: &[i32], offset: usize, mask: usize) -> JCompound {
    let mut palette_id = AHashMap::new();
    let mut palette_string = Vec::new();
    for idx in 0..64 {
        let biome = biomes[offset + (idx & mask)];
        let next_id = palette_id.len() as u64;
        palette_id.entry(biome).or_insert_with(|| {
            palette_string.push(get_biome_by_id(biome).to_owned());
            next_id
        });
    }

    let bits_per_object = ceil_log2(palette_string.len() as u32);
    if bits_per_object == 0 {
        return wrap_palette(palette_string, None);
    }

    // manually create packed integer data
    let objects_per_value = 64 / bits_per_object;
    let mut packed =
        Vec::with_capacity((64 + objects_per_value as usize - 1) / objects_per_value as usize);

    let mut shift = 0;
    let mut curr = 0;

    for biome_idx in 0..64 {
        let biome = biomes[offset + (biome_idx & mask)];

        curr |= *palette_id.get(&biome).unwrap() << shift;

        shift += bits_per_object;

        if shift + bits_per_object > 64 {
            // will next write overflow?
            // must move to next idx
            packed.push(curr as i64);
            shift = 0;
            curr = 0;
        }
    }

    // don't forget to write the last one
    if shift != 0 {
        packed.push(curr as i64);
    }

    assert_eq!(
        packed.len(),
        (64 + objects_per_value as usize - 1) / objects_per_value as usize
    );

    wrap_palette(palette_string, Some(packed))
}

fn wrap_palette(palette: impl Into<JList>, block_states: Option<Vec<i64>>) -> JCompound {
    let mut ret = jcompound! {
        "palette" => palette.into(),
    };
    if let Some(block_states) = block_states {
        ret.insert("data", block_states);
    }
    ret
}

fn wrap_palette_optimized(palette: impl Into<JList>, block_states: Vec<i64>) -> JCompound {
    let palette = palette.into();
    if palette.len() == 1 {
        wrap_palette(palette, None)
    } else {
        wrap_palette(palette, Some(block_states))
    }
}

fn update_layers(layers: &mut JList) {
    let _ = layers.try_insert(0, create_empty_layer());
}

fn create_empty_layer() -> JCompound {
    jcompound! {
        "height" => 64,
        "block" => "minecraft:air",
    }
}
