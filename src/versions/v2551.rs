use crate::MinecraftTypesMut;
use valence_nbt::{List, Value};
use world_transmuter_engine::{convert_object_in_map, convert_object_list_in_map, data_walker};

const VERSION: u32 = 2551;

pub(crate) fn register(types: MinecraftTypesMut) {
    let biome_type = types.biome();
    let block_name_type = types.block_name();
    types
        .world_gen_settings()
        .borrow_mut()
        .add_structure_walker(
            VERSION,
            data_walker(move |data, from_version, to_version| {
                let Some(Value::Compound(dimensions)) = data.get_mut("dimensions") else {
                    return;
                };
                for dimension in dimensions.values_mut() {
                    let Value::Compound(dimension) = dimension else {
                        continue;
                    };
                    let Some(Value::Compound(generator)) = dimension.get_mut("generator") else {
                        continue;
                    };
                    match generator.get("type") {
                        Some(Value::String(str)) if str == "minecraft:flat" => {
                            let Some(Value::Compound(settings)) = generator.get_mut("settings")
                            else {
                                continue;
                            };
                            convert_object_in_map(
                                biome_type,
                                settings,
                                "biome",
                                from_version,
                                to_version,
                            );
                            if let Some(Value::List(List::Compound(layers))) =
                                settings.get_mut("layers")
                            {
                                for layer in layers {
                                    convert_object_in_map(
                                        block_name_type,
                                        layer,
                                        "block",
                                        from_version,
                                        to_version,
                                    );
                                }
                            }
                        }
                        Some(Value::String(str)) if str == "minecraft:noise" => {
                            if let Some(Value::Compound(settings)) = generator.get_mut("settings") {
                                convert_object_in_map(
                                    block_name_type,
                                    settings,
                                    "default_block",
                                    from_version,
                                    to_version,
                                );
                                convert_object_in_map(
                                    block_name_type,
                                    settings,
                                    "default_fluid",
                                    from_version,
                                    to_version,
                                );
                            }
                            if let Some(Value::Compound(biome_source)) =
                                generator.get_mut("biome_source")
                            {
                                match biome_source.get("type") {
                                    Some(Value::String(str)) if str == "minecraft:fixed" => {
                                        convert_object_in_map(
                                            biome_type,
                                            biome_source,
                                            "biome",
                                            from_version,
                                            to_version,
                                        );
                                    }
                                    Some(Value::String(str)) if str == "minecraft:multi_noise" => {
                                        // Vanilla's schema is wrong. It should be DSL.fields("biomes", DSL.list(DSL.fields("biome")))
                                        // But it just contains the list part. That obviously can never be the case, because
                                        // the root object is a compound, not a list.

                                        if let Some(Value::List(List::Compound(biomes))) =
                                            biome_source.get_mut("biomes")
                                        {
                                            for biome in biomes {
                                                convert_object_in_map(
                                                    biome_type,
                                                    biome,
                                                    "biome",
                                                    from_version,
                                                    to_version,
                                                );
                                            }
                                        }
                                    }
                                    Some(Value::String(str)) if str == "minecraft:checkerboard" => {
                                        convert_object_list_in_map(
                                            biome_type,
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
                        _ => {}
                    }
                }
            }),
        );
}
