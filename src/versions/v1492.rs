use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use valence_nbt::{List, Value};

const VERSION: u32 = 1492;

struct StructureRenames {
    child_id: &'static str,
    template_renames: BTreeMap<&'static str, &'static str>,
}

impl StructureRenames {
    fn new(child_id: &'static str, template_renames: BTreeMap<&'static str, &'static str>) -> Self {
        Self {
            child_id,
            template_renames,
        }
    }
}

static RENAMES: OnceLock<BTreeMap<&'static str, StructureRenames>> = OnceLock::new();

fn renames() -> &'static BTreeMap<&'static str, StructureRenames> {
    RENAMES.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert(
            "EndCity",
            StructureRenames::new("ECP", {
                let mut map = BTreeMap::new();
                map.insert("second_floor", "second_floor_1");
                map.insert("third_floor", "third_floor_1");
                map.insert("third_floor_c", "third_floor_2");
                map
            }),
        );
        map.insert(
            "Mansion",
            StructureRenames::new("WMP", {
                let mut map = BTreeMap::new();
                map.insert("carpet_south", "carpet_south_1");
                map.insert("carpet_west", "carpet_west_1");
                map.insert("indoors_door", "indoors_door_1");
                map.insert("indoors_wall", "indoors_wall_1");
                map
            }),
        );
        map.insert(
            "Igloo",
            StructureRenames::new("Iglu", {
                let mut map = BTreeMap::new();
                map.insert("minecraft:igloo/igloo_bottom", "minecraft:igloo/bottom");
                map.insert("minecraft:igloo/igloo_middle", "minecraft:igloo/middle");
                map.insert("minecraft:igloo/igloo_top", "minecraft:igloo/top");
                map
            }),
        );
        map.insert(
            "Ocean_Ruin",
            StructureRenames::new("ORP", {
                let mut map = BTreeMap::new();
                map.insert(
                    "minecraft:ruin/big_ruin1_brick",
                    "minecraft:underwater_ruin/big_brick_1",
                );
                map.insert(
                    "minecraft:ruin/big_ruin2_brick",
                    "minecraft:underwater_ruin/big_brick_2",
                );
                map.insert(
                    "minecraft:ruin/big_ruin3_brick",
                    "minecraft:underwater_ruin/big_brick_3",
                );
                map.insert(
                    "minecraft:ruin/big_ruin8_brick",
                    "minecraft:underwater_ruin/big_brick_8",
                );
                map.insert(
                    "minecraft:ruin/big_ruin1_cracked",
                    "minecraft:underwater_ruin/big_cracked_1",
                );
                map.insert(
                    "minecraft:ruin/big_ruin2_cracked",
                    "minecraft:underwater_ruin/big_cracked_2",
                );
                map.insert(
                    "minecraft:ruin/big_ruin3_cracked",
                    "minecraft:underwater_ruin/big_cracked_3",
                );
                map.insert(
                    "minecraft:ruin/big_ruin8_cracked",
                    "minecraft:underwater_ruin/big_cracked_8",
                );
                map.insert(
                    "minecraft:ruin/big_ruin1_mossy",
                    "minecraft:underwater_ruin/big_mossy_1",
                );
                map.insert(
                    "minecraft:ruin/big_ruin2_mossy",
                    "minecraft:underwater_ruin/big_mossy_2",
                );
                map.insert(
                    "minecraft:ruin/big_ruin3_mossy",
                    "minecraft:underwater_ruin/big_mossy_3",
                );
                map.insert(
                    "minecraft:ruin/big_ruin8_mossy",
                    "minecraft:underwater_ruin/big_mossy_8",
                );
                map.insert(
                    "minecraft:ruin/big_ruin_warm4",
                    "minecraft:underwater_ruin/big_warm_4",
                );
                map.insert(
                    "minecraft:ruin/big_ruin_warm5",
                    "minecraft:underwater_ruin/big_warm_5",
                );
                map.insert(
                    "minecraft:ruin/big_ruin_warm6",
                    "minecraft:underwater_ruin/big_warm_6",
                );
                map.insert(
                    "minecraft:ruin/big_ruin_warm7",
                    "minecraft:underwater_ruin/big_warm_7",
                );
                map.insert(
                    "minecraft:ruin/ruin1_brick",
                    "minecraft:underwater_ruin/brick_1",
                );
                map.insert(
                    "minecraft:ruin/ruin2_brick",
                    "minecraft:underwater_ruin/brick_2",
                );
                map.insert(
                    "minecraft:ruin/ruin3_brick",
                    "minecraft:underwater_ruin/brick_3",
                );
                map.insert(
                    "minecraft:ruin/ruin4_brick",
                    "minecraft:underwater_ruin/brick_4",
                );
                map.insert(
                    "minecraft:ruin/ruin5_brick",
                    "minecraft:underwater_ruin/brick_5",
                );
                map.insert(
                    "minecraft:ruin/ruin6_brick",
                    "minecraft:underwater_ruin/brick_6",
                );
                map.insert(
                    "minecraft:ruin/ruin7_brick",
                    "minecraft:underwater_ruin/brick_7",
                );
                map.insert(
                    "minecraft:ruin/ruin8_brick",
                    "minecraft:underwater_ruin/brick_8",
                );
                map.insert(
                    "minecraft:ruin/ruin1_cracked",
                    "minecraft:underwater_ruin/cracked_1",
                );
                map.insert(
                    "minecraft:ruin/ruin2_cracked",
                    "minecraft:underwater_ruin/cracked_2",
                );
                map.insert(
                    "minecraft:ruin/ruin3_cracked",
                    "minecraft:underwater_ruin/cracked_3",
                );
                map.insert(
                    "minecraft:ruin/ruin4_cracked",
                    "minecraft:underwater_ruin/cracked_4",
                );
                map.insert(
                    "minecraft:ruin/ruin5_cracked",
                    "minecraft:underwater_ruin/cracked_5",
                );
                map.insert(
                    "minecraft:ruin/ruin6_cracked",
                    "minecraft:underwater_ruin/cracked_6",
                );
                map.insert(
                    "minecraft:ruin/ruin7_cracked",
                    "minecraft:underwater_ruin/cracked_7",
                );
                map.insert(
                    "minecraft:ruin/ruin8_cracked",
                    "minecraft:underwater_ruin/cracked_8",
                );
                map.insert(
                    "minecraft:ruin/ruin1_mossy",
                    "minecraft:underwater_ruin/mossy_1",
                );
                map.insert(
                    "minecraft:ruin/ruin2_mossy",
                    "minecraft:underwater_ruin/mossy_2",
                );
                map.insert(
                    "minecraft:ruin/ruin3_mossy",
                    "minecraft:underwater_ruin/mossy_3",
                );
                map.insert(
                    "minecraft:ruin/ruin4_mossy",
                    "minecraft:underwater_ruin/mossy_4",
                );
                map.insert(
                    "minecraft:ruin/ruin5_mossy",
                    "minecraft:underwater_ruin/mossy_5",
                );
                map.insert(
                    "minecraft:ruin/ruin6_mossy",
                    "minecraft:underwater_ruin/mossy_6",
                );
                map.insert(
                    "minecraft:ruin/ruin7_mossy",
                    "minecraft:underwater_ruin/mossy_7",
                );
                map.insert(
                    "minecraft:ruin/ruin8_mossy",
                    "minecraft:underwater_ruin/mossy_8",
                );
                map.insert(
                    "minecraft:ruin/ruin_warm1",
                    "minecraft:underwater_ruin/warm_1",
                );
                map.insert(
                    "minecraft:ruin/ruin_warm2",
                    "minecraft:underwater_ruin/warm_2",
                );
                map.insert(
                    "minecraft:ruin/ruin_warm3",
                    "minecraft:underwater_ruin/warm_3",
                );
                map.insert(
                    "minecraft:ruin/ruin_warm4",
                    "minecraft:underwater_ruin/warm_4",
                );
                map.insert(
                    "minecraft:ruin/ruin_warm5",
                    "minecraft:underwater_ruin/warm_5",
                );
                map.insert(
                    "minecraft:ruin/ruin_warm6",
                    "minecraft:underwater_ruin/warm_6",
                );
                map.insert(
                    "minecraft:ruin/ruin_warm7",
                    "minecraft:underwater_ruin/warm_7",
                );
                map.insert(
                    "minecraft:ruin/ruin_warm8",
                    "minecraft:underwater_ruin/warm_8",
                );
                map.insert(
                    "minecraft:ruin/big_brick_1",
                    "minecraft:underwater_ruin/big_brick_1",
                );
                map.insert(
                    "minecraft:ruin/big_brick_2",
                    "minecraft:underwater_ruin/big_brick_2",
                );
                map.insert(
                    "minecraft:ruin/big_brick_3",
                    "minecraft:underwater_ruin/big_brick_3",
                );
                map.insert(
                    "minecraft:ruin/big_brick_8",
                    "minecraft:underwater_ruin/big_brick_8",
                );
                map.insert(
                    "minecraft:ruin/big_mossy_1",
                    "minecraft:underwater_ruin/big_mossy_1",
                );
                map.insert(
                    "minecraft:ruin/big_mossy_2",
                    "minecraft:underwater_ruin/big_mossy_2",
                );
                map.insert(
                    "minecraft:ruin/big_mossy_3",
                    "minecraft:underwater_ruin/big_mossy_3",
                );
                map.insert(
                    "minecraft:ruin/big_mossy_8",
                    "minecraft:underwater_ruin/big_mossy_8",
                );
                map.insert(
                    "minecraft:ruin/big_cracked_1",
                    "minecraft:underwater_ruin/big_cracked_1",
                );
                map.insert(
                    "minecraft:ruin/big_cracked_2",
                    "minecraft:underwater_ruin/big_cracked_2",
                );
                map.insert(
                    "minecraft:ruin/big_cracked_3",
                    "minecraft:underwater_ruin/big_cracked_3",
                );
                map.insert(
                    "minecraft:ruin/big_cracked_8",
                    "minecraft:underwater_ruin/big_cracked_8",
                );
                map.insert(
                    "minecraft:ruin/big_warm_4",
                    "minecraft:underwater_ruin/big_warm_4",
                );
                map.insert(
                    "minecraft:ruin/big_warm_5",
                    "minecraft:underwater_ruin/big_warm_5",
                );
                map.insert(
                    "minecraft:ruin/big_warm_6",
                    "minecraft:underwater_ruin/big_warm_6",
                );
                map.insert(
                    "minecraft:ruin/big_warm_7",
                    "minecraft:underwater_ruin/big_warm_7",
                );
                map
            }),
        );
        map
    })
}

pub(crate) fn register(types: &MinecraftTypesMut) {
    types
        .structure_feature
        .borrow_mut()
        .add_structure_converter(
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                let Some(Value::String(id)) = data.get("id") else {
                    return;
                };

                let Some(StructureRenames {
                    child_id,
                    template_renames,
                }) = renames().get(&id[..])
                else {
                    return;
                };

                if let Some(Value::List(List::Compound(children))) = data.get_mut("Children") {
                    for child in children {
                        if matches!(child.get("id"), Some(Value::String(str)) if str == *child_id) {
                            let Some(Value::String(template)) = child.get_mut("Template") else {
                                continue;
                            };
                            if let Some(new_template) = template_renames.get(&template[..]).copied()
                            {
                                *template = new_template.to_owned();
                            }
                        }
                    }
                }
            }),
        );
}
