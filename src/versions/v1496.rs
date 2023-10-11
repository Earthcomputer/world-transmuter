use crate::helpers::bit_storage::{
    Direction, LocalPos, PackedBitStorage, Section, SectionInitializer,
};
use crate::helpers::block_state::BlockState;
use crate::helpers::flatten_chunk_v1451;
use crate::helpers::mc_namespace_map::McNamespaceMap;
use crate::{block_state_owned, static_string_set, types};
use ahash::AHashSet;
use java_string::JavaStr;
use std::sync::OnceLock;
use world_transmuter_engine::{map_data_converter_func, JCompound, JList, JValue};

const VERSION: u32 = 1496;

static LEAVES_TO_ID: OnceLock<McNamespaceMap<u8>> = OnceLock::new();

fn leaves_to_id() -> &'static McNamespaceMap<'static, u8> {
    LEAVES_TO_ID.get_or_init(|| {
        let mut map = McNamespaceMap::new();
        map.insert_mc("acacia_leaves", 0);
        map.insert_mc("birch_leaves", 1);
        map.insert_mc("dark_oak_leaves", 2);
        map.insert_mc("jungle_leaves", 3);
        map.insert_mc("oak_leaves", 4);
        map.insert_mc("spruce_leaves", 5);
        map
    })
}

static_string_set! {
    LOGS, logs, {
        "acacia_bark",
        "birch_bark",
        "dark_oak_bark",
        "jungle_bark",
        "oak_bark",
        "spruce_bark",
        "acacia_log",
        "birch_log",
        "dark_oak_log",
        "jungle_log",
        "oak_log",
        "spruce_log",
        "stripped_acacia_log",
        "stripped_birch_log",
        "stripped_dark_oak_log",
        "stripped_jungle_log",
        "stripped_oak_log",
        "stripped_spruce_log",
    }
}

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(level)) = data.get_mut("Level") else {
                return;
            };

            let chunk_x = level.get("xPos").and_then(|v| v.as_i32()).unwrap_or(0);
            let chunk_z = level.get("zPos").and_then(|v| v.as_i32()).unwrap_or(0);

            let Some(JValue::List(JList::Compound(sections))) = level.get_mut("Sections") else {
                return;
            };

            let mut new_sides = 0;

            const NONE: Option<Section<PackedBitStorage<&mut Vec<i64>>>> = None;
            let mut sections_arr = [NONE; 16];

            let mut skippable = true;

            for section in sections.iter_mut() {
                if let Some(section_obj) =
                    Section::wrap_1451(chunk_x, chunk_z, section, &mut LeavesSectionInitializer)
                {
                    let section_y = section_obj.section_y;
                    if section_y >= 0 && (section_y as usize) < sections_arr.len() {
                        sections_arr[section_y as usize] = Some(section_obj);
                        skippable = false;
                    }
                }
            }

            if skippable {
                return;
            }

            let mut positions_equal = AHashSet::new();

            for section in sections_arr.iter().flatten() {
                for index in 0..4096 {
                    let pos = LocalPos::from_raw(index);
                    let block = section.get_block(pos);
                    if block.map(|block| logs().contains(&block.name[..])) == Some(true) {
                        positions_equal.insert(pos.with_section_y(section.section_y as u8));
                    } else if block.map(|block| leaves_to_id().contains_key(&block.name))
                        == Some(true)
                    {
                        new_sides |= flatten_chunk_v1451::get_side_mask(
                            pos.x() == 0,
                            pos.x() == 15,
                            pos.z() == 0,
                            pos.z() == 15,
                        );
                    }
                }
            }

            // this is basically supposed to recalculate the distances, because a higher cap was added
            for distance in 1..7 {
                let positions_less = positions_equal;
                positions_equal = AHashSet::new();

                for pos in positions_less {
                    for dir in Direction::VALUES {
                        let Some(to_pos) = pos.try_offset(dir) else {
                            continue;
                        };
                        let Some(to_section) = (sections_arr[(to_pos.y() >> 4) as usize]).as_mut()
                        else {
                            continue;
                        };
                        let Some(to_block) = to_section.get_block(to_pos) else {
                            continue;
                        };
                        if leaves_to_id().contains_key(&to_block.name) {
                            let old_distance = to_block
                                .get_property("distance")
                                .and_then(|d| d.parse::<u8>().ok())
                                .unwrap_or(0);
                            if (distance as u8) < old_distance {
                                set_leaves_distance(to_section, to_pos, distance as u8);
                                positions_equal.insert(to_pos);
                            }
                        }
                    }
                }
            }

            let palettes: [_; 16] = sections_arr.map(|o| o.map(|sec| sec.palette));

            for section in sections {
                let y = section.get("Y").and_then(|v| v.as_i32()).unwrap_or(0);
                if y >= 0 && (y as usize) < palettes.len() {
                    if let Some(palette) = &palettes[y as usize] {
                        let palette_nbt = palette
                            .iter()
                            .map(|state| state.to_nbt())
                            .collect::<Vec<_>>();
                        section.insert("Palette", JList::Compound(palette_nbt));
                    }
                }
            }

            // if sides changed during process, update it now
            if new_sides != 0 {
                if !matches!(level.get("UpgradeData"), Some(JValue::Compound(_))) {
                    level.insert("UpgradeData", JCompound::new());
                }
                let Some(JValue::Compound(upgrade_data)) = level.get_mut("UpgradeData") else {
                    unreachable!()
                };
                upgrade_data.insert("Sides", new_sides as i8);
            }
        }),
    );
}

fn set_leaves_distance(
    section: &mut Section<PackedBitStorage<&mut Vec<i64>>>,
    pos: LocalPos,
    distance: u8,
) {
    let old_block = section.get_block(pos).unwrap();
    let persistent = old_block
        .get_property("persistent")
        .map(|str| str == "true")
        .unwrap_or(false);
    let new_state = block_state_owned!(old_block.name.clone(); ["persistent" => persistent.to_string(), "distance" => distance.to_string()]);
    section.set_block::<PackedBitStorage<Vec<i64>>>(pos, new_state);
}

struct LeavesSectionInitializer;

impl SectionInitializer for LeavesSectionInitializer {
    fn init_skippable(&mut self, palette: &mut [BlockState], _section_y: i32) -> bool {
        let mut skippable = true;

        for state in palette {
            if leaves_to_id().contains_key(state.name) {
                let persistent =
                    state.get_property("decayable") == Some(JavaStr::from_str("false"));

                state.properties.clear();
                state.set_property("persistent", if persistent { "true" } else { "false" });
                state.set_property("distance", "7");

                skippable = false;
            } else if logs().contains(state.name) {
                skippable = false;
            }
        }

        skippable
    }
}
