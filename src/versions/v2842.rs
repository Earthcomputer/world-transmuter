use crate::types;
use world_transmuter_engine::{
    convert_map_list_in_map, convert_object_in_map, convert_object_list_in_map,
    convert_values_in_map, data_walker, map_data_converter_func, rename_key, JList, JValue,
};

const VERSION: u32 = 2842;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(level)) = data.remove("Level") else {
                return;
            };

            // merge the level tag into the root data
            for (key, value) in level {
                data.insert(key, value);
            }

            // Rename the top level first
            rename_key(data, "TileEntities", "block_entities");
            rename_key(data, "TileTicks", "block_ticks");
            rename_key(data, "Entities", "entities");
            rename_key(data, "Sections", "sections");
            rename_key(data, "Structures", "structures");

            // 2nd level
            if let Some(JValue::Compound(structures)) = data.get_mut("structures") {
                rename_key(structures, "Starts", "starts");
            }
        }),
    );

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
