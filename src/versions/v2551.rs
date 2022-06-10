use rust_dataconverter_engine::{convert_object_in_map, convert_object_list_in_map, data_walker, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2551;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    let biome_type = types.biome;
    let block_name_type = types.block_name;
    types.world_gen_settings.borrow_mut().add_structure_walker(VERSION, data_walker::<T, _>(move |data: &mut T::Map, from_version, to_version| {
        if let Some(dimensions) = data.get_map_mut("dimensions") {
            for dimension in dimensions.values_mut() {
                if let Some(generator) = dimension.as_map_mut().and_then(|o| o.get_map_mut("generator")) {
                    let generator: &mut T::Map = generator;
                    match generator.get_string("type") {
                        Some("minecraft:flat") => {
                            if let Some(settings) = generator.get_map_mut("settings") {
                                convert_object_in_map::<_, T>(biome_type, settings, "biome", from_version, to_version);
                                if let Some(layers) = settings.get_list_mut("layers") {
                                    for layer in layers.iter_mut() {
                                        if let Some(layer) = layer.as_map_mut() {
                                            convert_object_in_map::<_, T>(block_name_type, layer, "block", from_version, to_version);
                                        }
                                    }
                                }
                            }
                        }
                        Some("minecraft:noise") => {
                            if let Some(settings) = generator.get_map_mut("settings") {
                                convert_object_in_map::<_, T>(block_name_type, settings, "default_block", from_version, to_version);
                                convert_object_in_map::<_, T>(block_name_type, settings, "default_fluid", from_version, to_version);
                            }
                            if let Some(biome_source) = generator.get_map_mut("biome_source") {
                                match biome_source.get_string("type") {
                                    Some("minecraft:fixed") => {
                                        convert_object_in_map::<_, T>(biome_type, biome_source, "biome", from_version, to_version);
                                    }
                                    Some("minecraft:multi_noise") => {
                                        // Vanilla's schema is wrong. It should be DSL.fields("biomes", DSL.list(DSL.fields("biome")))
                                        // But it just contains the list part. That obviously can never be the case, because
                                        // the root object is a compound, not a list.

                                        if let Some(biomes) = biome_source.get_list_mut("biomes") {
                                            for biome in biomes.iter_mut() {
                                                if let Some(biome) = biome.as_map_mut() {
                                                    convert_object_in_map::<_, T>(biome_type, biome, "biome", from_version, to_version);
                                                }
                                            }
                                        }
                                    }
                                    Some("minecraft:checkerboard") => {
                                        convert_object_list_in_map::<_, T>(biome_type, biome_source, "biomes", from_version, to_version);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }));
}
