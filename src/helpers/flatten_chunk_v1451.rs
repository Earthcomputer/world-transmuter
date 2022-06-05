use std::lazy::SyncOnceCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use bitvec::prelude::*;
use log::{error, warn};
use rust_dataconverter_engine::{DataConverterFunc, DataVersion, ListType, MapType, ObjectRef, ObjectType, Types};
use crate::{block_state, block_state_owned, make_bit_arr};
use crate::helpers::{block_flattening_v1450, item_name_v102};
use crate::helpers::bit_storage::{bitset_size, BitStorage, BitStorageMut, BitStorageOwned, ceil_log2, ChunkNibbleArray, LocalPos, PackedBitStorage};
use crate::helpers::block_state::{BlockState, BlockStateOwned};

const VIRTUAL_SET: BitArray<[usize; bitset_size(256)]> = make_bit_arr![256;
    54, 146, 25, 26, 51,
    53, 67, 108, 109, 114,
    128, 134, 135, 136, 156,
    163, 164, 180, 203, 55,
    85, 113, 188, 189, 190,
    191, 192, 93, 94, 101,
    102, 160, 106, 107, 183,
    184, 185, 186, 187, 132,
    139, 199,
];

const IDS_NEEDING_FIX_SET: BitArray<[usize; bitset_size(256)]> = make_bit_arr![256;
    2, 3, 110, 140, 144,
    25, 86, 26, 176, 177,
    175, 64, 71, 193, 194,
    195, 196, 197,
];

struct OwnedStates;
macro_rules! block_states {
    ($(fn $fn_name:ident() -> $field_name:ident, $field_name_owned:ident = block_state!($($tokens:tt)*));* $(;)?) => {
        $(
            static $field_name: SyncOnceCell<BlockState<'static>> = SyncOnceCell::new();
            #[allow(unused)]
            fn $fn_name() -> &'static BlockState<'static> {
                $field_name.get_or_init(|| {
                    block_state!($($tokens)*)
                })
            }
            static $field_name_owned: SyncOnceCell<BlockStateOwned> = SyncOnceCell::new();
            impl OwnedStates {
                #[allow(unused)]
                fn $fn_name() -> &'static BlockStateOwned {
                    $field_name_owned.get_or_init(|| {
                        $fn_name().to_owned()
                    })
                }
            }
        )*
    }
}

