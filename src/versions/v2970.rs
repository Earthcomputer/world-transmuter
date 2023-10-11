use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::types;
use java_string::JavaStr;
use log::error;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use world_transmuter_engine::{get_mut_multi, map_data_converter_func, JCompound, JList, JValue};

const VERSION: u32 = 2970;

static CONVERSION_MAP: OnceLock<BTreeMap<&JavaStr, BiomeRemap>> = OnceLock::new();

fn conversion_map() -> &'static BTreeMap<&'static JavaStr, BiomeRemap> {
    CONVERSION_MAP.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert(
            JavaStr::from_str("mineshaft"),
            BiomeRemap::create(
                &[(
                    &["badlands", "eroded_badlands", "wooded_badlands"],
                    "minecraft:mineshaft_mesa",
                )],
                "minecraft:mineshaft",
            ),
        );
        map.insert(
            JavaStr::from_str("shipwreck"),
            BiomeRemap::create(
                &[(&["beach", "snowy_beach"], "minecraft:shipwreck_beached")],
                "minecraft:shipwreck",
            ),
        );
        map.insert(
            JavaStr::from_str("ocean_ruin"),
            BiomeRemap::create(
                &[(
                    &["warm_ocean", "lukewarm_ocean", "deep_lukewarm_ocean"],
                    "minecraft:ocean_ruin_warm",
                )],
                "minecraft:ocean_ruin_cold",
            ),
        );
        map.insert(
            JavaStr::from_str("village"),
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
            JavaStr::from_str("ruined_portal"), // Fix MC-248814, ruined_portal_standard->ruined_portal
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
            JavaStr::from_str("pillager_outpost"),
            BiomeRemap::create(&[], "minecraft:pillager_outpost"),
        );
        map.insert(
            JavaStr::from_str("mansion"),
            BiomeRemap::create(&[], "minecraft:mansion"),
        );
        map.insert(
            JavaStr::from_str("jungle_pyramid"),
            BiomeRemap::create(&[], "minecraft:jungle_pyramid"),
        );
        map.insert(
            JavaStr::from_str("desert_pyramid"),
            BiomeRemap::create(&[], "minecraft:desert_pyramid"),
        );
        map.insert(
            JavaStr::from_str("igloo"),
            BiomeRemap::create(&[], "minecraft:igloo"),
        );
        map.insert(
            JavaStr::from_str("swamp_hut"),
            BiomeRemap::create(&[], "minecraft:swamp_hut"),
        );
        map.insert(
            JavaStr::from_str("stronghold"),
            BiomeRemap::create(&[], "minecraft:stronghold"),
        );
        map.insert(
            JavaStr::from_str("monument"),
            BiomeRemap::create(&[], "minecraft:monument"),
        );
        map.insert(
            JavaStr::from_str("fortress"),
            BiomeRemap::create(&[], "minecraft:fortress"),
        );
        map.insert(
            JavaStr::from_str("endcity"),
            BiomeRemap::create(&[], "minecraft:end_city"),
        );
        map.insert(
            JavaStr::from_str("buried_treasure"),
            BiomeRemap::create(&[], "minecraft:buried_treasure"),
        );
        map.insert(
            JavaStr::from_str("nether_fossil"),
            BiomeRemap::create(&[], "minecraft:nether_fossil"),
        );
        map.insert(
            JavaStr::from_str("bastion_remnant"),
            BiomeRemap::create(&[], "minecraft:bastion_remnant"),
        );
        map
    })
}

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let [Some(JValue::Compound(structures)), sections] =
                get_mut_multi(data, ["structures", "sections"])
            else {
                return;
            };
            if structures.is_empty() {
                return;
            }

            let biome_counts = count_biomes(sections.map(|sections| &*sections));

            if let Some(JValue::Compound(starts)) = structures.remove("starts") {
                let mut new_starts = JCompound::new();

                for (key, value) in starts {
                    let JValue::Compound(mut value) = value else {
                        continue;
                    };
                    match value.get("id") {
                        Some(JValue::String(id)) => {
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
            if let Some(JValue::Compound(references)) = structures.remove("References") {
                let mut new_references = JCompound::new();

                for (key, value) in references {
                    let JValue::LongArray(value) = value else {
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

fn count_biomes(sections: Option<&JValue>) -> BTreeMap<&JavaStr, u32> {
    let mut ret = BTreeMap::new();

    let Some(JValue::List(JList::Compound(sections))) = sections else {
        return ret;
    };

    for section in sections {
        let Some(JValue::Compound(biomes)) = section.get("biomes") else {
            continue;
        };
        let Some(JValue::List(JList::String(palette))) = biomes.get("palette") else {
            continue;
        };
        for biome in palette {
            *ret.entry(&biome[..]).or_default() += 1;
        }
    }

    ret
}

fn get_structure_converted(
    id: &JavaStr,
    biome_count: &BTreeMap<&JavaStr, u32>,
) -> Option<&'static JavaStr> {
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
    biome_to_new_structure: McNamespaceMap<'static, &'static JavaStr>,
    dfl: &'static JavaStr,
}

impl BiomeRemap {
    fn create(biome_map: &[(&[&'static str], &'static str)], new_id: &'static str) -> BiomeRemap {
        let mut biome_to_new_structure = McNamespaceMap::new();

        for (biomes, new_biome_structure) in biome_map {
            for biome in *biomes {
                if let Some(old_value) =
                    biome_to_new_structure.insert_mc(*biome, JavaStr::from_str(new_biome_structure))
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
            dfl: JavaStr::from_str(new_id),
        }
    }
}
