use crate::helpers::block_state::BlockState;
use ahash::AHashMap;
use java_string::JavaStr;
use std::collections::BTreeMap;
use std::sync::OnceLock;
use world_transmuter_engine::JCompound;

pub(crate) fn flatten_nbt(nbt: &JCompound) -> Option<JCompound> {
    if let Some(state) = BlockState::from_nbt(nbt) {
        let data = block_state_data();
        if let Some(id) = data.id_by_old_nbt.get(&state) {
            if let Some(ret) = get_state_for_id_raw(*id) {
                return Some(ret.to_nbt());
            }
        }
    }

    None
}

pub(crate) fn get_new_block_name(old: &impl AsRef<JavaStr>) -> &JavaStr {
    let old = old.as_ref();
    let data = block_state_data();
    if let Some(id) = data.id_by_old_name.get(old) {
        if let Some(ret) = get_state_for_id_raw(*id) {
            return ret.name;
        }
    }

    old
}

pub(crate) fn get_name_for_id(id: u16) -> &'static JavaStr {
    get_state_for_id_raw(id).map_or_else(|| JavaStr::from_str("minecraft:air"), |state| state.name)
}

pub(crate) fn get_state_for_id_raw(id: u16) -> Option<&'static BlockState<'static>> {
    if id >= 4096 {
        return None;
    }
    let data = block_state_data();
    data.flattened_by_id[id as usize].map(|index| &data.states[index as usize])
}

pub(crate) fn get_nbt_for_id(id: u16) -> JCompound {
    get_state_for_id_raw(id).map_or_else(
        || {
            let mut ret = JCompound::new();
            ret.insert("Name", "minecraft:air");
            ret
        },
        |state| state.to_nbt(),
    )
}

struct BlockStateData {
    states: Vec<BlockState<'static>>,
    flattened_by_id: [Option<u16>; 4096],
    block_defaults: [Option<u16>; 256],
    id_by_old_nbt: AHashMap<BlockState<'static>, u16>,
    id_by_old_name: AHashMap<&'static JavaStr, u16>,
}

static BLOCK_STATE_DATA: OnceLock<BlockStateData> = OnceLock::new();

fn block_state_data() -> &'static BlockStateData {
    BLOCK_STATE_DATA.get_or_init(|| {
        let mut state_indexes = AHashMap::new();
        let mut states = Vec::new();
        const NONE: Option<u16> = None;
        let mut flattened_by_id = [NONE; 4096];
        let mut block_defaults = [NONE; 256];
        let mut id_by_old_nbt = AHashMap::new();
        let mut id_by_old_name = AHashMap::new();

        let registrar = |id: u16, new: BlockState<'static>, olds: Vec<BlockState<'static>>| {
            let next_state_index = state_indexes.len();
            let new = *state_indexes.entry(new.clone()).or_insert_with(|| {
                debug_assert!(next_state_index < u16::MAX as usize);
                states.push(new);
                next_state_index as u16
            });
            let next_state_index = state_indexes.len();
            let old_ids: Vec<_> = olds
                .iter()
                .map(|old| {
                    *state_indexes.entry(old.clone()).or_insert_with(|| {
                        debug_assert!(next_state_index < u16::MAX as usize);
                        states.push(old.clone());
                        next_state_index as u16
                    })
                })
                .collect();

            debug_assert!(
                flattened_by_id[id as usize].is_none(),
                "Mapping already exists for id {}",
                id
            );

            flattened_by_id[id as usize] = Some(new);

            // it's important that we register ids from smallest to largest, so that
            // the default is going to be correct
            let block = id >> 4;
            block_defaults[block as usize].get_or_insert(new);

            for (old, old_id) in olds.iter().zip(old_ids) {
                id_by_old_name.entry(old.name).or_insert(old_id);
                id_by_old_nbt.insert(old.clone(), old_id);
            }
        };

        register_states(registrar);

        for (index, entry) in flattened_by_id.iter_mut().enumerate() {
            if entry.is_none() {
                *entry = block_defaults[index >> 4];
            }
        }

        BlockStateData {
            states,
            flattened_by_id,
            block_defaults,
            id_by_old_nbt,
            id_by_old_name,
        }
    })
}

macro_rules! register {
    ($registrar:ident, $id:literal, $new_name:literal $([$($new_prop_name:literal = $new_prop_value:literal),+])? $(, $($old_name:literal $([$($old_prop_name:literal = $old_prop_value:literal),+])?),+)?) => {
        $registrar($id, BlockState {
            name: JavaStr::from_str(concat!("minecraft:", $new_name)),
            properties: {
                #[allow(unused_mut)]
                let mut map = BTreeMap::new();
                $(
                    $(
                        map.insert(JavaStr::from_str($new_prop_name), JavaStr::from_str($new_prop_value));
                    )+
                )?
                map
            }
        }, vec![$($(
            BlockState {
                name: JavaStr::from_str(concat!("minecraft:", $old_name)),
                properties: {
                    #[allow(unused_mut)]
                    let mut map = BTreeMap::new();
                    $(
                        $(
                            map.insert(JavaStr::from_str($old_prop_name), JavaStr::from_str($old_prop_value));
                        )+
                    )?
                    map
                }
            },
        )+)?])
    }
}

