use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::types::MinecraftTypes;
use log::error;
use rust_dataconverter_engine::{get_mut_multi, map_data_converter_func};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use valence_nbt::{Compound, List, Value};

const VERSION: u32 = 2970;

static CONVERSION_MAP: OnceLock<BTreeMap<&str, BiomeRemap>> = OnceLock::new();

fn conversion_map() -> &'static BTreeMap<&'static str, BiomeRemap> {
    CONVERSION_MAP.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert(
            "mineshaft",
            BiomeRemap::create(
                &[(
                    &["badlands", "eroded_badlands", "wooded_badlands"],
                    "minecraft:mineshaft_mesa",
                )],
                "minecraft:mineshaft",
            ),
        );
        map.insert(
            "shipwreck",
            BiomeRemap::create(
                &[(&["beach", "snowy_beach"], "minecraft:shipwreck_beached")],
                "minecraft:shipwreck",
            ),
        );
        map.insert(
            "ocean_ruin",
            BiomeRemap::create(
                &[(
                    &["warm_ocean", "lukewarm_ocean", "deep_lukewarm_ocean"],
                    "minecraft:ocean_ruin_warm",
                )],
                "minecraft:ocean_ruin_cold",
            ),
        );
        map.insert(
            "village",
            BiomeRemap::create(
                &[
                    (&["desert"], "minecraft:village_desert"),
                    (&["savanna"], "minecraft:village_savanna"),
                    (&["snowy_plains"], "minecraft:village_snowy"),
                    (&["taiga"], "minecraft:village_taiga"),
                ],
                "minecraft:village_plains",
            ),
        );
        map.insert(
            "ruined_portal", // Fix MC-248814, ruined_portal_standard->ruined_portal
            BiomeRemap::create(
                &[
                    (&["desert"], "minecraft:ruined_portal_desert"),
                    (
                        &[
                            "badlands",
                            "eroded_badlands",
                            "wooded_badlands",
                            "windswept_hills",
                            "windswept_forest",
                            "windswept_gravelly_hills",
                            "savanna_plateau",
                            "windswept_savanna",
                            "stony_shore",
                            "meadow",
                            "frozen_peaks",
                            "jagged_peaks",
                            "stony_peaks",
                            "snowy_slopes",
                        ],
                        "minecraft:ruined_portal_mountain",
                    ),
                    (
                        &["bamboo_jungle", "jungle", "sparse_jungle"],
                        "minecraft:ruined_portal_jungle",
                    ),
                    (
                        &[
                            "deep_frozen_ocean",
                            "deep_cold_ocean",
                            "deep_ocean",
                            "deep_lukewarm_ocean",
                            "frozen_ocean",
                            "ocean",
                            "cold_ocean",
                            "lukewarm_ocean",
                            "warm_ocean",
                        ],
                        "minecraft:ruined_portal_ocean",
                    ),
                ],
                "minecraft:ruined_portal",
            ),
        );
        map.insert(
            "pillager_outpost",
            BiomeRemap::create(&[], "minecraft:pillager_outpost"),
        );
        map.insert("mansion", BiomeRemap::create(&[], "minecraft:mansion"));
        map.insert(
            "jungle_pyramid",
            BiomeRemap::create(&[], "minecraft:jungle_pyramid"),
        );
        map.insert(
            "desert_pyramid",
            BiomeRemap::create(&[], "minecraft:desert_pyramid"),
        );
        map.insert("igloo", BiomeRemap::create(&[], "minecraft:igloo"));
        map.insert("swamp_hut", BiomeRemap::create(&[], "minecraft:swamp_hut"));
        map.insert(
            "stronghold",
            BiomeRemap::create(&[], "minecraft:stronghold"),
        );
        map.insert("monument", BiomeRemap::create(&[], "minecraft:monument"));
        map.insert("fortress", BiomeRemap::create(&[], "minecraft:fortress"));
        map.insert("endcity", BiomeRemap::create(&[], "minecraft:end_city"));
        map.insert(
            "buried_treasure",
            BiomeRemap::create(&[], "minecraft:buried_treasure"),
        );
        map.insert(
            "nether_fossil",
            BiomeRemap::create(&[], "minecraft:nether_fossil"),
        );
        map.insert(
            "bastion_remnant",
            BiomeRemap::create(&[], "minecraft:bastion_remnant"),
        );
        map
    })
}

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.chunk.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let [Some(Value::Compound(structures)), sections] =
                get_mut_multi(data, ["structures", "sections"])
            else {
                return;
            };
            if structures.is_empty() {
                return;
            }

            let biome_counts = count_biomes(sections.map(|sections| &*sections));

            if let Some(Value::Compound(starts)) = structures.remove("starts") {
                let mut new_starts = Compound::new();

                for (key, value) in starts {
                    let Value::Compound(mut value) = value else {
                        continue;
                    };
                    match value.get("id") {
                        Some(Value::String(id)) => {
                            if id == "INVALID" {
                                continue;
                            }
                        }
                        _ => continue,
                    }

                    let Some(remapped) = get_structure_converted(&key[..], &biome_counts) else {
                        continue;
                    };
                    value.insert("id", remapped.to_owned());
                    new_starts.insert(remapped, value);
                }

                structures.insert("starts", new_starts);
            }

            // This TRULY is a guess, no idea what biomes the referent has.
            if let Some(Value::Compound(references)) = structures.remove("References") {
                let mut new_references = Compound::new();

                for (key, value) in references {
                    let Value::LongArray(value) = value else {
                        continue;
                    };
                    if value.is_empty() {
                        continue;
                    }

                    let Some(remapped) = get_structure_converted(&key[..], &biome_counts) else {
                        continue;
                    };
                    new_references.insert(remapped, value);
                }

                structures.insert("References", new_references);
            }
        }),
    );
}

