use rust_dataconverter_engine::{IdDataType, MapDataType, ObjectDataType};
use std::cell::{Ref, RefCell};

macro_rules! define_minecraft_types {
    ($($field_name:ident : $type:ident ($name:literal)),* $(,)?) => {
        pub struct MinecraftTypes<'a> {
            $(
                pub(crate) $field_name: RefCell<$type<'a>>,
            )*
        }

        impl<'a> MinecraftTypes<'a> {
            $(
                pub fn $field_name(&'a self) -> Ref<'a, $type<'a>> {
                    self.$field_name.borrow()
                }
            )*

            pub fn create_empty() -> Self {
                Self {
                    $(
                        $field_name: RefCell::new($type::new($name)),
                    )*
                }
            }
        }
    }
}

define_minecraft_types! {
    level: MapDataType("Level"),
    player: MapDataType("Player"),
    chunk: MapDataType("Chunk"),
    hotbar: MapDataType("Hotbar"),
    options: MapDataType("Options"),
    structure: MapDataType("Structure"),
    stats: MapDataType("Stats"),
    saved_data: MapDataType("SavedData"),
    advancements: MapDataType("Advancements"),
    poi_chunk: MapDataType("PoiChunk"),
    entity_chunk: MapDataType("EntityChunk"),
    tile_entity: IdDataType("TileEntity"),
    item_stack: IdDataType("ItemStack"),
    block_state: MapDataType("BlockName"),
    entity_name: ObjectDataType("EntityName"),
    entity: IdDataType("Entity"),
    block_name: ObjectDataType("BlockName"),
    item_name: ObjectDataType("ItemName"),
    untagged_spawner: MapDataType("Spawner"),
    structure_feature: MapDataType("StructureFeature"),
    objective: MapDataType("Objective"),
    team: MapDataType("Team"),
    recipe: ObjectDataType("RecipeName"),
    biome: ObjectDataType("Biome"),
    world_gen_settings: MapDataType("WorldGenSettings"),
    game_event_name: ObjectDataType("GameEventName"),

    multi_noise_biome_source_parameter_list: ObjectDataType("MULTI_NOISE_BIOME_SOURCE_PARAMETER_LIST"),
}

impl<'a> MinecraftTypes<'a> {
    pub fn register_versions(&'a self) {
        // General notes:
        // - Structure converters run before everything.
        // - ID specific converters run after structure converters.
        // - Structure walkers run after id specific converters.
        // - ID specific walkers run after structure walkers.
        crate::versions::register_versions(self);
    }
}