fn register_states(mut registrar: impl FnMut(u16, BlockState<'static>, Vec<BlockState<'static>>)) {
    register!(registrar, 0, "air", "air");
    register!(registrar, 16, "stone", "stone"["variant" = "stone"]);
    register!(registrar, 17, "granite", "stone"["variant" = "granite"]);
    register!(
        registrar,
        18,
        "polished_granite",
        "stone"["variant" = "smooth_granite"]
    );
    register!(registrar, 19, "diorite", "stone"["variant" = "diorite"]);
    register!(
        registrar,
        20,
        "polished_diorite",
        "stone"["variant" = "smooth_diorite"]
    );
    register!(registrar, 21, "andesite", "stone"["variant" = "andesite"]);
    register!(
        registrar,
        22,
        "polished_andesite",
        "stone"["variant" = "smooth_andesite"]
    );
    register!(
        registrar,
        32,
        "grass_block"["snowy" = "false"],
        "grass"["snowy" = "false"],
        "grass"["snowy" = "true"]
    );
    register!(registrar, 48, "dirt", "dirt"["snowy" = "false", "variant" = "dirt"], "dirt"["snowy" = "true", "variant" = "dirt"]);
    register!(registrar, 49, "coarse_dirt", "dirt"["snowy" = "false", "variant" = "coarse_dirt"], "dirt"["snowy" = "true", "variant" = "coarse_dirt"]);
    register!(registrar, 50, "podzol"["snowy" = "false"], "dirt"["snowy" = "false", "variant" = "podzol"], "dirt"["snowy" = "true", "variant" = "podzol"]);
    register!(registrar, 64, "cobblestone", "cobblestone");
    register!(registrar, 80, "oak_planks", "planks"["variant" = "oak"]);
    register!(
        registrar,
        81,
        "spruce_planks",
        "planks"["variant" = "spruce"]
    );
    register!(registrar, 82, "birch_planks", "planks"["variant" = "birch"]);
    register!(
        registrar,
        83,
        "jungle_planks",
        "planks"["variant" = "jungle"]
    );
    register!(
        registrar,
        84,
        "acacia_planks",
        "planks"["variant" = "acacia"]
    );
    register!(
        registrar,
        85,
        "dark_oak_planks",
        "planks"["variant" = "dark_oak"]
    );
    register!(registrar, 96, "oak_sapling"["stage" = "0"], "sapling"["stage" = "0", "type" = "oak"]);
    register!(registrar, 97, "spruce_sapling"["stage" = "0"], "sapling"["stage" = "0", "type" = "spruce"]);
    register!(registrar, 98, "birch_sapling"["stage" = "0"], "sapling"["stage" = "0", "type" = "birch"]);
    register!(registrar, 99, "jungle_sapling"["stage" = "0"], "sapling"["stage" = "0", "type" = "jungle"]);
    register!(registrar, 100, "acacia_sapling"["stage" = "0"], "sapling"["stage" = "0", "type" = "acacia"]);
    register!(registrar, 101, "dark_oak_sapling"["stage" = "0"], "sapling"["stage" = "0", "type" = "dark_oak"]);
    register!(registrar, 104, "oak_sapling"["stage" = "1"], "sapling"["stage" = "1", "type" = "oak"]);
    register!(registrar, 105, "spruce_sapling"["stage" = "1"], "sapling"["stage" = "1", "type" = "spruce"]);
    register!(registrar, 106, "birch_sapling"["stage" = "1"], "sapling"["stage" = "1", "type" = "birch"]);
    register!(registrar, 107, "jungle_sapling"["stage" = "1"], "sapling"["stage" = "1", "type" = "jungle"]);
    register!(registrar, 108, "acacia_sapling"["stage" = "1"], "sapling"["stage" = "1", "type" = "acacia"]);
    register!(registrar, 109, "dark_oak_sapling"["stage" = "1"], "sapling"["stage" = "1", "type" = "dark_oak"]);
    register!(registrar, 112, "bedrock", "bedrock");
    register!(
        registrar,
        128,
        "water"["level" = "0"],
        "flowing_water"["level" = "0"]
    );
    register!(
        registrar,
        129,
        "water"["level" = "1"],
        "flowing_water"["level" = "1"]
    );
    register!(
        registrar,
        130,
        "water"["level" = "2"],
        "flowing_water"["level" = "2"]
    );
    register!(
        registrar,
        131,
        "water"["level" = "3"],
        "flowing_water"["level" = "3"]
    );
    register!(
        registrar,
        132,
        "water"["level" = "4"],
        "flowing_water"["level" = "4"]
    );
    register!(
        registrar,
        133,
        "water"["level" = "5"],
        "flowing_water"["level" = "5"]
    );
    register!(
        registrar,
        134,
        "water"["level" = "6"],
        "flowing_water"["level" = "6"]
    );
    register!(
        registrar,
        135,
        "water"["level" = "7"],
        "flowing_water"["level" = "7"]
    );
    register!(
        registrar,
        136,
        "water"["level" = "8"],
        "flowing_water"["level" = "8"]
    );
    register!(
        registrar,
        137,
        "water"["level" = "9"],
        "flowing_water"["level" = "9"]
    );
    register!(
        registrar,
        138,
        "water"["level" = "10"],
        "flowing_water"["level" = "10"]
    );
    register!(
        registrar,
        139,
        "water"["level" = "11"],
        "flowing_water"["level" = "11"]
    );
    register!(
        registrar,
        140,
        "water"["level" = "12"],
        "flowing_water"["level" = "12"]
    );
    register!(
        registrar,
        141,
        "water"["level" = "13"],
        "flowing_water"["level" = "13"]
    );
    register!(
        registrar,
        142,
        "water"["level" = "14"],
        "flowing_water"["level" = "14"]
    );
    register!(
        registrar,
        143,
        "water"["level" = "15"],
        "flowing_water"["level" = "15"]
    );
    register!(
        registrar,
        144,
        "water"["level" = "0"],
        "water"["level" = "0"]
    );
    register!(
        registrar,
        145,
        "water"["level" = "1"],
        "water"["level" = "1"]
    );
    register!(
        registrar,
        146,
        "water"["level" = "2"],
        "water"["level" = "2"]
    );
    register!(
        registrar,
        147,
        "water"["level" = "3"],
        "water"["level" = "3"]
    );
    register!(
        registrar,
        148,
        "water"["level" = "4"],
        "water"["level" = "4"]
    );
    register!(
        registrar,
        149,
        "water"["level" = "5"],
        "water"["level" = "5"]
    );
    register!(
        registrar,
        150,
        "water"["level" = "6"],
        "water"["level" = "6"]
    );
    register!(
        registrar,
        151,
        "water"["level" = "7"],
        "water"["level" = "7"]
    );
    register!(
        registrar,
        152,
        "water"["level" = "8"],
        "water"["level" = "8"]
    );
    register!(
        registrar,
        153,
        "water"["level" = "9"],
        "water"["level" = "9"]
    );
    register!(
        registrar,
        154,
        "water"["level" = "10"],
        "water"["level" = "10"]
    );
    register!(
        registrar,
        155,
        "water"["level" = "11"],
        "water"["level" = "11"]
    );
    register!(
        registrar,
        156,
        "water"["level" = "12"],
        "water"["level" = "12"]
    );
    register!(
        registrar,
        157,
        "water"["level" = "13"],
        "water"["level" = "13"]
    );
    register!(
        registrar,
        158,
        "water"["level" = "14"],
        "water"["level" = "14"]
    );
    register!(
        registrar,
        159,
        "water"["level" = "15"],
        "water"["level" = "15"]
    );
    register!(
        registrar,
        160,
        "lava"["level" = "0"],
        "flowing_lava"["level" = "0"]
    );
    register!(
        registrar,
        161,
        "lava"["level" = "1"],
        "flowing_lava"["level" = "1"]
    );
    register!(
        registrar,
        162,
        "lava"["level" = "2"],
        "flowing_lava"["level" = "2"]
    );
    register!(
        registrar,
        163,
        "lava"["level" = "3"],
        "flowing_lava"["level" = "3"]
    );
    register!(
        registrar,
        164,
        "lava"["level" = "4"],
        "flowing_lava"["level" = "4"]
    );
    register!(
        registrar,
        165,
        "lava"["level" = "5"],
        "flowing_lava"["level" = "5"]
    );
    register!(
        registrar,
        166,
        "lava"["level" = "6"],
        "flowing_lava"["level" = "6"]
    );
    register!(
        registrar,
        167,
        "lava"["level" = "7"],
        "flowing_lava"["level" = "7"]
    );
    register!(
        registrar,
        168,
        "lava"["level" = "8"],
        "flowing_lava"["level" = "8"]
    );
    register!(
        registrar,
        169,
        "lava"["level" = "9"],
        "flowing_lava"["level" = "9"]
    );
    register!(
        registrar,
        170,
        "lava"["level" = "10"],
        "flowing_lava"["level" = "10"]
    );
    register!(
        registrar,
        171,
        "lava"["level" = "11"],
        "flowing_lava"["level" = "11"]
    );
    register!(
        registrar,
        172,
        "lava"["level" = "12"],
        "flowing_lava"["level" = "12"]
    );
    register!(
        registrar,
        173,
        "lava"["level" = "13"],
        "flowing_lava"["level" = "13"]
    );
    register!(
        registrar,
        174,
        "lava"["level" = "14"],
        "flowing_lava"["level" = "14"]
    );
    register!(
        registrar,
        175,
        "lava"["level" = "15"],
        "flowing_lava"["level" = "15"]
    );
    register!(registrar, 176, "lava"["level" = "0"], "lava"["level" = "0"]);
    register!(registrar, 177, "lava"["level" = "1"], "lava"["level" = "1"]);
    register!(registrar, 178, "lava"["level" = "2"], "lava"["level" = "2"]);
    register!(registrar, 179, "lava"["level" = "3"], "lava"["level" = "3"]);
    register!(registrar, 180, "lava"["level" = "4"], "lava"["level" = "4"]);
    register!(registrar, 181, "lava"["level" = "5"], "lava"["level" = "5"]);
    register!(registrar, 182, "lava"["level" = "6"], "lava"["level" = "6"]);
    register!(registrar, 183, "lava"["level" = "7"], "lava"["level" = "7"]);
    register!(registrar, 184, "lava"["level" = "8"], "lava"["level" = "8"]);
    register!(registrar, 185, "lava"["level" = "9"], "lava"["level" = "9"]);
    register!(
        registrar,
        186,
        "lava"["level" = "10"],
        "lava"["level" = "10"]
    );
    register!(
        registrar,
        187,
        "lava"["level" = "11"],
        "lava"["level" = "11"]
    );
    register!(
        registrar,
        188,
        "lava"["level" = "12"],
        "lava"["level" = "12"]
    );
    register!(
        registrar,
        189,
        "lava"["level" = "13"],
        "lava"["level" = "13"]
    );
    register!(
        registrar,
        190,
        "lava"["level" = "14"],
        "lava"["level" = "14"]
    );
    register!(
        registrar,
        191,
        "lava"["level" = "15"],
        "lava"["level" = "15"]
    );
    register!(registrar, 192, "sand", "sand"["variant" = "sand"]);
    register!(registrar, 193, "red_sand", "sand"["variant" = "red_sand"]);
    register!(registrar, 208, "gravel", "gravel");
    register!(registrar, 224, "gold_ore", "gold_ore");
    register!(registrar, 240, "iron_ore", "iron_ore");
    register!(registrar, 256, "coal_ore", "coal_ore");
    register!(registrar, 272, "oak_log"["axis" = "y"], "log"["axis" = "y", "variant" = "oak"]);
    register!(registrar, 273, "spruce_log"["axis" = "y"], "log"["axis" = "y", "variant" = "spruce"]);
    register!(registrar, 274, "birch_log"["axis" = "y"], "log"["axis" = "y", "variant" = "birch"]);
    register!(registrar, 275, "jungle_log"["axis" = "y"], "log"["axis" = "y", "variant" = "jungle"]);
    register!(registrar, 276, "oak_log"["axis" = "x"], "log"["axis" = "x", "variant" = "oak"]);
    register!(registrar, 277, "spruce_log"["axis" = "x"], "log"["axis" = "x", "variant" = "spruce"]);
    register!(registrar, 278, "birch_log"["axis" = "x"], "log"["axis" = "x", "variant" = "birch"]);
    register!(registrar, 279, "jungle_log"["axis" = "x"], "log"["axis" = "x", "variant" = "jungle"]);
    register!(registrar, 280, "oak_log"["axis" = "z"], "log"["axis" = "z", "variant" = "oak"]);
    register!(registrar, 281, "spruce_log"["axis" = "z"], "log"["axis" = "z", "variant" = "spruce"]);
    register!(registrar, 282, "birch_log"["axis" = "z"], "log"["axis" = "z", "variant" = "birch"]);
    register!(registrar, 283, "jungle_log"["axis" = "z"], "log"["axis" = "z", "variant" = "jungle"]);
    register!(registrar, 284, "oak_bark", "log"["axis" = "none", "variant" = "oak"]);
    register!(registrar, 285, "spruce_bark", "log"["axis" = "none", "variant" = "spruce"]);
    register!(registrar, 286, "birch_bark", "log"["axis" = "none", "variant" = "birch"]);
    register!(registrar, 287, "jungle_bark", "log"["axis" = "none", "variant" = "jungle"]);
    register!(registrar, 288, "oak_leaves"["check_decay" = "false", "decayable" = "true"], "leaves"["check_decay" = "false", "decayable" = "true", "variant" = "oak"]);
    register!(registrar, 289, "spruce_leaves"["check_decay" = "false", "decayable" = "true"], "leaves"["check_decay" = "false", "decayable" = "true", "variant" = "spruce"]);
    register!(registrar, 290, "birch_leaves"["check_decay" = "false", "decayable" = "true"], "leaves"["check_decay" = "false", "decayable" = "true", "variant" = "birch"]);
    register!(registrar, 291, "jungle_leaves"["check_decay" = "false", "decayable" = "true"], "leaves"["check_decay" = "false", "decayable" = "true", "variant" = "jungle"]);
    register!(registrar, 292, "oak_leaves"["check_decay" = "false", "decayable" = "false"], "leaves"["check_decay" = "false", "decayable" = "false", "variant" = "oak"]);
    register!(registrar, 293, "spruce_leaves"["check_decay" = "false", "decayable" = "false"], "leaves"["check_decay" = "false", "decayable" = "false", "variant" = "spruce"]);
    register!(registrar, 294, "birch_leaves"["check_decay" = "false", "decayable" = "false"], "leaves"["check_decay" = "false", "decayable" = "false", "variant" = "birch"]);
    register!(registrar, 295, "jungle_leaves"["check_decay" = "false", "decayable" = "false"], "leaves"["check_decay" = "false", "decayable" = "false", "variant" = "jungle"]);
    register!(registrar, 296, "oak_leaves"["check_decay" = "true", "decayable" = "true"], "leaves"["check_decay" = "true", "decayable" = "true", "variant" = "oak"]);
    register!(registrar, 297, "spruce_leaves"["check_decay" = "true", "decayable" = "true"], "leaves"["check_decay" = "true", "decayable" = "true", "variant" = "spruce"]);
    register!(registrar, 298, "birch_leaves"["check_decay" = "true", "decayable" = "true"], "leaves"["check_decay" = "true", "decayable" = "true", "variant" = "birch"]);
    register!(registrar, 299, "jungle_leaves"["check_decay" = "true", "decayable" = "true"], "leaves"["check_decay" = "true", "decayable" = "true", "variant" = "jungle"]);
    register!(registrar, 300, "oak_leaves"["check_decay" = "true", "decayable" = "false"], "leaves"["check_decay" = "true", "decayable" = "false", "variant" = "oak"]);
    register!(registrar, 301, "spruce_leaves"["check_decay" = "true", "decayable" = "false"], "leaves"["check_decay" = "true", "decayable" = "false", "variant" = "spruce"]);
    register!(registrar, 302, "birch_leaves"["check_decay" = "true", "decayable" = "false"], "leaves"["check_decay" = "true", "decayable" = "false", "variant" = "birch"]);
    register!(registrar, 303, "jungle_leaves"["check_decay" = "true", "decayable" = "false"], "leaves"["check_decay" = "true", "decayable" = "false", "variant" = "jungle"]);
    register!(registrar, 304, "sponge", "sponge"["wet" = "false"]);
    register!(registrar, 305, "wet_sponge", "sponge"["wet" = "true"]);
    register!(registrar, 320, "glass", "glass");
    register!(registrar, 336, "lapis_ore", "lapis_ore");
    register!(registrar, 352, "lapis_block", "lapis_block");
    register!(registrar, 368, "dispenser"["facing" = "down", "triggered" = "false"], "dispenser"["facing" = "down", "triggered" = "false"]);
    register!(registrar, 369, "dispenser"["facing" = "up", "triggered" = "false"], "dispenser"["facing" = "up", "triggered" = "false"]);
    register!(registrar, 370, "dispenser"["facing" = "north", "triggered" = "false"], "dispenser"["facing" = "north", "triggered" = "false"]);
    register!(registrar, 371, "dispenser"["facing" = "south", "triggered" = "false"], "dispenser"["facing" = "south", "triggered" = "false"]);
    register!(registrar, 372, "dispenser"["facing" = "west", "triggered" = "false"], "dispenser"["facing" = "west", "triggered" = "false"]);
    register!(registrar, 373, "dispenser"["facing" = "east", "triggered" = "false"], "dispenser"["facing" = "east", "triggered" = "false"]);
    register!(registrar, 376, "dispenser"["facing" = "down", "triggered" = "true"], "dispenser"["facing" = "down", "triggered" = "true"]);
    register!(registrar, 377, "dispenser"["facing" = "up", "triggered" = "true"], "dispenser"["facing" = "up", "triggered" = "true"]);
    register!(registrar, 378, "dispenser"["facing" = "north", "triggered" = "true"], "dispenser"["facing" = "north", "triggered" = "true"]);
    register!(registrar, 379, "dispenser"["facing" = "south", "triggered" = "true"], "dispenser"["facing" = "south", "triggered" = "true"]);
    register!(registrar, 380, "dispenser"["facing" = "west", "triggered" = "true"], "dispenser"["facing" = "west", "triggered" = "true"]);
    register!(registrar, 381, "dispenser"["facing" = "east", "triggered" = "true"], "dispenser"["facing" = "east", "triggered" = "true"]);
    register!(
        registrar,
        384,
        "sandstone",
        "sandstone"["type" = "sandstone"]
    );
    register!(
        registrar,
        385,
        "chiseled_sandstone",
        "sandstone"["type" = "chiseled_sandstone"]
    );
    register!(
        registrar,
        386,
        "cut_sandstone",
        "sandstone"["type" = "smooth_sandstone"]
    );
    register!(registrar, 400, "note_block", "noteblock");
    register!(registrar, 416, "red_bed"["facing" = "south", "occupied" = "false", "part" = "foot"], "bed"["facing" = "south", "occupied" = "false", "part" = "foot"], "bed"["facing" = "south", "occupied" = "true", "part" = "foot"]);
    register!(registrar, 417, "red_bed"["facing" = "west", "occupied" = "false", "part" = "foot"], "bed"["facing" = "west", "occupied" = "false", "part" = "foot"], "bed"["facing" = "west", "occupied" = "true", "part" = "foot"]);
    register!(registrar, 418, "red_bed"["facing" = "north", "occupied" = "false", "part" = "foot"], "bed"["facing" = "north", "occupied" = "false", "part" = "foot"], "bed"["facing" = "north", "occupied" = "true", "part" = "foot"]);
    register!(registrar, 419, "red_bed"["facing" = "east", "occupied" = "false", "part" = "foot"], "bed"["facing" = "east", "occupied" = "false", "part" = "foot"], "bed"["facing" = "east", "occupied" = "true", "part" = "foot"]);
    register!(registrar, 424, "red_bed"["facing" = "south", "occupied" = "false", "part" = "head"], "bed"["facing" = "south", "occupied" = "false", "part" = "head"]);
    register!(registrar, 425, "red_bed"["facing" = "west", "occupied" = "false", "part" = "head"], "bed"["facing" = "west", "occupied" = "false", "part" = "head"]);
    register!(registrar, 426, "red_bed"["facing" = "north", "occupied" = "false", "part" = "head"], "bed"["facing" = "north", "occupied" = "false", "part" = "head"]);
    register!(registrar, 427, "red_bed"["facing" = "east", "occupied" = "false", "part" = "head"], "bed"["facing" = "east", "occupied" = "false", "part" = "head"]);
    register!(registrar, 428, "red_bed"["facing" = "south", "occupied" = "true", "part" = "head"], "bed"["facing" = "south", "occupied" = "true", "part" = "head"]);
    register!(registrar, 429, "red_bed"["facing" = "west", "occupied" = "true", "part" = "head"], "bed"["facing" = "west", "occupied" = "true", "part" = "head"]);
    register!(registrar, 430, "red_bed"["facing" = "north", "occupied" = "true", "part" = "head"], "bed"["facing" = "north", "occupied" = "true", "part" = "head"]);
    register!(registrar, 431, "red_bed"["facing" = "east", "occupied" = "true", "part" = "head"], "bed"["facing" = "east", "occupied" = "true", "part" = "head"]);
    register!(registrar, 432, "powered_rail"["powered" = "false", "shape" = "north_south"], "golden_rail"["powered" = "false", "shape" = "north_south"]);
    register!(registrar, 433, "powered_rail"["powered" = "false", "shape" = "east_west"], "golden_rail"["powered" = "false", "shape" = "east_west"]);
    register!(registrar, 434, "powered_rail"["powered" = "false", "shape" = "ascending_east"], "golden_rail"["powered" = "false", "shape" = "ascending_east"]);
    register!(registrar, 435, "powered_rail"["powered" = "false", "shape" = "ascending_west"], "golden_rail"["powered" = "false", "shape" = "ascending_west"]);
    register!(registrar, 436, "powered_rail"["powered" = "false", "shape" = "ascending_north"], "golden_rail"["powered" = "false", "shape" = "ascending_north"]);
    register!(registrar, 437, "powered_rail"["powered" = "false", "shape" = "ascending_south"], "golden_rail"["powered" = "false", "shape" = "ascending_south"]);
    register!(registrar, 440, "powered_rail"["powered" = "true", "shape" = "north_south"], "golden_rail"["powered" = "true", "shape" = "north_south"]);
    register!(registrar, 441, "powered_rail"["powered" = "true", "shape" = "east_west"], "golden_rail"["powered" = "true", "shape" = "east_west"]);
    register!(registrar, 442, "powered_rail"["powered" = "true", "shape" = "ascending_east"], "golden_rail"["powered" = "true", "shape" = "ascending_east"]);
    register!(registrar, 443, "powered_rail"["powered" = "true", "shape" = "ascending_west"], "golden_rail"["powered" = "true", "shape" = "ascending_west"]);
    register!(registrar, 444, "powered_rail"["powered" = "true", "shape" = "ascending_north"], "golden_rail"["powered" = "true", "shape" = "ascending_north"]);
    register!(registrar, 445, "powered_rail"["powered" = "true", "shape" = "ascending_south"], "golden_rail"["powered" = "true", "shape" = "ascending_south"]);
    register!(registrar, 448, "detector_rail"["powered" = "false", "shape" = "north_south"], "detector_rail"["powered" = "false", "shape" = "north_south"]);
    register!(registrar, 449, "detector_rail"["powered" = "false", "shape" = "east_west"], "detector_rail"["powered" = "false", "shape" = "east_west"]);
    register!(registrar, 450, "detector_rail"["powered" = "false", "shape" = "ascending_east"], "detector_rail"["powered" = "false", "shape" = "ascending_east"]);
    register!(registrar, 451, "detector_rail"["powered" = "false", "shape" = "ascending_west"], "detector_rail"["powered" = "false", "shape" = "ascending_west"]);
    register!(registrar, 452, "detector_rail"["powered" = "false", "shape" = "ascending_north"], "detector_rail"["powered" = "false", "shape" = "ascending_north"]);
    register!(registrar, 453, "detector_rail"["powered" = "false", "shape" = "ascending_south"], "detector_rail"["powered" = "false", "shape" = "ascending_south"]);
    register!(registrar, 456, "detector_rail"["powered" = "true", "shape" = "north_south"], "detector_rail"["powered" = "true", "shape" = "north_south"]);
    register!(registrar, 457, "detector_rail"["powered" = "true", "shape" = "east_west"], "detector_rail"["powered" = "true", "shape" = "east_west"]);
    register!(registrar, 458, "detector_rail"["powered" = "true", "shape" = "ascending_east"], "detector_rail"["powered" = "true", "shape" = "ascending_east"]);
    register!(registrar, 459, "detector_rail"["powered" = "true", "shape" = "ascending_west"], "detector_rail"["powered" = "true", "shape" = "ascending_west"]);
    register!(registrar, 460, "detector_rail"["powered" = "true", "shape" = "ascending_north"], "detector_rail"["powered" = "true", "shape" = "ascending_north"]);
    register!(registrar, 461, "detector_rail"["powered" = "true", "shape" = "ascending_south"], "detector_rail"["powered" = "true", "shape" = "ascending_south"]);
    register!(registrar, 464, "sticky_piston"["extended" = "false", "facing" = "down"], "sticky_piston"["extended" = "false", "facing" = "down"]);
    register!(registrar, 465, "sticky_piston"["extended" = "false", "facing" = "up"], "sticky_piston"["extended" = "false", "facing" = "up"]);
    register!(registrar, 466, "sticky_piston"["extended" = "false", "facing" = "north"], "sticky_piston"["extended" = "false", "facing" = "north"]);
    register!(registrar, 467, "sticky_piston"["extended" = "false", "facing" = "south"], "sticky_piston"["extended" = "false", "facing" = "south"]);
    register!(registrar, 468, "sticky_piston"["extended" = "false", "facing" = "west"], "sticky_piston"["extended" = "false", "facing" = "west"]);
    register!(registrar, 469, "sticky_piston"["extended" = "false", "facing" = "east"], "sticky_piston"["extended" = "false", "facing" = "east"]);
    register!(registrar, 472, "sticky_piston"["extended" = "true", "facing" = "down"], "sticky_piston"["extended" = "true", "facing" = "down"]);
    register!(registrar, 473, "sticky_piston"["extended" = "true", "facing" = "up"], "sticky_piston"["extended" = "true", "facing" = "up"]);
    register!(registrar, 474, "sticky_piston"["extended" = "true", "facing" = "north"], "sticky_piston"["extended" = "true", "facing" = "north"]);
    register!(registrar, 475, "sticky_piston"["extended" = "true", "facing" = "south"], "sticky_piston"["extended" = "true", "facing" = "south"]);
    register!(registrar, 476, "sticky_piston"["extended" = "true", "facing" = "west"], "sticky_piston"["extended" = "true", "facing" = "west"]);
    register!(registrar, 477, "sticky_piston"["extended" = "true", "facing" = "east"], "sticky_piston"["extended" = "true", "facing" = "east"]);
    register!(registrar, 480, "cobweb", "web");
    register!(
        registrar,
        496,
        "dead_bush",
        "tallgrass"["type" = "dead_bush"]
    );
    register!(registrar, 497, "grass", "tallgrass"["type" = "tall_grass"]);
    register!(registrar, 498, "fern", "tallgrass"["type" = "fern"]);
    register!(registrar, 512, "dead_bush", "deadbush");
    register!(registrar, 528, "piston"["extended" = "false", "facing" = "down"], "piston"["extended" = "false", "facing" = "down"]);
    register!(registrar, 529, "piston"["extended" = "false", "facing" = "up"], "piston"["extended" = "false", "facing" = "up"]);
    register!(registrar, 530, "piston"["extended" = "false", "facing" = "north"], "piston"["extended" = "false", "facing" = "north"]);
    register!(registrar, 531, "piston"["extended" = "false", "facing" = "south"], "piston"["extended" = "false", "facing" = "south"]);
    register!(registrar, 532, "piston"["extended" = "false", "facing" = "west"], "piston"["extended" = "false", "facing" = "west"]);
    register!(registrar, 533, "piston"["extended" = "false", "facing" = "east"], "piston"["extended" = "false", "facing" = "east"]);
    register!(registrar, 536, "piston"["extended" = "true", "facing" = "down"], "piston"["extended" = "true", "facing" = "down"]);
    register!(registrar, 537, "piston"["extended" = "true", "facing" = "up"], "piston"["extended" = "true", "facing" = "up"]);
    register!(registrar, 538, "piston"["extended" = "true", "facing" = "north"], "piston"["extended" = "true", "facing" = "north"]);
    register!(registrar, 539, "piston"["extended" = "true", "facing" = "south"], "piston"["extended" = "true", "facing" = "south"]);
    register!(registrar, 540, "piston"["extended" = "true", "facing" = "west"], "piston"["extended" = "true", "facing" = "west"]);
    register!(registrar, 541, "piston"["extended" = "true", "facing" = "east"], "piston"["extended" = "true", "facing" = "east"]);
    register!(registrar, 544, "piston_head"["facing" = "down", "short" = "false", "type" = "normal"], "piston_head"["facing" = "down", "short" = "false", "type" = "normal"], "piston_head"["facing" = "down", "short" = "true", "type" = "normal"]);
    register!(registrar, 545, "piston_head"["facing" = "up", "short" = "false", "type" = "normal"], "piston_head"["facing" = "up", "short" = "false", "type" = "normal"], "piston_head"["facing" = "up", "short" = "true", "type" = "normal"]);
    register!(registrar, 546, "piston_head"["facing" = "north", "short" = "false", "type" = "normal"], "piston_head"["facing" = "north", "short" = "false", "type" = "normal"], "piston_head"["facing" = "north", "short" = "true", "type" = "normal"]);
    register!(registrar, 547, "piston_head"["facing" = "south", "short" = "false", "type" = "normal"], "piston_head"["facing" = "south", "short" = "false", "type" = "normal"], "piston_head"["facing" = "south", "short" = "true", "type" = "normal"]);
    register!(registrar, 548, "piston_head"["facing" = "west", "short" = "false", "type" = "normal"], "piston_head"["facing" = "west", "short" = "false", "type" = "normal"], "piston_head"["facing" = "west", "short" = "true", "type" = "normal"]);
    register!(registrar, 549, "piston_head"["facing" = "east", "short" = "false", "type" = "normal"], "piston_head"["facing" = "east", "short" = "false", "type" = "normal"], "piston_head"["facing" = "east", "short" = "true", "type" = "normal"]);
    register!(registrar, 552, "piston_head"["facing" = "down", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "down", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "down", "short" = "true", "type" = "sticky"]);
    register!(registrar, 553, "piston_head"["facing" = "up", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "up", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "up", "short" = "true", "type" = "sticky"]);
    register!(registrar, 554, "piston_head"["facing" = "north", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "north", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "north", "short" = "true", "type" = "sticky"]);
    register!(registrar, 555, "piston_head"["facing" = "south", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "south", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "south", "short" = "true", "type" = "sticky"]);
    register!(registrar, 556, "piston_head"["facing" = "west", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "west", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "west", "short" = "true", "type" = "sticky"]);
    register!(registrar, 557, "piston_head"["facing" = "east", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "east", "short" = "false", "type" = "sticky"], "piston_head"["facing" = "east", "short" = "true", "type" = "sticky"]);
    register!(registrar, 560, "white_wool", "wool"["color" = "white"]);
    register!(registrar, 561, "orange_wool", "wool"["color" = "orange"]);
    register!(registrar, 562, "magenta_wool", "wool"["color" = "magenta"]);
    register!(
        registrar,
        563,
        "light_blue_wool",
        "wool"["color" = "light_blue"]
    );
    register!(registrar, 564, "yellow_wool", "wool"["color" = "yellow"]);
    register!(registrar, 565, "lime_wool", "wool"["color" = "lime"]);
    register!(registrar, 566, "pink_wool", "wool"["color" = "pink"]);
    register!(registrar, 567, "gray_wool", "wool"["color" = "gray"]);
    register!(
        registrar,
        568,
        "light_gray_wool",
        "wool"["color" = "silver"]
    );
    register!(registrar, 569, "cyan_wool", "wool"["color" = "cyan"]);
    register!(registrar, 570, "purple_wool", "wool"["color" = "purple"]);
    register!(registrar, 571, "blue_wool", "wool"["color" = "blue"]);
    register!(registrar, 572, "brown_wool", "wool"["color" = "brown"]);
    register!(registrar, 573, "green_wool", "wool"["color" = "green"]);
    register!(registrar, 574, "red_wool", "wool"["color" = "red"]);
    register!(registrar, 575, "black_wool", "wool"["color" = "black"]);
    register!(registrar, 576, "moving_piston"["facing" = "down", "type" = "normal"], "piston_extension"["facing" = "down", "type" = "normal"]);
    register!(registrar, 577, "moving_piston"["facing" = "up", "type" = "normal"], "piston_extension"["facing" = "up", "type" = "normal"]);
    register!(registrar, 578, "moving_piston"["facing" = "north", "type" = "normal"], "piston_extension"["facing" = "north", "type" = "normal"]);
    register!(registrar, 579, "moving_piston"["facing" = "south", "type" = "normal"], "piston_extension"["facing" = "south", "type" = "normal"]);
    register!(registrar, 580, "moving_piston"["facing" = "west", "type" = "normal"], "piston_extension"["facing" = "west", "type" = "normal"]);
    register!(registrar, 581, "moving_piston"["facing" = "east", "type" = "normal"], "piston_extension"["facing" = "east", "type" = "normal"]);
    register!(registrar, 584, "moving_piston"["facing" = "down", "type" = "sticky"], "piston_extension"["facing" = "down", "type" = "sticky"]);
    register!(registrar, 585, "moving_piston"["facing" = "up", "type" = "sticky"], "piston_extension"["facing" = "up", "type" = "sticky"]);
    register!(registrar, 586, "moving_piston"["facing" = "north", "type" = "sticky"], "piston_extension"["facing" = "north", "type" = "sticky"]);
    register!(registrar, 587, "moving_piston"["facing" = "south", "type" = "sticky"], "piston_extension"["facing" = "south", "type" = "sticky"]);
    register!(registrar, 588, "moving_piston"["facing" = "west", "type" = "sticky"], "piston_extension"["facing" = "west", "type" = "sticky"]);
    register!(registrar, 589, "moving_piston"["facing" = "east", "type" = "sticky"], "piston_extension"["facing" = "east", "type" = "sticky"]);
    register!(
        registrar,
        592,
        "dandelion",
        "yellow_flower"["type" = "dandelion"]
    );
    register!(registrar, 608, "poppy", "red_flower"["type" = "poppy"]);
    register!(
        registrar,
        609,
        "blue_orchid",
        "red_flower"["type" = "blue_orchid"]
    );
    register!(registrar, 610, "allium", "red_flower"["type" = "allium"]);
    register!(
        registrar,
        611,
        "azure_bluet",
        "red_flower"["type" = "houstonia"]
    );
    register!(
        registrar,
        612,
        "red_tulip",
        "red_flower"["type" = "red_tulip"]
    );
    register!(
        registrar,
        613,
        "orange_tulip",
        "red_flower"["type" = "orange_tulip"]
    );
    register!(
        registrar,
        614,
        "white_tulip",
        "red_flower"["type" = "white_tulip"]
    );
    register!(
        registrar,
        615,
        "pink_tulip",
        "red_flower"["type" = "pink_tulip"]
    );
    register!(
        registrar,
        616,
        "oxeye_daisy",
        "red_flower"["type" = "oxeye_daisy"]
    );
    register!(registrar, 624, "brown_mushroom", "brown_mushroom");
    register!(registrar, 640, "red_mushroom", "red_mushroom");
    register!(registrar, 656, "gold_block", "gold_block");
    register!(registrar, 672, "iron_block", "iron_block");
    register!(registrar, 688, "stone_slab"["type" = "double"], "double_stone_slab"["seamless" = "false", "variant" = "stone"]);
    register!(registrar, 689, "sandstone_slab"["type" = "double"], "double_stone_slab"["seamless" = "false", "variant" = "sandstone"]);
    register!(registrar, 690, "petrified_oak_slab"["type" = "double"], "double_stone_slab"["seamless" = "false", "variant" = "wood_old"]);
    register!(registrar, 691, "cobblestone_slab"["type" = "double"], "double_stone_slab"["seamless" = "false", "variant" = "cobblestone"]);
    register!(registrar, 692, "brick_slab"["type" = "double"], "double_stone_slab"["seamless" = "false", "variant" = "brick"]);
    register!(registrar, 693, "stone_brick_slab"["type" = "double"], "double_stone_slab"["seamless" = "false", "variant" = "stone_brick"]);
    register!(registrar, 694, "nether_brick_slab"["type" = "double"], "double_stone_slab"["seamless" = "false", "variant" = "nether_brick"]);
    register!(registrar, 695, "quartz_slab"["type" = "double"], "double_stone_slab"["seamless" = "false", "variant" = "quartz"]);
    register!(registrar, 696, "smooth_stone", "double_stone_slab"["seamless" = "true", "variant" = "stone"]);
    register!(registrar, 697, "smooth_sandstone", "double_stone_slab"["seamless" = "true", "variant" = "sandstone"]);
    register!(registrar, 698, "petrified_oak_slab"["type" = "double"], "double_stone_slab"["seamless" = "true", "variant" = "wood_old"]);
    register!(registrar, 699, "cobblestone_slab"["type" = "double"], "double_stone_slab"["seamless" = "true", "variant" = "cobblestone"]);
    register!(registrar, 700, "brick_slab"["type" = "double"], "double_stone_slab"["seamless" = "true", "variant" = "brick"]);
    register!(registrar, 701, "stone_brick_slab"["type" = "double"], "double_stone_slab"["seamless" = "true", "variant" = "stone_brick"]);
    register!(registrar, 702, "nether_brick_slab"["type" = "double"], "double_stone_slab"["seamless" = "true", "variant" = "nether_brick"]);
    register!(registrar, 703, "smooth_quartz", "double_stone_slab"["seamless" = "true", "variant" = "quartz"]);
    register!(registrar, 704, "stone_slab"["type" = "bottom"], "stone_slab"["half" = "bottom", "variant" = "stone"]);
    register!(registrar, 705, "sandstone_slab"["type" = "bottom"], "stone_slab"["half" = "bottom", "variant" = "sandstone"]);
    register!(registrar, 706, "petrified_oak_slab"["type" = "bottom"], "stone_slab"["half" = "bottom", "variant" = "wood_old"]);
    register!(registrar, 707, "cobblestone_slab"["type" = "bottom"], "stone_slab"["half" = "bottom", "variant" = "cobblestone"]);
    register!(registrar, 708, "brick_slab"["type" = "bottom"], "stone_slab"["half" = "bottom", "variant" = "brick"]);
    register!(registrar, 709, "stone_brick_slab"["type" = "bottom"], "stone_slab"["half" = "bottom", "variant" = "stone_brick"]);
    register!(registrar, 710, "nether_brick_slab"["type" = "bottom"], "stone_slab"["half" = "bottom", "variant" = "nether_brick"]);
    register!(registrar, 711, "quartz_slab"["type" = "bottom"], "stone_slab"["half" = "bottom", "variant" = "quartz"]);
    register!(registrar, 712, "stone_slab"["type" = "top"], "stone_slab"["half" = "top", "variant" = "stone"]);
    register!(registrar, 713, "sandstone_slab"["type" = "top"], "stone_slab"["half" = "top", "variant" = "sandstone"]);
    register!(registrar, 714, "petrified_oak_slab"["type" = "top"], "stone_slab"["half" = "top", "variant" = "wood_old"]);
    register!(registrar, 715, "cobblestone_slab"["type" = "top"], "stone_slab"["half" = "top", "variant" = "cobblestone"]);
    register!(registrar, 716, "brick_slab"["type" = "top"], "stone_slab"["half" = "top", "variant" = "brick"]);
    register!(registrar, 717, "stone_brick_slab"["type" = "top"], "stone_slab"["half" = "top", "variant" = "stone_brick"]);
    register!(registrar, 718, "nether_brick_slab"["type" = "top"], "stone_slab"["half" = "top", "variant" = "nether_brick"]);
    register!(registrar, 719, "quartz_slab"["type" = "top"], "stone_slab"["half" = "top", "variant" = "quartz"]);
    register!(registrar, 720, "bricks", "brick_block");
    register!(
        registrar,
        736,
        "tnt"["unstable" = "false"],
        "tnt"["explode" = "false"]
    );
    register!(
        registrar,
        737,
        "tnt"["unstable" = "true"],
        "tnt"["explode" = "true"]
    );
    register!(registrar, 752, "bookshelf", "bookshelf");
    register!(registrar, 768, "mossy_cobblestone", "mossy_cobblestone");
    register!(registrar, 784, "obsidian", "obsidian");
    register!(
        registrar,
        801,
        "wall_torch"["facing" = "east"],
        "torch"["facing" = "east"]
    );
    register!(
        registrar,
        802,
        "wall_torch"["facing" = "west"],
        "torch"["facing" = "west"]
    );
    register!(
        registrar,
        803,
        "wall_torch"["facing" = "south"],
        "torch"["facing" = "south"]
    );
    register!(
        registrar,
        804,
        "wall_torch"["facing" = "north"],
        "torch"["facing" = "north"]
    );
    register!(registrar, 805, "torch", "torch"["facing" = "up"]);
    register!(registrar, 816, "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "0", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "0", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "0", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "0", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "0", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "0", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "0", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "0", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "0", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "0", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "0", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "0", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "0", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "0", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "0", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "0", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "0", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "0", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 817, "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "1", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "1", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "1", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "1", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "1", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "1", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "1", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "1", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "1", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "1", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "1", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "1", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "1", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "1", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "1", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "1", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "1", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "1", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 818, "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "2", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "2", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "2", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "2", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "2", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "2", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "2", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "2", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "2", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "2", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "2", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "2", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "2", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "2", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "2", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "2", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "2", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "2", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 819, "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "3", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "3", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "3", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "3", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "3", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "3", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "3", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "3", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "3", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "3", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "3", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "3", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "3", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "3", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "3", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "3", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "3", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "3", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 820, "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "4", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "4", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "4", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "4", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "4", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "4", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "4", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "4", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "4", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "4", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "4", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "4", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "4", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "4", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "4", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "4", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "4", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "4", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 821, "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "5", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "5", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "5", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "5", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "5", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "5", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "5", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "5", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "5", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "5", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "5", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "5", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "5", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "5", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "5", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "5", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "5", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "5", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 822, "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "6", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "6", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "6", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "6", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "6", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "6", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "6", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "6", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "6", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "6", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "6", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "6", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "6", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "6", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "6", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "6", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "6", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "6", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 823, "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "7", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "7", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "7", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "7", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "7", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "7", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "7", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "7", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "7", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "7", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "7", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "7", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "7", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "7", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "7", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "7", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "7", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "7", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 824, "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "8", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "8", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "8", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "8", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "8", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "8", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "8", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "8", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "8", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "8", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "8", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "8", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "8", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "8", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "8", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "8", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "8", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "8", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 825, "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "9", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "9", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "9", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "9", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "9", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "9", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "9", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "9", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "9", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "9", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "9", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "9", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "9", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "9", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "9", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "9", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "9", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "9", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 826, "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "10", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "10", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "10", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "10", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "10", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "10", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "10", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "10", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "10", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "10", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "10", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "10", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "10", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "10", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "10", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "10", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "10", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "10", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 827, "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "11", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "11", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "11", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "11", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "11", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "11", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "11", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "11", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "11", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "11", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "11", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "11", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "11", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "11", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "11", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "11", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "11", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "11", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 828, "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "12", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "12", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "12", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "12", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "12", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "12", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "12", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "12", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "12", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "12", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "12", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "12", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "12", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "12", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "12", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "12", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "12", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "12", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 829, "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "13", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "13", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "13", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "13", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "13", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "13", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "13", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "13", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "13", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "13", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "13", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "13", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "13", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "13", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "13", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "13", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "13", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "13", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 830, "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "14", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "14", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "14", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "14", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "14", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "14", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "14", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "14", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "14", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "14", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "14", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "14", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "14", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "14", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "14", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "14", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "14", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "14", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 831, "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "15", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "15", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "15", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "15", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "15", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "15", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "15", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "15", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "15", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "15", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "15", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "fire"["age" = "15", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "fire"["age" = "15", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "fire"["age" = "15", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "fire"["age" = "15", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "fire"["age" = "15", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "fire"["age" = "15", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "fire"["age" = "15", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 832, "mob_spawner", "mob_spawner");
    register!(registrar, 848, "oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 849, "oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 850, "oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 851, "oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 852, "oak_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "oak_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "oak_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "oak_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "oak_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "oak_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 853, "oak_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "oak_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "oak_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "oak_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "oak_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "oak_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 854, "oak_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "oak_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "oak_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "oak_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "oak_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "oak_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 855, "oak_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "oak_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "oak_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "oak_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "oak_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "oak_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 866, "chest"["facing" = "north", "type" = "single"], "chest"["facing" = "north"]);
    register!(registrar, 867, "chest"["facing" = "south", "type" = "single"], "chest"["facing" = "south"]);
    register!(registrar, 868, "chest"["facing" = "west", "type" = "single"], "chest"["facing" = "west"]);
    register!(registrar, 869, "chest"["facing" = "east", "type" = "single"], "chest"["facing" = "east"]);
    register!(registrar, 880, "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "0", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "0", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "0", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "0", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "0", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "0", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "0", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "0", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "0", "south" = "up", "west" = "up"]);
    register!(registrar, 881, "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "1", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "1", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "1", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "1", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "1", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "1", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "1", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "1", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "1", "south" = "up", "west" = "up"]);
    register!(registrar, 882, "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "2", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "2", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "2", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "2", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "2", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "2", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "2", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "2", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "2", "south" = "up", "west" = "up"]);
    register!(registrar, 883, "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "3", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "3", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "3", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "3", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "3", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "3", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "3", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "3", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "3", "south" = "up", "west" = "up"]);
    register!(registrar, 884, "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "4", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "4", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "4", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "4", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "4", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "4", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "4", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "4", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "4", "south" = "up", "west" = "up"]);
    register!(registrar, 885, "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "5", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "5", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "5", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "5", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "5", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "5", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "5", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "5", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "5", "south" = "up", "west" = "up"]);
    register!(registrar, 886, "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "6", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "6", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "6", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "6", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "6", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "6", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "6", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "6", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "6", "south" = "up", "west" = "up"]);
    register!(registrar, 887, "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "7", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "7", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "7", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "7", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "7", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "7", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "7", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "7", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "7", "south" = "up", "west" = "up"]);
    register!(registrar, 888, "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "8", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "8", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "8", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "8", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "8", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "8", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "8", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "8", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "8", "south" = "up", "west" = "up"]);
    register!(registrar, 889, "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "9", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "9", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "9", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "9", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "9", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "9", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "9", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "9", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "9", "south" = "up", "west" = "up"]);
    register!(registrar, 890, "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "10", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "10", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "10", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "10", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "10", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "10", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "10", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "10", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "10", "south" = "up", "west" = "up"]);
    register!(registrar, 891, "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "11", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "11", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "11", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "11", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "11", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "11", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "11", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "11", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "11", "south" = "up", "west" = "up"]);
    register!(registrar, 892, "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "12", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "12", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "12", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "12", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "12", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "12", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "12", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "12", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "12", "south" = "up", "west" = "up"]);
    register!(registrar, 893, "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "13", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "13", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "13", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "13", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "13", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "13", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "13", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "13", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "13", "south" = "up", "west" = "up"]);
    register!(registrar, 894, "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "14", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "14", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "14", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "14", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "14", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "14", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "14", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "14", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "14", "south" = "up", "west" = "up"]);
    register!(registrar, 895, "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "none", "power" = "15", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "side", "power" = "15", "south" = "up", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "none", "north" = "up", "power" = "15", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "none", "power" = "15", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "side", "power" = "15", "south" = "up", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "side", "north" = "up", "power" = "15", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "none", "power" = "15", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "side", "power" = "15", "south" = "up", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "none", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "none", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "none", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "side", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "side", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "side", "west" = "up"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "up", "west" = "none"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "up", "west" = "side"], "redstone_wire"["east" = "up", "north" = "up", "power" = "15", "south" = "up", "west" = "up"]);
    register!(registrar, 896, "diamond_ore", "diamond_ore");
    register!(registrar, 912, "diamond_block", "diamond_block");
    register!(registrar, 928, "crafting_table", "crafting_table");
    register!(registrar, 944, "wheat"["age" = "0"], "wheat"["age" = "0"]);
    register!(registrar, 945, "wheat"["age" = "1"], "wheat"["age" = "1"]);
    register!(registrar, 946, "wheat"["age" = "2"], "wheat"["age" = "2"]);
    register!(registrar, 947, "wheat"["age" = "3"], "wheat"["age" = "3"]);
    register!(registrar, 948, "wheat"["age" = "4"], "wheat"["age" = "4"]);
    register!(registrar, 949, "wheat"["age" = "5"], "wheat"["age" = "5"]);
    register!(registrar, 950, "wheat"["age" = "6"], "wheat"["age" = "6"]);
    register!(registrar, 951, "wheat"["age" = "7"], "wheat"["age" = "7"]);
    register!(
        registrar,
        960,
        "farmland"["moisture" = "0"],
        "farmland"["moisture" = "0"]
    );
    register!(
        registrar,
        961,
        "farmland"["moisture" = "1"],
        "farmland"["moisture" = "1"]
    );
    register!(
        registrar,
        962,
        "farmland"["moisture" = "2"],
        "farmland"["moisture" = "2"]
    );
    register!(
        registrar,
        963,
        "farmland"["moisture" = "3"],
        "farmland"["moisture" = "3"]
    );
    register!(
        registrar,
        964,
        "farmland"["moisture" = "4"],
        "farmland"["moisture" = "4"]
    );
    register!(
        registrar,
        965,
        "farmland"["moisture" = "5"],
        "farmland"["moisture" = "5"]
    );
    register!(
        registrar,
        966,
        "farmland"["moisture" = "6"],
        "farmland"["moisture" = "6"]
    );
    register!(
        registrar,
        967,
        "farmland"["moisture" = "7"],
        "farmland"["moisture" = "7"]
    );
    register!(registrar, 978, "furnace"["facing" = "north", "lit" = "false"], "furnace"["facing" = "north"]);
    register!(registrar, 979, "furnace"["facing" = "south", "lit" = "false"], "furnace"["facing" = "south"]);
    register!(registrar, 980, "furnace"["facing" = "west", "lit" = "false"], "furnace"["facing" = "west"]);
    register!(registrar, 981, "furnace"["facing" = "east", "lit" = "false"], "furnace"["facing" = "east"]);
    register!(registrar, 994, "furnace"["facing" = "north", "lit" = "true"], "lit_furnace"["facing" = "north"]);
    register!(registrar, 995, "furnace"["facing" = "south", "lit" = "true"], "lit_furnace"["facing" = "south"]);
    register!(registrar, 996, "furnace"["facing" = "west", "lit" = "true"], "lit_furnace"["facing" = "west"]);
    register!(registrar, 997, "furnace"["facing" = "east", "lit" = "true"], "lit_furnace"["facing" = "east"]);
    register!(
        registrar,
        1008,
        "sign"["rotation" = "0"],
        "standing_sign"["rotation" = "0"]
    );
    register!(
        registrar,
        1009,
        "sign"["rotation" = "1"],
        "standing_sign"["rotation" = "1"]
    );
    register!(
        registrar,
        1010,
        "sign"["rotation" = "2"],
        "standing_sign"["rotation" = "2"]
    );
    register!(
        registrar,
        1011,
        "sign"["rotation" = "3"],
        "standing_sign"["rotation" = "3"]
    );
    register!(
        registrar,
        1012,
        "sign"["rotation" = "4"],
        "standing_sign"["rotation" = "4"]
    );
    register!(
        registrar,
        1013,
        "sign"["rotation" = "5"],
        "standing_sign"["rotation" = "5"]
    );
    register!(
        registrar,
        1014,
        "sign"["rotation" = "6"],
        "standing_sign"["rotation" = "6"]
    );
    register!(
        registrar,
        1015,
        "sign"["rotation" = "7"],
        "standing_sign"["rotation" = "7"]
    );
    register!(
        registrar,
        1016,
        "sign"["rotation" = "8"],
        "standing_sign"["rotation" = "8"]
    );
    register!(
        registrar,
        1017,
        "sign"["rotation" = "9"],
        "standing_sign"["rotation" = "9"]
    );
    register!(
        registrar,
        1018,
        "sign"["rotation" = "10"],
        "standing_sign"["rotation" = "10"]
    );
    register!(
        registrar,
        1019,
        "sign"["rotation" = "11"],
        "standing_sign"["rotation" = "11"]
    );
    register!(
        registrar,
        1020,
        "sign"["rotation" = "12"],
        "standing_sign"["rotation" = "12"]
    );
    register!(
        registrar,
        1021,
        "sign"["rotation" = "13"],
        "standing_sign"["rotation" = "13"]
    );
    register!(
        registrar,
        1022,
        "sign"["rotation" = "14"],
        "standing_sign"["rotation" = "14"]
    );
    register!(
        registrar,
        1023,
        "sign"["rotation" = "15"],
        "standing_sign"["rotation" = "15"]
    );
    register!(registrar, 1024, "oak_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 1025, "oak_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 1026, "oak_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 1027, "oak_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 1028, "oak_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1029, "oak_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1030, "oak_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1031, "oak_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1032, "oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 1033, "oak_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "wooden_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"]);
    register!(registrar, 1034, "oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"]);
    register!(registrar, 1035, "oak_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "wooden_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "wooden_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1036, "oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 1037, "oak_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 1038, "oak_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 1039, "oak_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(
        registrar,
        1042,
        "ladder"["facing" = "north"],
        "ladder"["facing" = "north"]
    );
    register!(
        registrar,
        1043,
        "ladder"["facing" = "south"],
        "ladder"["facing" = "south"]
    );
    register!(
        registrar,
        1044,
        "ladder"["facing" = "west"],
        "ladder"["facing" = "west"]
    );
    register!(
        registrar,
        1045,
        "ladder"["facing" = "east"],
        "ladder"["facing" = "east"]
    );
    register!(
        registrar,
        1056,
        "rail"["shape" = "north_south"],
        "rail"["shape" = "north_south"]
    );
    register!(
        registrar,
        1057,
        "rail"["shape" = "east_west"],
        "rail"["shape" = "east_west"]
    );
    register!(
        registrar,
        1058,
        "rail"["shape" = "ascending_east"],
        "rail"["shape" = "ascending_east"]
    );
    register!(
        registrar,
        1059,
        "rail"["shape" = "ascending_west"],
        "rail"["shape" = "ascending_west"]
    );
    register!(
        registrar,
        1060,
        "rail"["shape" = "ascending_north"],
        "rail"["shape" = "ascending_north"]
    );
    register!(
        registrar,
        1061,
        "rail"["shape" = "ascending_south"],
        "rail"["shape" = "ascending_south"]
    );
    register!(
        registrar,
        1062,
        "rail"["shape" = "south_east"],
        "rail"["shape" = "south_east"]
    );
    register!(
        registrar,
        1063,
        "rail"["shape" = "south_west"],
        "rail"["shape" = "south_west"]
    );
    register!(
        registrar,
        1064,
        "rail"["shape" = "north_west"],
        "rail"["shape" = "north_west"]
    );
    register!(
        registrar,
        1065,
        "rail"["shape" = "north_east"],
        "rail"["shape" = "north_east"]
    );
    register!(registrar, 1072, "cobblestone_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "stone_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "stone_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "stone_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "stone_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "stone_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1073, "cobblestone_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "stone_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "stone_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "stone_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "stone_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "stone_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1074, "cobblestone_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "stone_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "stone_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "stone_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "stone_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "stone_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1075, "cobblestone_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "stone_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "stone_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "stone_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "stone_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "stone_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1076, "cobblestone_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "stone_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "stone_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "stone_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "stone_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "stone_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1077, "cobblestone_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "stone_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "stone_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "stone_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "stone_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "stone_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1078, "cobblestone_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "stone_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "stone_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "stone_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "stone_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "stone_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1079, "cobblestone_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "stone_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "stone_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "stone_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "stone_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "stone_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(
        registrar,
        1090,
        "wall_sign"["facing" = "north"],
        "wall_sign"["facing" = "north"]
    );
    register!(
        registrar,
        1091,
        "wall_sign"["facing" = "south"],
        "wall_sign"["facing" = "south"]
    );
    register!(
        registrar,
        1092,
        "wall_sign"["facing" = "west"],
        "wall_sign"["facing" = "west"]
    );
    register!(
        registrar,
        1093,
        "wall_sign"["facing" = "east"],
        "wall_sign"["facing" = "east"]
    );
    register!(registrar, 1104, "lever"["face" = "ceiling", "facing" = "west", "powered" = "false"], "lever"["facing" = "down_x", "powered" = "false"]);
    register!(registrar, 1105, "lever"["face" = "wall", "facing" = "east", "powered" = "false"], "lever"["facing" = "east", "powered" = "false"]);
    register!(registrar, 1106, "lever"["face" = "wall", "facing" = "west", "powered" = "false"], "lever"["facing" = "west", "powered" = "false"]);
    register!(registrar, 1107, "lever"["face" = "wall", "facing" = "south", "powered" = "false"], "lever"["facing" = "south", "powered" = "false"]);
    register!(registrar, 1108, "lever"["face" = "wall", "facing" = "north", "powered" = "false"], "lever"["facing" = "north", "powered" = "false"]);
    register!(registrar, 1109, "lever"["face" = "floor", "facing" = "north", "powered" = "false"], "lever"["facing" = "up_z", "powered" = "false"]);
    register!(registrar, 1110, "lever"["face" = "floor", "facing" = "west", "powered" = "false"], "lever"["facing" = "up_x", "powered" = "false"]);
    register!(registrar, 1111, "lever"["face" = "ceiling", "facing" = "north", "powered" = "false"], "lever"["facing" = "down_z", "powered" = "false"]);
    register!(registrar, 1112, "lever"["face" = "ceiling", "facing" = "west", "powered" = "true"], "lever"["facing" = "down_x", "powered" = "true"]);
    register!(registrar, 1113, "lever"["face" = "wall", "facing" = "east", "powered" = "true"], "lever"["facing" = "east", "powered" = "true"]);
    register!(registrar, 1114, "lever"["face" = "wall", "facing" = "west", "powered" = "true"], "lever"["facing" = "west", "powered" = "true"]);
    register!(registrar, 1115, "lever"["face" = "wall", "facing" = "south", "powered" = "true"], "lever"["facing" = "south", "powered" = "true"]);
    register!(registrar, 1116, "lever"["face" = "wall", "facing" = "north", "powered" = "true"], "lever"["facing" = "north", "powered" = "true"]);
    register!(registrar, 1117, "lever"["face" = "floor", "facing" = "north", "powered" = "true"], "lever"["facing" = "up_z", "powered" = "true"]);
    register!(registrar, 1118, "lever"["face" = "floor", "facing" = "west", "powered" = "true"], "lever"["facing" = "up_x", "powered" = "true"]);
    register!(registrar, 1119, "lever"["face" = "ceiling", "facing" = "north", "powered" = "true"], "lever"["facing" = "down_z", "powered" = "true"]);
    register!(
        registrar,
        1120,
        "stone_pressure_plate"["powered" = "false"],
        "stone_pressure_plate"["powered" = "false"]
    );
    register!(
        registrar,
        1121,
        "stone_pressure_plate"["powered" = "true"],
        "stone_pressure_plate"["powered" = "true"]
    );
    register!(registrar, 1136, "iron_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 1137, "iron_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 1138, "iron_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 1139, "iron_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 1140, "iron_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "iron_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "iron_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1141, "iron_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "iron_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "iron_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1142, "iron_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "iron_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "iron_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1143, "iron_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "iron_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "iron_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1144, "iron_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "iron_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "iron_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "iron_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "iron_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 1145, "iron_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "iron_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "iron_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"]);
    register!(registrar, 1146, "iron_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "iron_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "iron_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "iron_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "iron_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"]);
    register!(registrar, 1147, "iron_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "iron_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "iron_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "iron_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "iron_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "iron_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "iron_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "iron_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "iron_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 1148, "iron_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 1149, "iron_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 1150, "iron_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 1151, "iron_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(
        registrar,
        1152,
        "oak_pressure_plate"["powered" = "false"],
        "wooden_pressure_plate"["powered" = "false"]
    );
    register!(
        registrar,
        1153,
        "oak_pressure_plate"["powered" = "true"],
        "wooden_pressure_plate"["powered" = "true"]
    );
    register!(
        registrar,
        1168,
        "redstone_ore"["lit" = "false"],
        "redstone_ore"
    );
    register!(
        registrar,
        1184,
        "redstone_ore"["lit" = "true"],
        "lit_redstone_ore"
    );
    register!(registrar, 1201, "redstone_wall_torch"["facing" = "east", "lit" = "false"], "unlit_redstone_torch"["facing" = "east"]);
    register!(registrar, 1202, "redstone_wall_torch"["facing" = "west", "lit" = "false"], "unlit_redstone_torch"["facing" = "west"]);
    register!(registrar, 1203, "redstone_wall_torch"["facing" = "south", "lit" = "false"], "unlit_redstone_torch"["facing" = "south"]);
    register!(registrar, 1204, "redstone_wall_torch"["facing" = "north", "lit" = "false"], "unlit_redstone_torch"["facing" = "north"]);
    register!(
        registrar,
        1205,
        "redstone_torch"["lit" = "false"],
        "unlit_redstone_torch"["facing" = "up"]
    );
    register!(registrar, 1217, "redstone_wall_torch"["facing" = "east", "lit" = "true"], "redstone_torch"["facing" = "east"]);
    register!(registrar, 1218, "redstone_wall_torch"["facing" = "west", "lit" = "true"], "redstone_torch"["facing" = "west"]);
    register!(registrar, 1219, "redstone_wall_torch"["facing" = "south", "lit" = "true"], "redstone_torch"["facing" = "south"]);
    register!(registrar, 1220, "redstone_wall_torch"["facing" = "north", "lit" = "true"], "redstone_torch"["facing" = "north"]);
    register!(
        registrar,
        1221,
        "redstone_torch"["lit" = "true"],
        "redstone_torch"["facing" = "up"]
    );
    register!(registrar, 1232, "stone_button"["face" = "ceiling", "facing" = "north", "powered" = "false"], "stone_button"["facing" = "down", "powered" = "false"]);
    register!(registrar, 1233, "stone_button"["face" = "wall", "facing" = "east", "powered" = "false"], "stone_button"["facing" = "east", "powered" = "false"]);
    register!(registrar, 1234, "stone_button"["face" = "wall", "facing" = "west", "powered" = "false"], "stone_button"["facing" = "west", "powered" = "false"]);
    register!(registrar, 1235, "stone_button"["face" = "wall", "facing" = "south", "powered" = "false"], "stone_button"["facing" = "south", "powered" = "false"]);
    register!(registrar, 1236, "stone_button"["face" = "wall", "facing" = "north", "powered" = "false"], "stone_button"["facing" = "north", "powered" = "false"]);
    register!(registrar, 1237, "stone_button"["face" = "floor", "facing" = "north", "powered" = "false"], "stone_button"["facing" = "up", "powered" = "false"]);
    register!(registrar, 1240, "stone_button"["face" = "ceiling", "facing" = "north", "powered" = "true"], "stone_button"["facing" = "down", "powered" = "true"]);
    register!(registrar, 1241, "stone_button"["face" = "wall", "facing" = "east", "powered" = "true"], "stone_button"["facing" = "east", "powered" = "true"]);
    register!(registrar, 1242, "stone_button"["face" = "wall", "facing" = "west", "powered" = "true"], "stone_button"["facing" = "west", "powered" = "true"]);
    register!(registrar, 1243, "stone_button"["face" = "wall", "facing" = "south", "powered" = "true"], "stone_button"["facing" = "south", "powered" = "true"]);
    register!(registrar, 1244, "stone_button"["face" = "wall", "facing" = "north", "powered" = "true"], "stone_button"["facing" = "north", "powered" = "true"]);
    register!(registrar, 1245, "stone_button"["face" = "floor", "facing" = "north", "powered" = "true"], "stone_button"["facing" = "up", "powered" = "true"]);
    register!(
        registrar,
        1248,
        "snow"["layers" = "1"],
        "snow_layer"["layers" = "1"]
    );
    register!(
        registrar,
        1249,
        "snow"["layers" = "2"],
        "snow_layer"["layers" = "2"]
    );
    register!(
        registrar,
        1250,
        "snow"["layers" = "3"],
        "snow_layer"["layers" = "3"]
    );
    register!(
        registrar,
        1251,
        "snow"["layers" = "4"],
        "snow_layer"["layers" = "4"]
    );
    register!(
        registrar,
        1252,
        "snow"["layers" = "5"],
        "snow_layer"["layers" = "5"]
    );
    register!(
        registrar,
        1253,
        "snow"["layers" = "6"],
        "snow_layer"["layers" = "6"]
    );
    register!(
        registrar,
        1254,
        "snow"["layers" = "7"],
        "snow_layer"["layers" = "7"]
    );
    register!(
        registrar,
        1255,
        "snow"["layers" = "8"],
        "snow_layer"["layers" = "8"]
    );
    register!(registrar, 1264, "ice", "ice");
    register!(registrar, 1280, "snow_block", "snow");
    register!(
        registrar,
        1296,
        "cactus"["age" = "0"],
        "cactus"["age" = "0"]
    );
    register!(
        registrar,
        1297,
        "cactus"["age" = "1"],
        "cactus"["age" = "1"]
    );
    register!(
        registrar,
        1298,
        "cactus"["age" = "2"],
        "cactus"["age" = "2"]
    );
    register!(
        registrar,
        1299,
        "cactus"["age" = "3"],
        "cactus"["age" = "3"]
    );
    register!(
        registrar,
        1300,
        "cactus"["age" = "4"],
        "cactus"["age" = "4"]
    );
    register!(
        registrar,
        1301,
        "cactus"["age" = "5"],
        "cactus"["age" = "5"]
    );
    register!(
        registrar,
        1302,
        "cactus"["age" = "6"],
        "cactus"["age" = "6"]
    );
    register!(
        registrar,
        1303,
        "cactus"["age" = "7"],
        "cactus"["age" = "7"]
    );
    register!(
        registrar,
        1304,
        "cactus"["age" = "8"],
        "cactus"["age" = "8"]
    );
    register!(
        registrar,
        1305,
        "cactus"["age" = "9"],
        "cactus"["age" = "9"]
    );
    register!(
        registrar,
        1306,
        "cactus"["age" = "10"],
        "cactus"["age" = "10"]
    );
    register!(
        registrar,
        1307,
        "cactus"["age" = "11"],
        "cactus"["age" = "11"]
    );
    register!(
        registrar,
        1308,
        "cactus"["age" = "12"],
        "cactus"["age" = "12"]
    );
    register!(
        registrar,
        1309,
        "cactus"["age" = "13"],
        "cactus"["age" = "13"]
    );
    register!(
        registrar,
        1310,
        "cactus"["age" = "14"],
        "cactus"["age" = "14"]
    );
    register!(
        registrar,
        1311,
        "cactus"["age" = "15"],
        "cactus"["age" = "15"]
    );
    register!(registrar, 1312, "clay", "clay");
    register!(
        registrar,
        1328,
        "sugar_cane"["age" = "0"],
        "reeds"["age" = "0"]
    );
    register!(
        registrar,
        1329,
        "sugar_cane"["age" = "1"],
        "reeds"["age" = "1"]
    );
    register!(
        registrar,
        1330,
        "sugar_cane"["age" = "2"],
        "reeds"["age" = "2"]
    );
    register!(
        registrar,
        1331,
        "sugar_cane"["age" = "3"],
        "reeds"["age" = "3"]
    );
    register!(
        registrar,
        1332,
        "sugar_cane"["age" = "4"],
        "reeds"["age" = "4"]
    );
    register!(
        registrar,
        1333,
        "sugar_cane"["age" = "5"],
        "reeds"["age" = "5"]
    );
    register!(
        registrar,
        1334,
        "sugar_cane"["age" = "6"],
        "reeds"["age" = "6"]
    );
    register!(
        registrar,
        1335,
        "sugar_cane"["age" = "7"],
        "reeds"["age" = "7"]
    );
    register!(
        registrar,
        1336,
        "sugar_cane"["age" = "8"],
        "reeds"["age" = "8"]
    );
    register!(
        registrar,
        1337,
        "sugar_cane"["age" = "9"],
        "reeds"["age" = "9"]
    );
    register!(
        registrar,
        1338,
        "sugar_cane"["age" = "10"],
        "reeds"["age" = "10"]
    );
    register!(
        registrar,
        1339,
        "sugar_cane"["age" = "11"],
        "reeds"["age" = "11"]
    );
    register!(
        registrar,
        1340,
        "sugar_cane"["age" = "12"],
        "reeds"["age" = "12"]
    );
    register!(
        registrar,
        1341,
        "sugar_cane"["age" = "13"],
        "reeds"["age" = "13"]
    );
    register!(
        registrar,
        1342,
        "sugar_cane"["age" = "14"],
        "reeds"["age" = "14"]
    );
    register!(
        registrar,
        1343,
        "sugar_cane"["age" = "15"],
        "reeds"["age" = "15"]
    );
    register!(
        registrar,
        1344,
        "jukebox"["has_record" = "false"],
        "jukebox"["has_record" = "false"]
    );
    register!(
        registrar,
        1345,
        "jukebox"["has_record" = "true"],
        "jukebox"["has_record" = "true"]
    );
    register!(registrar, 1360, "oak_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "fence"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "fence"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "fence"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "fence"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "fence"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "fence"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "fence"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "fence"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "fence"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "fence"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "fence"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "fence"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "fence"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "fence"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "fence"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(
        registrar,
        1376,
        "carved_pumpkin"["facing" = "south"],
        "pumpkin"["facing" = "south"]
    );
    register!(
        registrar,
        1377,
        "carved_pumpkin"["facing" = "west"],
        "pumpkin"["facing" = "west"]
    );
    register!(
        registrar,
        1378,
        "carved_pumpkin"["facing" = "north"],
        "pumpkin"["facing" = "north"]
    );
    register!(
        registrar,
        1379,
        "carved_pumpkin"["facing" = "east"],
        "pumpkin"["facing" = "east"]
    );
    register!(registrar, 1392, "netherrack", "netherrack");
    register!(registrar, 1408, "soul_sand", "soul_sand");
    register!(registrar, 1424, "glowstone", "glowstone");
    register!(
        registrar,
        1441,
        "portal"["axis" = "x"],
        "portal"["axis" = "x"]
    );
    register!(
        registrar,
        1442,
        "portal"["axis" = "z"],
        "portal"["axis" = "z"]
    );
    register!(
        registrar,
        1456,
        "jack_o_lantern"["facing" = "south"],
        "lit_pumpkin"["facing" = "south"]
    );
    register!(
        registrar,
        1457,
        "jack_o_lantern"["facing" = "west"],
        "lit_pumpkin"["facing" = "west"]
    );
    register!(
        registrar,
        1458,
        "jack_o_lantern"["facing" = "north"],
        "lit_pumpkin"["facing" = "north"]
    );
    register!(
        registrar,
        1459,
        "jack_o_lantern"["facing" = "east"],
        "lit_pumpkin"["facing" = "east"]
    );
    register!(
        registrar,
        1472,
        "cake"["bites" = "0"],
        "cake"["bites" = "0"]
    );
    register!(
        registrar,
        1473,
        "cake"["bites" = "1"],
        "cake"["bites" = "1"]
    );
    register!(
        registrar,
        1474,
        "cake"["bites" = "2"],
        "cake"["bites" = "2"]
    );
    register!(
        registrar,
        1475,
        "cake"["bites" = "3"],
        "cake"["bites" = "3"]
    );
    register!(
        registrar,
        1476,
        "cake"["bites" = "4"],
        "cake"["bites" = "4"]
    );
    register!(
        registrar,
        1477,
        "cake"["bites" = "5"],
        "cake"["bites" = "5"]
    );
    register!(
        registrar,
        1478,
        "cake"["bites" = "6"],
        "cake"["bites" = "6"]
    );
    register!(registrar, 1488, "repeater"["delay" = "1", "facing" = "south", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "1", "facing" = "south", "locked" = "false"], "unpowered_repeater"["delay" = "1", "facing" = "south", "locked" = "true"]);
    register!(registrar, 1489, "repeater"["delay" = "1", "facing" = "west", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "1", "facing" = "west", "locked" = "false"], "unpowered_repeater"["delay" = "1", "facing" = "west", "locked" = "true"]);
    register!(registrar, 1490, "repeater"["delay" = "1", "facing" = "north", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "1", "facing" = "north", "locked" = "false"], "unpowered_repeater"["delay" = "1", "facing" = "north", "locked" = "true"]);
    register!(registrar, 1491, "repeater"["delay" = "1", "facing" = "east", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "1", "facing" = "east", "locked" = "false"], "unpowered_repeater"["delay" = "1", "facing" = "east", "locked" = "true"]);
    register!(registrar, 1492, "repeater"["delay" = "2", "facing" = "south", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "2", "facing" = "south", "locked" = "false"], "unpowered_repeater"["delay" = "2", "facing" = "south", "locked" = "true"]);
    register!(registrar, 1493, "repeater"["delay" = "2", "facing" = "west", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "2", "facing" = "west", "locked" = "false"], "unpowered_repeater"["delay" = "2", "facing" = "west", "locked" = "true"]);
    register!(registrar, 1494, "repeater"["delay" = "2", "facing" = "north", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "2", "facing" = "north", "locked" = "false"], "unpowered_repeater"["delay" = "2", "facing" = "north", "locked" = "true"]);
    register!(registrar, 1495, "repeater"["delay" = "2", "facing" = "east", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "2", "facing" = "east", "locked" = "false"], "unpowered_repeater"["delay" = "2", "facing" = "east", "locked" = "true"]);
    register!(registrar, 1496, "repeater"["delay" = "3", "facing" = "south", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "3", "facing" = "south", "locked" = "false"], "unpowered_repeater"["delay" = "3", "facing" = "south", "locked" = "true"]);
    register!(registrar, 1497, "repeater"["delay" = "3", "facing" = "west", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "3", "facing" = "west", "locked" = "false"], "unpowered_repeater"["delay" = "3", "facing" = "west", "locked" = "true"]);
    register!(registrar, 1498, "repeater"["delay" = "3", "facing" = "north", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "3", "facing" = "north", "locked" = "false"], "unpowered_repeater"["delay" = "3", "facing" = "north", "locked" = "true"]);
    register!(registrar, 1499, "repeater"["delay" = "3", "facing" = "east", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "3", "facing" = "east", "locked" = "false"], "unpowered_repeater"["delay" = "3", "facing" = "east", "locked" = "true"]);
    register!(registrar, 1500, "repeater"["delay" = "4", "facing" = "south", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "4", "facing" = "south", "locked" = "false"], "unpowered_repeater"["delay" = "4", "facing" = "south", "locked" = "true"]);
    register!(registrar, 1501, "repeater"["delay" = "4", "facing" = "west", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "4", "facing" = "west", "locked" = "false"], "unpowered_repeater"["delay" = "4", "facing" = "west", "locked" = "true"]);
    register!(registrar, 1502, "repeater"["delay" = "4", "facing" = "north", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "4", "facing" = "north", "locked" = "false"], "unpowered_repeater"["delay" = "4", "facing" = "north", "locked" = "true"]);
    register!(registrar, 1503, "repeater"["delay" = "4", "facing" = "east", "locked" = "false", "powered" = "false"], "unpowered_repeater"["delay" = "4", "facing" = "east", "locked" = "false"], "unpowered_repeater"["delay" = "4", "facing" = "east", "locked" = "true"]);
    register!(registrar, 1504, "repeater"["delay" = "1", "facing" = "south", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "1", "facing" = "south", "locked" = "false"], "powered_repeater"["delay" = "1", "facing" = "south", "locked" = "true"]);
    register!(registrar, 1505, "repeater"["delay" = "1", "facing" = "west", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "1", "facing" = "west", "locked" = "false"], "powered_repeater"["delay" = "1", "facing" = "west", "locked" = "true"]);
    register!(registrar, 1506, "repeater"["delay" = "1", "facing" = "north", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "1", "facing" = "north", "locked" = "false"], "powered_repeater"["delay" = "1", "facing" = "north", "locked" = "true"]);
    register!(registrar, 1507, "repeater"["delay" = "1", "facing" = "east", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "1", "facing" = "east", "locked" = "false"], "powered_repeater"["delay" = "1", "facing" = "east", "locked" = "true"]);
    register!(registrar, 1508, "repeater"["delay" = "2", "facing" = "south", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "2", "facing" = "south", "locked" = "false"], "powered_repeater"["delay" = "2", "facing" = "south", "locked" = "true"]);
    register!(registrar, 1509, "repeater"["delay" = "2", "facing" = "west", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "2", "facing" = "west", "locked" = "false"], "powered_repeater"["delay" = "2", "facing" = "west", "locked" = "true"]);
    register!(registrar, 1510, "repeater"["delay" = "2", "facing" = "north", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "2", "facing" = "north", "locked" = "false"], "powered_repeater"["delay" = "2", "facing" = "north", "locked" = "true"]);
    register!(registrar, 1511, "repeater"["delay" = "2", "facing" = "east", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "2", "facing" = "east", "locked" = "false"], "powered_repeater"["delay" = "2", "facing" = "east", "locked" = "true"]);
    register!(registrar, 1512, "repeater"["delay" = "3", "facing" = "south", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "3", "facing" = "south", "locked" = "false"], "powered_repeater"["delay" = "3", "facing" = "south", "locked" = "true"]);
    register!(registrar, 1513, "repeater"["delay" = "3", "facing" = "west", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "3", "facing" = "west", "locked" = "false"], "powered_repeater"["delay" = "3", "facing" = "west", "locked" = "true"]);
    register!(registrar, 1514, "repeater"["delay" = "3", "facing" = "north", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "3", "facing" = "north", "locked" = "false"], "powered_repeater"["delay" = "3", "facing" = "north", "locked" = "true"]);
    register!(registrar, 1515, "repeater"["delay" = "3", "facing" = "east", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "3", "facing" = "east", "locked" = "false"], "powered_repeater"["delay" = "3", "facing" = "east", "locked" = "true"]);
    register!(registrar, 1516, "repeater"["delay" = "4", "facing" = "south", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "4", "facing" = "south", "locked" = "false"], "powered_repeater"["delay" = "4", "facing" = "south", "locked" = "true"]);
    register!(registrar, 1517, "repeater"["delay" = "4", "facing" = "west", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "4", "facing" = "west", "locked" = "false"], "powered_repeater"["delay" = "4", "facing" = "west", "locked" = "true"]);
    register!(registrar, 1518, "repeater"["delay" = "4", "facing" = "north", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "4", "facing" = "north", "locked" = "false"], "powered_repeater"["delay" = "4", "facing" = "north", "locked" = "true"]);
    register!(registrar, 1519, "repeater"["delay" = "4", "facing" = "east", "locked" = "false", "powered" = "true"], "powered_repeater"["delay" = "4", "facing" = "east", "locked" = "false"], "powered_repeater"["delay" = "4", "facing" = "east", "locked" = "true"]);
    register!(
        registrar,
        1520,
        "white_stained_glass",
        "stained_glass"["color" = "white"]
    );
    register!(
        registrar,
        1521,
        "orange_stained_glass",
        "stained_glass"["color" = "orange"]
    );
    register!(
        registrar,
        1522,
        "magenta_stained_glass",
        "stained_glass"["color" = "magenta"]
    );
    register!(
        registrar,
        1523,
        "light_blue_stained_glass",
        "stained_glass"["color" = "light_blue"]
    );
    register!(
        registrar,
        1524,
        "yellow_stained_glass",
        "stained_glass"["color" = "yellow"]
    );
    register!(
        registrar,
        1525,
        "lime_stained_glass",
        "stained_glass"["color" = "lime"]
    );
    register!(
        registrar,
        1526,
        "pink_stained_glass",
        "stained_glass"["color" = "pink"]
    );
    register!(
        registrar,
        1527,
        "gray_stained_glass",
        "stained_glass"["color" = "gray"]
    );
    register!(
        registrar,
        1528,
        "light_gray_stained_glass",
        "stained_glass"["color" = "silver"]
    );
    register!(
        registrar,
        1529,
        "cyan_stained_glass",
        "stained_glass"["color" = "cyan"]
    );
    register!(
        registrar,
        1530,
        "purple_stained_glass",
        "stained_glass"["color" = "purple"]
    );
    register!(
        registrar,
        1531,
        "blue_stained_glass",
        "stained_glass"["color" = "blue"]
    );
    register!(
        registrar,
        1532,
        "brown_stained_glass",
        "stained_glass"["color" = "brown"]
    );
    register!(
        registrar,
        1533,
        "green_stained_glass",
        "stained_glass"["color" = "green"]
    );
    register!(
        registrar,
        1534,
        "red_stained_glass",
        "stained_glass"["color" = "red"]
    );
    register!(
        registrar,
        1535,
        "black_stained_glass",
        "stained_glass"["color" = "black"]
    );
    register!(registrar, 1536, "oak_trapdoor"["facing" = "north", "half" = "bottom", "open" = "false"], "trapdoor"["facing" = "north", "half" = "bottom", "open" = "false"]);
    register!(registrar, 1537, "oak_trapdoor"["facing" = "south", "half" = "bottom", "open" = "false"], "trapdoor"["facing" = "south", "half" = "bottom", "open" = "false"]);
    register!(registrar, 1538, "oak_trapdoor"["facing" = "west", "half" = "bottom", "open" = "false"], "trapdoor"["facing" = "west", "half" = "bottom", "open" = "false"]);
    register!(registrar, 1539, "oak_trapdoor"["facing" = "east", "half" = "bottom", "open" = "false"], "trapdoor"["facing" = "east", "half" = "bottom", "open" = "false"]);
    register!(registrar, 1540, "oak_trapdoor"["facing" = "north", "half" = "bottom", "open" = "true"], "trapdoor"["facing" = "north", "half" = "bottom", "open" = "true"]);
    register!(registrar, 1541, "oak_trapdoor"["facing" = "south", "half" = "bottom", "open" = "true"], "trapdoor"["facing" = "south", "half" = "bottom", "open" = "true"]);
    register!(registrar, 1542, "oak_trapdoor"["facing" = "west", "half" = "bottom", "open" = "true"], "trapdoor"["facing" = "west", "half" = "bottom", "open" = "true"]);
    register!(registrar, 1543, "oak_trapdoor"["facing" = "east", "half" = "bottom", "open" = "true"], "trapdoor"["facing" = "east", "half" = "bottom", "open" = "true"]);
    register!(registrar, 1544, "oak_trapdoor"["facing" = "north", "half" = "top", "open" = "false"], "trapdoor"["facing" = "north", "half" = "top", "open" = "false"]);
    register!(registrar, 1545, "oak_trapdoor"["facing" = "south", "half" = "top", "open" = "false"], "trapdoor"["facing" = "south", "half" = "top", "open" = "false"]);
    register!(registrar, 1546, "oak_trapdoor"["facing" = "west", "half" = "top", "open" = "false"], "trapdoor"["facing" = "west", "half" = "top", "open" = "false"]);
    register!(registrar, 1547, "oak_trapdoor"["facing" = "east", "half" = "top", "open" = "false"], "trapdoor"["facing" = "east", "half" = "top", "open" = "false"]);
    register!(registrar, 1548, "oak_trapdoor"["facing" = "north", "half" = "top", "open" = "true"], "trapdoor"["facing" = "north", "half" = "top", "open" = "true"]);
    register!(registrar, 1549, "oak_trapdoor"["facing" = "south", "half" = "top", "open" = "true"], "trapdoor"["facing" = "south", "half" = "top", "open" = "true"]);
    register!(registrar, 1550, "oak_trapdoor"["facing" = "west", "half" = "top", "open" = "true"], "trapdoor"["facing" = "west", "half" = "top", "open" = "true"]);
    register!(registrar, 1551, "oak_trapdoor"["facing" = "east", "half" = "top", "open" = "true"], "trapdoor"["facing" = "east", "half" = "top", "open" = "true"]);
    register!(
        registrar,
        1552,
        "infested_stone",
        "monster_egg"["variant" = "stone"]
    );
    register!(
        registrar,
        1553,
        "infested_cobblestone",
        "monster_egg"["variant" = "cobblestone"]
    );
    register!(
        registrar,
        1554,
        "infested_stone_bricks",
        "monster_egg"["variant" = "stone_brick"]
    );
    register!(
        registrar,
        1555,
        "infested_mossy_stone_bricks",
        "monster_egg"["variant" = "mossy_brick"]
    );
    register!(
        registrar,
        1556,
        "infested_cracked_stone_bricks",
        "monster_egg"["variant" = "cracked_brick"]
    );
    register!(
        registrar,
        1557,
        "infested_chiseled_stone_bricks",
        "monster_egg"["variant" = "chiseled_brick"]
    );
    register!(
        registrar,
        1568,
        "stone_bricks",
        "stonebrick"["variant" = "stonebrick"]
    );
    register!(
        registrar,
        1569,
        "mossy_stone_bricks",
        "stonebrick"["variant" = "mossy_stonebrick"]
    );
    register!(
        registrar,
        1570,
        "cracked_stone_bricks",
        "stonebrick"["variant" = "cracked_stonebrick"]
    );
    register!(
        registrar,
        1571,
        "chiseled_stone_bricks",
        "stonebrick"["variant" = "chiseled_stonebrick"]
    );
    register!(registrar, 1584, "brown_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "false", "down" = "false"], "brown_mushroom_block"["variant" = "all_inside"]);
    register!(registrar, 1585, "brown_mushroom_block"["north" = "true", "east" = "false", "south" = "false", "west" = "true", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "north_west"]);
    register!(registrar, 1586, "brown_mushroom_block"["north" = "true", "east" = "false", "south" = "false", "west" = "false", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "north"]);
    register!(registrar, 1587, "brown_mushroom_block"["north" = "true", "east" = "true", "south" = "false", "west" = "false", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "north_east"]);
    register!(registrar, 1588, "brown_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "true", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "west"]);
    register!(registrar, 1589, "brown_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "center"]);
    register!(registrar, 1590, "brown_mushroom_block"["north" = "false", "east" = "true", "south" = "false", "west" = "false", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "east"]);
    register!(registrar, 1591, "brown_mushroom_block"["north" = "false", "east" = "false", "south" = "true", "west" = "true", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "south_west"]);
    register!(registrar, 1592, "brown_mushroom_block"["north" = "false", "east" = "false", "south" = "true", "west" = "false", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "south"]);
    register!(registrar, 1593, "brown_mushroom_block"["north" = "false", "east" = "true", "south" = "true", "west" = "false", "up" = "true", "down" = "false"], "brown_mushroom_block"["variant" = "south_east"]);
    register!(registrar, 1594, "mushroom_stem"["north" = "true", "east" = "true", "south" = "true", "west" = "true", "up" = "false", "down" = "false"], "brown_mushroom_block"["variant" = "stem"]);
    register!(registrar, 1595, "brown_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "false", "down" = "false"]);
    register!(registrar, 1596, "brown_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "false", "down" = "false"]);
    register!(registrar, 1597, "brown_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "false", "down" = "false"]);
    register!(registrar, 1598, "brown_mushroom_block"["north" = "true", "east" = "true", "south" = "true", "west" = "true", "up" = "true", "down" = "true"], "brown_mushroom_block"["variant" = "all_outside"]);
    register!(registrar, 1599, "mushroom_stem"["north" = "true", "east" = "true", "south" = "true", "west" = "true", "up" = "true", "down" = "true"], "brown_mushroom_block"["variant" = "all_stem"]);
    register!(registrar, 1600, "red_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "false", "down" = "false"], "red_mushroom_block"["variant" = "all_inside"]);
    register!(registrar, 1601, "red_mushroom_block"["north" = "true", "east" = "false", "south" = "false", "west" = "true", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "north_west"]);
    register!(registrar, 1602, "red_mushroom_block"["north" = "true", "east" = "false", "south" = "false", "west" = "false", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "north"]);
    register!(registrar, 1603, "red_mushroom_block"["north" = "true", "east" = "true", "south" = "false", "west" = "false", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "north_east"]);
    register!(registrar, 1604, "red_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "true", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "west"]);
    register!(registrar, 1605, "red_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "center"]);
    register!(registrar, 1606, "red_mushroom_block"["north" = "false", "east" = "true", "south" = "false", "west" = "false", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "east"]);
    register!(registrar, 1607, "red_mushroom_block"["north" = "false", "east" = "false", "south" = "true", "west" = "true", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "south_west"]);
    register!(registrar, 1608, "red_mushroom_block"["north" = "false", "east" = "false", "south" = "true", "west" = "false", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "south"]);
    register!(registrar, 1609, "red_mushroom_block"["north" = "false", "east" = "true", "south" = "true", "west" = "false", "up" = "true", "down" = "false"], "red_mushroom_block"["variant" = "south_east"]);
    register!(registrar, 1610, "mushroom_stem"["north" = "true", "east" = "true", "south" = "true", "west" = "true", "up" = "false", "down" = "false"], "red_mushroom_block"["variant" = "stem"]);
    register!(registrar, 1611, "red_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "false", "down" = "false"]);
    register!(registrar, 1612, "red_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "false", "down" = "false"]);
    register!(registrar, 1613, "red_mushroom_block"["north" = "false", "east" = "false", "south" = "false", "west" = "false", "up" = "false", "down" = "false"]);
    register!(registrar, 1614, "red_mushroom_block"["north" = "true", "east" = "true", "south" = "true", "west" = "true", "up" = "true", "down" = "true"], "red_mushroom_block"["variant" = "all_outside"]);
    register!(registrar, 1615, "mushroom_stem"["north" = "true", "east" = "true", "south" = "true", "west" = "true", "up" = "true", "down" = "true"], "red_mushroom_block"["variant" = "all_stem"]);
    register!(registrar, 1616, "iron_bars"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "iron_bars"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "iron_bars"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "iron_bars"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "iron_bars"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "iron_bars"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "iron_bars"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "iron_bars"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "iron_bars"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "iron_bars"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "iron_bars"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "iron_bars"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "iron_bars"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "iron_bars"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "iron_bars"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "iron_bars"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "iron_bars"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 1632, "glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "glass_pane"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "glass_pane"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "glass_pane"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "glass_pane"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "glass_pane"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "glass_pane"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "glass_pane"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "glass_pane"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "glass_pane"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "glass_pane"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "glass_pane"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "glass_pane"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "glass_pane"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "glass_pane"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 1648, "melon_block", "melon_block");
    register!(registrar, 1664, "pumpkin_stem"["age" = "0"], "pumpkin_stem"["age" = "0", "facing" = "east"], "pumpkin_stem"["age" = "0", "facing" = "north"], "pumpkin_stem"["age" = "0", "facing" = "south"], "pumpkin_stem"["age" = "0", "facing" = "up"], "pumpkin_stem"["age" = "0", "facing" = "west"]);
    register!(registrar, 1665, "pumpkin_stem"["age" = "1"], "pumpkin_stem"["age" = "1", "facing" = "east"], "pumpkin_stem"["age" = "1", "facing" = "north"], "pumpkin_stem"["age" = "1", "facing" = "south"], "pumpkin_stem"["age" = "1", "facing" = "up"], "pumpkin_stem"["age" = "1", "facing" = "west"]);
    register!(registrar, 1666, "pumpkin_stem"["age" = "2"], "pumpkin_stem"["age" = "2", "facing" = "east"], "pumpkin_stem"["age" = "2", "facing" = "north"], "pumpkin_stem"["age" = "2", "facing" = "south"], "pumpkin_stem"["age" = "2", "facing" = "up"], "pumpkin_stem"["age" = "2", "facing" = "west"]);
    register!(registrar, 1667, "pumpkin_stem"["age" = "3"], "pumpkin_stem"["age" = "3", "facing" = "east"], "pumpkin_stem"["age" = "3", "facing" = "north"], "pumpkin_stem"["age" = "3", "facing" = "south"], "pumpkin_stem"["age" = "3", "facing" = "up"], "pumpkin_stem"["age" = "3", "facing" = "west"]);
    register!(registrar, 1668, "pumpkin_stem"["age" = "4"], "pumpkin_stem"["age" = "4", "facing" = "east"], "pumpkin_stem"["age" = "4", "facing" = "north"], "pumpkin_stem"["age" = "4", "facing" = "south"], "pumpkin_stem"["age" = "4", "facing" = "up"], "pumpkin_stem"["age" = "4", "facing" = "west"]);
    register!(registrar, 1669, "pumpkin_stem"["age" = "5"], "pumpkin_stem"["age" = "5", "facing" = "east"], "pumpkin_stem"["age" = "5", "facing" = "north"], "pumpkin_stem"["age" = "5", "facing" = "south"], "pumpkin_stem"["age" = "5", "facing" = "up"], "pumpkin_stem"["age" = "5", "facing" = "west"]);
    register!(registrar, 1670, "pumpkin_stem"["age" = "6"], "pumpkin_stem"["age" = "6", "facing" = "east"], "pumpkin_stem"["age" = "6", "facing" = "north"], "pumpkin_stem"["age" = "6", "facing" = "south"], "pumpkin_stem"["age" = "6", "facing" = "up"], "pumpkin_stem"["age" = "6", "facing" = "west"]);
    register!(registrar, 1671, "pumpkin_stem"["age" = "7"], "pumpkin_stem"["age" = "7", "facing" = "east"], "pumpkin_stem"["age" = "7", "facing" = "north"], "pumpkin_stem"["age" = "7", "facing" = "south"], "pumpkin_stem"["age" = "7", "facing" = "up"], "pumpkin_stem"["age" = "7", "facing" = "west"]);
    register!(registrar, 1680, "melon_stem"["age" = "0"], "melon_stem"["age" = "0", "facing" = "east"], "melon_stem"["age" = "0", "facing" = "north"], "melon_stem"["age" = "0", "facing" = "south"], "melon_stem"["age" = "0", "facing" = "up"], "melon_stem"["age" = "0", "facing" = "west"]);
    register!(registrar, 1681, "melon_stem"["age" = "1"], "melon_stem"["age" = "1", "facing" = "east"], "melon_stem"["age" = "1", "facing" = "north"], "melon_stem"["age" = "1", "facing" = "south"], "melon_stem"["age" = "1", "facing" = "up"], "melon_stem"["age" = "1", "facing" = "west"]);
    register!(registrar, 1682, "melon_stem"["age" = "2"], "melon_stem"["age" = "2", "facing" = "east"], "melon_stem"["age" = "2", "facing" = "north"], "melon_stem"["age" = "2", "facing" = "south"], "melon_stem"["age" = "2", "facing" = "up"], "melon_stem"["age" = "2", "facing" = "west"]);
    register!(registrar, 1683, "melon_stem"["age" = "3"], "melon_stem"["age" = "3", "facing" = "east"], "melon_stem"["age" = "3", "facing" = "north"], "melon_stem"["age" = "3", "facing" = "south"], "melon_stem"["age" = "3", "facing" = "up"], "melon_stem"["age" = "3", "facing" = "west"]);
    register!(registrar, 1684, "melon_stem"["age" = "4"], "melon_stem"["age" = "4", "facing" = "east"], "melon_stem"["age" = "4", "facing" = "north"], "melon_stem"["age" = "4", "facing" = "south"], "melon_stem"["age" = "4", "facing" = "up"], "melon_stem"["age" = "4", "facing" = "west"]);
    register!(registrar, 1685, "melon_stem"["age" = "5"], "melon_stem"["age" = "5", "facing" = "east"], "melon_stem"["age" = "5", "facing" = "north"], "melon_stem"["age" = "5", "facing" = "south"], "melon_stem"["age" = "5", "facing" = "up"], "melon_stem"["age" = "5", "facing" = "west"]);
    register!(registrar, 1686, "melon_stem"["age" = "6"], "melon_stem"["age" = "6", "facing" = "east"], "melon_stem"["age" = "6", "facing" = "north"], "melon_stem"["age" = "6", "facing" = "south"], "melon_stem"["age" = "6", "facing" = "up"], "melon_stem"["age" = "6", "facing" = "west"]);
    register!(registrar, 1687, "melon_stem"["age" = "7"], "melon_stem"["age" = "7", "facing" = "east"], "melon_stem"["age" = "7", "facing" = "north"], "melon_stem"["age" = "7", "facing" = "south"], "melon_stem"["age" = "7", "facing" = "up"], "melon_stem"["age" = "7", "facing" = "west"]);
    register!(registrar, 1696, "vine"["east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "vine"["east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "vine"["east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"]);
    register!(registrar, 1697, "vine"["east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "vine"["east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "vine"["east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"]);
    register!(registrar, 1698, "vine"["east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "vine"["east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "vine"["east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"]);
    register!(registrar, 1699, "vine"["east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "vine"["east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "vine"["east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 1700, "vine"["east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "vine"["east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "vine"["east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"]);
    register!(registrar, 1701, "vine"["east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "vine"["east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "vine"["east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"]);
    register!(registrar, 1702, "vine"["east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "vine"["east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "vine"["east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"]);
    register!(registrar, 1703, "vine"["east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "vine"["east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "vine"["east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 1704, "vine"["east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "vine"["east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "vine"["east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"]);
    register!(registrar, 1705, "vine"["east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "vine"["east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "vine"["east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"]);
    register!(registrar, 1706, "vine"["east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "vine"["east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "vine"["east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"]);
    register!(registrar, 1707, "vine"["east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "vine"["east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "vine"["east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 1708, "vine"["east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "vine"["east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "vine"["east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"]);
    register!(registrar, 1709, "vine"["east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "vine"["east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "vine"["east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"]);
    register!(registrar, 1710, "vine"["east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "vine"["east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "vine"["east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"]);
    register!(registrar, 1711, "vine"["east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "vine"["east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "vine"["east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(registrar, 1712, "oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 1713, "oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 1714, "oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 1715, "oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 1716, "oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 1717, "oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 1718, "oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 1719, "oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 1720, "oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 1721, "oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 1722, "oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 1723, "oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 1724, "oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 1725, "oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 1726, "oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 1727, "oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 1728, "brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1729, "brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1730, "brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1731, "brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1732, "brick_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "brick_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "brick_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "brick_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "brick_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "brick_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1733, "brick_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "brick_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "brick_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "brick_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "brick_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "brick_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1734, "brick_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "brick_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "brick_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "brick_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "brick_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "brick_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1735, "brick_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "brick_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "brick_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "brick_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "brick_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "brick_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1744, "stone_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "stone_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "stone_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "stone_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "stone_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "stone_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1745, "stone_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "stone_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "stone_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "stone_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "stone_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "stone_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1746, "stone_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "stone_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "stone_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "stone_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "stone_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "stone_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1747, "stone_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "stone_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "stone_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "stone_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "stone_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "stone_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1748, "stone_brick_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "stone_brick_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "stone_brick_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "stone_brick_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "stone_brick_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "stone_brick_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1749, "stone_brick_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "stone_brick_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "stone_brick_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "stone_brick_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "stone_brick_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "stone_brick_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1750, "stone_brick_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "stone_brick_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "stone_brick_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "stone_brick_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "stone_brick_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "stone_brick_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1751, "stone_brick_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "stone_brick_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "stone_brick_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "stone_brick_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "stone_brick_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "stone_brick_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(
        registrar,
        1760,
        "mycelium"["snowy" = "false"],
        "mycelium"["snowy" = "false"],
        "mycelium"["snowy" = "true"]
    );
    register!(registrar, 1776, "lily_pad", "waterlily");
    register!(registrar, 1792, "nether_bricks", "nether_brick");
    register!(registrar, 1808, "nether_brick_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "nether_brick_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "nether_brick_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "nether_brick_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "nether_brick_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "nether_brick_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "nether_brick_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "nether_brick_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "nether_brick_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "nether_brick_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "nether_brick_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "nether_brick_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "nether_brick_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "nether_brick_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "nether_brick_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "nether_brick_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "nether_brick_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 1824, "nether_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "nether_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "nether_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "nether_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "nether_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "nether_brick_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1825, "nether_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "nether_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "nether_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "nether_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "nether_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "nether_brick_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1826, "nether_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "nether_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "nether_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "nether_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "nether_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "nether_brick_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1827, "nether_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "nether_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "nether_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "nether_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "nether_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "nether_brick_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 1828, "nether_brick_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "nether_brick_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "nether_brick_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "nether_brick_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "nether_brick_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "nether_brick_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1829, "nether_brick_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "nether_brick_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "nether_brick_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "nether_brick_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "nether_brick_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "nether_brick_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1830, "nether_brick_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "nether_brick_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "nether_brick_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "nether_brick_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "nether_brick_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "nether_brick_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 1831, "nether_brick_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "nether_brick_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "nether_brick_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "nether_brick_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "nether_brick_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "nether_brick_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(
        registrar,
        1840,
        "nether_wart"["age" = "0"],
        "nether_wart"["age" = "0"]
    );
    register!(
        registrar,
        1841,
        "nether_wart"["age" = "1"],
        "nether_wart"["age" = "1"]
    );
    register!(
        registrar,
        1842,
        "nether_wart"["age" = "2"],
        "nether_wart"["age" = "2"]
    );
    register!(
        registrar,
        1843,
        "nether_wart"["age" = "3"],
        "nether_wart"["age" = "3"]
    );
    register!(registrar, 1856, "enchanting_table", "enchanting_table");
    register!(registrar, 1872, "brewing_stand"["has_bottle_0" = "false", "has_bottle_1" = "false", "has_bottle_2" = "false"], "brewing_stand"["has_bottle_0" = "false", "has_bottle_1" = "false", "has_bottle_2" = "false"]);
    register!(registrar, 1873, "brewing_stand"["has_bottle_0" = "true", "has_bottle_1" = "false", "has_bottle_2" = "false"], "brewing_stand"["has_bottle_0" = "true", "has_bottle_1" = "false", "has_bottle_2" = "false"]);
    register!(registrar, 1874, "brewing_stand"["has_bottle_0" = "false", "has_bottle_1" = "true", "has_bottle_2" = "false"], "brewing_stand"["has_bottle_0" = "false", "has_bottle_1" = "true", "has_bottle_2" = "false"]);
    register!(registrar, 1875, "brewing_stand"["has_bottle_0" = "true", "has_bottle_1" = "true", "has_bottle_2" = "false"], "brewing_stand"["has_bottle_0" = "true", "has_bottle_1" = "true", "has_bottle_2" = "false"]);
    register!(registrar, 1876, "brewing_stand"["has_bottle_0" = "false", "has_bottle_1" = "false", "has_bottle_2" = "true"], "brewing_stand"["has_bottle_0" = "false", "has_bottle_1" = "false", "has_bottle_2" = "true"]);
    register!(registrar, 1877, "brewing_stand"["has_bottle_0" = "true", "has_bottle_1" = "false", "has_bottle_2" = "true"], "brewing_stand"["has_bottle_0" = "true", "has_bottle_1" = "false", "has_bottle_2" = "true"]);
    register!(registrar, 1878, "brewing_stand"["has_bottle_0" = "false", "has_bottle_1" = "true", "has_bottle_2" = "true"], "brewing_stand"["has_bottle_0" = "false", "has_bottle_1" = "true", "has_bottle_2" = "true"]);
    register!(registrar, 1879, "brewing_stand"["has_bottle_0" = "true", "has_bottle_1" = "true", "has_bottle_2" = "true"], "brewing_stand"["has_bottle_0" = "true", "has_bottle_1" = "true", "has_bottle_2" = "true"]);
    register!(
        registrar,
        1888,
        "cauldron"["level" = "0"],
        "cauldron"["level" = "0"]
    );
    register!(
        registrar,
        1889,
        "cauldron"["level" = "1"],
        "cauldron"["level" = "1"]
    );
    register!(
        registrar,
        1890,
        "cauldron"["level" = "2"],
        "cauldron"["level" = "2"]
    );
    register!(
        registrar,
        1891,
        "cauldron"["level" = "3"],
        "cauldron"["level" = "3"]
    );
    register!(registrar, 1904, "end_portal", "end_portal");
    register!(registrar, 1920, "end_portal_frame"["eye" = "false", "facing" = "south"], "end_portal_frame"["eye" = "false", "facing" = "south"]);
    register!(registrar, 1921, "end_portal_frame"["eye" = "false", "facing" = "west"], "end_portal_frame"["eye" = "false", "facing" = "west"]);
    register!(registrar, 1922, "end_portal_frame"["eye" = "false", "facing" = "north"], "end_portal_frame"["eye" = "false", "facing" = "north"]);
    register!(registrar, 1923, "end_portal_frame"["eye" = "false", "facing" = "east"], "end_portal_frame"["eye" = "false", "facing" = "east"]);
    register!(registrar, 1924, "end_portal_frame"["eye" = "true", "facing" = "south"], "end_portal_frame"["eye" = "true", "facing" = "south"]);
    register!(registrar, 1925, "end_portal_frame"["eye" = "true", "facing" = "west"], "end_portal_frame"["eye" = "true", "facing" = "west"]);
    register!(registrar, 1926, "end_portal_frame"["eye" = "true", "facing" = "north"], "end_portal_frame"["eye" = "true", "facing" = "north"]);
    register!(registrar, 1927, "end_portal_frame"["eye" = "true", "facing" = "east"], "end_portal_frame"["eye" = "true", "facing" = "east"]);
    register!(registrar, 1936, "end_stone", "end_stone");
    register!(registrar, 1952, "dragon_egg", "dragon_egg");
    register!(
        registrar,
        1968,
        "redstone_lamp"["lit" = "false"],
        "redstone_lamp"
    );
    register!(
        registrar,
        1984,
        "redstone_lamp"["lit" = "true"],
        "lit_redstone_lamp"
    );
    register!(
        registrar,
        2000,
        "oak_slab"["type" = "double"],
        "double_wooden_slab"["variant" = "oak"]
    );
    register!(
        registrar,
        2001,
        "spruce_slab"["type" = "double"],
        "double_wooden_slab"["variant" = "spruce"]
    );
    register!(
        registrar,
        2002,
        "birch_slab"["type" = "double"],
        "double_wooden_slab"["variant" = "birch"]
    );
    register!(
        registrar,
        2003,
        "jungle_slab"["type" = "double"],
        "double_wooden_slab"["variant" = "jungle"]
    );
    register!(
        registrar,
        2004,
        "acacia_slab"["type" = "double"],
        "double_wooden_slab"["variant" = "acacia"]
    );
    register!(
        registrar,
        2005,
        "dark_oak_slab"["type" = "double"],
        "double_wooden_slab"["variant" = "dark_oak"]
    );
    register!(registrar, 2016, "oak_slab"["type" = "bottom"], "wooden_slab"["half" = "bottom", "variant" = "oak"]);
    register!(registrar, 2017, "spruce_slab"["type" = "bottom"], "wooden_slab"["half" = "bottom", "variant" = "spruce"]);
    register!(registrar, 2018, "birch_slab"["type" = "bottom"], "wooden_slab"["half" = "bottom", "variant" = "birch"]);
    register!(registrar, 2019, "jungle_slab"["type" = "bottom"], "wooden_slab"["half" = "bottom", "variant" = "jungle"]);
    register!(registrar, 2020, "acacia_slab"["type" = "bottom"], "wooden_slab"["half" = "bottom", "variant" = "acacia"]);
    register!(registrar, 2021, "dark_oak_slab"["type" = "bottom"], "wooden_slab"["half" = "bottom", "variant" = "dark_oak"]);
    register!(registrar, 2024, "oak_slab"["type" = "top"], "wooden_slab"["half" = "top", "variant" = "oak"]);
    register!(registrar, 2025, "spruce_slab"["type" = "top"], "wooden_slab"["half" = "top", "variant" = "spruce"]);
    register!(registrar, 2026, "birch_slab"["type" = "top"], "wooden_slab"["half" = "top", "variant" = "birch"]);
    register!(registrar, 2027, "jungle_slab"["type" = "top"], "wooden_slab"["half" = "top", "variant" = "jungle"]);
    register!(registrar, 2028, "acacia_slab"["type" = "top"], "wooden_slab"["half" = "top", "variant" = "acacia"]);
    register!(registrar, 2029, "dark_oak_slab"["type" = "top"], "wooden_slab"["half" = "top", "variant" = "dark_oak"]);
    register!(registrar, 2032, "cocoa"["age" = "0", "facing" = "south"], "cocoa"["age" = "0", "facing" = "south"]);
    register!(registrar, 2033, "cocoa"["age" = "0", "facing" = "west"], "cocoa"["age" = "0", "facing" = "west"]);
    register!(registrar, 2034, "cocoa"["age" = "0", "facing" = "north"], "cocoa"["age" = "0", "facing" = "north"]);
    register!(registrar, 2035, "cocoa"["age" = "0", "facing" = "east"], "cocoa"["age" = "0", "facing" = "east"]);
    register!(registrar, 2036, "cocoa"["age" = "1", "facing" = "south"], "cocoa"["age" = "1", "facing" = "south"]);
    register!(registrar, 2037, "cocoa"["age" = "1", "facing" = "west"], "cocoa"["age" = "1", "facing" = "west"]);
    register!(registrar, 2038, "cocoa"["age" = "1", "facing" = "north"], "cocoa"["age" = "1", "facing" = "north"]);
    register!(registrar, 2039, "cocoa"["age" = "1", "facing" = "east"], "cocoa"["age" = "1", "facing" = "east"]);
    register!(registrar, 2040, "cocoa"["age" = "2", "facing" = "south"], "cocoa"["age" = "2", "facing" = "south"]);
    register!(registrar, 2041, "cocoa"["age" = "2", "facing" = "west"], "cocoa"["age" = "2", "facing" = "west"]);
    register!(registrar, 2042, "cocoa"["age" = "2", "facing" = "north"], "cocoa"["age" = "2", "facing" = "north"]);
    register!(registrar, 2043, "cocoa"["age" = "2", "facing" = "east"], "cocoa"["age" = "2", "facing" = "east"]);
    register!(registrar, 2048, "sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2049, "sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2050, "sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2051, "sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2052, "sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2053, "sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2054, "sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2055, "sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2064, "emerald_ore", "emerald_ore");
    register!(
        registrar,
        2082,
        "ender_chest"["facing" = "north"],
        "ender_chest"["facing" = "north"]
    );
    register!(
        registrar,
        2083,
        "ender_chest"["facing" = "south"],
        "ender_chest"["facing" = "south"]
    );
    register!(
        registrar,
        2084,
        "ender_chest"["facing" = "west"],
        "ender_chest"["facing" = "west"]
    );
    register!(
        registrar,
        2085,
        "ender_chest"["facing" = "east"],
        "ender_chest"["facing" = "east"]
    );
    register!(registrar, 2096, "tripwire_hook"["attached" = "false", "facing" = "south", "powered" = "false"], "tripwire_hook"["attached" = "false", "facing" = "south", "powered" = "false"]);
    register!(registrar, 2097, "tripwire_hook"["attached" = "false", "facing" = "west", "powered" = "false"], "tripwire_hook"["attached" = "false", "facing" = "west", "powered" = "false"]);
    register!(registrar, 2098, "tripwire_hook"["attached" = "false", "facing" = "north", "powered" = "false"], "tripwire_hook"["attached" = "false", "facing" = "north", "powered" = "false"]);
    register!(registrar, 2099, "tripwire_hook"["attached" = "false", "facing" = "east", "powered" = "false"], "tripwire_hook"["attached" = "false", "facing" = "east", "powered" = "false"]);
    register!(registrar, 2100, "tripwire_hook"["attached" = "true", "facing" = "south", "powered" = "false"], "tripwire_hook"["attached" = "true", "facing" = "south", "powered" = "false"]);
    register!(registrar, 2101, "tripwire_hook"["attached" = "true", "facing" = "west", "powered" = "false"], "tripwire_hook"["attached" = "true", "facing" = "west", "powered" = "false"]);
    register!(registrar, 2102, "tripwire_hook"["attached" = "true", "facing" = "north", "powered" = "false"], "tripwire_hook"["attached" = "true", "facing" = "north", "powered" = "false"]);
    register!(registrar, 2103, "tripwire_hook"["attached" = "true", "facing" = "east", "powered" = "false"], "tripwire_hook"["attached" = "true", "facing" = "east", "powered" = "false"]);
    register!(registrar, 2104, "tripwire_hook"["attached" = "false", "facing" = "south", "powered" = "true"], "tripwire_hook"["attached" = "false", "facing" = "south", "powered" = "true"]);
    register!(registrar, 2105, "tripwire_hook"["attached" = "false", "facing" = "west", "powered" = "true"], "tripwire_hook"["attached" = "false", "facing" = "west", "powered" = "true"]);
    register!(registrar, 2106, "tripwire_hook"["attached" = "false", "facing" = "north", "powered" = "true"], "tripwire_hook"["attached" = "false", "facing" = "north", "powered" = "true"]);
    register!(registrar, 2107, "tripwire_hook"["attached" = "false", "facing" = "east", "powered" = "true"], "tripwire_hook"["attached" = "false", "facing" = "east", "powered" = "true"]);
    register!(registrar, 2108, "tripwire_hook"["attached" = "true", "facing" = "south", "powered" = "true"], "tripwire_hook"["attached" = "true", "facing" = "south", "powered" = "true"]);
    register!(registrar, 2109, "tripwire_hook"["attached" = "true", "facing" = "west", "powered" = "true"], "tripwire_hook"["attached" = "true", "facing" = "west", "powered" = "true"]);
    register!(registrar, 2110, "tripwire_hook"["attached" = "true", "facing" = "north", "powered" = "true"], "tripwire_hook"["attached" = "true", "facing" = "north", "powered" = "true"]);
    register!(registrar, 2111, "tripwire_hook"["attached" = "true", "facing" = "east", "powered" = "true"], "tripwire_hook"["attached" = "true", "facing" = "east", "powered" = "true"]);
    register!(registrar, 2112, "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "false", "south" = "true", "west" = "true"]);
    register!(registrar, 2113, "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2114, "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"]);
    register!(registrar, 2115, "tripwire"["attached" = "false", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"]);
    register!(registrar, 2116, "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "false", "south" = "true", "west" = "true"]);
    register!(registrar, 2117, "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "true", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "false", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "false", "east" = "true", "north" = "true", "powered" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2118, "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"]);
    register!(registrar, 2119, "tripwire"["attached" = "true", "disarmed" = "false", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"]);
    register!(registrar, 2120, "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "false", "south" = "true", "west" = "true"]);
    register!(registrar, 2121, "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "false", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2122, "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"]);
    register!(registrar, 2123, "tripwire"["attached" = "false", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"]);
    register!(registrar, 2124, "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "false", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "false", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "false", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "false", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "false", "south" = "true", "west" = "true"]);
    register!(registrar, 2125, "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "true", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "false", "powered" = "true", "south" = "true", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "true", "south" = "false", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "true", "south" = "false", "west" = "true"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "true", "south" = "true", "west" = "false"], "tripwire"["attached" = "true", "disarmed" = "true", "east" = "true", "north" = "true", "powered" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2126, "tripwire"["attached" = "true", "disarmed" = "true", "east" = "false", "north" = "false", "powered" = "false", "south" = "false", "west" = "false"]);
    register!(registrar, 2128, "emerald_block", "emerald_block");
    register!(registrar, 2144, "spruce_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "spruce_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "spruce_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "spruce_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "spruce_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "spruce_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2145, "spruce_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "spruce_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "spruce_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "spruce_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "spruce_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "spruce_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2146, "spruce_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "spruce_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "spruce_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "spruce_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "spruce_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "spruce_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2147, "spruce_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "spruce_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "spruce_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "spruce_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "spruce_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "spruce_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2148, "spruce_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "spruce_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "spruce_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "spruce_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "spruce_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "spruce_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2149, "spruce_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "spruce_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "spruce_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "spruce_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "spruce_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "spruce_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2150, "spruce_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "spruce_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "spruce_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "spruce_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "spruce_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "spruce_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2151, "spruce_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "spruce_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "spruce_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "spruce_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "spruce_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "spruce_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2160, "birch_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "birch_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "birch_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "birch_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "birch_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "birch_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2161, "birch_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "birch_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "birch_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "birch_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "birch_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "birch_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2162, "birch_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "birch_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "birch_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "birch_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "birch_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "birch_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2163, "birch_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "birch_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "birch_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "birch_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "birch_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "birch_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2164, "birch_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "birch_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "birch_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "birch_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "birch_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "birch_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2165, "birch_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "birch_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "birch_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "birch_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "birch_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "birch_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2166, "birch_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "birch_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "birch_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "birch_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "birch_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "birch_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2167, "birch_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "birch_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "birch_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "birch_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "birch_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "birch_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2176, "jungle_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "jungle_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "jungle_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "jungle_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "jungle_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "jungle_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2177, "jungle_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "jungle_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "jungle_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "jungle_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "jungle_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "jungle_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2178, "jungle_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "jungle_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "jungle_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "jungle_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "jungle_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "jungle_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2179, "jungle_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "jungle_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "jungle_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "jungle_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "jungle_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "jungle_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2180, "jungle_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "jungle_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "jungle_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "jungle_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "jungle_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "jungle_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2181, "jungle_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "jungle_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "jungle_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "jungle_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "jungle_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "jungle_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2182, "jungle_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "jungle_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "jungle_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "jungle_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "jungle_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "jungle_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2183, "jungle_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "jungle_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "jungle_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "jungle_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "jungle_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "jungle_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2192, "command_block"["conditional" = "false", "facing" = "down"], "command_block"["conditional" = "false", "facing" = "down"]);
    register!(registrar, 2193, "command_block"["conditional" = "false", "facing" = "up"], "command_block"["conditional" = "false", "facing" = "up"]);
    register!(registrar, 2194, "command_block"["conditional" = "false", "facing" = "north"], "command_block"["conditional" = "false", "facing" = "north"]);
    register!(registrar, 2195, "command_block"["conditional" = "false", "facing" = "south"], "command_block"["conditional" = "false", "facing" = "south"]);
    register!(registrar, 2196, "command_block"["conditional" = "false", "facing" = "west"], "command_block"["conditional" = "false", "facing" = "west"]);
    register!(registrar, 2197, "command_block"["conditional" = "false", "facing" = "east"], "command_block"["conditional" = "false", "facing" = "east"]);
    register!(registrar, 2200, "command_block"["conditional" = "true", "facing" = "down"], "command_block"["conditional" = "true", "facing" = "down"]);
    register!(registrar, 2201, "command_block"["conditional" = "true", "facing" = "up"], "command_block"["conditional" = "true", "facing" = "up"]);
    register!(registrar, 2202, "command_block"["conditional" = "true", "facing" = "north"], "command_block"["conditional" = "true", "facing" = "north"]);
    register!(registrar, 2203, "command_block"["conditional" = "true", "facing" = "south"], "command_block"["conditional" = "true", "facing" = "south"]);
    register!(registrar, 2204, "command_block"["conditional" = "true", "facing" = "west"], "command_block"["conditional" = "true", "facing" = "west"]);
    register!(registrar, 2205, "command_block"["conditional" = "true", "facing" = "east"], "command_block"["conditional" = "true", "facing" = "east"]);
    register!(registrar, 2208, "beacon", "beacon");
    register!(registrar, 2224, "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "false", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "false", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "true", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "true", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "true", "up" = "false", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "true", "up" = "false", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "true", "up" = "true", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "true", "up" = "true", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "false", "up" = "false", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "false", "up" = "false", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "false", "up" = "true", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "false", "up" = "true", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "true", "up" = "false", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "true", "up" = "false", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "true", "up" = "true", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "true", "up" = "true", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "false", "up" = "false", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "false", "up" = "false", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "false", "up" = "true", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "false", "up" = "true", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "true", "up" = "false", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "true", "up" = "false", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "true", "up" = "true", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "true", "up" = "true", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "false", "up" = "false", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "false", "up" = "false", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "false", "up" = "true", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "false", "up" = "true", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "true", "up" = "false", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "true", "up" = "false", "variant" = "cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "true", "up" = "true", "variant" = "cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "true", "up" = "true", "variant" = "cobblestone", "west" = "true"]);
    register!(registrar, 2225, "mossy_cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "false", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "false", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "true", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "false", "up" = "true", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "true", "up" = "false", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "true", "up" = "false", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "true", "up" = "true", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "false", "south" = "true", "up" = "true", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "false", "up" = "false", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "false", "up" = "false", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "false", "up" = "true", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "false", "up" = "true", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "true", "up" = "false", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "true", "up" = "false", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "true", "up" = "true", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "false", "north" = "true", "south" = "true", "up" = "true", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "false", "up" = "false", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "false", "up" = "false", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "false", "up" = "true", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "false", "up" = "true", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "true", "up" = "false", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "true", "up" = "false", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "true", "up" = "true", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "false", "south" = "true", "up" = "true", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "false", "up" = "false", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "false", "up" = "false", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "false", "up" = "true", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "false", "up" = "true", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "true", "up" = "false", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "true", "up" = "false", "variant" = "mossy_cobblestone", "west" = "true"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "true", "up" = "true", "variant" = "mossy_cobblestone", "west" = "false"], "cobblestone_wall"["east" = "true", "north" = "true", "south" = "true", "up" = "true", "variant" = "mossy_cobblestone", "west" = "true"]);
    // There are a few changes made to flower pot here, notably handling how legacy data is handled.
    // The TE itself should contain the target item and from there the proper state can be determined. However, there are
    // blocks that do not contain a TE. So we need to make sure there is a default to fall on.
    // I simply followed the legacy handling from BlockFlowerPot from 1.8.8 to find what legacy data mapped to what.
    // It"s better than defaulting everything to a cactus.
    register!(registrar, 2240, "flower_pot", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "0"], "flower_pot"["contents" = "allium", "legacy_data" = "0"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "0"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "0"], "flower_pot"["contents" = "cactus", "legacy_data" = "0"], "flower_pot"["contents" = "dandelion", "legacy_data" = "0"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "0"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "0"], "flower_pot"["contents" = "empty", "legacy_data" = "0"], "flower_pot"["contents" = "fern", "legacy_data" = "0"], "flower_pot"["contents" = "houstonia", "legacy_data" = "0"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "0"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "0"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "0"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "0"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "0"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "0"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "0"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "0"], "flower_pot"["contents" = "rose", "legacy_data" = "0"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "0"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "0"]);
    register!(registrar, 2241, "potted_poppy", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "1"], "flower_pot"["contents" = "allium", "legacy_data" = "1"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "1"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "1"], "flower_pot"["contents" = "cactus", "legacy_data" = "1"], "flower_pot"["contents" = "dandelion", "legacy_data" = "1"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "1"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "1"], "flower_pot"["contents" = "empty", "legacy_data" = "1"], "flower_pot"["contents" = "fern", "legacy_data" = "1"], "flower_pot"["contents" = "houstonia", "legacy_data" = "1"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "1"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "1"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "1"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "1"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "1"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "1"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "1"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "1"], "flower_pot"["contents" = "rose", "legacy_data" = "1"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "1"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "1"]);
    register!(registrar, 2242, "potted_dandelion", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "2"], "flower_pot"["contents" = "allium", "legacy_data" = "2"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "2"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "2"], "flower_pot"["contents" = "cactus", "legacy_data" = "2"], "flower_pot"["contents" = "dandelion", "legacy_data" = "2"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "2"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "2"], "flower_pot"["contents" = "empty", "legacy_data" = "2"], "flower_pot"["contents" = "fern", "legacy_data" = "2"], "flower_pot"["contents" = "houstonia", "legacy_data" = "2"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "2"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "2"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "2"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "2"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "2"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "2"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "2"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "2"], "flower_pot"["contents" = "rose", "legacy_data" = "2"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "2"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "2"]);
    register!(registrar, 2243, "potted_oak_sapling", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "3"], "flower_pot"["contents" = "allium", "legacy_data" = "3"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "3"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "3"], "flower_pot"["contents" = "cactus", "legacy_data" = "3"], "flower_pot"["contents" = "dandelion", "legacy_data" = "3"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "3"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "3"], "flower_pot"["contents" = "empty", "legacy_data" = "3"], "flower_pot"["contents" = "fern", "legacy_data" = "3"], "flower_pot"["contents" = "houstonia", "legacy_data" = "3"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "3"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "3"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "3"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "3"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "3"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "3"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "3"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "3"], "flower_pot"["contents" = "rose", "legacy_data" = "3"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "3"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "3"]);
    register!(registrar, 2244, "potted_spruce_sapling", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "4"], "flower_pot"["contents" = "allium", "legacy_data" = "4"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "4"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "4"], "flower_pot"["contents" = "cactus", "legacy_data" = "4"], "flower_pot"["contents" = "dandelion", "legacy_data" = "4"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "4"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "4"], "flower_pot"["contents" = "empty", "legacy_data" = "4"], "flower_pot"["contents" = "fern", "legacy_data" = "4"], "flower_pot"["contents" = "houstonia", "legacy_data" = "4"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "4"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "4"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "4"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "4"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "4"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "4"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "4"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "4"], "flower_pot"["contents" = "rose", "legacy_data" = "4"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "4"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "4"]);
    register!(registrar, 2245, "potted_birch_sapling", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "5"], "flower_pot"["contents" = "allium", "legacy_data" = "5"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "5"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "5"], "flower_pot"["contents" = "cactus", "legacy_data" = "5"], "flower_pot"["contents" = "dandelion", "legacy_data" = "5"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "5"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "5"], "flower_pot"["contents" = "empty", "legacy_data" = "5"], "flower_pot"["contents" = "fern", "legacy_data" = "5"], "flower_pot"["contents" = "houstonia", "legacy_data" = "5"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "5"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "5"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "5"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "5"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "5"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "5"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "5"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "5"], "flower_pot"["contents" = "rose", "legacy_data" = "5"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "5"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "5"]);
    register!(registrar, 2246, "potted_jungle_sapling", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "6"], "flower_pot"["contents" = "allium", "legacy_data" = "6"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "6"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "6"], "flower_pot"["contents" = "cactus", "legacy_data" = "6"], "flower_pot"["contents" = "dandelion", "legacy_data" = "6"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "6"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "6"], "flower_pot"["contents" = "empty", "legacy_data" = "6"], "flower_pot"["contents" = "fern", "legacy_data" = "6"], "flower_pot"["contents" = "houstonia", "legacy_data" = "6"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "6"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "6"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "6"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "6"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "6"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "6"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "6"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "6"], "flower_pot"["contents" = "rose", "legacy_data" = "6"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "6"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "6"]);
    register!(registrar, 2247, "potted_red_mushroom", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "7"], "flower_pot"["contents" = "allium", "legacy_data" = "7"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "7"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "7"], "flower_pot"["contents" = "cactus", "legacy_data" = "7"], "flower_pot"["contents" = "dandelion", "legacy_data" = "7"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "7"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "7"], "flower_pot"["contents" = "empty", "legacy_data" = "7"], "flower_pot"["contents" = "fern", "legacy_data" = "7"], "flower_pot"["contents" = "houstonia", "legacy_data" = "7"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "7"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "7"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "7"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "7"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "7"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "7"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "7"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "7"], "flower_pot"["contents" = "rose", "legacy_data" = "7"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "7"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "7"]);
    register!(registrar, 2248, "potted_brown_mushroom", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "8"], "flower_pot"["contents" = "allium", "legacy_data" = "8"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "8"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "8"], "flower_pot"["contents" = "cactus", "legacy_data" = "8"], "flower_pot"["contents" = "dandelion", "legacy_data" = "8"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "8"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "8"], "flower_pot"["contents" = "empty", "legacy_data" = "8"], "flower_pot"["contents" = "fern", "legacy_data" = "8"], "flower_pot"["contents" = "houstonia", "legacy_data" = "8"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "8"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "8"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "8"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "8"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "8"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "8"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "8"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "8"], "flower_pot"["contents" = "rose", "legacy_data" = "8"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "8"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "8"]);
    register!(registrar, 2249, "potted_cactus", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "9"], "flower_pot"["contents" = "allium", "legacy_data" = "9"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "9"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "9"], "flower_pot"["contents" = "cactus", "legacy_data" = "9"], "flower_pot"["contents" = "dandelion", "legacy_data" = "9"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "9"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "9"], "flower_pot"["contents" = "empty", "legacy_data" = "9"], "flower_pot"["contents" = "fern", "legacy_data" = "9"], "flower_pot"["contents" = "houstonia", "legacy_data" = "9"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "9"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "9"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "9"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "9"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "9"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "9"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "9"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "9"], "flower_pot"["contents" = "rose", "legacy_data" = "9"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "9"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "9"]);
    register!(registrar, 2250, "potted_dead_bush", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "10"], "flower_pot"["contents" = "allium", "legacy_data" = "10"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "10"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "10"], "flower_pot"["contents" = "cactus", "legacy_data" = "10"], "flower_pot"["contents" = "dandelion", "legacy_data" = "10"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "10"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "10"], "flower_pot"["contents" = "empty", "legacy_data" = "10"], "flower_pot"["contents" = "fern", "legacy_data" = "10"], "flower_pot"["contents" = "houstonia", "legacy_data" = "10"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "10"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "10"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "10"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "10"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "10"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "10"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "10"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "10"], "flower_pot"["contents" = "rose", "legacy_data" = "10"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "10"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "10"]);
    register!(registrar, 2251, "potted_fern", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "11"], "flower_pot"["contents" = "allium", "legacy_data" = "11"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "11"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "11"], "flower_pot"["contents" = "cactus", "legacy_data" = "11"], "flower_pot"["contents" = "dandelion", "legacy_data" = "11"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "11"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "11"], "flower_pot"["contents" = "empty", "legacy_data" = "11"], "flower_pot"["contents" = "fern", "legacy_data" = "11"], "flower_pot"["contents" = "houstonia", "legacy_data" = "11"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "11"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "11"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "11"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "11"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "11"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "11"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "11"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "11"], "flower_pot"["contents" = "rose", "legacy_data" = "11"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "11"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "11"]);
    register!(registrar, 2252, "potted_acacia_sapling", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "12"], "flower_pot"["contents" = "allium", "legacy_data" = "12"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "12"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "12"], "flower_pot"["contents" = "cactus", "legacy_data" = "12"], "flower_pot"["contents" = "dandelion", "legacy_data" = "12"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "12"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "12"], "flower_pot"["contents" = "empty", "legacy_data" = "12"], "flower_pot"["contents" = "fern", "legacy_data" = "12"], "flower_pot"["contents" = "houstonia", "legacy_data" = "12"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "12"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "12"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "12"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "12"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "12"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "12"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "12"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "12"], "flower_pot"["contents" = "rose", "legacy_data" = "12"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "12"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "12"]);
    register!(registrar, 2253, "potted_dark_oak_sapling", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "13"], "flower_pot"["contents" = "allium", "legacy_data" = "13"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "13"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "13"], "flower_pot"["contents" = "cactus", "legacy_data" = "13"], "flower_pot"["contents" = "dandelion", "legacy_data" = "13"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "13"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "13"], "flower_pot"["contents" = "empty", "legacy_data" = "13"], "flower_pot"["contents" = "fern", "legacy_data" = "13"], "flower_pot"["contents" = "houstonia", "legacy_data" = "13"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "13"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "13"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "13"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "13"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "13"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "13"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "13"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "13"], "flower_pot"["contents" = "rose", "legacy_data" = "13"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "13"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "13"]);
    register!(registrar, 2254, "flower_pot", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "14"], "flower_pot"["contents" = "allium", "legacy_data" = "14"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "14"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "14"], "flower_pot"["contents" = "cactus", "legacy_data" = "14"], "flower_pot"["contents" = "dandelion", "legacy_data" = "14"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "14"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "14"], "flower_pot"["contents" = "empty", "legacy_data" = "14"], "flower_pot"["contents" = "fern", "legacy_data" = "14"], "flower_pot"["contents" = "houstonia", "legacy_data" = "14"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "14"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "14"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "14"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "14"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "14"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "14"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "14"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "14"], "flower_pot"["contents" = "rose", "legacy_data" = "14"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "14"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "14"]);
    register!(registrar, 2255, "flower_pot", "flower_pot"["contents" = "acacia_sapling", "legacy_data" = "15"], "flower_pot"["contents" = "allium", "legacy_data" = "15"], "flower_pot"["contents" = "birch_sapling", "legacy_data" = "15"], "flower_pot"["contents" = "blue_orchid", "legacy_data" = "15"], "flower_pot"["contents" = "cactus", "legacy_data" = "15"], "flower_pot"["contents" = "dandelion", "legacy_data" = "15"], "flower_pot"["contents" = "dark_oak_sapling", "legacy_data" = "15"], "flower_pot"["contents" = "dead_bush", "legacy_data" = "15"], "flower_pot"["contents" = "empty", "legacy_data" = "15"], "flower_pot"["contents" = "fern", "legacy_data" = "15"], "flower_pot"["contents" = "houstonia", "legacy_data" = "15"], "flower_pot"["contents" = "jungle_sapling", "legacy_data" = "15"], "flower_pot"["contents" = "mushroom_brown", "legacy_data" = "15"], "flower_pot"["contents" = "mushroom_red", "legacy_data" = "15"], "flower_pot"["contents" = "oak_sapling", "legacy_data" = "15"], "flower_pot"["contents" = "orange_tulip", "legacy_data" = "15"], "flower_pot"["contents" = "oxeye_daisy", "legacy_data" = "15"], "flower_pot"["contents" = "pink_tulip", "legacy_data" = "15"], "flower_pot"["contents" = "red_tulip", "legacy_data" = "15"], "flower_pot"["contents" = "rose", "legacy_data" = "15"], "flower_pot"["contents" = "spruce_sapling", "legacy_data" = "15"], "flower_pot"["contents" = "white_tulip", "legacy_data" = "15"]);
    register!(
        registrar,
        2256,
        "carrots"["age" = "0"],
        "carrots"["age" = "0"]
    );
    register!(
        registrar,
        2257,
        "carrots"["age" = "1"],
        "carrots"["age" = "1"]
    );
    register!(
        registrar,
        2258,
        "carrots"["age" = "2"],
        "carrots"["age" = "2"]
    );
    register!(
        registrar,
        2259,
        "carrots"["age" = "3"],
        "carrots"["age" = "3"]
    );
    register!(
        registrar,
        2260,
        "carrots"["age" = "4"],
        "carrots"["age" = "4"]
    );
    register!(
        registrar,
        2261,
        "carrots"["age" = "5"],
        "carrots"["age" = "5"]
    );
    register!(
        registrar,
        2262,
        "carrots"["age" = "6"],
        "carrots"["age" = "6"]
    );
    register!(
        registrar,
        2263,
        "carrots"["age" = "7"],
        "carrots"["age" = "7"]
    );
    register!(
        registrar,
        2272,
        "potatoes"["age" = "0"],
        "potatoes"["age" = "0"]
    );
    register!(
        registrar,
        2273,
        "potatoes"["age" = "1"],
        "potatoes"["age" = "1"]
    );
    register!(
        registrar,
        2274,
        "potatoes"["age" = "2"],
        "potatoes"["age" = "2"]
    );
    register!(
        registrar,
        2275,
        "potatoes"["age" = "3"],
        "potatoes"["age" = "3"]
    );
    register!(
        registrar,
        2276,
        "potatoes"["age" = "4"],
        "potatoes"["age" = "4"]
    );
    register!(
        registrar,
        2277,
        "potatoes"["age" = "5"],
        "potatoes"["age" = "5"]
    );
    register!(
        registrar,
        2278,
        "potatoes"["age" = "6"],
        "potatoes"["age" = "6"]
    );
    register!(
        registrar,
        2279,
        "potatoes"["age" = "7"],
        "potatoes"["age" = "7"]
    );
    register!(registrar, 2288, "oak_button"["face" = "ceiling", "facing" = "north", "powered" = "false"], "wooden_button"["facing" = "down", "powered" = "false"]);
    register!(registrar, 2289, "oak_button"["face" = "wall", "facing" = "east", "powered" = "false"], "wooden_button"["facing" = "east", "powered" = "false"]);
    register!(registrar, 2290, "oak_button"["face" = "wall", "facing" = "west", "powered" = "false"], "wooden_button"["facing" = "west", "powered" = "false"]);
    register!(registrar, 2291, "oak_button"["face" = "wall", "facing" = "south", "powered" = "false"], "wooden_button"["facing" = "south", "powered" = "false"]);
    register!(registrar, 2292, "oak_button"["face" = "wall", "facing" = "north", "powered" = "false"], "wooden_button"["facing" = "north", "powered" = "false"]);
    register!(registrar, 2293, "oak_button"["face" = "floor", "facing" = "north", "powered" = "false"], "wooden_button"["facing" = "up", "powered" = "false"]);
    register!(registrar, 2296, "oak_button"["face" = "ceiling", "facing" = "north", "powered" = "true"], "wooden_button"["facing" = "down", "powered" = "true"]);
    register!(registrar, 2297, "oak_button"["face" = "wall", "facing" = "east", "powered" = "true"], "wooden_button"["facing" = "east", "powered" = "true"]);
    register!(registrar, 2298, "oak_button"["face" = "wall", "facing" = "west", "powered" = "true"], "wooden_button"["facing" = "west", "powered" = "true"]);
    register!(registrar, 2299, "oak_button"["face" = "wall", "facing" = "south", "powered" = "true"], "wooden_button"["facing" = "south", "powered" = "true"]);
    register!(registrar, 2300, "oak_button"["face" = "wall", "facing" = "north", "powered" = "true"], "wooden_button"["facing" = "north", "powered" = "true"]);
    register!(registrar, 2301, "oak_button"["face" = "floor", "facing" = "north", "powered" = "true"], "wooden_button"["facing" = "up", "powered" = "true"]);
    register!(registrar, 2304, "%%FILTER_ME%%"["facing" = "down", "nodrop" = "false"], "skull"["facing" = "down", "nodrop" = "false"]);
    register!(registrar, 2305, "%%FILTER_ME%%"["facing" = "up", "nodrop" = "false"], "skull"["facing" = "up", "nodrop" = "false"]);
    register!(registrar, 2306, "%%FILTER_ME%%"["facing" = "north", "nodrop" = "false"], "skull"["facing" = "north", "nodrop" = "false"]);
    register!(registrar, 2307, "%%FILTER_ME%%"["facing" = "south", "nodrop" = "false"], "skull"["facing" = "south", "nodrop" = "false"]);
    register!(registrar, 2308, "%%FILTER_ME%%"["facing" = "west", "nodrop" = "false"], "skull"["facing" = "west", "nodrop" = "false"]);
    register!(registrar, 2309, "%%FILTER_ME%%"["facing" = "east", "nodrop" = "false"], "skull"["facing" = "east", "nodrop" = "false"]);
    register!(registrar, 2312, "%%FILTER_ME%%"["facing" = "down", "nodrop" = "true"], "skull"["facing" = "down", "nodrop" = "true"]);
    register!(registrar, 2313, "%%FILTER_ME%%"["facing" = "up", "nodrop" = "true"], "skull"["facing" = "up", "nodrop" = "true"]);
    register!(registrar, 2314, "%%FILTER_ME%%"["facing" = "north", "nodrop" = "true"], "skull"["facing" = "north", "nodrop" = "true"]);
    register!(registrar, 2315, "%%FILTER_ME%%"["facing" = "south", "nodrop" = "true"], "skull"["facing" = "south", "nodrop" = "true"]);
    register!(registrar, 2316, "%%FILTER_ME%%"["facing" = "west", "nodrop" = "true"], "skull"["facing" = "west", "nodrop" = "true"]);
    register!(registrar, 2317, "%%FILTER_ME%%"["facing" = "east", "nodrop" = "true"], "skull"["facing" = "east", "nodrop" = "true"]);
    register!(registrar, 2320, "anvil"["facing" = "south"], "anvil"["damage" = "0", "facing" = "south"]);
    register!(registrar, 2321, "anvil"["facing" = "west"], "anvil"["damage" = "0", "facing" = "west"]);
    register!(registrar, 2322, "anvil"["facing" = "north"], "anvil"["damage" = "0", "facing" = "north"]);
    register!(registrar, 2323, "anvil"["facing" = "east"], "anvil"["damage" = "0", "facing" = "east"]);
    register!(registrar, 2324, "chipped_anvil"["facing" = "south"], "anvil"["damage" = "1", "facing" = "south"]);
    register!(registrar, 2325, "chipped_anvil"["facing" = "west"], "anvil"["damage" = "1", "facing" = "west"]);
    register!(registrar, 2326, "chipped_anvil"["facing" = "north"], "anvil"["damage" = "1", "facing" = "north"]);
    register!(registrar, 2327, "chipped_anvil"["facing" = "east"], "anvil"["damage" = "1", "facing" = "east"]);
    register!(registrar, 2328, "damaged_anvil"["facing" = "south"], "anvil"["damage" = "2", "facing" = "south"]);
    register!(registrar, 2329, "damaged_anvil"["facing" = "west"], "anvil"["damage" = "2", "facing" = "west"]);
    register!(registrar, 2330, "damaged_anvil"["facing" = "north"], "anvil"["damage" = "2", "facing" = "north"]);
    register!(registrar, 2331, "damaged_anvil"["facing" = "east"], "anvil"["damage" = "2", "facing" = "east"]);
    register!(registrar, 2338, "trapped_chest"["facing" = "north", "type" = "single"], "trapped_chest"["facing" = "north"]);
    register!(registrar, 2339, "trapped_chest"["facing" = "south", "type" = "single"], "trapped_chest"["facing" = "south"]);
    register!(registrar, 2340, "trapped_chest"["facing" = "west", "type" = "single"], "trapped_chest"["facing" = "west"]);
    register!(registrar, 2341, "trapped_chest"["facing" = "east", "type" = "single"], "trapped_chest"["facing" = "east"]);
    register!(
        registrar,
        2352,
        "light_weighted_pressure_plate"["power" = "0"],
        "light_weighted_pressure_plate"["power" = "0"]
    );
    register!(
        registrar,
        2353,
        "light_weighted_pressure_plate"["power" = "1"],
        "light_weighted_pressure_plate"["power" = "1"]
    );
    register!(
        registrar,
        2354,
        "light_weighted_pressure_plate"["power" = "2"],
        "light_weighted_pressure_plate"["power" = "2"]
    );
    register!(
        registrar,
        2355,
        "light_weighted_pressure_plate"["power" = "3"],
        "light_weighted_pressure_plate"["power" = "3"]
    );
    register!(
        registrar,
        2356,
        "light_weighted_pressure_plate"["power" = "4"],
        "light_weighted_pressure_plate"["power" = "4"]
    );
    register!(
        registrar,
        2357,
        "light_weighted_pressure_plate"["power" = "5"],
        "light_weighted_pressure_plate"["power" = "5"]
    );
    register!(
        registrar,
        2358,
        "light_weighted_pressure_plate"["power" = "6"],
        "light_weighted_pressure_plate"["power" = "6"]
    );
    register!(
        registrar,
        2359,
        "light_weighted_pressure_plate"["power" = "7"],
        "light_weighted_pressure_plate"["power" = "7"]
    );
    register!(
        registrar,
        2360,
        "light_weighted_pressure_plate"["power" = "8"],
        "light_weighted_pressure_plate"["power" = "8"]
    );
    register!(
        registrar,
        2361,
        "light_weighted_pressure_plate"["power" = "9"],
        "light_weighted_pressure_plate"["power" = "9"]
    );
    register!(
        registrar,
        2362,
        "light_weighted_pressure_plate"["power" = "10"],
        "light_weighted_pressure_plate"["power" = "10"]
    );
    register!(
        registrar,
        2363,
        "light_weighted_pressure_plate"["power" = "11"],
        "light_weighted_pressure_plate"["power" = "11"]
    );
    register!(
        registrar,
        2364,
        "light_weighted_pressure_plate"["power" = "12"],
        "light_weighted_pressure_plate"["power" = "12"]
    );
    register!(
        registrar,
        2365,
        "light_weighted_pressure_plate"["power" = "13"],
        "light_weighted_pressure_plate"["power" = "13"]
    );
    register!(
        registrar,
        2366,
        "light_weighted_pressure_plate"["power" = "14"],
        "light_weighted_pressure_plate"["power" = "14"]
    );
    register!(
        registrar,
        2367,
        "light_weighted_pressure_plate"["power" = "15"],
        "light_weighted_pressure_plate"["power" = "15"]
    );
    register!(
        registrar,
        2368,
        "heavy_weighted_pressure_plate"["power" = "0"],
        "heavy_weighted_pressure_plate"["power" = "0"]
    );
    register!(
        registrar,
        2369,
        "heavy_weighted_pressure_plate"["power" = "1"],
        "heavy_weighted_pressure_plate"["power" = "1"]
    );
    register!(
        registrar,
        2370,
        "heavy_weighted_pressure_plate"["power" = "2"],
        "heavy_weighted_pressure_plate"["power" = "2"]
    );
    register!(
        registrar,
        2371,
        "heavy_weighted_pressure_plate"["power" = "3"],
        "heavy_weighted_pressure_plate"["power" = "3"]
    );
    register!(
        registrar,
        2372,
        "heavy_weighted_pressure_plate"["power" = "4"],
        "heavy_weighted_pressure_plate"["power" = "4"]
    );
    register!(
        registrar,
        2373,
        "heavy_weighted_pressure_plate"["power" = "5"],
        "heavy_weighted_pressure_plate"["power" = "5"]
    );
    register!(
        registrar,
        2374,
        "heavy_weighted_pressure_plate"["power" = "6"],
        "heavy_weighted_pressure_plate"["power" = "6"]
    );
    register!(
        registrar,
        2375,
        "heavy_weighted_pressure_plate"["power" = "7"],
        "heavy_weighted_pressure_plate"["power" = "7"]
    );
    register!(
        registrar,
        2376,
        "heavy_weighted_pressure_plate"["power" = "8"],
        "heavy_weighted_pressure_plate"["power" = "8"]
    );
    register!(
        registrar,
        2377,
        "heavy_weighted_pressure_plate"["power" = "9"],
        "heavy_weighted_pressure_plate"["power" = "9"]
    );
    register!(
        registrar,
        2378,
        "heavy_weighted_pressure_plate"["power" = "10"],
        "heavy_weighted_pressure_plate"["power" = "10"]
    );
    register!(
        registrar,
        2379,
        "heavy_weighted_pressure_plate"["power" = "11"],
        "heavy_weighted_pressure_plate"["power" = "11"]
    );
    register!(
        registrar,
        2380,
        "heavy_weighted_pressure_plate"["power" = "12"],
        "heavy_weighted_pressure_plate"["power" = "12"]
    );
    register!(
        registrar,
        2381,
        "heavy_weighted_pressure_plate"["power" = "13"],
        "heavy_weighted_pressure_plate"["power" = "13"]
    );
    register!(
        registrar,
        2382,
        "heavy_weighted_pressure_plate"["power" = "14"],
        "heavy_weighted_pressure_plate"["power" = "14"]
    );
    register!(
        registrar,
        2383,
        "heavy_weighted_pressure_plate"["power" = "15"],
        "heavy_weighted_pressure_plate"["power" = "15"]
    );
    register!(registrar, 2384, "comparator"["facing" = "south", "mode" = "compare", "powered" = "false"], "unpowered_comparator"["facing" = "south", "mode" = "compare", "powered" = "false"]);
    register!(registrar, 2385, "comparator"["facing" = "west", "mode" = "compare", "powered" = "false"], "unpowered_comparator"["facing" = "west", "mode" = "compare", "powered" = "false"]);
    register!(registrar, 2386, "comparator"["facing" = "north", "mode" = "compare", "powered" = "false"], "unpowered_comparator"["facing" = "north", "mode" = "compare", "powered" = "false"]);
    register!(registrar, 2387, "comparator"["facing" = "east", "mode" = "compare", "powered" = "false"], "unpowered_comparator"["facing" = "east", "mode" = "compare", "powered" = "false"]);
    register!(registrar, 2388, "comparator"["facing" = "south", "mode" = "subtract", "powered" = "false"], "unpowered_comparator"["facing" = "south", "mode" = "subtract", "powered" = "false"]);
    register!(registrar, 2389, "comparator"["facing" = "west", "mode" = "subtract", "powered" = "false"], "unpowered_comparator"["facing" = "west", "mode" = "subtract", "powered" = "false"]);
    register!(registrar, 2390, "comparator"["facing" = "north", "mode" = "subtract", "powered" = "false"], "unpowered_comparator"["facing" = "north", "mode" = "subtract", "powered" = "false"]);
    register!(registrar, 2391, "comparator"["facing" = "east", "mode" = "subtract", "powered" = "false"], "unpowered_comparator"["facing" = "east", "mode" = "subtract", "powered" = "false"]);
    register!(registrar, 2392, "comparator"["facing" = "south", "mode" = "compare", "powered" = "true"], "unpowered_comparator"["facing" = "south", "mode" = "compare", "powered" = "true"]);
    register!(registrar, 2393, "comparator"["facing" = "west", "mode" = "compare", "powered" = "true"], "unpowered_comparator"["facing" = "west", "mode" = "compare", "powered" = "true"]);
    register!(registrar, 2394, "comparator"["facing" = "north", "mode" = "compare", "powered" = "true"], "unpowered_comparator"["facing" = "north", "mode" = "compare", "powered" = "true"]);
    register!(registrar, 2395, "comparator"["facing" = "east", "mode" = "compare", "powered" = "true"], "unpowered_comparator"["facing" = "east", "mode" = "compare", "powered" = "true"]);
    register!(registrar, 2396, "comparator"["facing" = "south", "mode" = "subtract", "powered" = "true"], "unpowered_comparator"["facing" = "south", "mode" = "subtract", "powered" = "true"]);
    register!(registrar, 2397, "comparator"["facing" = "west", "mode" = "subtract", "powered" = "true"], "unpowered_comparator"["facing" = "west", "mode" = "subtract", "powered" = "true"]);
    register!(registrar, 2398, "comparator"["facing" = "north", "mode" = "subtract", "powered" = "true"], "unpowered_comparator"["facing" = "north", "mode" = "subtract", "powered" = "true"]);
    register!(registrar, 2399, "comparator"["facing" = "east", "mode" = "subtract", "powered" = "true"], "unpowered_comparator"["facing" = "east", "mode" = "subtract", "powered" = "true"]);
    register!(registrar, 2400, "comparator"["facing" = "south", "mode" = "compare", "powered" = "false"], "powered_comparator"["facing" = "south", "mode" = "compare", "powered" = "false"]);
    register!(registrar, 2401, "comparator"["facing" = "west", "mode" = "compare", "powered" = "false"], "powered_comparator"["facing" = "west", "mode" = "compare", "powered" = "false"]);
    register!(registrar, 2402, "comparator"["facing" = "north", "mode" = "compare", "powered" = "false"], "powered_comparator"["facing" = "north", "mode" = "compare", "powered" = "false"]);
    register!(registrar, 2403, "comparator"["facing" = "east", "mode" = "compare", "powered" = "false"], "powered_comparator"["facing" = "east", "mode" = "compare", "powered" = "false"]);
    register!(registrar, 2404, "comparator"["facing" = "south", "mode" = "subtract", "powered" = "false"], "powered_comparator"["facing" = "south", "mode" = "subtract", "powered" = "false"]);
    register!(registrar, 2405, "comparator"["facing" = "west", "mode" = "subtract", "powered" = "false"], "powered_comparator"["facing" = "west", "mode" = "subtract", "powered" = "false"]);
    register!(registrar, 2406, "comparator"["facing" = "north", "mode" = "subtract", "powered" = "false"], "powered_comparator"["facing" = "north", "mode" = "subtract", "powered" = "false"]);
    register!(registrar, 2407, "comparator"["facing" = "east", "mode" = "subtract", "powered" = "false"], "powered_comparator"["facing" = "east", "mode" = "subtract", "powered" = "false"]);
    register!(registrar, 2408, "comparator"["facing" = "south", "mode" = "compare", "powered" = "true"], "powered_comparator"["facing" = "south", "mode" = "compare", "powered" = "true"]);
    register!(registrar, 2409, "comparator"["facing" = "west", "mode" = "compare", "powered" = "true"], "powered_comparator"["facing" = "west", "mode" = "compare", "powered" = "true"]);
    register!(registrar, 2410, "comparator"["facing" = "north", "mode" = "compare", "powered" = "true"], "powered_comparator"["facing" = "north", "mode" = "compare", "powered" = "true"]);
    register!(registrar, 2411, "comparator"["facing" = "east", "mode" = "compare", "powered" = "true"], "powered_comparator"["facing" = "east", "mode" = "compare", "powered" = "true"]);
    register!(registrar, 2412, "comparator"["facing" = "south", "mode" = "subtract", "powered" = "true"], "powered_comparator"["facing" = "south", "mode" = "subtract", "powered" = "true"]);
    register!(registrar, 2413, "comparator"["facing" = "west", "mode" = "subtract", "powered" = "true"], "powered_comparator"["facing" = "west", "mode" = "subtract", "powered" = "true"]);
    register!(registrar, 2414, "comparator"["facing" = "north", "mode" = "subtract", "powered" = "true"], "powered_comparator"["facing" = "north", "mode" = "subtract", "powered" = "true"]);
    register!(registrar, 2415, "comparator"["facing" = "east", "mode" = "subtract", "powered" = "true"], "powered_comparator"["facing" = "east", "mode" = "subtract", "powered" = "true"]);
    register!(registrar, 2416, "daylight_detector"["inverted" = "false", "power" = "0"], "daylight_detector"["power" = "0"]);
    register!(registrar, 2417, "daylight_detector"["inverted" = "false", "power" = "1"], "daylight_detector"["power" = "1"]);
    register!(registrar, 2418, "daylight_detector"["inverted" = "false", "power" = "2"], "daylight_detector"["power" = "2"]);
    register!(registrar, 2419, "daylight_detector"["inverted" = "false", "power" = "3"], "daylight_detector"["power" = "3"]);
    register!(registrar, 2420, "daylight_detector"["inverted" = "false", "power" = "4"], "daylight_detector"["power" = "4"]);
    register!(registrar, 2421, "daylight_detector"["inverted" = "false", "power" = "5"], "daylight_detector"["power" = "5"]);
    register!(registrar, 2422, "daylight_detector"["inverted" = "false", "power" = "6"], "daylight_detector"["power" = "6"]);
    register!(registrar, 2423, "daylight_detector"["inverted" = "false", "power" = "7"], "daylight_detector"["power" = "7"]);
    register!(registrar, 2424, "daylight_detector"["inverted" = "false", "power" = "8"], "daylight_detector"["power" = "8"]);
    register!(registrar, 2425, "daylight_detector"["inverted" = "false", "power" = "9"], "daylight_detector"["power" = "9"]);
    register!(registrar, 2426, "daylight_detector"["inverted" = "false", "power" = "10"], "daylight_detector"["power" = "10"]);
    register!(registrar, 2427, "daylight_detector"["inverted" = "false", "power" = "11"], "daylight_detector"["power" = "11"]);
    register!(registrar, 2428, "daylight_detector"["inverted" = "false", "power" = "12"], "daylight_detector"["power" = "12"]);
    register!(registrar, 2429, "daylight_detector"["inverted" = "false", "power" = "13"], "daylight_detector"["power" = "13"]);
    register!(registrar, 2430, "daylight_detector"["inverted" = "false", "power" = "14"], "daylight_detector"["power" = "14"]);
    register!(registrar, 2431, "daylight_detector"["inverted" = "false", "power" = "15"], "daylight_detector"["power" = "15"]);
    register!(registrar, 2432, "redstone_block", "redstone_block");
    register!(registrar, 2448, "nether_quartz_ore", "quartz_ore");
    register!(registrar, 2464, "hopper"["enabled" = "true", "facing" = "down"], "hopper"["enabled" = "true", "facing" = "down"]);
    register!(registrar, 2466, "hopper"["enabled" = "true", "facing" = "north"], "hopper"["enabled" = "true", "facing" = "north"]);
    register!(registrar, 2467, "hopper"["enabled" = "true", "facing" = "south"], "hopper"["enabled" = "true", "facing" = "south"]);
    register!(registrar, 2468, "hopper"["enabled" = "true", "facing" = "west"], "hopper"["enabled" = "true", "facing" = "west"]);
    register!(registrar, 2469, "hopper"["enabled" = "true", "facing" = "east"], "hopper"["enabled" = "true", "facing" = "east"]);
    register!(registrar, 2472, "hopper"["enabled" = "false", "facing" = "down"], "hopper"["enabled" = "false", "facing" = "down"]);
    register!(registrar, 2474, "hopper"["enabled" = "false", "facing" = "north"], "hopper"["enabled" = "false", "facing" = "north"]);
    register!(registrar, 2475, "hopper"["enabled" = "false", "facing" = "south"], "hopper"["enabled" = "false", "facing" = "south"]);
    register!(registrar, 2476, "hopper"["enabled" = "false", "facing" = "west"], "hopper"["enabled" = "false", "facing" = "west"]);
    register!(registrar, 2477, "hopper"["enabled" = "false", "facing" = "east"], "hopper"["enabled" = "false", "facing" = "east"]);
    register!(
        registrar,
        2480,
        "quartz_block",
        "quartz_block"["variant" = "default"]
    );
    register!(
        registrar,
        2481,
        "chiseled_quartz_block",
        "quartz_block"["variant" = "chiseled"]
    );
    register!(
        registrar,
        2482,
        "quartz_pillar"["axis" = "y"],
        "quartz_block"["variant" = "lines_y"]
    );
    register!(
        registrar,
        2483,
        "quartz_pillar"["axis" = "x"],
        "quartz_block"["variant" = "lines_x"]
    );
    register!(
        registrar,
        2484,
        "quartz_pillar"["axis" = "z"],
        "quartz_block"["variant" = "lines_z"]
    );
    register!(registrar, 2496, "quartz_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "quartz_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "quartz_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "quartz_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "quartz_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "quartz_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2497, "quartz_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "quartz_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "quartz_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "quartz_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "quartz_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "quartz_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2498, "quartz_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "quartz_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "quartz_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "quartz_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "quartz_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "quartz_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2499, "quartz_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "quartz_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "quartz_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "quartz_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "quartz_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "quartz_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2500, "quartz_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "quartz_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "quartz_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "quartz_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "quartz_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "quartz_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2501, "quartz_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "quartz_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "quartz_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "quartz_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "quartz_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "quartz_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2502, "quartz_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "quartz_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "quartz_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "quartz_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "quartz_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "quartz_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2503, "quartz_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "quartz_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "quartz_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "quartz_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "quartz_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "quartz_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2512, "activator_rail"["powered" = "false", "shape" = "north_south"], "activator_rail"["powered" = "false", "shape" = "north_south"]);
    register!(registrar, 2513, "activator_rail"["powered" = "false", "shape" = "east_west"], "activator_rail"["powered" = "false", "shape" = "east_west"]);
    register!(registrar, 2514, "activator_rail"["powered" = "false", "shape" = "ascending_east"], "activator_rail"["powered" = "false", "shape" = "ascending_east"]);
    register!(registrar, 2515, "activator_rail"["powered" = "false", "shape" = "ascending_west"], "activator_rail"["powered" = "false", "shape" = "ascending_west"]);
    register!(registrar, 2516, "activator_rail"["powered" = "false", "shape" = "ascending_north"], "activator_rail"["powered" = "false", "shape" = "ascending_north"]);
    register!(registrar, 2517, "activator_rail"["powered" = "false", "shape" = "ascending_south"], "activator_rail"["powered" = "false", "shape" = "ascending_south"]);
    register!(registrar, 2520, "activator_rail"["powered" = "true", "shape" = "north_south"], "activator_rail"["powered" = "true", "shape" = "north_south"]);
    register!(registrar, 2521, "activator_rail"["powered" = "true", "shape" = "east_west"], "activator_rail"["powered" = "true", "shape" = "east_west"]);
    register!(registrar, 2522, "activator_rail"["powered" = "true", "shape" = "ascending_east"], "activator_rail"["powered" = "true", "shape" = "ascending_east"]);
    register!(registrar, 2523, "activator_rail"["powered" = "true", "shape" = "ascending_west"], "activator_rail"["powered" = "true", "shape" = "ascending_west"]);
    register!(registrar, 2524, "activator_rail"["powered" = "true", "shape" = "ascending_north"], "activator_rail"["powered" = "true", "shape" = "ascending_north"]);
    register!(registrar, 2525, "activator_rail"["powered" = "true", "shape" = "ascending_south"], "activator_rail"["powered" = "true", "shape" = "ascending_south"]);
    register!(registrar, 2528, "dropper"["facing" = "down", "triggered" = "false"], "dropper"["facing" = "down", "triggered" = "false"]);
    register!(registrar, 2529, "dropper"["facing" = "up", "triggered" = "false"], "dropper"["facing" = "up", "triggered" = "false"]);
    register!(registrar, 2530, "dropper"["facing" = "north", "triggered" = "false"], "dropper"["facing" = "north", "triggered" = "false"]);
    register!(registrar, 2531, "dropper"["facing" = "south", "triggered" = "false"], "dropper"["facing" = "south", "triggered" = "false"]);
    register!(registrar, 2532, "dropper"["facing" = "west", "triggered" = "false"], "dropper"["facing" = "west", "triggered" = "false"]);
    register!(registrar, 2533, "dropper"["facing" = "east", "triggered" = "false"], "dropper"["facing" = "east", "triggered" = "false"]);
    register!(registrar, 2536, "dropper"["facing" = "down", "triggered" = "true"], "dropper"["facing" = "down", "triggered" = "true"]);
    register!(registrar, 2537, "dropper"["facing" = "up", "triggered" = "true"], "dropper"["facing" = "up", "triggered" = "true"]);
    register!(registrar, 2538, "dropper"["facing" = "north", "triggered" = "true"], "dropper"["facing" = "north", "triggered" = "true"]);
    register!(registrar, 2539, "dropper"["facing" = "south", "triggered" = "true"], "dropper"["facing" = "south", "triggered" = "true"]);
    register!(registrar, 2540, "dropper"["facing" = "west", "triggered" = "true"], "dropper"["facing" = "west", "triggered" = "true"]);
    register!(registrar, 2541, "dropper"["facing" = "east", "triggered" = "true"], "dropper"["facing" = "east", "triggered" = "true"]);
    register!(
        registrar,
        2544,
        "white_terracotta",
        "stained_hardened_clay"["color" = "white"]
    );
    register!(
        registrar,
        2545,
        "orange_terracotta",
        "stained_hardened_clay"["color" = "orange"]
    );
    register!(
        registrar,
        2546,
        "magenta_terracotta",
        "stained_hardened_clay"["color" = "magenta"]
    );
    register!(
        registrar,
        2547,
        "light_blue_terracotta",
        "stained_hardened_clay"["color" = "light_blue"]
    );
    register!(
        registrar,
        2548,
        "yellow_terracotta",
        "stained_hardened_clay"["color" = "yellow"]
    );
    register!(
        registrar,
        2549,
        "lime_terracotta",
        "stained_hardened_clay"["color" = "lime"]
    );
    register!(
        registrar,
        2550,
        "pink_terracotta",
        "stained_hardened_clay"["color" = "pink"]
    );
    register!(
        registrar,
        2551,
        "gray_terracotta",
        "stained_hardened_clay"["color" = "gray"]
    );
    register!(
        registrar,
        2552,
        "light_gray_terracotta",
        "stained_hardened_clay"["color" = "silver"]
    );
    register!(
        registrar,
        2553,
        "cyan_terracotta",
        "stained_hardened_clay"["color" = "cyan"]
    );
    register!(
        registrar,
        2554,
        "purple_terracotta",
        "stained_hardened_clay"["color" = "purple"]
    );
    register!(
        registrar,
        2555,
        "blue_terracotta",
        "stained_hardened_clay"["color" = "blue"]
    );
    register!(
        registrar,
        2556,
        "brown_terracotta",
        "stained_hardened_clay"["color" = "brown"]
    );
    register!(
        registrar,
        2557,
        "green_terracotta",
        "stained_hardened_clay"["color" = "green"]
    );
    register!(
        registrar,
        2558,
        "red_terracotta",
        "stained_hardened_clay"["color" = "red"]
    );
    register!(
        registrar,
        2559,
        "black_terracotta",
        "stained_hardened_clay"["color" = "black"]
    );
    register!(registrar, 2560, "white_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "white", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "white", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "white", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "white", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "white", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "white", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "white", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "white", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2561, "orange_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "orange", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "orange", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "orange", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "orange", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "orange", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "orange", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "orange", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "orange", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2562, "magenta_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "magenta", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "magenta", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "magenta", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "magenta", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "magenta", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "magenta", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "magenta", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "magenta", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2563, "light_blue_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "light_blue", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "light_blue", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "light_blue", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "light_blue", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "light_blue", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "light_blue", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "light_blue", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "light_blue", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2564, "yellow_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "yellow", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "yellow", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "yellow", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "yellow", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "yellow", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "yellow", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "yellow", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "yellow", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2565, "lime_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "lime", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "lime", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "lime", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "lime", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "lime", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "lime", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "lime", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "lime", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2566, "pink_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "pink", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "pink", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "pink", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "pink", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "pink", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "pink", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "pink", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "pink", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2567, "gray_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "gray", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "gray", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "gray", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "gray", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "gray", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "gray", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "gray", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "gray", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2568, "light_gray_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "silver", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "silver", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "silver", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "silver", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "silver", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "silver", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "silver", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "silver", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2569, "cyan_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "cyan", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "cyan", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "cyan", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "cyan", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "cyan", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "cyan", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "cyan", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "cyan", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2570, "purple_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "purple", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "purple", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "purple", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "purple", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "purple", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "purple", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "purple", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "purple", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2571, "blue_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "blue", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "blue", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "blue", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "blue", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "blue", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "blue", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "blue", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "blue", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2572, "brown_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "brown", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "brown", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "brown", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "brown", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "brown", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "brown", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "brown", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "brown", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2573, "green_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "green", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "green", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "green", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "green", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "green", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "green", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "green", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "green", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2574, "red_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "red", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "red", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "red", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "red", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "red", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "red", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "red", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "red", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2575, "black_stained_glass_pane"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "false", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "false", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "black", "east" = "false", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "false", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "black", "east" = "false", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "false", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "black", "east" = "false", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "false", "north" = "true", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "black", "east" = "true", "north" = "false", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "true", "north" = "false", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "black", "east" = "true", "north" = "false", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "true", "north" = "false", "south" = "true", "west" = "true"], "stained_glass_pane"["color" = "black", "east" = "true", "north" = "true", "south" = "false", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "true", "north" = "true", "south" = "false", "west" = "true"], "stained_glass_pane"["color" = "black", "east" = "true", "north" = "true", "south" = "true", "west" = "false"], "stained_glass_pane"["color" = "black", "east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 2576, "acacia_leaves"["check_decay" = "false", "decayable" = "true"], "leaves2"["check_decay" = "false", "decayable" = "true", "variant" = "acacia"]);
    register!(registrar, 2577, "dark_oak_leaves"["check_decay" = "false", "decayable" = "true"], "leaves2"["check_decay" = "false", "decayable" = "true", "variant" = "dark_oak"]);
    register!(registrar, 2580, "acacia_leaves"["check_decay" = "false", "decayable" = "false"], "leaves2"["check_decay" = "false", "decayable" = "false", "variant" = "acacia"]);
    register!(registrar, 2581, "dark_oak_leaves"["check_decay" = "false", "decayable" = "false"], "leaves2"["check_decay" = "false", "decayable" = "false", "variant" = "dark_oak"]);
    register!(registrar, 2584, "acacia_leaves"["check_decay" = "true", "decayable" = "true"], "leaves2"["check_decay" = "true", "decayable" = "true", "variant" = "acacia"]);
    register!(registrar, 2585, "dark_oak_leaves"["check_decay" = "true", "decayable" = "true"], "leaves2"["check_decay" = "true", "decayable" = "true", "variant" = "dark_oak"]);
    register!(registrar, 2588, "acacia_leaves"["check_decay" = "true", "decayable" = "false"], "leaves2"["check_decay" = "true", "decayable" = "false", "variant" = "acacia"]);
    register!(registrar, 2589, "dark_oak_leaves"["check_decay" = "true", "decayable" = "false"], "leaves2"["check_decay" = "true", "decayable" = "false", "variant" = "dark_oak"]);
    register!(registrar, 2592, "acacia_log"["axis" = "y"], "log2"["axis" = "y", "variant" = "acacia"]);
    register!(registrar, 2593, "dark_oak_log"["axis" = "y"], "log2"["axis" = "y", "variant" = "dark_oak"]);
    register!(registrar, 2596, "acacia_log"["axis" = "x"], "log2"["axis" = "x", "variant" = "acacia"]);
    register!(registrar, 2597, "dark_oak_log"["axis" = "x"], "log2"["axis" = "x", "variant" = "dark_oak"]);
    register!(registrar, 2600, "acacia_log"["axis" = "z"], "log2"["axis" = "z", "variant" = "acacia"]);
    register!(registrar, 2601, "dark_oak_log"["axis" = "z"], "log2"["axis" = "z", "variant" = "dark_oak"]);
    register!(registrar, 2604, "acacia_bark", "log2"["axis" = "none", "variant" = "acacia"]);
    register!(registrar, 2605, "dark_oak_bark", "log2"["axis" = "none", "variant" = "dark_oak"]);
    register!(registrar, 2608, "acacia_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "acacia_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "acacia_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "acacia_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "acacia_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "acacia_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2609, "acacia_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "acacia_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "acacia_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "acacia_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "acacia_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "acacia_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2610, "acacia_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "acacia_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "acacia_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "acacia_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "acacia_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "acacia_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2611, "acacia_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "acacia_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "acacia_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "acacia_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "acacia_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "acacia_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2612, "acacia_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "acacia_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "acacia_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "acacia_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "acacia_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "acacia_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2613, "acacia_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "acacia_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "acacia_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "acacia_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "acacia_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "acacia_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2614, "acacia_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "acacia_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "acacia_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "acacia_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "acacia_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "acacia_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2615, "acacia_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "acacia_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "acacia_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "acacia_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "acacia_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "acacia_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2624, "dark_oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "dark_oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "dark_oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "dark_oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "dark_oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "dark_oak_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2625, "dark_oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "dark_oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "dark_oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "dark_oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "dark_oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "dark_oak_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2626, "dark_oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "dark_oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "dark_oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "dark_oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "dark_oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "dark_oak_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2627, "dark_oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "dark_oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "dark_oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "dark_oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "dark_oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "dark_oak_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2628, "dark_oak_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "dark_oak_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "dark_oak_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "dark_oak_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "dark_oak_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "dark_oak_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2629, "dark_oak_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "dark_oak_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "dark_oak_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "dark_oak_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "dark_oak_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "dark_oak_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2630, "dark_oak_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "dark_oak_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "dark_oak_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "dark_oak_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "dark_oak_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "dark_oak_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2631, "dark_oak_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "dark_oak_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "dark_oak_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "dark_oak_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "dark_oak_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "dark_oak_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2640, "slime_block", "slime");
    register!(registrar, 2656, "barrier", "barrier");
    register!(registrar, 2672, "iron_trapdoor"["facing" = "north", "half" = "bottom", "open" = "false"], "iron_trapdoor"["facing" = "north", "half" = "bottom", "open" = "false"]);
    register!(registrar, 2673, "iron_trapdoor"["facing" = "south", "half" = "bottom", "open" = "false"], "iron_trapdoor"["facing" = "south", "half" = "bottom", "open" = "false"]);
    register!(registrar, 2674, "iron_trapdoor"["facing" = "west", "half" = "bottom", "open" = "false"], "iron_trapdoor"["facing" = "west", "half" = "bottom", "open" = "false"]);
    register!(registrar, 2675, "iron_trapdoor"["facing" = "east", "half" = "bottom", "open" = "false"], "iron_trapdoor"["facing" = "east", "half" = "bottom", "open" = "false"]);
    register!(registrar, 2676, "iron_trapdoor"["facing" = "north", "half" = "bottom", "open" = "true"], "iron_trapdoor"["facing" = "north", "half" = "bottom", "open" = "true"]);
    register!(registrar, 2677, "iron_trapdoor"["facing" = "south", "half" = "bottom", "open" = "true"], "iron_trapdoor"["facing" = "south", "half" = "bottom", "open" = "true"]);
    register!(registrar, 2678, "iron_trapdoor"["facing" = "west", "half" = "bottom", "open" = "true"], "iron_trapdoor"["facing" = "west", "half" = "bottom", "open" = "true"]);
    register!(registrar, 2679, "iron_trapdoor"["facing" = "east", "half" = "bottom", "open" = "true"], "iron_trapdoor"["facing" = "east", "half" = "bottom", "open" = "true"]);
    register!(registrar, 2680, "iron_trapdoor"["facing" = "north", "half" = "top", "open" = "false"], "iron_trapdoor"["facing" = "north", "half" = "top", "open" = "false"]);
    register!(registrar, 2681, "iron_trapdoor"["facing" = "south", "half" = "top", "open" = "false"], "iron_trapdoor"["facing" = "south", "half" = "top", "open" = "false"]);
    register!(registrar, 2682, "iron_trapdoor"["facing" = "west", "half" = "top", "open" = "false"], "iron_trapdoor"["facing" = "west", "half" = "top", "open" = "false"]);
    register!(registrar, 2683, "iron_trapdoor"["facing" = "east", "half" = "top", "open" = "false"], "iron_trapdoor"["facing" = "east", "half" = "top", "open" = "false"]);
    register!(registrar, 2684, "iron_trapdoor"["facing" = "north", "half" = "top", "open" = "true"], "iron_trapdoor"["facing" = "north", "half" = "top", "open" = "true"]);
    register!(registrar, 2685, "iron_trapdoor"["facing" = "south", "half" = "top", "open" = "true"], "iron_trapdoor"["facing" = "south", "half" = "top", "open" = "true"]);
    register!(registrar, 2686, "iron_trapdoor"["facing" = "west", "half" = "top", "open" = "true"], "iron_trapdoor"["facing" = "west", "half" = "top", "open" = "true"]);
    register!(registrar, 2687, "iron_trapdoor"["facing" = "east", "half" = "top", "open" = "true"], "iron_trapdoor"["facing" = "east", "half" = "top", "open" = "true"]);
    register!(
        registrar,
        2688,
        "prismarine",
        "prismarine"["variant" = "prismarine"]
    );
    register!(
        registrar,
        2689,
        "prismarine_bricks",
        "prismarine"["variant" = "prismarine_bricks"]
    );
    register!(
        registrar,
        2690,
        "dark_prismarine",
        "prismarine"["variant" = "dark_prismarine"]
    );
    register!(registrar, 2704, "sea_lantern", "sea_lantern");
    register!(
        registrar,
        2720,
        "hay_block"["axis" = "y"],
        "hay_block"["axis" = "y"]
    );
    register!(
        registrar,
        2724,
        "hay_block"["axis" = "x"],
        "hay_block"["axis" = "x"]
    );
    register!(
        registrar,
        2728,
        "hay_block"["axis" = "z"],
        "hay_block"["axis" = "z"]
    );
    register!(registrar, 2736, "white_carpet", "carpet"["color" = "white"]);
    register!(
        registrar,
        2737,
        "orange_carpet",
        "carpet"["color" = "orange"]
    );
    register!(
        registrar,
        2738,
        "magenta_carpet",
        "carpet"["color" = "magenta"]
    );
    register!(
        registrar,
        2739,
        "light_blue_carpet",
        "carpet"["color" = "light_blue"]
    );
    register!(
        registrar,
        2740,
        "yellow_carpet",
        "carpet"["color" = "yellow"]
    );
    register!(registrar, 2741, "lime_carpet", "carpet"["color" = "lime"]);
    register!(registrar, 2742, "pink_carpet", "carpet"["color" = "pink"]);
    register!(registrar, 2743, "gray_carpet", "carpet"["color" = "gray"]);
    register!(
        registrar,
        2744,
        "light_gray_carpet",
        "carpet"["color" = "silver"]
    );
    register!(registrar, 2745, "cyan_carpet", "carpet"["color" = "cyan"]);
    register!(
        registrar,
        2746,
        "purple_carpet",
        "carpet"["color" = "purple"]
    );
    register!(registrar, 2747, "blue_carpet", "carpet"["color" = "blue"]);
    register!(registrar, 2748, "brown_carpet", "carpet"["color" = "brown"]);
    register!(registrar, 2749, "green_carpet", "carpet"["color" = "green"]);
    register!(registrar, 2750, "red_carpet", "carpet"["color" = "red"]);
    register!(registrar, 2751, "black_carpet", "carpet"["color" = "black"]);
    register!(registrar, 2752, "terracotta", "hardened_clay");
    register!(registrar, 2768, "coal_block", "coal_block");
    register!(registrar, 2784, "packed_ice", "packed_ice");
    register!(registrar, 2800, "sunflower"["half" = "lower"], "double_plant"["facing" = "east", "half" = "lower", "variant" = "sunflower"], "double_plant"["facing" = "north", "half" = "lower", "variant" = "sunflower"], "double_plant"["facing" = "south", "half" = "lower", "variant" = "sunflower"], "double_plant"["facing" = "west", "half" = "lower", "variant" = "sunflower"]);
    register!(registrar, 2801, "lilac"["half" = "lower"], "double_plant"["facing" = "east", "half" = "lower", "variant" = "syringa"], "double_plant"["facing" = "north", "half" = "lower", "variant" = "syringa"], "double_plant"["facing" = "south", "half" = "lower", "variant" = "syringa"], "double_plant"["facing" = "west", "half" = "lower", "variant" = "syringa"]);
    register!(registrar, 2802, "tall_grass"["half" = "lower"], "double_plant"["facing" = "east", "half" = "lower", "variant" = "double_grass"], "double_plant"["facing" = "north", "half" = "lower", "variant" = "double_grass"], "double_plant"["facing" = "south", "half" = "lower", "variant" = "double_grass"], "double_plant"["facing" = "west", "half" = "lower", "variant" = "double_grass"]);
    register!(registrar, 2803, "large_fern"["half" = "lower"], "double_plant"["facing" = "east", "half" = "lower", "variant" = "double_fern"], "double_plant"["facing" = "north", "half" = "lower", "variant" = "double_fern"], "double_plant"["facing" = "south", "half" = "lower", "variant" = "double_fern"], "double_plant"["facing" = "west", "half" = "lower", "variant" = "double_fern"]);
    register!(registrar, 2804, "rose_bush"["half" = "lower"], "double_plant"["facing" = "east", "half" = "lower", "variant" = "double_rose"], "double_plant"["facing" = "north", "half" = "lower", "variant" = "double_rose"], "double_plant"["facing" = "south", "half" = "lower", "variant" = "double_rose"], "double_plant"["facing" = "west", "half" = "lower", "variant" = "double_rose"]);
    register!(registrar, 2805, "peony"["half" = "lower"], "double_plant"["facing" = "east", "half" = "lower", "variant" = "paeonia"], "double_plant"["facing" = "north", "half" = "lower", "variant" = "paeonia"], "double_plant"["facing" = "south", "half" = "lower", "variant" = "paeonia"], "double_plant"["facing" = "west", "half" = "lower", "variant" = "paeonia"]);
    register!(registrar, 2808, "peony"["half" = "upper"], "double_plant"["facing" = "south", "half" = "upper", "variant" = "double_fern"], "double_plant"["facing" = "south", "half" = "upper", "variant" = "double_grass"], "double_plant"["facing" = "south", "half" = "upper", "variant" = "double_rose"], "double_plant"["facing" = "south", "half" = "upper", "variant" = "paeonia"], "double_plant"["facing" = "south", "half" = "upper", "variant" = "sunflower"], "double_plant"["facing" = "south", "half" = "upper", "variant" = "syringa"]);
    register!(registrar, 2809, "peony"["half" = "upper"], "double_plant"["facing" = "west", "half" = "upper", "variant" = "double_fern"], "double_plant"["facing" = "west", "half" = "upper", "variant" = "double_grass"], "double_plant"["facing" = "west", "half" = "upper", "variant" = "double_rose"], "double_plant"["facing" = "west", "half" = "upper", "variant" = "paeonia"], "double_plant"["facing" = "west", "half" = "upper", "variant" = "sunflower"], "double_plant"["facing" = "west", "half" = "upper", "variant" = "syringa"]);
    register!(registrar, 2810, "peony"["half" = "upper"], "double_plant"["facing" = "north", "half" = "upper", "variant" = "double_fern"], "double_plant"["facing" = "north", "half" = "upper", "variant" = "double_grass"], "double_plant"["facing" = "north", "half" = "upper", "variant" = "double_rose"], "double_plant"["facing" = "north", "half" = "upper", "variant" = "paeonia"], "double_plant"["facing" = "north", "half" = "upper", "variant" = "sunflower"], "double_plant"["facing" = "north", "half" = "upper", "variant" = "syringa"]);
    register!(registrar, 2811, "peony"["half" = "upper"], "double_plant"["facing" = "east", "half" = "upper", "variant" = "double_fern"], "double_plant"["facing" = "east", "half" = "upper", "variant" = "double_grass"], "double_plant"["facing" = "east", "half" = "upper", "variant" = "double_rose"], "double_plant"["facing" = "east", "half" = "upper", "variant" = "paeonia"], "double_plant"["facing" = "east", "half" = "upper", "variant" = "sunflower"], "double_plant"["facing" = "east", "half" = "upper", "variant" = "syringa"]);
    register!(
        registrar,
        2816,
        "white_banner"["rotation" = "0"],
        "standing_banner"["rotation" = "0"]
    );
    register!(
        registrar,
        2817,
        "white_banner"["rotation" = "1"],
        "standing_banner"["rotation" = "1"]
    );
    register!(
        registrar,
        2818,
        "white_banner"["rotation" = "2"],
        "standing_banner"["rotation" = "2"]
    );
    register!(
        registrar,
        2819,
        "white_banner"["rotation" = "3"],
        "standing_banner"["rotation" = "3"]
    );
    register!(
        registrar,
        2820,
        "white_banner"["rotation" = "4"],
        "standing_banner"["rotation" = "4"]
    );
    register!(
        registrar,
        2821,
        "white_banner"["rotation" = "5"],
        "standing_banner"["rotation" = "5"]
    );
    register!(
        registrar,
        2822,
        "white_banner"["rotation" = "6"],
        "standing_banner"["rotation" = "6"]
    );
    register!(
        registrar,
        2823,
        "white_banner"["rotation" = "7"],
        "standing_banner"["rotation" = "7"]
    );
    register!(
        registrar,
        2824,
        "white_banner"["rotation" = "8"],
        "standing_banner"["rotation" = "8"]
    );
    register!(
        registrar,
        2825,
        "white_banner"["rotation" = "9"],
        "standing_banner"["rotation" = "9"]
    );
    register!(
        registrar,
        2826,
        "white_banner"["rotation" = "10"],
        "standing_banner"["rotation" = "10"]
    );
    register!(
        registrar,
        2827,
        "white_banner"["rotation" = "11"],
        "standing_banner"["rotation" = "11"]
    );
    register!(
        registrar,
        2828,
        "white_banner"["rotation" = "12"],
        "standing_banner"["rotation" = "12"]
    );
    register!(
        registrar,
        2829,
        "white_banner"["rotation" = "13"],
        "standing_banner"["rotation" = "13"]
    );
    register!(
        registrar,
        2830,
        "white_banner"["rotation" = "14"],
        "standing_banner"["rotation" = "14"]
    );
    register!(
        registrar,
        2831,
        "white_banner"["rotation" = "15"],
        "standing_banner"["rotation" = "15"]
    );
    register!(
        registrar,
        2834,
        "white_wall_banner"["facing" = "north"],
        "wall_banner"["facing" = "north"]
    );
    register!(
        registrar,
        2835,
        "white_wall_banner"["facing" = "south"],
        "wall_banner"["facing" = "south"]
    );
    register!(
        registrar,
        2836,
        "white_wall_banner"["facing" = "west"],
        "wall_banner"["facing" = "west"]
    );
    register!(
        registrar,
        2837,
        "white_wall_banner"["facing" = "east"],
        "wall_banner"["facing" = "east"]
    );
    register!(registrar, 2848, "daylight_detector"["inverted" = "true", "power" = "0"], "daylight_detector_inverted"["power" = "0"]);
    register!(registrar, 2849, "daylight_detector"["inverted" = "true", "power" = "1"], "daylight_detector_inverted"["power" = "1"]);
    register!(registrar, 2850, "daylight_detector"["inverted" = "true", "power" = "2"], "daylight_detector_inverted"["power" = "2"]);
    register!(registrar, 2851, "daylight_detector"["inverted" = "true", "power" = "3"], "daylight_detector_inverted"["power" = "3"]);
    register!(registrar, 2852, "daylight_detector"["inverted" = "true", "power" = "4"], "daylight_detector_inverted"["power" = "4"]);
    register!(registrar, 2853, "daylight_detector"["inverted" = "true", "power" = "5"], "daylight_detector_inverted"["power" = "5"]);
    register!(registrar, 2854, "daylight_detector"["inverted" = "true", "power" = "6"], "daylight_detector_inverted"["power" = "6"]);
    register!(registrar, 2855, "daylight_detector"["inverted" = "true", "power" = "7"], "daylight_detector_inverted"["power" = "7"]);
    register!(registrar, 2856, "daylight_detector"["inverted" = "true", "power" = "8"], "daylight_detector_inverted"["power" = "8"]);
    register!(registrar, 2857, "daylight_detector"["inverted" = "true", "power" = "9"], "daylight_detector_inverted"["power" = "9"]);
    register!(registrar, 2858, "daylight_detector"["inverted" = "true", "power" = "10"], "daylight_detector_inverted"["power" = "10"]);
    register!(registrar, 2859, "daylight_detector"["inverted" = "true", "power" = "11"], "daylight_detector_inverted"["power" = "11"]);
    register!(registrar, 2860, "daylight_detector"["inverted" = "true", "power" = "12"], "daylight_detector_inverted"["power" = "12"]);
    register!(registrar, 2861, "daylight_detector"["inverted" = "true", "power" = "13"], "daylight_detector_inverted"["power" = "13"]);
    register!(registrar, 2862, "daylight_detector"["inverted" = "true", "power" = "14"], "daylight_detector_inverted"["power" = "14"]);
    register!(registrar, 2863, "daylight_detector"["inverted" = "true", "power" = "15"], "daylight_detector_inverted"["power" = "15"]);
    register!(
        registrar,
        2864,
        "red_sandstone",
        "red_sandstone"["type" = "red_sandstone"]
    );
    register!(
        registrar,
        2865,
        "chiseled_red_sandstone",
        "red_sandstone"["type" = "chiseled_red_sandstone"]
    );
    register!(
        registrar,
        2866,
        "cut_red_sandstone",
        "red_sandstone"["type" = "smooth_red_sandstone"]
    );
    register!(registrar, 2880, "red_sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "red_sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "red_sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "red_sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "red_sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "red_sandstone_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2881, "red_sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "red_sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "red_sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "red_sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "red_sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "red_sandstone_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2882, "red_sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "red_sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "red_sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "red_sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "red_sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "red_sandstone_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2883, "red_sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "red_sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "red_sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "red_sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "red_sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "red_sandstone_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 2884, "red_sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "red_sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "red_sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "red_sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "red_sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "red_sandstone_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2885, "red_sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "red_sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "red_sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "red_sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "red_sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "red_sandstone_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2886, "red_sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "red_sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "red_sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "red_sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "red_sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "red_sandstone_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2887, "red_sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "red_sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "red_sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "red_sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "red_sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "red_sandstone_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(registrar, 2896, "red_sandstone_slab"["type" = "double"], "double_stone_slab2"["seamless" = "false", "variant" = "red_sandstone"]);
    register!(registrar, 2904, "smooth_red_sandstone", "double_stone_slab2"["seamless" = "true", "variant" = "red_sandstone"]);
    register!(registrar, 2912, "red_sandstone_slab"["type" = "bottom"], "stone_slab2"["half" = "bottom", "variant" = "red_sandstone"]);
    register!(registrar, 2920, "red_sandstone_slab"["type" = "top"], "stone_slab2"["half" = "top", "variant" = "red_sandstone"]);
    register!(registrar, 2928, "spruce_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "spruce_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "spruce_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2929, "spruce_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "spruce_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "spruce_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2930, "spruce_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "spruce_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "spruce_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2931, "spruce_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "spruce_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "spruce_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2932, "spruce_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "spruce_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "spruce_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2933, "spruce_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "spruce_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "spruce_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2934, "spruce_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "spruce_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "spruce_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2935, "spruce_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "spruce_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "spruce_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2936, "spruce_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "spruce_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "spruce_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2937, "spruce_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "spruce_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "spruce_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2938, "spruce_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "spruce_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "spruce_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2939, "spruce_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "spruce_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "spruce_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2940, "spruce_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "spruce_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "spruce_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2941, "spruce_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "spruce_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "spruce_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2942, "spruce_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "spruce_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "spruce_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2943, "spruce_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "spruce_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "spruce_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2944, "birch_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "birch_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "birch_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2945, "birch_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "birch_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "birch_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2946, "birch_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "birch_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "birch_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2947, "birch_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "birch_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "birch_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2948, "birch_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "birch_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "birch_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2949, "birch_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "birch_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "birch_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2950, "birch_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "birch_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "birch_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2951, "birch_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "birch_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "birch_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2952, "birch_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "birch_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "birch_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2953, "birch_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "birch_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "birch_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2954, "birch_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "birch_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "birch_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2955, "birch_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "birch_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "birch_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2956, "birch_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "birch_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "birch_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2957, "birch_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "birch_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "birch_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2958, "birch_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "birch_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "birch_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2959, "birch_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "birch_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "birch_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2960, "jungle_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "jungle_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "jungle_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2961, "jungle_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "jungle_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "jungle_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2962, "jungle_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "jungle_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "jungle_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2963, "jungle_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "jungle_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "jungle_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2964, "jungle_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "jungle_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "jungle_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2965, "jungle_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "jungle_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "jungle_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2966, "jungle_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "jungle_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "jungle_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2967, "jungle_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "jungle_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "jungle_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2968, "jungle_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "jungle_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "jungle_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2969, "jungle_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "jungle_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "jungle_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2970, "jungle_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "jungle_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "jungle_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2971, "jungle_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "jungle_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "jungle_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2972, "jungle_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "jungle_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "jungle_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2973, "jungle_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "jungle_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "jungle_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2974, "jungle_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "jungle_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "jungle_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2975, "jungle_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "jungle_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "jungle_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2976, "dark_oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "dark_oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "dark_oak_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2977, "dark_oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "dark_oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "dark_oak_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2978, "dark_oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "dark_oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "dark_oak_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2979, "dark_oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "dark_oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "dark_oak_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2980, "dark_oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "dark_oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "dark_oak_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2981, "dark_oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "dark_oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "dark_oak_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2982, "dark_oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "dark_oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "dark_oak_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2983, "dark_oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "dark_oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "dark_oak_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2984, "dark_oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "dark_oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "dark_oak_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2985, "dark_oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "dark_oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "dark_oak_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2986, "dark_oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "dark_oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "dark_oak_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2987, "dark_oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "dark_oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "dark_oak_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 2988, "dark_oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "dark_oak_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "dark_oak_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2989, "dark_oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "dark_oak_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "dark_oak_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2990, "dark_oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "dark_oak_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "dark_oak_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2991, "dark_oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "dark_oak_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "dark_oak_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 2992, "acacia_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "acacia_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "false"], "acacia_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2993, "acacia_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "acacia_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "false"], "acacia_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2994, "acacia_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "acacia_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "false"], "acacia_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2995, "acacia_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "acacia_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "false"], "acacia_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "false"]);
    register!(registrar, 2996, "acacia_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "acacia_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "false"], "acacia_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2997, "acacia_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "acacia_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "false"], "acacia_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2998, "acacia_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "acacia_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "false"], "acacia_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 2999, "acacia_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "acacia_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "false"], "acacia_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "false"]);
    register!(registrar, 3000, "acacia_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "acacia_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "false", "powered" = "true"], "acacia_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 3001, "acacia_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "acacia_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "false", "powered" = "true"], "acacia_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 3002, "acacia_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "acacia_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "false", "powered" = "true"], "acacia_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 3003, "acacia_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "acacia_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "false", "powered" = "true"], "acacia_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "false", "powered" = "true"]);
    register!(registrar, 3004, "acacia_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "acacia_fence_gate"["facing" = "south", "in_wall" = "false", "open" = "true", "powered" = "true"], "acacia_fence_gate"["facing" = "south", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 3005, "acacia_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "acacia_fence_gate"["facing" = "west", "in_wall" = "false", "open" = "true", "powered" = "true"], "acacia_fence_gate"["facing" = "west", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 3006, "acacia_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "acacia_fence_gate"["facing" = "north", "in_wall" = "false", "open" = "true", "powered" = "true"], "acacia_fence_gate"["facing" = "north", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 3007, "acacia_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "acacia_fence_gate"["facing" = "east", "in_wall" = "false", "open" = "true", "powered" = "true"], "acacia_fence_gate"["facing" = "east", "in_wall" = "true", "open" = "true", "powered" = "true"]);
    register!(registrar, 3008, "spruce_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "spruce_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "spruce_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "spruce_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "spruce_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "spruce_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "spruce_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "spruce_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "spruce_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "spruce_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "spruce_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "spruce_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "spruce_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "spruce_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "spruce_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "spruce_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "spruce_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 3024, "birch_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "birch_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "birch_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "birch_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "birch_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "birch_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "birch_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "birch_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "birch_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "birch_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "birch_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "birch_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "birch_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "birch_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "birch_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "birch_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "birch_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 3040, "jungle_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "jungle_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "jungle_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "jungle_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "jungle_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "jungle_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "jungle_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "jungle_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "jungle_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "jungle_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "jungle_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "jungle_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "jungle_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "jungle_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "jungle_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "jungle_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "jungle_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 3056, "dark_oak_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "dark_oak_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "dark_oak_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "dark_oak_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "dark_oak_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "dark_oak_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "dark_oak_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "dark_oak_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "dark_oak_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "dark_oak_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "dark_oak_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "dark_oak_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "dark_oak_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "dark_oak_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "dark_oak_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "dark_oak_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "dark_oak_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 3072, "acacia_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "acacia_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "false"], "acacia_fence"["east" = "false", "north" = "false", "south" = "false", "west" = "true"], "acacia_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "false"], "acacia_fence"["east" = "false", "north" = "false", "south" = "true", "west" = "true"], "acacia_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "false"], "acacia_fence"["east" = "false", "north" = "true", "south" = "false", "west" = "true"], "acacia_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "false"], "acacia_fence"["east" = "false", "north" = "true", "south" = "true", "west" = "true"], "acacia_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "false"], "acacia_fence"["east" = "true", "north" = "false", "south" = "false", "west" = "true"], "acacia_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "false"], "acacia_fence"["east" = "true", "north" = "false", "south" = "true", "west" = "true"], "acacia_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "false"], "acacia_fence"["east" = "true", "north" = "true", "south" = "false", "west" = "true"], "acacia_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "false"], "acacia_fence"["east" = "true", "north" = "true", "south" = "true", "west" = "true"]);
    register!(registrar, 3088, "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3089, "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3090, "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3091, "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3092, "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3093, "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3094, "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3095, "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3096, "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 3097, "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "spruce_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"]);
    register!(registrar, 3098, "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"]);
    register!(registrar, 3099, "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "spruce_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "spruce_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3104, "birch_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3105, "birch_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3106, "birch_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3107, "birch_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3108, "birch_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "birch_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "birch_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3109, "birch_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "birch_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "birch_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3110, "birch_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "birch_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "birch_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3111, "birch_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "birch_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "birch_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3112, "birch_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "birch_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "birch_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "birch_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "birch_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 3113, "birch_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "birch_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "birch_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"]);
    register!(registrar, 3114, "birch_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "birch_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "birch_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "birch_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "birch_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"]);
    register!(registrar, 3115, "birch_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "birch_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "birch_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "birch_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "birch_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "birch_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "birch_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "birch_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "birch_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3120, "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3121, "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3122, "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3123, "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3124, "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3125, "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3126, "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3127, "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3128, "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 3129, "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "jungle_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"]);
    register!(registrar, 3130, "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"]);
    register!(registrar, 3131, "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "jungle_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "jungle_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3136, "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3137, "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3138, "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3139, "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3140, "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3141, "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3142, "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3143, "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3144, "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 3145, "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "acacia_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"]);
    register!(registrar, 3146, "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"]);
    register!(registrar, 3147, "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "acacia_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "acacia_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3152, "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3153, "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3154, "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3155, "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "false", "powered" = "true"]);
    register!(registrar, 3156, "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3157, "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3158, "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3159, "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "left", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "lower", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(registrar, 3160, "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "false"]);
    register!(registrar, 3161, "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "false"], "dark_oak_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "false"]);
    register!(registrar, 3162, "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "north", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "south", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "west", "half" = "upper", "hinge" = "left", "open" = "true", "powered" = "true"]);
    register!(registrar, 3163, "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "east", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "north", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "south", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"], "dark_oak_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "false", "powered" = "true"], "dark_oak_door"["facing" = "west", "half" = "upper", "hinge" = "right", "open" = "true", "powered" = "true"]);
    register!(
        registrar,
        3168,
        "end_rod"["facing" = "down"],
        "end_rod"["facing" = "down"]
    );
    register!(
        registrar,
        3169,
        "end_rod"["facing" = "up"],
        "end_rod"["facing" = "up"]
    );
    register!(
        registrar,
        3170,
        "end_rod"["facing" = "north"],
        "end_rod"["facing" = "north"]
    );
    register!(
        registrar,
        3171,
        "end_rod"["facing" = "south"],
        "end_rod"["facing" = "south"]
    );
    register!(
        registrar,
        3172,
        "end_rod"["facing" = "west"],
        "end_rod"["facing" = "west"]
    );
    register!(
        registrar,
        3173,
        "end_rod"["facing" = "east"],
        "end_rod"["facing" = "east"]
    );
    register!(registrar, 3184, "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "chorus_plant"["down" = "false", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "chorus_plant"["down" = "false", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "chorus_plant"["down" = "false", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "chorus_plant"["down" = "false", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "chorus_plant"["down" = "false", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "chorus_plant"["down" = "false", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "chorus_plant"["down" = "false", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "chorus_plant"["down" = "false", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "chorus_plant"["down" = "false", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "chorus_plant"["down" = "false", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "chorus_plant"["down" = "false", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "chorus_plant"["down" = "false", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "chorus_plant"["down" = "false", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "chorus_plant"["down" = "false", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "chorus_plant"["down" = "false", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "chorus_plant"["down" = "false", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "chorus_plant"["down" = "false", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "chorus_plant"["down" = "false", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "chorus_plant"["down" = "true", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "true", "east" = "false", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "chorus_plant"["down" = "true", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "chorus_plant"["down" = "true", "east" = "false", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "chorus_plant"["down" = "true", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "chorus_plant"["down" = "true", "east" = "false", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "chorus_plant"["down" = "true", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "chorus_plant"["down" = "true", "east" = "false", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "chorus_plant"["down" = "true", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "true", "east" = "false", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "chorus_plant"["down" = "true", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "chorus_plant"["down" = "true", "east" = "false", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "chorus_plant"["down" = "true", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "chorus_plant"["down" = "true", "east" = "false", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "chorus_plant"["down" = "true", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "chorus_plant"["down" = "true", "east" = "false", "north" = "true", "south" = "true", "up" = "true", "west" = "true"], "chorus_plant"["down" = "true", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "true", "east" = "true", "north" = "false", "south" = "false", "up" = "false", "west" = "true"], "chorus_plant"["down" = "true", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "false"], "chorus_plant"["down" = "true", "east" = "true", "north" = "false", "south" = "false", "up" = "true", "west" = "true"], "chorus_plant"["down" = "true", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "false"], "chorus_plant"["down" = "true", "east" = "true", "north" = "false", "south" = "true", "up" = "false", "west" = "true"], "chorus_plant"["down" = "true", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "false"], "chorus_plant"["down" = "true", "east" = "true", "north" = "false", "south" = "true", "up" = "true", "west" = "true"], "chorus_plant"["down" = "true", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "false"], "chorus_plant"["down" = "true", "east" = "true", "north" = "true", "south" = "false", "up" = "false", "west" = "true"], "chorus_plant"["down" = "true", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "false"], "chorus_plant"["down" = "true", "east" = "true", "north" = "true", "south" = "false", "up" = "true", "west" = "true"], "chorus_plant"["down" = "true", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "false"], "chorus_plant"["down" = "true", "east" = "true", "north" = "true", "south" = "true", "up" = "false", "west" = "true"], "chorus_plant"["down" = "true", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "false"], "chorus_plant"["down" = "true", "east" = "true", "north" = "true", "south" = "true", "up" = "true", "west" = "true"]);
    register!(
        registrar,
        3200,
        "chorus_flower"["age" = "0"],
        "chorus_flower"["age" = "0"]
    );
    register!(
        registrar,
        3201,
        "chorus_flower"["age" = "1"],
        "chorus_flower"["age" = "1"]
    );
    register!(
        registrar,
        3202,
        "chorus_flower"["age" = "2"],
        "chorus_flower"["age" = "2"]
    );
    register!(
        registrar,
        3203,
        "chorus_flower"["age" = "3"],
        "chorus_flower"["age" = "3"]
    );
    register!(
        registrar,
        3204,
        "chorus_flower"["age" = "4"],
        "chorus_flower"["age" = "4"]
    );
    register!(
        registrar,
        3205,
        "chorus_flower"["age" = "5"],
        "chorus_flower"["age" = "5"]
    );
    register!(registrar, 3216, "purpur_block", "purpur_block");
    register!(
        registrar,
        3232,
        "purpur_pillar"["axis" = "y"],
        "purpur_pillar"["axis" = "y"]
    );
    register!(
        registrar,
        3236,
        "purpur_pillar"["axis" = "x"],
        "purpur_pillar"["axis" = "x"]
    );
    register!(
        registrar,
        3240,
        "purpur_pillar"["axis" = "z"],
        "purpur_pillar"["axis" = "z"]
    );
    register!(registrar, 3248, "purpur_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"], "purpur_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_left"], "purpur_stairs"["facing" = "east", "half" = "bottom", "shape" = "inner_right"], "purpur_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_left"], "purpur_stairs"["facing" = "east", "half" = "bottom", "shape" = "outer_right"], "purpur_stairs"["facing" = "east", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 3249, "purpur_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"], "purpur_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_left"], "purpur_stairs"["facing" = "west", "half" = "bottom", "shape" = "inner_right"], "purpur_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_left"], "purpur_stairs"["facing" = "west", "half" = "bottom", "shape" = "outer_right"], "purpur_stairs"["facing" = "west", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 3250, "purpur_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"], "purpur_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_left"], "purpur_stairs"["facing" = "south", "half" = "bottom", "shape" = "inner_right"], "purpur_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_left"], "purpur_stairs"["facing" = "south", "half" = "bottom", "shape" = "outer_right"], "purpur_stairs"["facing" = "south", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 3251, "purpur_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"], "purpur_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_left"], "purpur_stairs"["facing" = "north", "half" = "bottom", "shape" = "inner_right"], "purpur_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_left"], "purpur_stairs"["facing" = "north", "half" = "bottom", "shape" = "outer_right"], "purpur_stairs"["facing" = "north", "half" = "bottom", "shape" = "straight"]);
    register!(registrar, 3252, "purpur_stairs"["facing" = "east", "half" = "top", "shape" = "straight"], "purpur_stairs"["facing" = "east", "half" = "top", "shape" = "inner_left"], "purpur_stairs"["facing" = "east", "half" = "top", "shape" = "inner_right"], "purpur_stairs"["facing" = "east", "half" = "top", "shape" = "outer_left"], "purpur_stairs"["facing" = "east", "half" = "top", "shape" = "outer_right"], "purpur_stairs"["facing" = "east", "half" = "top", "shape" = "straight"]);
    register!(registrar, 3253, "purpur_stairs"["facing" = "west", "half" = "top", "shape" = "straight"], "purpur_stairs"["facing" = "west", "half" = "top", "shape" = "inner_left"], "purpur_stairs"["facing" = "west", "half" = "top", "shape" = "inner_right"], "purpur_stairs"["facing" = "west", "half" = "top", "shape" = "outer_left"], "purpur_stairs"["facing" = "west", "half" = "top", "shape" = "outer_right"], "purpur_stairs"["facing" = "west", "half" = "top", "shape" = "straight"]);
    register!(registrar, 3254, "purpur_stairs"["facing" = "south", "half" = "top", "shape" = "straight"], "purpur_stairs"["facing" = "south", "half" = "top", "shape" = "inner_left"], "purpur_stairs"["facing" = "south", "half" = "top", "shape" = "inner_right"], "purpur_stairs"["facing" = "south", "half" = "top", "shape" = "outer_left"], "purpur_stairs"["facing" = "south", "half" = "top", "shape" = "outer_right"], "purpur_stairs"["facing" = "south", "half" = "top", "shape" = "straight"]);
    register!(registrar, 3255, "purpur_stairs"["facing" = "north", "half" = "top", "shape" = "straight"], "purpur_stairs"["facing" = "north", "half" = "top", "shape" = "inner_left"], "purpur_stairs"["facing" = "north", "half" = "top", "shape" = "inner_right"], "purpur_stairs"["facing" = "north", "half" = "top", "shape" = "outer_left"], "purpur_stairs"["facing" = "north", "half" = "top", "shape" = "outer_right"], "purpur_stairs"["facing" = "north", "half" = "top", "shape" = "straight"]);
    register!(
        registrar,
        3264,
        "purpur_slab"["type" = "double"],
        "purpur_double_slab"["variant" = "default"]
    );
    register!(registrar, 3280, "purpur_slab"["type" = "bottom"], "purpur_slab"["half" = "bottom", "variant" = "default"]);
    register!(registrar, 3288, "purpur_slab"["type" = "top"], "purpur_slab"["half" = "top", "variant" = "default"]);
    register!(registrar, 3296, "end_stone_bricks", "end_bricks");
    register!(
        registrar,
        3312,
        "beetroots"["age" = "0"],
        "beetroots"["age" = "0"]
    );
    register!(
        registrar,
        3313,
        "beetroots"["age" = "1"],
        "beetroots"["age" = "1"]
    );
    register!(
        registrar,
        3314,
        "beetroots"["age" = "2"],
        "beetroots"["age" = "2"]
    );
    register!(
        registrar,
        3315,
        "beetroots"["age" = "3"],
        "beetroots"["age" = "3"]
    );
    register!(registrar, 3328, "grass_path", "grass_path");
    register!(registrar, 3344, "end_gateway", "end_gateway");
    register!(registrar, 3360, "repeating_command_block"["conditional" = "false", "facing" = "down"], "repeating_command_block"["conditional" = "false", "facing" = "down"]);
    register!(registrar, 3361, "repeating_command_block"["conditional" = "false", "facing" = "up"], "repeating_command_block"["conditional" = "false", "facing" = "up"]);
    register!(registrar, 3362, "repeating_command_block"["conditional" = "false", "facing" = "north"], "repeating_command_block"["conditional" = "false", "facing" = "north"]);
    register!(registrar, 3363, "repeating_command_block"["conditional" = "false", "facing" = "south"], "repeating_command_block"["conditional" = "false", "facing" = "south"]);
    register!(registrar, 3364, "repeating_command_block"["conditional" = "false", "facing" = "west"], "repeating_command_block"["conditional" = "false", "facing" = "west"]);
    register!(registrar, 3365, "repeating_command_block"["conditional" = "false", "facing" = "east"], "repeating_command_block"["conditional" = "false", "facing" = "east"]);
    register!(registrar, 3368, "repeating_command_block"["conditional" = "true", "facing" = "down"], "repeating_command_block"["conditional" = "true", "facing" = "down"]);
    register!(registrar, 3369, "repeating_command_block"["conditional" = "true", "facing" = "up"], "repeating_command_block"["conditional" = "true", "facing" = "up"]);
    register!(registrar, 3370, "repeating_command_block"["conditional" = "true", "facing" = "north"], "repeating_command_block"["conditional" = "true", "facing" = "north"]);
    register!(registrar, 3371, "repeating_command_block"["conditional" = "true", "facing" = "south"], "repeating_command_block"["conditional" = "true", "facing" = "south"]);
    register!(registrar, 3372, "repeating_command_block"["conditional" = "true", "facing" = "west"], "repeating_command_block"["conditional" = "true", "facing" = "west"]);
    register!(registrar, 3373, "repeating_command_block"["conditional" = "true", "facing" = "east"], "repeating_command_block"["conditional" = "true", "facing" = "east"]);
    register!(registrar, 3376, "chain_command_block"["conditional" = "false", "facing" = "down"], "chain_command_block"["conditional" = "false", "facing" = "down"]);
    register!(registrar, 3377, "chain_command_block"["conditional" = "false", "facing" = "up"], "chain_command_block"["conditional" = "false", "facing" = "up"]);
    register!(registrar, 3378, "chain_command_block"["conditional" = "false", "facing" = "north"], "chain_command_block"["conditional" = "false", "facing" = "north"]);
    register!(registrar, 3379, "chain_command_block"["conditional" = "false", "facing" = "south"], "chain_command_block"["conditional" = "false", "facing" = "south"]);
    register!(registrar, 3380, "chain_command_block"["conditional" = "false", "facing" = "west"], "chain_command_block"["conditional" = "false", "facing" = "west"]);
    register!(registrar, 3381, "chain_command_block"["conditional" = "false", "facing" = "east"], "chain_command_block"["conditional" = "false", "facing" = "east"]);
    register!(registrar, 3384, "chain_command_block"["conditional" = "true", "facing" = "down"], "chain_command_block"["conditional" = "true", "facing" = "down"]);
    register!(registrar, 3385, "chain_command_block"["conditional" = "true", "facing" = "up"], "chain_command_block"["conditional" = "true", "facing" = "up"]);
    register!(registrar, 3386, "chain_command_block"["conditional" = "true", "facing" = "north"], "chain_command_block"["conditional" = "true", "facing" = "north"]);
    register!(registrar, 3387, "chain_command_block"["conditional" = "true", "facing" = "south"], "chain_command_block"["conditional" = "true", "facing" = "south"]);
    register!(registrar, 3388, "chain_command_block"["conditional" = "true", "facing" = "west"], "chain_command_block"["conditional" = "true", "facing" = "west"]);
    register!(registrar, 3389, "chain_command_block"["conditional" = "true", "facing" = "east"], "chain_command_block"["conditional" = "true", "facing" = "east"]);
    register!(
        registrar,
        3392,
        "frosted_ice"["age" = "0"],
        "frosted_ice"["age" = "0"]
    );
    register!(
        registrar,
        3393,
        "frosted_ice"["age" = "1"],
        "frosted_ice"["age" = "1"]
    );
    register!(
        registrar,
        3394,
        "frosted_ice"["age" = "2"],
        "frosted_ice"["age" = "2"]
    );
    register!(
        registrar,
        3395,
        "frosted_ice"["age" = "3"],
        "frosted_ice"["age" = "3"]
    );
    register!(registrar, 3408, "magma_block", "magma");
    register!(registrar, 3424, "nether_wart_block", "nether_wart_block");
    register!(registrar, 3440, "red_nether_bricks", "red_nether_brick");
    register!(
        registrar,
        3456,
        "bone_block"["axis" = "y"],
        "bone_block"["axis" = "y"]
    );
    register!(
        registrar,
        3460,
        "bone_block"["axis" = "x"],
        "bone_block"["axis" = "x"]
    );
    register!(
        registrar,
        3464,
        "bone_block"["axis" = "z"],
        "bone_block"["axis" = "z"]
    );
    register!(registrar, 3472, "structure_void", "structure_void");
    register!(registrar, 3488, "observer"["facing" = "down", "powered" = "false"], "observer"["facing" = "down", "powered" = "false"]);
    register!(registrar, 3489, "observer"["facing" = "up", "powered" = "false"], "observer"["facing" = "up", "powered" = "false"]);
    register!(registrar, 3490, "observer"["facing" = "north", "powered" = "false"], "observer"["facing" = "north", "powered" = "false"]);
    register!(registrar, 3491, "observer"["facing" = "south", "powered" = "false"], "observer"["facing" = "south", "powered" = "false"]);
    register!(registrar, 3492, "observer"["facing" = "west", "powered" = "false"], "observer"["facing" = "west", "powered" = "false"]);
    register!(registrar, 3493, "observer"["facing" = "east", "powered" = "false"], "observer"["facing" = "east", "powered" = "false"]);
    register!(registrar, 3496, "observer"["facing" = "down", "powered" = "true"], "observer"["facing" = "down", "powered" = "true"]);
    register!(registrar, 3497, "observer"["facing" = "up", "powered" = "true"], "observer"["facing" = "up", "powered" = "true"]);
    register!(registrar, 3498, "observer"["facing" = "north", "powered" = "true"], "observer"["facing" = "north", "powered" = "true"]);
    register!(registrar, 3499, "observer"["facing" = "south", "powered" = "true"], "observer"["facing" = "south", "powered" = "true"]);
    register!(registrar, 3500, "observer"["facing" = "west", "powered" = "true"], "observer"["facing" = "west", "powered" = "true"]);
    register!(registrar, 3501, "observer"["facing" = "east", "powered" = "true"], "observer"["facing" = "east", "powered" = "true"]);
    register!(
        registrar,
        3504,
        "white_shulker_box"["facing" = "down"],
        "white_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3505,
        "white_shulker_box"["facing" = "up"],
        "white_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3506,
        "white_shulker_box"["facing" = "north"],
        "white_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3507,
        "white_shulker_box"["facing" = "south"],
        "white_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3508,
        "white_shulker_box"["facing" = "west"],
        "white_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3509,
        "white_shulker_box"["facing" = "east"],
        "white_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3520,
        "orange_shulker_box"["facing" = "down"],
        "orange_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3521,
        "orange_shulker_box"["facing" = "up"],
        "orange_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3522,
        "orange_shulker_box"["facing" = "north"],
        "orange_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3523,
        "orange_shulker_box"["facing" = "south"],
        "orange_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3524,
        "orange_shulker_box"["facing" = "west"],
        "orange_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3525,
        "orange_shulker_box"["facing" = "east"],
        "orange_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3536,
        "magenta_shulker_box"["facing" = "down"],
        "magenta_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3537,
        "magenta_shulker_box"["facing" = "up"],
        "magenta_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3538,
        "magenta_shulker_box"["facing" = "north"],
        "magenta_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3539,
        "magenta_shulker_box"["facing" = "south"],
        "magenta_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3540,
        "magenta_shulker_box"["facing" = "west"],
        "magenta_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3541,
        "magenta_shulker_box"["facing" = "east"],
        "magenta_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3552,
        "light_blue_shulker_box"["facing" = "down"],
        "light_blue_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3553,
        "light_blue_shulker_box"["facing" = "up"],
        "light_blue_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3554,
        "light_blue_shulker_box"["facing" = "north"],
        "light_blue_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3555,
        "light_blue_shulker_box"["facing" = "south"],
        "light_blue_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3556,
        "light_blue_shulker_box"["facing" = "west"],
        "light_blue_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3557,
        "light_blue_shulker_box"["facing" = "east"],
        "light_blue_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3568,
        "yellow_shulker_box"["facing" = "down"],
        "yellow_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3569,
        "yellow_shulker_box"["facing" = "up"],
        "yellow_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3570,
        "yellow_shulker_box"["facing" = "north"],
        "yellow_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3571,
        "yellow_shulker_box"["facing" = "south"],
        "yellow_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3572,
        "yellow_shulker_box"["facing" = "west"],
        "yellow_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3573,
        "yellow_shulker_box"["facing" = "east"],
        "yellow_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3584,
        "lime_shulker_box"["facing" = "down"],
        "lime_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3585,
        "lime_shulker_box"["facing" = "up"],
        "lime_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3586,
        "lime_shulker_box"["facing" = "north"],
        "lime_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3587,
        "lime_shulker_box"["facing" = "south"],
        "lime_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3588,
        "lime_shulker_box"["facing" = "west"],
        "lime_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3589,
        "lime_shulker_box"["facing" = "east"],
        "lime_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3600,
        "pink_shulker_box"["facing" = "down"],
        "pink_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3601,
        "pink_shulker_box"["facing" = "up"],
        "pink_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3602,
        "pink_shulker_box"["facing" = "north"],
        "pink_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3603,
        "pink_shulker_box"["facing" = "south"],
        "pink_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3604,
        "pink_shulker_box"["facing" = "west"],
        "pink_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3605,
        "pink_shulker_box"["facing" = "east"],
        "pink_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3616,
        "gray_shulker_box"["facing" = "down"],
        "gray_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3617,
        "gray_shulker_box"["facing" = "up"],
        "gray_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3618,
        "gray_shulker_box"["facing" = "north"],
        "gray_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3619,
        "gray_shulker_box"["facing" = "south"],
        "gray_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3620,
        "gray_shulker_box"["facing" = "west"],
        "gray_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3621,
        "gray_shulker_box"["facing" = "east"],
        "gray_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3632,
        "light_gray_shulker_box"["facing" = "down"],
        "silver_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3633,
        "light_gray_shulker_box"["facing" = "up"],
        "silver_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3634,
        "light_gray_shulker_box"["facing" = "north"],
        "silver_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3635,
        "light_gray_shulker_box"["facing" = "south"],
        "silver_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3636,
        "light_gray_shulker_box"["facing" = "west"],
        "silver_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3637,
        "light_gray_shulker_box"["facing" = "east"],
        "silver_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3648,
        "cyan_shulker_box"["facing" = "down"],
        "cyan_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3649,
        "cyan_shulker_box"["facing" = "up"],
        "cyan_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3650,
        "cyan_shulker_box"["facing" = "north"],
        "cyan_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3651,
        "cyan_shulker_box"["facing" = "south"],
        "cyan_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3652,
        "cyan_shulker_box"["facing" = "west"],
        "cyan_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3653,
        "cyan_shulker_box"["facing" = "east"],
        "cyan_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3664,
        "purple_shulker_box"["facing" = "down"],
        "purple_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3665,
        "purple_shulker_box"["facing" = "up"],
        "purple_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3666,
        "purple_shulker_box"["facing" = "north"],
        "purple_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3667,
        "purple_shulker_box"["facing" = "south"],
        "purple_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3668,
        "purple_shulker_box"["facing" = "west"],
        "purple_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3669,
        "purple_shulker_box"["facing" = "east"],
        "purple_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3680,
        "blue_shulker_box"["facing" = "down"],
        "blue_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3681,
        "blue_shulker_box"["facing" = "up"],
        "blue_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3682,
        "blue_shulker_box"["facing" = "north"],
        "blue_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3683,
        "blue_shulker_box"["facing" = "south"],
        "blue_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3684,
        "blue_shulker_box"["facing" = "west"],
        "blue_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3685,
        "blue_shulker_box"["facing" = "east"],
        "blue_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3696,
        "brown_shulker_box"["facing" = "down"],
        "brown_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3697,
        "brown_shulker_box"["facing" = "up"],
        "brown_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3698,
        "brown_shulker_box"["facing" = "north"],
        "brown_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3699,
        "brown_shulker_box"["facing" = "south"],
        "brown_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3700,
        "brown_shulker_box"["facing" = "west"],
        "brown_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3701,
        "brown_shulker_box"["facing" = "east"],
        "brown_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3712,
        "green_shulker_box"["facing" = "down"],
        "green_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3713,
        "green_shulker_box"["facing" = "up"],
        "green_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3714,
        "green_shulker_box"["facing" = "north"],
        "green_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3715,
        "green_shulker_box"["facing" = "south"],
        "green_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3716,
        "green_shulker_box"["facing" = "west"],
        "green_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3717,
        "green_shulker_box"["facing" = "east"],
        "green_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3728,
        "red_shulker_box"["facing" = "down"],
        "red_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3729,
        "red_shulker_box"["facing" = "up"],
        "red_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3730,
        "red_shulker_box"["facing" = "north"],
        "red_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3731,
        "red_shulker_box"["facing" = "south"],
        "red_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3732,
        "red_shulker_box"["facing" = "west"],
        "red_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3733,
        "red_shulker_box"["facing" = "east"],
        "red_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3744,
        "black_shulker_box"["facing" = "down"],
        "black_shulker_box"["facing" = "down"]
    );
    register!(
        registrar,
        3745,
        "black_shulker_box"["facing" = "up"],
        "black_shulker_box"["facing" = "up"]
    );
    register!(
        registrar,
        3746,
        "black_shulker_box"["facing" = "north"],
        "black_shulker_box"["facing" = "north"]
    );
    register!(
        registrar,
        3747,
        "black_shulker_box"["facing" = "south"],
        "black_shulker_box"["facing" = "south"]
    );
    register!(
        registrar,
        3748,
        "black_shulker_box"["facing" = "west"],
        "black_shulker_box"["facing" = "west"]
    );
    register!(
        registrar,
        3749,
        "black_shulker_box"["facing" = "east"],
        "black_shulker_box"["facing" = "east"]
    );
    register!(
        registrar,
        3760,
        "white_glazed_terracotta"["facing" = "south"],
        "white_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3761,
        "white_glazed_terracotta"["facing" = "west"],
        "white_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3762,
        "white_glazed_terracotta"["facing" = "north"],
        "white_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3763,
        "white_glazed_terracotta"["facing" = "east"],
        "white_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3776,
        "orange_glazed_terracotta"["facing" = "south"],
        "orange_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3777,
        "orange_glazed_terracotta"["facing" = "west"],
        "orange_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3778,
        "orange_glazed_terracotta"["facing" = "north"],
        "orange_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3779,
        "orange_glazed_terracotta"["facing" = "east"],
        "orange_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3792,
        "magenta_glazed_terracotta"["facing" = "south"],
        "magenta_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3793,
        "magenta_glazed_terracotta"["facing" = "west"],
        "magenta_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3794,
        "magenta_glazed_terracotta"["facing" = "north"],
        "magenta_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3795,
        "magenta_glazed_terracotta"["facing" = "east"],
        "magenta_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3808,
        "light_blue_glazed_terracotta"["facing" = "south"],
        "light_blue_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3809,
        "light_blue_glazed_terracotta"["facing" = "west"],
        "light_blue_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3810,
        "light_blue_glazed_terracotta"["facing" = "north"],
        "light_blue_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3811,
        "light_blue_glazed_terracotta"["facing" = "east"],
        "light_blue_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3824,
        "yellow_glazed_terracotta"["facing" = "south"],
        "yellow_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3825,
        "yellow_glazed_terracotta"["facing" = "west"],
        "yellow_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3826,
        "yellow_glazed_terracotta"["facing" = "north"],
        "yellow_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3827,
        "yellow_glazed_terracotta"["facing" = "east"],
        "yellow_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3840,
        "lime_glazed_terracotta"["facing" = "south"],
        "lime_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3841,
        "lime_glazed_terracotta"["facing" = "west"],
        "lime_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3842,
        "lime_glazed_terracotta"["facing" = "north"],
        "lime_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3843,
        "lime_glazed_terracotta"["facing" = "east"],
        "lime_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3856,
        "pink_glazed_terracotta"["facing" = "south"],
        "pink_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3857,
        "pink_glazed_terracotta"["facing" = "west"],
        "pink_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3858,
        "pink_glazed_terracotta"["facing" = "north"],
        "pink_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3859,
        "pink_glazed_terracotta"["facing" = "east"],
        "pink_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3872,
        "gray_glazed_terracotta"["facing" = "south"],
        "gray_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3873,
        "gray_glazed_terracotta"["facing" = "west"],
        "gray_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3874,
        "gray_glazed_terracotta"["facing" = "north"],
        "gray_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3875,
        "gray_glazed_terracotta"["facing" = "east"],
        "gray_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3888,
        "light_gray_glazed_terracotta"["facing" = "south"],
        "silver_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3889,
        "light_gray_glazed_terracotta"["facing" = "west"],
        "silver_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3890,
        "light_gray_glazed_terracotta"["facing" = "north"],
        "silver_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3891,
        "light_gray_glazed_terracotta"["facing" = "east"],
        "silver_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3904,
        "cyan_glazed_terracotta"["facing" = "south"],
        "cyan_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3905,
        "cyan_glazed_terracotta"["facing" = "west"],
        "cyan_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3906,
        "cyan_glazed_terracotta"["facing" = "north"],
        "cyan_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3907,
        "cyan_glazed_terracotta"["facing" = "east"],
        "cyan_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3920,
        "purple_glazed_terracotta"["facing" = "south"],
        "purple_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3921,
        "purple_glazed_terracotta"["facing" = "west"],
        "purple_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3922,
        "purple_glazed_terracotta"["facing" = "north"],
        "purple_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3923,
        "purple_glazed_terracotta"["facing" = "east"],
        "purple_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3936,
        "blue_glazed_terracotta"["facing" = "south"],
        "blue_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3937,
        "blue_glazed_terracotta"["facing" = "west"],
        "blue_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3938,
        "blue_glazed_terracotta"["facing" = "north"],
        "blue_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3939,
        "blue_glazed_terracotta"["facing" = "east"],
        "blue_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3952,
        "brown_glazed_terracotta"["facing" = "south"],
        "brown_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3953,
        "brown_glazed_terracotta"["facing" = "west"],
        "brown_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3954,
        "brown_glazed_terracotta"["facing" = "north"],
        "brown_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3955,
        "brown_glazed_terracotta"["facing" = "east"],
        "brown_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3968,
        "green_glazed_terracotta"["facing" = "south"],
        "green_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3969,
        "green_glazed_terracotta"["facing" = "west"],
        "green_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3970,
        "green_glazed_terracotta"["facing" = "north"],
        "green_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3971,
        "green_glazed_terracotta"["facing" = "east"],
        "green_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        3984,
        "red_glazed_terracotta"["facing" = "south"],
        "red_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        3985,
        "red_glazed_terracotta"["facing" = "west"],
        "red_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        3986,
        "red_glazed_terracotta"["facing" = "north"],
        "red_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        3987,
        "red_glazed_terracotta"["facing" = "east"],
        "red_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        4000,
        "black_glazed_terracotta"["facing" = "south"],
        "black_glazed_terracotta"["facing" = "south"]
    );
    register!(
        registrar,
        4001,
        "black_glazed_terracotta"["facing" = "west"],
        "black_glazed_terracotta"["facing" = "west"]
    );
    register!(
        registrar,
        4002,
        "black_glazed_terracotta"["facing" = "north"],
        "black_glazed_terracotta"["facing" = "north"]
    );
    register!(
        registrar,
        4003,
        "black_glazed_terracotta"["facing" = "east"],
        "black_glazed_terracotta"["facing" = "east"]
    );
    register!(
        registrar,
        4016,
        "white_concrete",
        "concrete"["color" = "white"]
    );
    register!(
        registrar,
        4017,
        "orange_concrete",
        "concrete"["color" = "orange"]
    );
    register!(
        registrar,
        4018,
        "magenta_concrete",
        "concrete"["color" = "magenta"]
    );
    register!(
        registrar,
        4019,
        "light_blue_concrete",
        "concrete"["color" = "light_blue"]
    );
    register!(
        registrar,
        4020,
        "yellow_concrete",
        "concrete"["color" = "yellow"]
    );
    register!(
        registrar,
        4021,
        "lime_concrete",
        "concrete"["color" = "lime"]
    );
    register!(
        registrar,
        4022,
        "pink_concrete",
        "concrete"["color" = "pink"]
    );
    register!(
        registrar,
        4023,
        "gray_concrete",
        "concrete"["color" = "gray"]
    );
    register!(
        registrar,
        4024,
        "light_gray_concrete",
        "concrete"["color" = "silver"]
    );
    register!(
        registrar,
        4025,
        "cyan_concrete",
        "concrete"["color" = "cyan"]
    );
    register!(
        registrar,
        4026,
        "purple_concrete",
        "concrete"["color" = "purple"]
    );
    register!(
        registrar,
        4027,
        "blue_concrete",
        "concrete"["color" = "blue"]
    );
    register!(
        registrar,
        4028,
        "brown_concrete",
        "concrete"["color" = "brown"]
    );
    register!(
        registrar,
        4029,
        "green_concrete",
        "concrete"["color" = "green"]
    );
    register!(registrar, 4030, "red_concrete", "concrete"["color" = "red"]);
    register!(
        registrar,
        4031,
        "black_concrete",
        "concrete"["color" = "black"]
    );
    register!(
        registrar,
        4032,
        "white_concrete_powder",
        "concrete_powder"["color" = "white"]
    );
    register!(
        registrar,
        4033,
        "orange_concrete_powder",
        "concrete_powder"["color" = "orange"]
    );
    register!(
        registrar,
        4034,
        "magenta_concrete_powder",
        "concrete_powder"["color" = "magenta"]
    );
    register!(
        registrar,
        4035,
        "light_blue_concrete_powder",
        "concrete_powder"["color" = "light_blue"]
    );
    register!(
        registrar,
        4036,
        "yellow_concrete_powder",
        "concrete_powder"["color" = "yellow"]
    );
    register!(
        registrar,
        4037,
        "lime_concrete_powder",
        "concrete_powder"["color" = "lime"]
    );
    register!(
        registrar,
        4038,
        "pink_concrete_powder",
        "concrete_powder"["color" = "pink"]
    );
    register!(
        registrar,
        4039,
        "gray_concrete_powder",
        "concrete_powder"["color" = "gray"]
    );
    register!(
        registrar,
        4040,
        "light_gray_concrete_powder",
        "concrete_powder"["color" = "silver"]
    );
    register!(
        registrar,
        4041,
        "cyan_concrete_powder",
        "concrete_powder"["color" = "cyan"]
    );
    register!(
        registrar,
        4042,
        "purple_concrete_powder",
        "concrete_powder"["color" = "purple"]
    );
    register!(
        registrar,
        4043,
        "blue_concrete_powder",
        "concrete_powder"["color" = "blue"]
    );
    register!(
        registrar,
        4044,
        "brown_concrete_powder",
        "concrete_powder"["color" = "brown"]
    );
    register!(
        registrar,
        4045,
        "green_concrete_powder",
        "concrete_powder"["color" = "green"]
    );
    register!(
        registrar,
        4046,
        "red_concrete_powder",
        "concrete_powder"["color" = "red"]
    );
    register!(
        registrar,
        4047,
        "black_concrete_powder",
        "concrete_powder"["color" = "black"]
    );
    register!(
        registrar,
        4080,
        "structure_block"["mode" = "save"],
        "structure_block"["mode" = "save"]
    );
    register!(
        registrar,
        4081,
        "structure_block"["mode" = "load"],
        "structure_block"["mode" = "load"]
    );
    register!(
        registrar,
        4082,
        "structure_block"["mode" = "corner"],
        "structure_block"["mode" = "corner"]
    );
    register!(
        registrar,
        4083,
        "structure_block"["mode" = "data"],
        "structure_block"["mode" = "data"]
    );
}
