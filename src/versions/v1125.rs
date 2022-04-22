use rust_dataconverter_engine::{data_converter_func, data_walker, ListType, MapType, ObjectRef, ObjectType, Types};
use crate::helpers::hooks::DataHookValueTypeEnforceNamespaced;
use crate::helpers::rename::rename_keys_in_map;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1125;
const BED_BLOCK_ID: i8 = 26;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            let chunk_x = level.get_i64("xPos").unwrap_or(0) as i32;
            let chunk_z = level.get_i64("zPos").unwrap_or(0) as i32;

            if level.get_list("TileEntities").is_none() {
                level.set("TileEntities", T::Object::create_list(T::List::create_empty()));
            }

            let [tile_entities, sections] = level.get_mut_multi(["TileEntities", "Sections"]);
            let tile_entities = tile_entities.unwrap().as_list_mut().unwrap(); // unwrap is safe because the presence of TileEntities is ensured above

            if let Some(sections) = sections.and_then(|o| o.as_list()) {
                for section in sections.iter() {
                    if let Some(section) = section.as_map() {
                        let section: &T::Map = section;
                        let section_y = section.get_i64("Y").unwrap_or(0) as i32;
                        let blocks = match section.get("Blocks").map(|o| o.as_ref()) {
                            Some(ObjectRef::ByteArray(bytes)) => bytes,
                            _ => continue
                        };

                        for (index, block_id) in blocks.iter().copied().enumerate() {
                            if block_id != BED_BLOCK_ID {
                                continue;
                            }

                            let local_x = (index & 15) as i32;
                            let local_y = ((index >> 4) & 15) as i32;
                            let local_z = ((index >> 8) & 15) as i32;

                            let mut new_tile = T::Map::create_empty();
                            new_tile.set("id", T::Object::create_string("minecraft:bed".to_owned()));
                            new_tile.set("x", T::Object::create_int(local_x + (chunk_x << 4)));
                            new_tile.set("y", T::Object::create_int(local_y + (section_y << 4)));
                            new_tile.set("z", T::Object::create_int(local_z + (chunk_z << 4)));
                            new_tile.set("color", T::Object::create_short(14)); // Red

                            tile_entities.add(T::Object::create_map(new_tile));
                        }
                    }
                }
            }
        }
    }));

    types.item_stack.borrow_mut().add_converter_for_id("minecraft:bed", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_i64("Damage").unwrap_or(0) == 0 {
            data.set("Damage", T::Object::create_short(14)); // Red
        }
    }));

    let biome_type = types.biome;
    let entity_name_type = types.entity_name;
    types.advancements.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(adventuring_time) = data.get_map_mut("minecraft:adventure/adventuring_time") {
            rename_keys_in_map::<T>(biome_type, adventuring_time, "criteria", from_version, to_version);
        }
        if let Some(kill_a_mob) = data.get_map_mut("minecraft:adventure/kill_a_mob") {
            rename_keys_in_map::<T>(entity_name_type, kill_a_mob, "criteria", from_version, to_version);
        }
        if let Some(kill_all_mobs) = data.get_map_mut("minecraft:adventure/kill_all_mobs") {
            rename_keys_in_map::<T>(entity_name_type, kill_all_mobs, "criteria", from_version, to_version);
        }
        if let Some(bred_all_animals) = data.get_map_mut("minecraft:adventure/bred_all_animals") {
            rename_keys_in_map::<T>(entity_name_type, bred_all_animals, "criteria", from_version, to_version);
        }
    }));

    // Enforce namespacing for ids
    types.biome.borrow_mut().add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced::<T>::new());
}