block_states! {
    fn pumpkin() -> PUMPKIN, PUMPKIN_OWNED = block_state!("minecraft:pumpkin");
    fn snowy_podzol() -> SNOWY_PODZOL, SNOWY_PODZOL_OWNED = block_state!("minecraft:podzol"["snowy" = "true"]);
    fn snowy_grass() -> SNOWY_GRASS, SNOWY_GRASS_OWNED = block_state!("minecraft:grass"["snowy" = "true"]);
    fn snowy_mycelium() -> SNOWY_MYCELIUM, SNOWY_MYCELIUM_OWNED = block_state!("minecraft:mycelium"["snowy" = "true"]);
    fn upper_sunflower() -> UPPER_SUNFLOWER, UPPER_SUNFLOWER_OWNED = block_state!("minecraft:sunflower"["half" = "upper"]);
    fn upper_lilac() -> UPPER_LILAC, UPPER_LILAC_OWNED = block_state!("minecraft:lilac"["half" = "upper"]);
    fn upper_tall_grass() -> UPPER_TALL_GRASS, UPPER_TALL_GRASS_OWNED = block_state!("minecraft:tall_grass"["half" = "upper"]);
    fn upper_large_fern() -> UPPER_LARGE_FERN, UPPER_LARGE_FERN_OWNED = block_state!("minecraft:large_fern"["half" = "upper"]);
    fn upper_rose_bush() -> UPPER_ROSE_BUSH, UPPER_ROSE_BUSH_OWNED = block_state!("minecraft:rose_bush"["half" = "upper"]);
    fn upper_peony() -> UPPER_PEONY, UPPER_PEONY_OWNED = block_state!("minecraft:peony"["half" = "upper"]);
    fn air() -> AIR, AIR_OWNED = block_state!("minecraft:air");
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct FlowerPotState {
    block_name: String,
    data: u8,
}
impl FlowerPotState {
    fn new(block_name: impl Into<String>, data: u8) -> Self {
        Self { block_name: block_name.into(), data }
    }
}
static FLOWER_POT_MAP: SyncOnceCell<rust_dataconverter_engine::Map<FlowerPotState, BlockState<'static>>> = SyncOnceCell::new();
fn flower_pot_map() -> &'static rust_dataconverter_engine::Map<FlowerPotState, BlockState<'static>> {
    FLOWER_POT_MAP.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert(FlowerPotState::new("minecraft:air", 0), block_state!("minecraft:flower_pot"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 0), block_state!("minecraft:potted_poppy"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 1), block_state!("minecraft:potted_blue_orchid"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 2), block_state!("minecraft:potted_allium"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 3), block_state!("minecraft:potted_azure_bluet"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 4), block_state!("minecraft:potted_red_tulip"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 5), block_state!("minecraft:potted_orange_tulip"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 6), block_state!("minecraft:potted_white_tulip"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 7), block_state!("minecraft:potted_pink_tulip"));
        map.insert(FlowerPotState::new("minecraft:red_flower", 8), block_state!("minecraft:potted_oxeye_daisy"));
        map.insert(FlowerPotState::new("minecraft:yellow_flower", 0), block_state!("minecraft:potted_dandelion"));
        map.insert(FlowerPotState::new("minecraft:sapling", 0), block_state!("minecraft:potted_oak_sapling"));
        map.insert(FlowerPotState::new("minecraft:sapling", 1), block_state!("minecraft:potted_spruce_sapling"));
        map.insert(FlowerPotState::new("minecraft:sapling", 2), block_state!("minecraft:potted_birch_sapling"));
        map.insert(FlowerPotState::new("minecraft:sapling", 3), block_state!("minecraft:potted_jungle_sapling"));
        map.insert(FlowerPotState::new("minecraft:sapling", 4), block_state!("minecraft:potted_acacia_sapling"));
        map.insert(FlowerPotState::new("minecraft:sapling", 5), block_state!("minecraft:potted_dark_oak_sapling"));
        map.insert(FlowerPotState::new("minecraft:red_mushroom", 0), block_state!("minecraft:potted_red_mushroom"));
        map.insert(FlowerPotState::new("minecraft:brown_mushroom", 0), block_state!("minecraft:potted_brown_mushroom"));
        map.insert(FlowerPotState::new("minecraft:deadbush", 0), block_state!("minecraft:potted_dead_bush"));
        map.insert(FlowerPotState::new("minecraft:tallgrass", 2), block_state!("minecraft:potted_fern"));
        map.insert(FlowerPotState::new("minecraft:cactus", 0), block_state!("minecraft:potted_cactus")); // we change default to empty
        map
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct SkullState {
    id: u8,
    dir_or_rotation: String,
}
impl SkullState {
    fn new(id: u8, dir_or_rotation: impl Into<String>) -> Self {
        Self { id, dir_or_rotation: dir_or_rotation.into() }
    }
}
static SKULL_MAP: SyncOnceCell<rust_dataconverter_engine::Map<SkullState, BlockStateOwned>> = SyncOnceCell::new();
fn skull_map() -> &'static rust_dataconverter_engine::Map<SkullState, BlockStateOwned> {
    SKULL_MAP.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        let mut map_skull = |old_id: u8, new_id: &str, skull_type: &str| {
            for dir in ["north", "east", "south", "west"] {
                map.insert(SkullState::new(old_id, dir), block_state_owned!(format!("{}_wall_{}", new_id, skull_type); ["facing" => dir]));
            }
            for rotation in 0..16 {
                map.insert(SkullState::new(old_id, rotation.to_string()), block_state_owned!(format!("{}_{}", new_id, skull_type); ["rotation" => rotation.to_string()]));
            }
        };
        map_skull(0, "skeleton", "skull");
        map_skull(1, "wither_skeleton", "skull");
        map_skull(2, "zombie", "head");
        map_skull(3, "player", "head");
        map_skull(4, "creeper", "head");
        map_skull(5, "dragon", "head");
        map
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct DoorState {
    id: String,
    facing: String,
    half: String,
    hinge: String,
    open: bool,
    powered: bool,
}
impl DoorState {
    fn new(id: impl Into<String>, facing: impl Into<String>, half: impl Into<String>, hinge: impl Into<String>, open: bool, powered: bool) -> Self {
        Self { id: id.into(), facing: facing.into(), half: half.into(), hinge: hinge.into(), open, powered }
    }
}
static DOOR_MAP: SyncOnceCell<rust_dataconverter_engine::Map<DoorState, BlockStateOwned>> = SyncOnceCell::new();
fn door_map() -> &'static rust_dataconverter_engine::Map<DoorState, BlockStateOwned> {
    DOOR_MAP.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        let mut map_door = |typ: &str, old_id: u16| {
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "lower", "left", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "lower", "hinge" => "left", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "lower", "left", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "lower", "hinge" => "left", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "lower", "left", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "lower", "hinge" => "left", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "lower", "left", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "lower", "hinge" => "left", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "lower", "right", false, false), block_flattening_v1450::get_state_for_id_raw(old_id).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "lower", "right", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "lower", "hinge" => "right", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "lower", "right", true, false), block_flattening_v1450::get_state_for_id_raw(old_id + 4).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "lower", "right", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "lower", "hinge" => "right", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "upper", "left", false, false), block_flattening_v1450::get_state_for_id_raw(old_id + 8).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "upper", "left", false, true), block_flattening_v1450::get_state_for_id_raw(old_id + 10).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "upper", "left", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "upper", "hinge" => "left", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "upper", "left", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "upper", "hinge" => "left", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "upper", "right", false, false), block_flattening_v1450::get_state_for_id_raw(old_id + 9).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "upper", "right", false, true), block_flattening_v1450::get_state_for_id_raw(old_id + 11).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "upper", "right", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "upper", "hinge" => "right", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "east", "upper", "right", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "east", "half" => "upper", "hinge" => "right", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "lower", "left", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "lower", "hinge" => "left", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "lower", "left", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "lower", "hinge" => "left", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "lower", "left", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "lower", "hinge" => "left", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "lower", "left", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "lower", "hinge" => "left", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "lower", "right", false, false), block_flattening_v1450::get_state_for_id_raw(old_id + 3).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "lower", "right", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "lower", "hinge" => "right", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "lower", "right", true, false), block_flattening_v1450::get_state_for_id_raw(old_id + 7).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "lower", "right", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "lower", "hinge" => "right", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "upper", "left", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "upper", "hinge" => "left", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "upper", "left", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "upper", "hinge" => "left", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "upper", "left", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "upper", "hinge" => "left", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "upper", "left", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "upper", "hinge" => "left", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "upper", "right", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "upper", "hinge" => "right", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "upper", "right", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "upper", "hinge" => "right", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "upper", "right", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "upper", "hinge" => "right", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "north", "upper", "right", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "north", "half" => "upper", "hinge" => "right", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "lower", "left", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "lower", "hinge" => "left", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "lower", "left", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "lower", "hinge" => "left", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "lower", "left", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "lower", "hinge" => "left", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "lower", "left", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "lower", "hinge" => "left", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "lower", "right", false, false), block_flattening_v1450::get_state_for_id_raw(old_id + 1).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "lower", "right", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "lower", "hinge" => "right", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "lower", "right", true, false), block_flattening_v1450::get_state_for_id_raw(old_id + 5).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "lower", "right", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "lower", "hinge" => "right", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "upper", "left", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "upper", "hinge" => "left", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "upper", "left", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "upper", "hinge" => "left", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "upper", "left", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "upper", "hinge" => "left", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "upper", "left", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "upper", "hinge" => "left", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "upper", "right", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "upper", "hinge" => "right", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "upper", "right", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "upper", "hinge" => "right", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "upper", "right", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "upper", "hinge" => "right", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "south", "upper", "right", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "south", "half" => "upper", "hinge" => "right", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "lower", "left", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "lower", "hinge" => "left", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "lower", "left", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "lower", "hinge" => "left", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "lower", "left", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "lower", "hinge" => "left", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "lower", "left", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "lower", "hinge" => "left", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "lower", "right", false, false), block_flattening_v1450::get_state_for_id_raw(old_id + 2).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "lower", "right", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "lower", "hinge" => "right", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "lower", "right", true, false), block_flattening_v1450::get_state_for_id_raw(old_id + 6).unwrap().to_owned());
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "lower", "right", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "lower", "hinge" => "right", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "upper", "left", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "upper", "hinge" => "left", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "upper", "left", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "upper", "hinge" => "left", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "upper", "left", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "upper", "hinge" => "left", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "upper", "left", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "upper", "hinge" => "left", "open" => "true", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "upper", "right", false, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "upper", "hinge" => "right", "open" => "false", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "upper", "right", false, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "upper", "hinge" => "right", "open" => "false", "powered" => "true"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "upper", "right", true, false), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "upper", "hinge" => "right", "open" => "true", "powered" => "false"]));
            map.insert(DoorState::new(format!("minecraft:{}", typ), "west", "upper", "right", true, true), block_state_owned!(format!("minecraft:{}", typ); ["facing" => "west", "half" => "upper", "hinge" => "right", "open" => "true", "powered" => "true"]));
        };

        map_door("oak_door", 1024);
        map_door("iron_door", 1136);
        map_door("spruce_door", 3088);
        map_door("birch_door", 3104);
        map_door("jungle_door", 3120);
        map_door("acacia_door", 3136);
        map_door("dark_oak_door", 3152);

        map
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct NoteBlockState {
    powered: bool,
    note: u8,
}
impl NoteBlockState {
    fn new(powered: bool, note: u8) -> Self {
        Self { powered, note }
    }
}
static NOTE_BLOCK_MAP: SyncOnceCell<rust_dataconverter_engine::Map<NoteBlockState, BlockStateOwned>> = SyncOnceCell::new();
fn note_block_map() -> &'static rust_dataconverter_engine::Map<NoteBlockState, BlockStateOwned> {
    NOTE_BLOCK_MAP.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        for note in 0..26 {
            for powered in [false, true] {
                map.insert(NoteBlockState::new(powered, note), block_state_owned!("minecraft:note_block"; ["powered" => powered.to_string(), "note" => note.to_string()]));
            }
        }
        map
    })
}