fn count_biomes(sections: Option<&Value>) -> BTreeMap<&str, u32> {
    let mut ret = BTreeMap::new();

    let Some(Value::List(List::Compound(sections))) = sections else {
        return ret;
    };

    for section in sections {
        let Some(Value::Compound(biomes)) = section.get("biomes") else {
            continue;
        };
        let Some(Value::List(List::String(palette))) = biomes.get("palette") else {
            continue;
        };
        for biome in palette {
            *ret.entry(&biome[..]).or_default() += 1;
        }
    }

    ret
}

fn get_structure_converted(id: &str, biome_count: &BTreeMap<&str, u32>) -> Option<&'static str> {
    let id = id.to_lowercase();
    let Some(remap) = conversion_map().get(&id[..]) else {
        error!("Unknown structure {}", id);
        return None;
    };

    if remap.biome_to_new_structure.is_empty() || biome_count.is_empty() {
        return Some(remap.dfl);
    }

    let mut remap_count = BTreeMap::new();

    for (biome, count) in biome_count {
        if let Some(remapped_structure) = remap.biome_to_new_structure.get(biome) {
            *remap_count.entry(*remapped_structure).or_default() += *count;
        }
    }

    let mut converted = remap.dfl;
    let mut max_count = 0u32;

    for (remapped_structure, count) in remap_count {
        if count > max_count {
            max_count = count;
            converted = remapped_structure;
        }
    }

    Some(converted)
}

struct BiomeRemap {
    biome_to_new_structure: McNamespaceMap<'static, &'static str>,
    dfl: &'static str,
}

impl BiomeRemap {
    fn create(biome_map: &[(&[&'static str], &'static str)], new_id: &'static str) -> BiomeRemap {
        let mut biome_to_new_structure = McNamespaceMap::new();

        for (biomes, new_biome_structure) in biome_map {
            for biome in *biomes {
                if let Some(old_value) =
                    biome_to_new_structure.insert_mc(biome, *new_biome_structure)
                {
                    error!(
                        "Duplicate biome remap: {} -> {}, but already mapped to {}",
                        *biome, *new_biome_structure, old_value
                    );
                }
            }
        }

        BiomeRemap {
            biome_to_new_structure,
            dfl: new_id,
        }
    }
}
