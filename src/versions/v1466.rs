use rust_dataconverter_engine::{convert_map_in_map, convert_map_list_in_map, convert_object_in_map, convert_values_in_map, data_converter_func, data_walker, ListType, MapType, ObjectRef, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1466;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
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

    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let level = match data.get_map_mut("Level") {
            Some(level) => level,
            None => return
        };

        let terrain_populated = level.get_bool("TerrainPopulated").unwrap_or(false);
        let light_populated = level.get_bool("LightPopulated").unwrap_or(true);

        let new_status = match (terrain_populated, light_populated) {
            (false, _) => "carved",
            (true, false) => "decorated",
            (true, true) => "mobs_spawned",
        };

        level.set("Status", T::Object::create_string(new_status.to_owned()));
        level.set("hasLegacyStructureData", T::Object::create_bool(true));

        // convert biome byte[] to int[]
        if let Some(biomes) = level.get("Biomes").and_then(|o| match o.as_ref() {
            ObjectRef::ByteArray(arr) => Some(arr),
            _ => None
        }) {
            let new_biomes = biomes.iter().map(|b| *b as u8 as i32).collect();
            level.set("Biomes", T::Object::create_int_array(new_biomes));
        }

        // ProtoChunks have their own dedicated tick list, so we must convert the TileTicks to that.
        if let Some(tile_ticks) = level.get_list("TileTicks") {
            let mut sections = T::List::create_empty();
            for _ in 0..16 {
                sections.add(T::Object::create_list(T::List::create_empty()));
            }

            for tile_tick in tile_ticks.iter() {
                if let Some(tile_tick) = tile_tick.as_map() {
                    let x = tile_tick.get_i64("x").unwrap_or(0) as u8;
                    let y = tile_tick.get_i64("y").unwrap_or(0) as u8;
                    let z = tile_tick.get_i64("z").unwrap_or(0) as u8;
                    let coordinate = (x & 15) as u16 | (((y & 15) as u16) << 4) | (((z & 15) as u16) << 8);
                    sections.get_mut((y >> 4) as usize).as_list_mut().unwrap().add(T::Object::create_short(coordinate as i16))
                }
            }

            level.set("ToBeTicked", T::Object::create_list(sections));
        }
    }));

    let block_name_type = types.block_name;
    let block_state_type = types.block_state;
    let entity_type = types.entity;
    let structure_feature_type = types.structure_feature;
    let tile_entity_type = types.tile_entity;
    types.chunk.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        let level = match data.get_map_mut("Level") {
            Some(level) => level,
            None => return
        };

        convert_map_list_in_map::<_, T>(entity_type, level, "Entities", from_version, to_version);
        convert_map_list_in_map::<_, T>(tile_entity_type, level, "TileEntities", from_version, to_version);

        if let Some(tile_ticks) = level.get_list_mut("TileTicks") {
            for tile_tick in tile_ticks.iter_mut() {
                if let Some(tile_tick) = tile_tick.as_map_mut() {
                    convert_object_in_map::<_, T>(block_name_type, tile_tick, "i", from_version, to_version);
                }
            }
        }

        if let Some(sections) = level.get_list_mut("Sections") {
            for section in sections.iter_mut() {
                if let Some(section) = section.as_map_mut() {
                    convert_map_list_in_map::<_, T>(block_state_type, section, "Palette", from_version, to_version);
                }
            }
        }

        if let Some(structures) = level.get_map_mut("Structures") {
            convert_values_in_map::<_, T>(structure_feature_type, structures, "Starts", from_version, to_version);
        }
    }));
    let block_state_type = types.block_state;
    types.structure_feature.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(children) = data.get_list_mut("Children") {
            for child in children.iter_mut() {
                if let Some(child) = child.as_map_mut() {
                    convert_map_in_map::<_, T>(block_state_type, child, "CA", from_version, to_version);
                    convert_map_in_map::<_, T>(block_state_type, child, "CB", from_version, to_version);
                    convert_map_in_map::<_, T>(block_state_type, child, "CC", from_version, to_version);
                    convert_map_in_map::<_, T>(block_state_type, child, "CD", from_version, to_version);
                }
            }
        }
    }));
}