static DYE_COLOR_MAP: [&'static str; 16] = [
    "white", "orange", "magenta", "light_blue",
    "yellow", "lime", "pink", "gray",
    "light_gray", "cyan", "purple", "blue",
    "brown", "green", "red", "black"
];

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct BedState {
    facing: String,
    occupied: bool,
    part: String,
    color: u8,
}
impl BedState {
    fn new(facing: impl Into<String>, occupied: bool, part: impl Into<String>, color: u8) -> Self {
        Self { facing: facing.into(), occupied, part: part.into(), color }
    }
}
static BED_BLOCK_MAP: SyncOnceCell<rust_dataconverter_engine::Map<BedState, BlockStateOwned>> = SyncOnceCell::new();
fn bed_block_map() -> &'static rust_dataconverter_engine::Map<BedState, BlockStateOwned> {
    BED_BLOCK_MAP.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        for (color_id, color_name) in DYE_COLOR_MAP.iter().enumerate() {
            if *color_name == "red" {
                continue;
            }
            for facing in ["north", "east", "south", "west"] {
                for occupied in [false, true] {
                    for part in ["head", "foot"] {
                        map.insert(BedState::new(facing, occupied, part, color_id as u8), block_state_owned!(format!("minecraft:{}_bed", *color_name); ["facing" => facing, "occupied" => occupied.to_string(), "part" => part]));
                    }
                }
            }
        }
        map
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct BannerState {
    rotation_or_facing: String,
    color: u8,
}
impl BannerState {
    fn new(rotation_or_facing: impl Into<String>, color: u8) -> Self {
        Self { rotation_or_facing: rotation_or_facing.into(), color }
    }
}
static BANNER_BLOCK_MAP: SyncOnceCell<rust_dataconverter_engine::Map<BannerState, BlockStateOwned>> = SyncOnceCell::new();
fn banner_block_map() -> &'static rust_dataconverter_engine::Map<BannerState, BlockStateOwned> {
    BANNER_BLOCK_MAP.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();

        for (color_id, color_name) in DYE_COLOR_MAP.iter().rev().enumerate() {
            if *color_name == "white" {
                continue;
            }
            let color_id = color_id as u8;
            for rotation in 0..16 {
                map.insert(BannerState::new(rotation.to_string(), color_id), block_state_owned!(format!("minecraft:{}_banner", *color_name); ["rotation" => rotation.to_string()]));
            }
            for facing in ["north", "east", "south", "west"] {
                map.insert(BannerState::new(facing, color_id), block_state_owned!(format!("minecraft:{}_wall_banner", *color_name); ["facing" => facing]));
            }
        }

        map
    })
}

