use log::warn;
use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::helpers::bit_storage::{LocalPos, PackedBitStorage, Section, SectionInitializer};
use crate::helpers::block_state::BlockState;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1624;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            let chunk_x = level.get_i64("xPos").unwrap_or(0) as i32;
            let chunk_z = level.get_i64("zPos").unwrap_or(0) as i32;

            if let [Some(sections), Some(tile_entities)] = level.get_mut_multi(["Sections", "TileEntities"]) {
                if let (Some(sections), Some(tile_entities)) = (sections.as_list_mut(), tile_entities.as_list_mut()) {
                    let mut positions = rust_dataconverter_engine::Map::new();
                    for section in sections.iter_mut() {
                        if let Some(section) = section.as_map_mut() {
                            if let Some(section_obj) = Section::<PackedBitStorage<_>>::new::<T>(chunk_x, chunk_z, section, &mut TrappedChestSectionInitializer) {
                                for index in 0..4096 {
                                    let pos = LocalPos { index };
                                    if section_obj.get_block(pos).map(|block| block.name == "minecraft:trapped_chest") == Some(true) {
                                        positions.insert(pos.with_section_y(section_obj.section_y as u8), ());
                                    }
                                }
                            }
                        }
                    }
                    for tile_entity in tile_entities.iter_mut() {
                        if let Some(tile_entity) = tile_entity.as_map_mut() {
                            let x = tile_entity.get_i64("x").unwrap_or(0) as i32;
                            let y = tile_entity.get_i64("y").unwrap_or(0) as i32;
                            let z = tile_entity.get_i64("z").unwrap_or(0) as i32;
                            if positions.contains_key(&LocalPos::new((x & 15) as u8, y as u8, (z & 15) as u8)) {
                                if tile_entity.get_string("id") != Some("minecraft:chest") {
                                    warn!("Block Entity ({},{},{}) was expected to be a chest (V1624)", x, y, z);
                                }
                                tile_entity.set("id", T::Object::create_string("minecraft:trapped_chest".to_owned()));
                            }
                        }
                    }
                }
            }
        }
    }));
}

struct TrappedChestSectionInitializer;

impl<T: Types + ?Sized> SectionInitializer<T> for TrappedChestSectionInitializer {
    fn init_skippable(&mut self, palette: &mut [BlockState], _section_y: i32) -> bool {
        palette.iter().any(|block| block.name == "minecraft:trapped_chest")
    }
}
