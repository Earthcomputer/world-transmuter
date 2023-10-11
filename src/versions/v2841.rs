use crate::helpers::bit_storage::{AlignedBitStorage, LocalPos, NullSectionInitializer, Section};
use crate::helpers::block_state::BlockStateOwned;
use crate::{static_string_mc_set, types};
use ahash::AHashMap;
use java_string::JavaStr;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{map_data_converter_func, rename_key, JCompound, JList, JValue};

const VERSION: u32 = 2841;

static_string_mc_set! {
    ALWAYS_WATERLOGGED, always_waterlogged, {
        "bubble_column",
        "kelp",
        "kelp_plant",
        "seagrass",
        "tall_seagrass",
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

            // Why it's renamed here and not the next data version is beyond me.
            if matches!(level.get("LiquidTicks"), Some(JValue::Compound(_))) {
                rename_key(level, "LiquidTicks", "fluid_ticks");
            }

            let mut min_section = 0;
            if let Some(JValue::List(JList::Compound(sections))) = level.get("Sections") {
                for section in sections {
                    let section_y = section.get("Y").and_then(|v| v.as_i32()).unwrap_or(0);
                    if section_y < min_section && section.contains_key("biomes") {
                        min_section = section_y;
                    }
                }
            }

            level.insert("yPos", min_section as i8);

            if level.contains_key("fluid_ticks") || level.contains_key("TileTicks") {
                return;
            }

            let fluid_ticks = level.remove("LiquidsToBeTicked");
            let block_ticks = level.remove("ToBeTicked");

            let mut section_blocks = AHashMap::new();
            if let Some(JValue::List(JList::Compound(sections))) = level.get("Sections") {
                for section in sections {
                    let section_y = section.get("Y").and_then(|v| v.as_i32()).unwrap_or(0);
                    if let Some(block_states) = Section::<AlignedBitStorage<&[i64]>>::wrap_2832(
                        chunk_x,
                        chunk_z,
                        section,
                        &mut NullSectionInitializer,
                    ) {
                        section_blocks.insert(section_y, block_states);
                    }
                }
            }

            let fluid_ticks = migrate_tick_list(
                fluid_ticks,
                false,
                &section_blocks,
                chunk_x,
                min_section,
                chunk_z,
            );
            let block_ticks = migrate_tick_list(
                block_ticks,
                true,
                &section_blocks,
                chunk_x,
                min_section,
                chunk_z,
            );
            level.insert("fluid_ticks", JList::Compound(fluid_ticks));
            level.insert("block_ticks", JList::Compound(block_ticks));
        }),
    );
}

fn migrate_tick_list(
    ticks: Option<JValue>,
    block_ticks: bool,
    section_blocks: &AHashMap<i32, Section<AlignedBitStorage<&[i64]>>>,
    section_x: i32,
    min_section: i32,
    section_z: i32,
) -> Vec<JCompound> {
    let mut ret = Vec::new();

    let Some(JValue::List(JList::List(ticks))) = ticks else {
        return ret;
    };

    for (section_index, section_ticks) in ticks.into_iter().enumerate() {
        let section_y = section_index as i32 + min_section;
        if let JList::Short(section_ticks) = section_ticks {
            let palette = section_blocks.get(&section_y);
            for local_index in section_ticks {
                let local_index = LocalPos::from_raw(local_index as u16);
                let state = palette.and_then(|palette| palette.get_block(local_index));
                let subject_id = if block_ticks {
                    state
                        .map(|state| &state.name[..])
                        .unwrap_or(JavaStr::from_str("minecraft:air"))
                } else {
                    state
                        .map(get_liquid_id)
                        .unwrap_or(JavaStr::from_str("minecraft:empty"))
                };
                let new_tick =
                    create_new_tick(subject_id, local_index, section_x, section_y, section_z);
                ret.push(new_tick);
            }
        }
    }

    ret
}

fn create_new_tick(
    subject_id: &JavaStr,
    local_index: LocalPos,
    section_x: i32,
    section_y: i32,
    section_z: i32,
) -> JCompound {
    jcompound! {
        "i" => subject_id,
        "x" => local_index.x() as i32 + (section_x << 4),
        "y" => local_index.y() as i32 + (section_y << 4),
        "z" => local_index.z() as i32 + (section_z << 4),
        "t" => 0,
        "p" => 0,
    }
}

pub fn get_block_id(state: Option<&JCompound>) -> &JavaStr {
    if let Some(state) = state {
        if let Some(JValue::String(name)) = state.get("Name") {
            return &name[..];
        }
    }
    JavaStr::from_str("minecraft:air")
}

fn get_liquid_id(state: &BlockStateOwned) -> &JavaStr {
    if always_waterlogged().contains(&state.name[..]) {
        return JavaStr::from_str("minecraft:water");
    }

    // Both vanilla and Paper's DataConverter handle this incorrectly, they assume that the blockstate properties' string values are integers and booleans
    // See https://github.com/PaperMC/DataConverter/issues/6
    if state.name == "minecraft:water" {
        if state
            .get_property("level")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0)
            == 0
        {
            JavaStr::from_str("minecraft:water")
        } else {
            JavaStr::from_str("minecraft:flowing_water")
        }
    } else if state.name == "minecraft:lava" {
        if state
            .get_property("level")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0)
            == 0
        {
            JavaStr::from_str("minecraft:lava")
        } else {
            JavaStr::from_str("minecraft:flowing_lava")
        }
    } else {
        if state
            .get_property("waterlogged")
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false)
        {
            JavaStr::from_str("minecraft:water")
        } else {
            JavaStr::from_str("minecraft:empty")
        }
    }
}
