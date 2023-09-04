use crate::helpers::bit_storage::{LocalPos, PackedBitStorage, Section, SectionInitializer};
use crate::helpers::block_state::BlockState;
use crate::types;
use ahash::AHashSet;
use log::warn;
use valence_nbt::{List, Value};
use world_transmuter_engine::{get_mut_multi, map_data_converter_func};

const VERSION: u32 = 1624;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        let Some(Value::Compound(level)) = data.get_mut("Level") else { return };
        let chunk_x = level.get("xPos").and_then(|v| v.as_i32()).unwrap_or(0);
        let chunk_z = level.get("zPos").and_then(|v| v.as_i32()).unwrap_or(0);

        let [Some(Value::List(List::Compound(sections))), Some(Value::List(List::Compound(tile_entities)))] = get_mut_multi(level, ["Sections", "TileEntities"]) else { return };
        let mut positions = AHashSet::new();
        for section in sections {
            if let Some(section_obj) = Section::<PackedBitStorage<_>>::wrap_1451(chunk_x, chunk_z, section, &mut TrappedChestSectionInitializer) {
                for index in 0..4096 {
                    let pos = LocalPos::from_raw(index);
                    if section_obj.get_block(pos).map(|block| block.name == "minecraft:trapped_chest") == Some(true) {
                        positions.insert(pos.with_section_y(section_obj.section_y as u8));
                    }
                }
            }
        }
        for tile_entity in tile_entities {
            let x = tile_entity.get("x").and_then(|v| v.as_i32()).unwrap_or(0);
            let y = tile_entity.get("y").and_then(|v| v.as_i32()).unwrap_or(0);
            let z = tile_entity.get("z").and_then(|v| v.as_i32()).unwrap_or(0);
            if positions.contains(&LocalPos::new((x & 15) as u8, y as u8, (z & 15) as u8)) {
                if !matches!(tile_entity.get("id"), Some(Value::String(str)) if str == "minecraft:chest") {
                    warn!("Block Entity ({},{},{}) was expected to be a chest (V1624)", x, y, z);
                }
                tile_entity.insert("id", "minecraft:trapped_chest");
            }
        }
    }));
}

struct TrappedChestSectionInitializer;

impl SectionInitializer for TrappedChestSectionInitializer {
    fn init_skippable(&mut self, palette: &mut [BlockState], _section_y: i32) -> bool {
        palette
            .iter()
            .any(|block| block.name == "minecraft:trapped_chest")
    }
}
