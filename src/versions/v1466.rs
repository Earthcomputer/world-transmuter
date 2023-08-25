use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{
    convert_map_in_map, convert_map_list_in_map, convert_object_in_map, convert_values_in_map,
    data_walker, map_data_converter_func,
};
use valence_nbt::{List, Value};

const VERSION: u32 = 1466;

pub(crate) fn register(types: &MinecraftTypesMut) {
    // There is a rather critical change I've made to this converter: changing the chunk status determination.
    // In Vanilla, this is determined by whether the terrain has been populated and whether the chunk is lit.
    // For reference, here is the full status progression (at the time of 18w06a):
    // empty -> base -> carved -> decorated -> lighted -> mobs_spawned -> finalized -> fullchunk -> postprocessed
    // So one of those must be picked.
    // If the chunk is lit and terrain is populated, the Vanilla converter will set the status to "mobs_spawned."
    // If it is anything else, it will be "empty"
    // I've changed it to the following: if terrain is populated, it is set to at least decorated. If it is populated
    // and lit, it is set to "mobs_spawned"
    // But what if it is not populated? If it is not populated, ignore the lit field - obviously that's just broken.
    // It can't be lit and not populated.
    // Let's take a look at chunk generation logic for a chunk that is not populated, or even near a populated chunk.
    // It actually will generate a chunk up to the "carved" stage. It generates the base terrain, (i.e using noise
    // to figure out where stone is, dirt, grass) and it will generate caves. Nothing else though. No populators.
    // So "carved" is the correct stage to use, not empty. Setting it to empty would clobber chunk data, when we don't
    // need to. If it is populated, at least set it to decorated. If it is lit and populated, set it to mobs_spawned. Else,
    // it is carved.
    // This change also fixes the random light check "bug" (really this is Mojang's fault for fucking up the status conversion here)
    // caused by spigot, which would not set the lit value for some chunks. Now those chunks will not be regenerated.

    types.chunk.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(level)) = data.get_mut("Level") else {
                return;
            };

            let terrain_populated = level
                .get("TerrainPopulated")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let light_populated = level
                .get("LightPopulated")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            let new_status = match (terrain_populated, light_populated) {
                (false, _) => "carved",
                (true, false) => "decorated",
                (true, true) => "mobs_spawned",
            };

            level.insert("Status", new_status);
            level.insert("hasLegacyStructureData", true);

            // convert biome byte[] to int[]
            if let Some(biomes) = level.get("Biomes").and_then(|o| match o {
                Value::ByteArray(arr) => Some(arr),
                _ => None,
            }) {
                let new_biomes: Vec<_> = biomes.iter().map(|b| *b as u8 as i32).collect();
                level.insert("Biomes", new_biomes);
            }

            // ProtoChunks have their own dedicated tick list, so we must convert the TileTicks to that.
            if let Some(Value::List(tile_ticks)) = level.get("TileTicks") {
                let mut sections = vec![Vec::new(); 16];

                if let List::Compound(tile_ticks) = tile_ticks {
                    for tile_tick in tile_ticks {
                        let x = tile_tick.get("x").and_then(|v| v.as_i8()).unwrap_or(0) as u8;
                        let y = tile_tick.get("y").and_then(|v| v.as_i8()).unwrap_or(0) as u8;
                        let z = tile_tick.get("z").and_then(|v| v.as_i8()).unwrap_or(0) as u8;
                        let coordinate =
                            (x & 15) as i16 | (((y & 15) as i16) << 4) | (((z & 15) as i16) << 8);
                        sections[(y >> 4) as usize].push(coordinate);
                    }
                }

                level.insert(
                    "ToBeTicked",
                    List::List(sections.into_iter().map(List::from).collect::<Vec<_>>()),
                );
            }
        }),
    );

    let block_name_type = types.block_name;
    let block_state_type = types.block_state;
    let entity_type = types.entity;
    let structure_feature_type = types.structure_feature;
    let tile_entity_type = types.tile_entity;
    types.chunk.borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            let Some(Value::Compound(level)) = data.get_mut("Level") else {
                return;
            };

            convert_map_list_in_map(entity_type, level, "Entities", from_version, to_version);
            convert_map_list_in_map(
                tile_entity_type,
                level,
                "TileEntities",
                from_version,
                to_version,
            );

            if let Some(Value::List(List::Compound(tile_ticks))) = level.get_mut("TileTicks") {
                for tile_tick in tile_ticks {
                    convert_object_in_map(
                        block_name_type,
                        tile_tick,
                        "i",
                        from_version,
                        to_version,
                    );
                }
            }

            if let Some(Value::List(List::Compound(sections))) = level.get_mut("Sections") {
                for section in sections.iter_mut() {
                    convert_map_list_in_map(
                        block_state_type,
                        section,
                        "Palette",
                        from_version,
                        to_version,
                    );
                }
            }

            if let Some(Value::Compound(structures)) = level.get_mut("Structures") {
                convert_values_in_map(
                    structure_feature_type,
                    structures,
                    "Starts",
                    from_version,
                    to_version,
                );
            }
        }),
    );
    let block_state_type = types.block_state;
    types.structure_feature.borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data, from_version, to_version| {
            if let Some(Value::List(List::Compound(children))) = data.get_mut("Children") {
                for child in children {
                    convert_map_in_map(block_state_type, child, "CA", from_version, to_version);
                    convert_map_in_map(block_state_type, child, "CB", from_version, to_version);
                    convert_map_in_map(block_state_type, child, "CC", from_version, to_version);
                    convert_map_in_map(block_state_type, child, "CD", from_version, to_version);
                }
            }
        }),
    );
}
