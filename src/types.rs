use std::cell::Cell;
use std::mem::MaybeUninit;
use std::sync::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};
use world_transmuter_engine::{DynamicDataType, IdDataType, MapDataType, ObjectDataType};

static mut TYPES: MaybeUninit<MinecraftTypes> = MaybeUninit::uninit();
thread_local! {
    static IS_INITIALIZING_ON_THIS_THREAD: Cell<bool> = const { Cell::new(false) };
}
static TYPES_INITIALIZER: Once = Once::new();

fn types() -> &'static MinecraftTypes {
    if !IS_INITIALIZING_ON_THIS_THREAD.with(|v| v.get()) {
        TYPES_INITIALIZER.call_once(|| {
            IS_INITIALIZING_ON_THIS_THREAD.with(|v| v.set(true));

            unsafe {
                // SAFETY: we are inside a call_once, so not possible for this line to be reached
                // multiple times concurrently. Also, since call_once is blocking, this line
                // must be reached before the only read of this value, which is below.
                TYPES = MaybeUninit::new(MinecraftTypes::create_empty());
            }

            // General notes:
            // - Structure converters run before everything.
            // - ID specific converters run after structure converters.
            // - Structure walkers run after id specific converters.
            // - ID specific walkers run after structure walkers.
            crate::versions::register_versions();

            IS_INITIALIZING_ON_THIS_THREAD.with(|v| v.set(false));
        });
    }
    unsafe {
        // SAFETY: this line cannot be reached without the initialization above. See the safety comment there for details.
        TYPES.assume_init_ref()
    }
}

macro_rules! define_minecraft_types {
    ($($field_name:ident $field_name_mut:ident $field_name_ref:ident : $type:ident ($name:literal)),* $(,)?) => {

        struct MinecraftTypes {
            $(
                $field_name: RwLock<$type<'static>>,
            )*
        }

        impl MinecraftTypes {
            fn create_empty() -> Self {
                Self {
                    $(
                        $field_name: RwLock::new($type::new($name)),
                    )*
                }
            }
        }

        $(
        pub fn $field_name() -> RwLockReadGuard<'static, $type<'static>> {
            match types().$field_name.try_read() {
                Ok(guard) => guard,
                Err(TryLockError::WouldBlock) => panic!(concat!("Tried to get an immutable reference to ", stringify!($field_name), " while there is a mutable reference")),
                Err(TryLockError::Poisoned(err)) => panic!("{}", err),
            }
        }

        pub(crate) fn $field_name_mut() -> RwLockWriteGuard<'static, $type<'static>> {
            match types().$field_name.try_write() {
                Ok(guard) => guard,
                Err(TryLockError::WouldBlock) => panic!(concat!("Tried to get an mutable reference to ", stringify!($field_name), " while there are immutable references")),
                Err(TryLockError::Poisoned(err)) => panic!("{}", err),
            }
        }

        #[allow(unused)]
        pub(crate) fn $field_name_ref() -> &'static RwLock<$type<'static>> {
            &types().$field_name
        }
        )*
    }
}

define_minecraft_types! {
    level level_mut level_ref: MapDataType("Level"),
    player player_mut player_ref: MapDataType("Player"),
    chunk chunk_mut chunk_ref: MapDataType("Chunk"),
    hotbar hotbar_mut hotbar_ref: MapDataType("Hotbar"),
    options options_mut options_ref: MapDataType("Options"),
    structure structure_mut structure_ref: MapDataType("Structure"),
    stats stats_mut stats_ref: MapDataType("Stats"),
    advancements advancements_mut advancements_ref: MapDataType("Advancements"),
    poi_chunk poi_chunk_mut poi_chunk_ref: MapDataType("PoiChunk"),
    entity_chunk entity_chunk_mut entity_chunk_ref: MapDataType("EntityChunk"),
    tile_entity tile_entity_mut tile_entity_ref: IdDataType("TileEntity"),
    item_stack item_stack_mut item_stack_ref: IdDataType("ItemStack"),
    block_state block_state_mut block_state_ref: MapDataType("BlockName"),
    flat_block_state flat_block_state_mut flat_block_state_ref: ObjectDataType("FlatBlockState"),
    data_components data_components_mut data_components_ref: MapDataType("DataComponents"),
    villager_trade villager_trade_mut villager_trade_ref: MapDataType("VillagerTrade"),
    particle particle_mut particle_ref: DynamicDataType("Particle"),
    entity_name entity_name_mut entity_name_ref: ObjectDataType("EntityName"),
    entity entity_mut entity_ref: IdDataType("Entity"),
    block_name block_name_mut block_name_ref: ObjectDataType("BlockName"),
    item_name item_name_mut item_name_ref: ObjectDataType("ItemName"),
    untagged_spawner untagged_spawner_mut untagged_spawner_ref: MapDataType("Spawner"),
    structure_feature structure_feature_mut structure_feature_ref: MapDataType("StructureFeature"),
    objective objective_mut objective_ref: MapDataType("Objective"),
    team team_mut team_ref: MapDataType("Team"),
    recipe recipe_mut recipe_ref: ObjectDataType("RecipeName"),
    biome biome_mut biome_ref: ObjectDataType("Biome"),
    world_gen_settings world_gen_settings_mut world_gen_settings_ref: MapDataType("WorldGenSettings"),
    game_event_name game_event_name_mut game_event_name_ref: ObjectDataType("GameEventName"),

    multi_noise_biome_source_parameter_list multi_noise_biome_source_parameter_list_mut multi_noise_biome_source_parameter_list_ref: ObjectDataType("MultiNoiseBiomeSourceParameterList"),

    saved_data_random_sequences saved_data_random_sequences_mut saved_data_random_sequences_ref: MapDataType("SavedData/RandomSequences"),
    saved_data_scoreboard saved_data_scoreboard_mut saved_data_scoreboard_ref: MapDataType("SavedData/Scoreboard"),
    saved_data_structure_feature_indices saved_data_structure_feature_indices_mut saved_data_structure_feature_indices_ref: MapDataType("SavedData/StructureFeatureIndices"),
    saved_data_map_data saved_data_map_data_mut saved_data_map_data_ref: MapDataType("SavedData/MapData"),
    saved_data_raids saved_data_raids_mut saved_data_raids_ref: MapDataType("SavedData/Raids"),
    saved_data_command_storage saved_data_command_strage_mut saved_data_command_storage_ref: MapDataType("SavedData/CommandStorage"),
    saved_data_forced_chunks saved_data_forced_chunks_mut saved_data_forced_chunks_ref: MapDataType("SavedData/Chunks"),
    saved_data_map_index saved_data_map_index_mut saved_data_map_index_ref: MapDataType("SavedData/IdCounts"),
}
