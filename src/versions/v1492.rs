use crate::types;
use java_string::JavaStr;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 1492;

struct StructureRenames {
    child_id: &'static JavaStr,
    template_renames: BTreeMap<&'static JavaStr, &'static JavaStr>,
}

impl StructureRenames {
    fn new(
        child_id: &'static (impl AsRef<JavaStr> + ?Sized),
        template_renames: BTreeMap<&'static JavaStr, &'static JavaStr>,
    ) -> Self {
        Self {
            child_id: child_id.as_ref(),
            template_renames,
        }
    }
}

static RENAMES: OnceLock<BTreeMap<&'static JavaStr, StructureRenames>> = OnceLock::new();

fn renames() -> &'static BTreeMap<&'static JavaStr, StructureRenames> {
    RENAMES.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert(
            JavaStr::from_str("EndCity"),
            StructureRenames::new("ECP", {
                let mut map = BTreeMap::new();
                map.insert(
                    JavaStr::from_str("second_floor"),
                    JavaStr::from_str("second_floor_1"),
                );
                map.insert(
                    JavaStr::from_str("third_floor"),
                    JavaStr::from_str("third_floor_1"),
                );
                map.insert(
                    JavaStr::from_str("third_floor_c"),
                    JavaStr::from_str("third_floor_2"),
                );
                map
            }),
        );
        map.insert(
            JavaStr::from_str("Mansion"),
            StructureRenames::new("WMP", {
                let mut map = BTreeMap::new();
                map.insert(
                    JavaStr::from_str("carpet_south"),
                    JavaStr::from_str("carpet_south_1"),
                );
                map.insert(
                    JavaStr::from_str("carpet_west"),
                    JavaStr::from_str("carpet_west_1"),
                );
                map.insert(
                    JavaStr::from_str("indoors_door"),
                    JavaStr::from_str("indoors_door_1"),
                );
                map.insert(
                    JavaStr::from_str("indoors_wall"),
                    JavaStr::from_str("indoors_wall_1"),
                );
                map
            }),
        );
        map.insert(
            JavaStr::from_str("Igloo"),
            StructureRenames::new("Iglu", {
                let mut map = BTreeMap::new();
                map.insert(
                    JavaStr::from_str("minecraft:igloo/igloo_bottom"),
                    JavaStr::from_str("minecraft:igloo/bottom"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:igloo/igloo_middle"),
                    JavaStr::from_str("minecraft:igloo/middle"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:igloo/igloo_top"),
                    JavaStr::from_str("minecraft:igloo/top"),
                );
                map
            }),
        );
        map.insert(
            JavaStr::from_str("Ocean_Ruin"),
            StructureRenames::new("ORP", {
                let mut map = BTreeMap::new();
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin1_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_brick_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin2_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_brick_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin3_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_brick_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin8_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_brick_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin1_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_cracked_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin2_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_cracked_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin3_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_cracked_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin8_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_cracked_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin1_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_mossy_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin2_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_mossy_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin3_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_mossy_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin8_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_mossy_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin_warm4"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_warm_4"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin_warm5"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_warm_5"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin_warm6"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_warm_6"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_ruin_warm7"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_warm_7"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin1_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/brick_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin2_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/brick_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin3_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/brick_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin4_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/brick_4"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin5_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/brick_5"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin6_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/brick_6"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin7_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/brick_7"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin8_brick"),
                    JavaStr::from_str("minecraft:underwater_ruin/brick_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin1_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/cracked_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin2_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/cracked_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin3_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/cracked_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin4_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/cracked_4"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin5_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/cracked_5"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin6_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/cracked_6"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin7_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/cracked_7"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin8_cracked"),
                    JavaStr::from_str("minecraft:underwater_ruin/cracked_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin1_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/mossy_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin2_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/mossy_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin3_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/mossy_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin4_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/mossy_4"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin5_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/mossy_5"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin6_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/mossy_6"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin7_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/mossy_7"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin8_mossy"),
                    JavaStr::from_str("minecraft:underwater_ruin/mossy_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin_warm1"),
                    JavaStr::from_str("minecraft:underwater_ruin/warm_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin_warm2"),
                    JavaStr::from_str("minecraft:underwater_ruin/warm_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin_warm3"),
                    JavaStr::from_str("minecraft:underwater_ruin/warm_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin_warm4"),
                    JavaStr::from_str("minecraft:underwater_ruin/warm_4"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin_warm5"),
                    JavaStr::from_str("minecraft:underwater_ruin/warm_5"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin_warm6"),
                    JavaStr::from_str("minecraft:underwater_ruin/warm_6"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin_warm7"),
                    JavaStr::from_str("minecraft:underwater_ruin/warm_7"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/ruin_warm8"),
                    JavaStr::from_str("minecraft:underwater_ruin/warm_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_brick_1"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_brick_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_brick_2"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_brick_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_brick_3"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_brick_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_brick_8"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_brick_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_mossy_1"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_mossy_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_mossy_2"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_mossy_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_mossy_3"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_mossy_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_mossy_8"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_mossy_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_cracked_1"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_cracked_1"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_cracked_2"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_cracked_2"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_cracked_3"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_cracked_3"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_cracked_8"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_cracked_8"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_warm_4"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_warm_4"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_warm_5"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_warm_5"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_warm_6"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_warm_6"),
                );
                map.insert(
                    JavaStr::from_str("minecraft:ruin/big_warm_7"),
                    JavaStr::from_str("minecraft:underwater_ruin/big_warm_7"),
                );
                map
            }),
        );
        map
    })
}

pub(crate) fn register() {
    types::structure_feature_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::String(id)) = data.get("id") else {
                return;
            };

            let Some(StructureRenames {
                child_id,
                template_renames,
            }) = renames().get(&id[..])
            else {
                return;
            };

            if let Some(JValue::List(JList::Compound(children))) = data.get_mut("Children") {
                for child in children {
                    if matches!(child.get("id"), Some(JValue::String(str)) if str == *child_id) {
                        let Some(JValue::String(template)) = child.get_mut("Template") else {
                            continue;
                        };
                        if let Some(new_template) = template_renames.get(&template[..]).copied() {
                            *template = new_template.to_owned();
                        }
                    }
                }
            }
        }),
    );
}
