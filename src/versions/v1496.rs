use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::helpers::bit_storage::{PackedBitStorage, SectionInitializer, Section, LocalPos, Direction};
use crate::helpers::block_state::{BlockState, BlockStateOwned};
use crate::helpers::flatten_chunk_v1451;
use crate::{MinecraftTypesMut, block_state_owned};

const VERSION: u32 = 1496;

static LEAVES_TO_ID: SyncOnceCell<rust_dataconverter_engine::Map<String, u8>> = SyncOnceCell::new();

fn leaves_to_id() -> &'static rust_dataconverter_engine::Map<String, u8> {
    LEAVES_TO_ID.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:acacia_leaves".to_owned(), 0);
        map.insert("minecraft:birch_leaves".to_owned(), 1);
        map.insert("minecraft:dark_oak_leaves".to_owned(), 2);
        map.insert("minecraft:jungle_leaves".to_owned(), 3);
        map.insert("minecraft:oak_leaves".to_owned(), 4);
        map.insert("minecraft:spruce_leaves".to_owned(), 5);
        map
    })
}

static LOGS: SyncOnceCell<rust_dataconverter_engine::Map<String, ()>> = SyncOnceCell::new();

fn logs() -> &'static rust_dataconverter_engine::Map<String, ()> {
    LOGS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:acacia_bark".to_owned(), ());
        map.insert("minecraft:birch_bark".to_owned(), ());
        map.insert("minecraft:dark_oak_bark".to_owned(), ());
        map.insert("minecraft:jungle_bark".to_owned(), ());
        map.insert("minecraft:oak_bark".to_owned(), ());
        map.insert("minecraft:spruce_bark".to_owned(), ());
        map.insert("minecraft:acacia_log".to_owned(), ());
        map.insert("minecraft:birch_log".to_owned(), ());
        map.insert("minecraft:dark_oak_log".to_owned(), ());
        map.insert("minecraft:jungle_log".to_owned(), ());
        map.insert("minecraft:oak_log".to_owned(), ());
        map.insert("minecraft:spruce_log".to_owned(), ());
        map.insert("minecraft:stripped_acacia_log".to_owned(), ());
        map.insert("minecraft:stripped_birch_log".to_owned(), ());
        map.insert("minecraft:stripped_dark_oak_log".to_owned(), ());
        map.insert("minecraft:stripped_jungle_log".to_owned(), ());
        map.insert("minecraft:stripped_oak_log".to_owned(), ());
        map.insert("minecraft:stripped_spruce_log".to_owned(), ());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let level = match data.get_map_mut("Level") {
            Some(level) => level,
            None => return
        };

        let chunk_x = level.get_i64("xPos").unwrap_or(0) as i32;
        let chunk_z = level.get_i64("zPos").unwrap_or(0) as i32;

        let sections = match level.get_list_mut("Sections") {
            Some(sections) => sections,
            None => return
        };

        let mut new_sides = 0;

        const NONE: Option<Section<PackedBitStorage<&mut Vec<i64>>>> = None;
        let mut sections_arr = [NONE; 16];

        let mut skippable = true;

        for section in sections.iter_mut() {
            if let Some(section) = section.as_map_mut() {
                if let Some(section_obj) = Section::new::<T>(chunk_x, chunk_z, section, &mut LeavesSectionInitializer) {
                    let section_y = section_obj.section_y;
                    if section_y >= 0 && (section_y as usize) < sections_arr.len() {
                        sections_arr[section_y as usize] = Some(section_obj);
                        skippable = false;
                    }
                }
            }
        }

        if skippable {
            return;
        }

        let mut positions_equal = rust_dataconverter_engine::Map::new();

        for section in sections_arr.iter().flatten() {
            for index in 0..4096 {
                let pos = LocalPos { index };
                let block = section.get_block(pos);
                if block.map(|block| logs().contains_key(&block.name)) == Some(true) {
                    positions_equal.insert(pos.with_section_y(section.section_y as u8), ());
                } else if block.map(|block| leaves_to_id().contains_key(&block.name)) == Some(true) {
                    new_sides |= flatten_chunk_v1451::get_side_mask(pos.x() == 0, pos.x() == 15, pos.z() == 0, pos.z() == 15);
                }
            }
        }

        // this is basically supposed to recalculate the distances, because a higher cap was added
        for distance in 1..7 {
            let positions_less = positions_equal;
            positions_equal = rust_dataconverter_engine::Map::new();

            for pos in positions_less.keys().copied() {
                for dir in Direction::VALUES {
                    let _: Option<_> = try {
                        let to_pos = pos.try_offset(dir)?;
                        let to_section = (&mut sections_arr[(to_pos.y() >> 4) as usize]).as_mut()?;
                        let to_block = to_section.get_block(to_pos)?;
                        if leaves_to_id().contains_key(&to_block.name) {
                            let old_distance = to_block.properties.get("distance").and_then(|d| d.parse::<u8>().ok()).unwrap_or(0);
                            if (distance as u8) < old_distance {
                                set_leaves_distance(to_section, to_pos, distance as u8);
                                positions_equal.insert(to_pos, ());
                            }
                        }
                    };
                }
            }
        }

        let palettes: [_; 16] = sections_arr.map(|o| o.map(|sec| sec.palette));

        for section in sections.iter_mut() {
            if let Some(section) = section.as_map_mut() {
                let y = section.get_i64("Y").unwrap_or(0) as i32;
                if y >= 0 && (y as usize) < palettes.len() {
                    if let Some(palette) = &palettes[y as usize] {
                        let mut palette_nbt = T::List::create_empty();
                        for state in palette {
                            palette_nbt.add(T::Object::create_map(state.to_nbt::<T>()));
                        }
                        section.set("Palette", T::Object::create_list(palette_nbt));
                    }
                }
            }
        }

        // if sides changed during process, update it now
        if new_sides != 0 {
            if level.get_map("UpgradeData").is_none() {
                level.set("UpgradeData", T::Object::create_map(T::Map::create_empty()));
            }
            let upgrade_data = level.get_map_mut("UpgradeData").unwrap();
            upgrade_data.set("Sides", T::Object::create_byte(new_sides as i8));
        }
    }));
}

fn set_leaves_distance(
    section: &mut Section<PackedBitStorage<&mut Vec<i64>>>,
    pos: LocalPos,
    distance: u8
) {
    let old_block = section.get_block(pos).unwrap();
    let persistent = old_block.properties.get("persistent").map(|str| str == "true").unwrap_or(false);
    let new_state = block_state_owned!(old_block.name.clone(); ["persistent" => persistent.to_string(), "distance" => distance.to_string()]);
    section.set_block::<PackedBitStorage<Vec<i64>>>(pos, new_state);
}

struct LeavesSectionInitializer;

impl<T: Types + ?Sized> SectionInitializer<T> for LeavesSectionInitializer {
    fn init_skippable(&mut self, palette: &mut [BlockState], _section_y: i32) -> bool {
        let mut skippable = true;

        for state in palette {
            if leaves_to_id().contains_key(state.name) {
                let persistent = state.properties.get("decayable").copied() == Some("false");

                state.properties.clear();
                state.properties.insert("persistent", if persistent { "true" } else { "false" });
                state.properties.insert("distance", "7");

                skippable = false;
            } else if logs().contains_key(state.name) {
                skippable = false;
            }
        }

        skippable
    }
}
