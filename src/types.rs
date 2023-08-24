use rust_dataconverter_engine::{IdDataType, MapDataType, ObjectDataType};
use std::cell::{Ref, RefCell};
use std::pin::Pin;

macro_rules! define_minecraft_types {
    ($($field_name:ident : $type:ident ($name:literal)),* $(,)?) => {
        #[repr(C)] // important for safety of pinned field offsets
        struct MinecraftTypesArena<'a> {
            $(
                $field_name: RefCell<$type<'a>>,
            )*
        }

        pub(crate) struct MinecraftTypesMut<'a> {
            $(
                pub(crate) $field_name: &'a RefCell<$type<'a>>,
            )*
            arena: Pin<Box<MinecraftTypesArena<'a>>>,
        }

        pub struct MinecraftTypes<'a> {
            arena: Pin<Box<MinecraftTypesArena<'a>>>,
        }

        impl<'a> MinecraftTypes<'a> {
            $(
                pub fn $field_name(&'a self) -> Ref<'a, $type<'a>> {
                    self.arena.$field_name.borrow()
                }
            )*

            fn create_empty() -> MinecraftTypesMut<'a> {
                let arena = Box::pin(
                    MinecraftTypesArena {
                        $(
                            $field_name: RefCell::new($type::new($name)),
                        )*
                    }
                );

                MinecraftTypesMut {
                    $(
                        // SAFETY: outer struct is still pinned for the lifetime of the reference, and has a defined repr
                        $field_name: unsafe { &*(&Pin::into_inner_unchecked(arena.as_ref()).$field_name as *const _) },
                    )*
                    arena,
                }
            }

            fn to_minecraft_types(types: MinecraftTypesMut<'a>) -> Self {
                Self {
                    arena: types.arena,
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
    pub fn create() -> Self {
        use crate::versions::*;

        let ret = Self::create_empty();

        // General notes:
        // - Structure converters run before everything.
        // - ID specific converters run after structure converters.
        // - Structure walkers run after id specific converters.
        // - ID specific walkers run after structure walkers.

        register_versions(&ret);

        Self::to_minecraft_types(ret)
    }
}