pub(crate) fn get_side_mask(no_left: bool, no_right: bool, no_back: bool, no_forward: bool) -> u8 {
    match (no_left, no_right, no_back, no_forward) {
        (_, true, true, _) => 2,
        (true, false, true, _) => 128,
        (false, false, true, _) => 1,
        (true, _, false, true) => 32,
        (false, true, false, true) => 8,
        (false, false, false, true) => 16,
        (_, true, false, false) => 4,
        (true, false, false, false) => 64,
        _ => 0
    }
}

pub(crate) struct ConverterFlattenChunk<T: Types + ?Sized> {
    _phantom: PhantomData<T>,
}

impl<T: Types + ?Sized> ConverterFlattenChunk<T> {
    pub(crate) fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}

impl<T: Types + ?Sized> DataConverterFunc<T::Map> for ConverterFlattenChunk<T> {
    fn convert(&self, data: &mut T::Map, _from_version: DataVersion, _to_version: DataVersion) {
        if let Some(level) = data.get_map_mut("Level") {
            UpgradeChunk::<T>::upgrade(level);
        }
    }
}

struct UpgradeChunk<'a, T: Types + ?Sized> {
    sides: u8,
    sections: [Option<Section>; 16],
    block_x: i32,
    block_z: i32,
    tile_entities: rust_dataconverter_engine::Map<LocalPos, &'a T::Map>,
    // in the case of skulls, this doesn't fully remove it, just removes some properties
    tile_entities_to_remove: rust_dataconverter_engine::Map<LocalPos, ()>,
    converted_from_alpha_format: bool,
}

impl<'a, T: Types + ?Sized> UpgradeChunk<'a, T> {
    fn upgrade(level: &mut T::Map) {
        let mut upgrade_chunk = UpgradeChunk::<T>::from_nbt(level);
        upgrade_chunk.do_upgrade();
        let UpgradeChunk { sides, sections, tile_entities_to_remove, .. } = upgrade_chunk;
        Self::write_back_to_level(level, sides, sections, tile_entities_to_remove);
    }

