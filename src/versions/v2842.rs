use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::{
    convert_map_list_in_map, convert_object_in_map, convert_object_list_in_map,
    convert_values_in_map, data_walker, map_data_converter_func, rename_key,
};
use valence_nbt::{List, Value};

const VERSION: u32 = 2842;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.chunk.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(level)) = data.remove("Level") else {
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
            if let Some(Value::Compound(structures)) = data.get_mut("structures") {
                rename_key(structures, "Starts", "starts");
            }
        }),
    );

    let entity_type = types.entity;
    let tile_entity_type = types.tile_entity;
    let block_name_type = types.block_name;
    let biome_type = types.biome;
    let block_state_type = types.block_state;
    let structure_feature_type = types.structure_feature;
    types.chunk.borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            convert_map_list_in_map(entity_type, data, "entities", from_version, to_version);
            convert_map_list_in_map(
                tile_entity_type,
                data,
                "block_entities",
                from_version,
                to_version,
            );

            if let Some(Value::List(List::Compound(block_ticks))) = data.get_mut("block_ticks") {
                for block_tick in block_ticks {
                    convert_object_in_map(
                        block_name_type,
                        block_tick,
                        "i",
                        from_version,
                        to_version,
                    );
                }
            }

            if let Some(Value::List(List::Compound(sections))) = data.get_mut("sections") {
                for section in sections {
                    if let Some(Value::Compound(biomes)) = section.get_mut("biomes") {
                        convert_object_list_in_map(
                            biome_type,
                            biomes,
                            "palette",
                            from_version,
                            to_version,
                        );
                    }
                    if let Some(Value::Compound(block_states)) = section.get_mut("block_states") {
                        convert_map_list_in_map(
                            block_state_type,
                            block_states,
                            "palette",
                            from_version,
                            to_version,
                        );
                    }
                }
            }

            if let Some(Value::Compound(structures)) = data.get_mut("structures") {
                convert_values_in_map(
                    structure_feature_type,
                    structures,
                    "starts",
                    from_version,
                    to_version,
                );
            }
        }),
    );
}
