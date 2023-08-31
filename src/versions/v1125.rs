use crate::helpers::hooks::DataHookValueTypeEnforceNamespaced;
use crate::helpers::rename::rename_keys_in_map;
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{data_walker, get_mut_multi, map_data_converter_func};
use valence_nbt::{compound, Compound, List, Value};

const VERSION: u32 = 1125;
const BED_BLOCK_ID: i8 = 26;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.chunk().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::Compound(level)) = data.get_mut("Level") else {
                return;
            };
            let chunk_x = level.get("xPos").and_then(|v| v.as_i32()).unwrap_or(0);
            let chunk_z = level.get("zPos").and_then(|v| v.as_i32()).unwrap_or(0);

            if !matches!(level.get("TileEntities"), Some(Value::List(_))) {
                level.insert("TileEntities", List::new());
            }

            let [tile_entities, sections] = get_mut_multi(level, ["TileEntities", "Sections"]);
            let Some(Value::List(tile_entities)) = tile_entities else {
                unreachable!()
            }; // unreachable because the presence of TileEntities is ensured above

            if let Some(Value::List(List::Compound(sections))) = sections {
                for section in sections {
                    let section_y = section.get("Y").and_then(|v| v.as_i32()).unwrap_or(0);
                    let Some(Value::ByteArray(blocks)) = section.get("Blocks") else {
                        continue;
                    };

                    for (index, block_id) in blocks.iter().copied().enumerate() {
                        if block_id != BED_BLOCK_ID {
                            continue;
                        }

                        let local_x = (index & 15) as i32;
                        let local_y = ((index >> 4) & 15) as i32;
                        let local_z = ((index >> 8) & 15) as i32;

                        let new_tile = compound! {
                            "id" => "minecraft:bed",
                            "x" => local_x + (chunk_x << 4),
                            "y" => local_y + (section_y << 4),
                            "z" => local_z + (chunk_z << 4),
                            "color" => 14i16, // Red
                        };

                        let _ = tile_entities.try_push(new_tile);
                    }
                }
            }
        }),
    );

    types.item_stack().borrow_mut().add_converter_for_id(
        "minecraft:bed",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.get("Damage").and_then(|v| v.as_i64()).unwrap_or(0) == 0 {
                data.insert("Damage", 14i16); // Red
            }
        }),
    );

    let biome_type = types.biome();
    let entity_name_type = types.entity_name();
    types.advancements().borrow_mut().add_structure_walker(
        VERSION,
        data_walker(move |data: &mut Compound, from_version, to_version| {
            if let Some(Value::Compound(adventuring_time)) =
                data.get_mut("minecraft:adventure/adventuring_time")
            {
                rename_keys_in_map(
                    biome_type,
                    adventuring_time,
                    "criteria",
                    from_version,
                    to_version,
                );
            }
            if let Some(Value::Compound(kill_a_mob)) =
                data.get_mut("minecraft:adventure/kill_a_mob")
            {
                rename_keys_in_map(
                    entity_name_type,
                    kill_a_mob,
                    "criteria",
                    from_version,
                    to_version,
                );
            }
            if let Some(Value::Compound(kill_all_mobs)) =
                data.get_mut("minecraft:adventure/kill_all_mobs")
            {
                rename_keys_in_map(
                    entity_name_type,
                    kill_all_mobs,
                    "criteria",
                    from_version,
                    to_version,
                );
            }
            if let Some(Value::Compound(bred_all_animals)) =
                data.get_mut("minecraft:adventure/bred_all_animals")
            {
                rename_keys_in_map(
                    entity_name_type,
                    bred_all_animals,
                    "criteria",
                    from_version,
                    to_version,
                );
            }
        }),
    );

    // Enforce namespacing for ids
    types
        .biome()
        .borrow_mut()
        .add_structure_hook(VERSION, DataHookValueTypeEnforceNamespaced);
}