    fn from_nbt(level: &'a T::Map) -> Self {
        let block_x = (level.get_i64("xPos").unwrap_or(0) as i32) << 4;
        let block_z = (level.get_i64("zPos").unwrap_or(0) as i32) << 4;
        let mut tile_entities_map = rust_dataconverter_engine::Map::new();
        if let Some(tile_entities) = level.get_list("TileEntities") {
            for tile_entity in tile_entities.iter() {
                if let Some(tile_entity) = tile_entity.as_map() {
                    let x = ((tile_entity.get_i64("x").unwrap_or(0) as i32 - block_x) & 15) as u8;
                    let y = (tile_entity.get_i64("y").unwrap_or(0) & 255) as u8;
                    let z = ((tile_entity.get_i64("z").unwrap_or(0) as i32 - block_z) & 15) as u8;
                    if tile_entities_map.insert(LocalPos::new(x, y, z), tile_entity).is_some() {
                        warn!("In chunk: {}x{} found a duplicate block entity at position (ConverterFlattenChunk): [{}, {}, {}]", block_x, block_z, x, y, z);
                    }
                }
            }
        }

        let converted_from_alpha_format = level.get_bool("convertedFromAlphaFormat").unwrap_or(false);
        const NONE: Option<Section> = None;
        let mut sections_arr = [NONE; 16];
        let mut sides = 0;
        if let Some(sections) = level.get_list("Sections") {
            for section in sections.iter() {
                if let Some(section) = section.as_map() {
                    let section = Section::from_nbt::<T>(section, &mut sides);
                    let section_y = section.y;
                    if section_y < 0 || section_y > 15 {
                        warn!("In chunk: {}x{} found an invalid chunk section y (ConverterFlattenChunk): {}", block_x, block_z, section_y);
                        continue;
                    }
                    if sections_arr[section_y as usize].is_some() {
                        warn!("In chunk: {}x{} found a duplicate chunk section (ConverterFlattenChunk): {}", block_x, block_z, section_y);
                    }
                    sections_arr[section_y as usize] = Some(section);
                }
            }
        }

        Self {
            sides,
            sections: sections_arr,
            block_x,
            block_z,
            tile_entities: tile_entities_map,
            tile_entities_to_remove: rust_dataconverter_engine::Map::new(),
            converted_from_alpha_format,
        }
    }

