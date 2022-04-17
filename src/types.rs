use std::pin::Pin;
use rust_dataconverter_engine::{IdDataType, MapDataType, ObjectDataType, Types};

pub struct MinecraftTypes<'a, T: Types + ?Sized> {
    // private field to prevent struct being initialized via member initialization,
    // allows public fields to be added without it being a breaking change.
    _private: (),
    pub level: MapDataType<'a, T>,
    pub player: MapDataType<'a, T>,
    pub chunk: MapDataType<'a, T>,
    pub hotbar: MapDataType<'a, T>,
    pub options: MapDataType<'a, T>,
    pub structure: MapDataType<'a, T>,
    pub stats: MapDataType<'a, T>,
    pub saved_data: MapDataType<'a, T>,
    pub advancements: MapDataType<'a, T>,
    pub poi_chunk: MapDataType<'a, T>,
    pub entity_chunk: MapDataType<'a, T>,
    pub tile_entity: IdDataType<'a, T>,
    pub item_stack: IdDataType<'a, T>,
    pub block_state: MapDataType<'a, T>,
    pub entity_name: ObjectDataType<'a, T>,
    pub entity: IdDataType<'a, T>,
    pub block_name: ObjectDataType<'a, T>,
    pub item_name: ObjectDataType<'a, T>,
    pub untagged_spawner: MapDataType<'a, T>,
    pub structure_feature: MapDataType<'a, T>,
    pub objective: MapDataType<'a, T>,
    pub team: MapDataType<'a, T>,
    pub recipe: ObjectDataType<'a, T>,
    pub biome: ObjectDataType<'a, T>,
    pub world_gen_settings: MapDataType<'a, T>,
}

impl<'a, T: Types + ?Sized> MinecraftTypes<'a, T> {
    pub fn new() -> Self {
        Self {
            _private: (),
            level: MapDataType::new("Level"),
            player: MapDataType::new("Player"),
            chunk: MapDataType::new("Chunk"),
            hotbar: MapDataType::new("CreativeHotbar"),
            options: MapDataType::new("Options"),
            structure: MapDataType::new("Structure"),
            stats: MapDataType::new("Stats"),
            saved_data: MapDataType::new("SavedData"),
            advancements: MapDataType::new("Advancements"),
            poi_chunk: MapDataType::new("PoiChunk"),
            entity_chunk: MapDataType::new("EntityChunk"),
            tile_entity: IdDataType::new("TileEntity"),
            item_stack: IdDataType::new("ItemStack"),
            block_state: MapDataType::new("BlockState"),
            entity_name: ObjectDataType::new("EntityName"),
            entity: IdDataType::new("Entity"),
            block_name: ObjectDataType::new("BlockName"),
            item_name: ObjectDataType::new("ItemName"),
            untagged_spawner: MapDataType::new("Spawner"),
            structure_feature: MapDataType::new("StructureFeature"),
            objective: MapDataType::new("Objective"),
            team: MapDataType::new("Team"),
            recipe: ObjectDataType::new("RecipeName"),
            biome: ObjectDataType::new("Biome"),
            world_gen_settings: MapDataType::new("MapGenSettings"),
        }
    }

    pub fn populate(self: Pin<&mut MinecraftTypes<'a, T>>) {
        use crate::versions::*;

        // General notes:
        // - Structure converters run before everything.
        // - ID specific converters run after structure converters.
        // - Structure walkers run after id specific converters.
        // - ID specific walkers run after structure walkers.

        v99::register(self);
    }
}
