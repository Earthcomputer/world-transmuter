use crate::types;
use world_transmuter_engine::{
    convert_object_in_map, convert_object_list_in_map, map_data_walker, JList, JValue,
};

const VERSION: u32 = 2551;

pub(crate) fn register() {
    types::world_gen_settings_mut().add_structure_walker(
        VERSION,
        map_data_walker(move |data, from_version, to_version| {
            let Some(JValue::Compound(dimensions)) = data.get_mut("dimensions") else {
                return;
            };
            for dimension in dimensions.values_mut() {
                let JValue::Compound(dimension) = dimension else {
                    continue;
                };
                let Some(JValue::Compound(generator)) = dimension.get_mut("generator") else {
                    continue;
                };
                match generator.get("type") {
                    Some(JValue::String(str)) if str == "minecraft:flat" => {
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
                    Some(JValue::String(str)) if str == "minecraft:noise" => {
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
                            match biome_source.get("type") {
                                Some(JValue::String(str)) if str == "minecraft:fixed" => {
                                    convert_object_in_map(
                                        types::biome_ref(),
                                        biome_source,
                                        "biome",
                                        from_version,
                                        to_version,
                                    );
                                }
                                Some(JValue::String(str)) if str == "minecraft:multi_noise" => {
                                    // Vanilla's schema is wrong. It should be DSL.fields("biomes", DSL.list(DSL.fields("biome")))
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
                                Some(JValue::String(str)) if str == "minecraft:checkerboard" => {
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
                    _ => {}
                }
            }
        }),
    );
}