    fn do_upgrade(&mut self) {
        for i in 0..self.sections.len() {
            let section = match &mut self.sections[i] {
                Some(sec) => sec,
                None => continue
            };
            let section_y = section.y as u8;

            for (state_id, positions) in std::mem::take(&mut section.to_fix) {
                match state_id {
                    2 => { // grass block
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if pos.y() == 255 {
                                continue;
                            }
                            if self.get_block(pos).name != "minecraft:grass_block" {
                                continue;
                            }
                            let block_above = &self.get_block(pos.up()).name;
                            if block_above == "minecraft:snow" || block_above == "minecraft:snow_layer" {
                                self.set_block(pos, snowy_grass().to_owned());
                            }
                        }
                    }
                    3 => { // dirt
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if pos.y() == 255 {
                                continue;
                            }
                            if self.get_block(pos).name != "minecraft:podzol" {
                                continue;
                            }
                            let block_above = &self.get_block(pos.up()).name;
                            if block_above == "minecraft:snow" || block_above == "minecraft:snow_layer" {
                                self.set_block(pos, snowy_podzol().to_owned());
                            }
                        }
                    },
                    25 => { // note block
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if let Some(tile) = self.remove_tile_entity(pos) {
                                let powered = tile.get_bool("powered").unwrap_or(false);
                                let note = tile.get_i64("note").unwrap_or(0).clamp(0, 24) as u8;
                                let state = note_block_map().get(&NoteBlockState::new(powered, note)).unwrap().clone();
                                self.set_block(pos, state);
                            }
                        }
                    },
                    26 => { // bed
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if let Some(tile) = self.tile_entities.get(&pos).copied() {
                                let color = tile.get_i64("color").unwrap_or(0) as i32;
                                if color != 14 && color >= 0 && color < 16 {
                                    let state = self.get_block(pos);
                                    if let (Some(facing), Some(occupied), Some(part)) = (
                                        state.properties.get("facing"),
                                        state.properties.get("occupied").and_then(|str| str.parse::<bool>().ok()),
                                        state.properties.get("part")
                                    ) {
                                        if let Some(state) = bed_block_map().get(&BedState::new(facing, occupied, part, color as u8)) {
                                            self.set_block(pos, state.clone());
                                        }
                                    }
                                }
                            }
                        }
                    },
                    64 | // oak door
                    71 | // iron door
                    193 | // spruce door
                    194 | // birch door
                    195 | // jungle door
                    196 | // acacia door
                    197 => { // dark oak door
                        // aka the door updater
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if pos.y() == 255 {
                                continue;
                            }
                            let state = self.get_block(pos);
                            if !state.name.ends_with("_door") {
                                continue;
                            }
                            if state.properties.get("half").map(|str| str == "lower") != Some(true) {
                                continue;
                            }

                            let pos_above = pos.up();
                            let state_above = self.get_block(pos_above);
                            if state_above.name != state.name {
                                continue;
                            }

                            if let (Some(facing), Some(open), Some(hinge), Some(powered)) = (
                                state.properties.get("facing").cloned(),
                                state.properties.get("open").and_then(|str| str.parse::<bool>().ok()),
                                if self.converted_from_alpha_format { Some("left".to_owned()) } else { state_above.properties.get("hinge").cloned() },
                                if self.converted_from_alpha_format { Some(false) } else { state_above.properties.get("powered").and_then(|str| str.parse::<bool>().ok()) }
                            ) {
                                let name = state.name.clone();
                                let lower_state = door_map().get(&DoorState::new(name.clone(), facing.clone(), "lower", hinge.clone(), open, powered))
                                    .unwrap_or_else(|| door_map().get(&DoorState::new("minecraft:oak_door", "north", "lower", "left", false, false)).unwrap())
                                    .clone();
                                self.set_block(pos, lower_state);
                                let upper_state = door_map().get(&DoorState::new(name.clone(), facing.clone(), "upper", hinge.clone(), open, powered))
                                    .unwrap_or_else(|| door_map().get(&DoorState::new("minecraft:oak_door", "north", "upper", "left", false, false)).unwrap())
                                    .clone();
                                self.set_block(pos_above, upper_state);
                            }
                        }
                    },
                    86 => { // pumpkin
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if pos.y() == 0 {
                                continue;
                            }
                            let state = self.get_block(pos);

                            // I guess this is some terrible hack to convert carved pumpkins from world gen into
                            // regular pumpkins?

                            if state.name == "minecraft:carved_pumpkin" {
                                let block_below = &self.get_block(pos.down()).name;
                                if block_below == "minecraft:grass_block" || block_below == "minecraft:dirt" {
                                    self.set_block(pos, pumpkin().to_owned());
                                }
                            }
                        }
                    },
                    110 => { // mycelium
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if pos.y() == 255 {
                                continue;
                            }
                            let state = self.get_block(pos);
                            if state.name == "minecraft:mycelium" {
                                let block_above = &self.get_block(pos.up()).name;
                                if block_above == "minecraft:snow" || block_above == "minecraft:snow_layer" {
                                    self.set_block(pos, snowy_mycelium().to_owned());
                                }
                            }
                        }
                    },
                    140 => { // flower pot
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if let Some(tile) = self.remove_tile_entity(pos) {
                                let item = if let Some(id) = tile.get_i64("Item") {
                                    // the item name converter should have migrated to number, however no legacy converter
                                    // ever did this. so we can get data with versions above v102 (old worlds, converted prior to DFU)
                                    // that didn't convert. so just do it here.
                                    item_name_v102::get_name_from_id(id as i32).unwrap_or("")
                                } else {
                                    tile.get_string("Item").unwrap_or("")
                                };
                                let data = tile.get_i64("Data").unwrap_or(0) as u8;

                                let state = flower_pot_map().get(&FlowerPotState::new(item, data))
                                    .unwrap_or_else(|| flower_pot_map().get(&FlowerPotState::new("minecraft:air", 0)).unwrap())
                                    .to_owned();
                                self.set_block(pos, state);
                            }
                        }
                    },
                    144 => { // mob head
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            // in the case of skulls, this doesn't fully remove it, just removes some properties
                            if let Some(tile) = self.remove_tile_entity(pos) {
                                let facing = self.get_block(pos).properties.get("facing").map(|str| str.as_str()).unwrap_or("north");
                                let skull_type = tile.get_i64("SkullType").unwrap_or(0) as u8;
                                let state = if facing == "up" || facing == "down" {
                                    SkullState::new(skull_type, tile.get_i64("Rot").unwrap_or(0).to_string())
                                } else {
                                    SkullState::new(skull_type, facing)
                                };

                                let state = skull_map().get(&state)
                                    .unwrap_or_else(|| skull_map().get(&SkullState::new(0, "north")).unwrap())
                                    .clone();
                                self.set_block(pos, state);
                            }
                        }
                    },
                    175 => { // sunflower
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if pos.y() == 0 {
                                continue;
                            }
                            if self.get_block(pos).properties.get("half").map(|str| str == "upper") != Some(true) {
                                continue;
                            }

                            let state_below = self.get_block(pos.down());
                            match state_below.name.as_str() {
                                "minecraft:sunflower" => {
                                    self.set_block(pos, upper_sunflower().to_owned());
                                },
                                "minecraft:lilac" => {
                                    self.set_block(pos, upper_lilac().to_owned());
                                },
                                "minecraft:tall_grass" => {
                                    self.set_block(pos, upper_tall_grass().to_owned());
                                },
                                "minecraft:large_fern" => {
                                    self.set_block(pos, upper_large_fern().to_owned());
                                },
                                "minecraft:rose_bush" => {
                                    self.set_block(pos, upper_rose_bush().to_owned());
                                },
                                "minecraft:peony" => {
                                    self.set_block(pos, upper_peony().to_owned());
                                }
                                _ => {}
                            }
                        }
                    },
                    176 | // free standing banner
                    177 => { // wall mounted banner
                        for pos in positions {
                            let pos = pos.with_section_y(section_y);
                            if let Some(tile) = self.tile_entities.get(&pos).copied() {
                                let base = tile.get_i64("Base").unwrap_or(0) as i32;
                                if base != 15 && base >= 0 && base < 16 {
                                    let state = self.get_block(pos);
                                    if let Some(rotation_or_facing) = state.properties.get(if state_id == 176 { "rotation" } else { "facing" }) {
                                        if let Some(update) = banner_block_map().get(&BannerState::new(rotation_or_facing, base as u8)) {
                                            self.set_block(pos, update.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn write_back_to_level(
        level: &mut T::Map,
        sides: u8,
        sections: [Option<Section>; 16],
        tile_entities_to_remove: rust_dataconverter_engine::Map<LocalPos, ()>
    ) {
        // apply tile entity removals
        let mut remove_tile_entities = false;
        if let Some(tile_entities) = level.get_list_mut("TileEntities") {
            for index in (0..tile_entities.size()).rev() {
                let mut remove = false;
                if let Some(te) = tile_entities.get_mut(index).as_map_mut() {
                    let pos = LocalPos::new(te.get_i64("x").unwrap_or(0) as u8, te.get_i64("y").unwrap_or(0) as u8, te.get_i64("z").unwrap_or(0) as u8);
                    if tile_entities_to_remove.contains_key(&pos) {
                        if te.get_string("id") == Some("minecraft:skull") {
                            te.remove("SkullType");
                            te.remove("facing");
                            te.remove("Rot");
                        } else {
                            remove = true;
                        }
                    }
                }
                if remove {
                    tile_entities.remove(index);
                }
            }

            remove_tile_entities = tile_entities.is_empty();
        }
        if remove_tile_entities {
            level.remove("TileEntities");
        }

        // rewrite sections and add upgrade data
        let mut indices = T::Map::create_empty();
        let mut sections_list = T::List::create_empty();
        for section in sections {
            if let Some(section) = section {
                indices.set(section.y.to_string(), T::Object::create_int_array(<&Vec<LocalPos>>::into_iter(&section.update).map(|pos| pos.index as i32).collect()));

                // find the existing section with the y coordinate, and write to it
                let existing_section = level.get_list_mut("Section")
                    .and_then(|list| list.iter_mut()
                        .find(|sec| sec.as_map().and_then(|map| map.get_i64("Y")).map(|y| y as i32) == Some(section.y)));
                if let Some(existing_section) = existing_section {
                    let existing_section = existing_section.as_map_mut().unwrap();
                    section.into_nbt::<T>(existing_section);
                    sections_list.add(T::Object::create_map(std::mem::replace(existing_section, T::Map::create_empty())));
                }
            }
        }

        level.set("Sections", T::Object::create_list(sections_list));

        let mut upgrade_data = T::Map::create_empty();
        upgrade_data.set("Sides", T::Object::create_byte(sides as i8));
        upgrade_data.set("Indices", T::Object::create_map(indices));

        level.set("UpgradeData", T::Object::create_map(upgrade_data));
    }

    fn get_section(&self, pos: LocalPos) -> &Option<Section> {
        &self.sections[(pos.y() >> 4) as usize]
    }

    fn get_section_mut(&mut self, pos: LocalPos) -> &mut Option<Section> {
        &mut self.sections[(pos.y() >> 4) as usize]
    }

    fn get_block(&self, pos: LocalPos) -> &BlockStateOwned {
        self.get_section(pos).as_ref()
            .and_then(|sec| sec.buffer.as_ref().map(|buf| (buf, &sec.palette_states)))
            .map(|(buf, palette)| {
                &palette[buf[pos.index as usize & (buf.len() - 1)] as usize]
            })
            .unwrap_or_else(|| OwnedStates::air())
    }

    fn set_block(&mut self, pos: LocalPos, mut state: BlockStateOwned) {
        if state.name == "minecraft:%%FILTER_ME%%" {
            state = OwnedStates::air().clone();
        }

        let (buffer, palette, palette_states) = self.get_section_mut(pos).as_mut()
            .and_then(|sec| sec.buffer.as_mut().map(|buf| (buf, &mut sec.palette, &mut sec.palette_states)))
            .expect("Tried to set a block in a non-existent section");

        let palette_id = if palette.contains_key(&state) {
            *palette.get(&state).unwrap()
        } else {
            let palette_id = palette.len() as u16;
            palette.insert(state.clone(), palette_id);
            palette_states.push(state);
            palette_id
        };

        buffer[pos.index as usize & (buffer.len() - 1)] = palette_id;
    }

    // in the case of skulls, this doesn't fully remove it, just removes some properties
    fn remove_tile_entity(&mut self, pos: LocalPos) -> Option<&'a T::Map> {
        let te = self.tile_entities.get(&pos).copied();
        if te.is_some() {
            self.tile_entities_to_remove.insert(pos, ());
        }
        te
    }
}

struct Section {
    palette: rust_dataconverter_engine::Map<BlockStateOwned, u16>,
    palette_states: Vec<BlockStateOwned>,
    to_fix: rust_dataconverter_engine::Map<u16, Vec<LocalPos>>,
    update: Vec<LocalPos>,
    y: i32,
    buffer: Option<[u16; 4096]>,
}

impl Section {
    fn from_nbt<T: Types + ?Sized>(nbt: &T::Map, sides: &mut u8) -> Self {
        let mut palette = rust_dataconverter_engine::Map::new();
        let mut palette_states = Vec::new();
        let mut to_fix = rust_dataconverter_engine::Map::<_, Vec<LocalPos>>::new();
        let mut update = Vec::new();
        let y = nbt.get_i64("Y").unwrap_or(0) as i32;
        let buffer = nbt.get("Blocks").and_then(|o| match o.as_ref() {
            ObjectRef::ByteArray(arr) => Some(arr),
            _ => None
        }).filter(|blocks| {
            if blocks.len() != 4096 {
                error!("Blocks array should be 4096 bytes not: {}", blocks.len());
                return false;
            }
            true
        }).map(|blocks| {
            let data = ChunkNibbleArray::wrap::<T>(nbt, "Data");
            let add = ChunkNibbleArray::wrap::<T>(nbt, "Add");

            palette.insert(air(), 0);
            palette_states.push(air().to_owned());

            let mut buffer = MaybeUninit::<u16>::uninit_array::<4096>();
            for index in 0..buffer.len() {
                let pos = LocalPos { index: index as u16 };

                let mut state_id = (blocks[index] as u8 as u16) << 4;
                if let Some(data) = &data {
                    state_id |= data.get(index as u16) as u16;
                }
                if let Some(add) = &add {
                    state_id |= (add.get(index as u16) as u16) << 12;
                }
                if *IDS_NEEDING_FIX_SET.get((state_id >> 4) as usize).as_deref().unwrap_or(&false) {
                    to_fix.entry(state_id >> 4).or_default().push(pos);
                }
                if *VIRTUAL_SET.get((state_id >> 4) as usize).as_deref().unwrap_or(&false) {
                    let additional_sides = get_side_mask(pos.x() == 0, pos.x() == 15, pos.z() == 0, pos.z() == 15);
                    if additional_sides == 0 {
                        update.push(pos);
                    } else {
                        *sides |= additional_sides;
                    }
                }

                let mut state = block_flattening_v1450::get_state_for_id_raw(state_id).unwrap_or_else(|| air());
                if state.name == "minecraft::%%FILTER_ME%%" {
                    state = air();
                }
                let next_palette_index = palette.len() as u16;
                let palette_index = *palette.entry(state).or_insert_with(|| {
                    palette_states.push(state.to_owned());
                    next_palette_index
                });
                buffer[index] = MaybeUninit::new(palette_index);
            }
            // SAFETY: buffer is initialized in the above loop
            unsafe { MaybeUninit::array_assume_init(buffer) }
        });

        let palette: rust_dataconverter_engine::Map<_, _> = palette.into_iter().map(|(k, v)| (k.to_owned(), v)).collect();

        Self {
            palette,
            palette_states,
            to_fix,
            update,
            y,
            buffer,
        }
    }

    fn into_nbt<T: Types + ?Sized>(self, dest: &mut T::Map) {
        let buffer = match self.buffer {
            Some(buf) => buf,
            None => return
        };

        let mut palette = T::List::create_empty();
        for state in self.palette_states {
            palette.add(T::Object::create_map(state.to_nbt::<T>()));
        }
        dest.set("Palette", T::Object::create_list(palette));

        let bit_size = ceil_log2(self.palette.len() as u32).max(4);
        let mut packed_ids = PackedBitStorage::new(bit_size, buffer.len());
        for (index, value) in buffer.iter().enumerate() {
            packed_ids.set(index, *value as u32);
        }

        dest.set("BlockStates", T::Object::create_long_array(packed_ids.into_raw()));

        dest.remove("Blocks");
        dest.remove("Data");
        dest.remove("Add");
    }
}
