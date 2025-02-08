use crate::{static_string_mc_set, types};
use java_string::{JavaStr, JavaString};
use world_transmuter_engine::{map_data_converter_func, JCompound, JList, JValue, JValueRef};

const VERSION: u32 = 2701;

static_string_mc_set! {
    piece_type = {
        "jigsaw",
        "nvi",
        "pcp",
        "bastionremnant",
        "runtime",
    }
}

static_string_mc_set! {
    features = {
        "tree",
        "flower",
        "block_pile",
        "random_patch",
    }
}

struct Getter<T> {
    value: T,
}

impl Getter<&str> {
    fn get<'a>(&self, obj: JValueRef<'a>) -> Option<JValueRef<'a>> {
        match obj {
            JValueRef::Compound(compound) => compound.get(self.value).map(|v| v.as_value_ref()),
            _ => None,
        }
    }
}

impl Getter<i32> {
    fn get<'a>(&self, obj: JValueRef<'a>) -> Option<JValueRef<'a>> {
        match obj {
            JValueRef::List(list) => list.get(self.value as usize),
            _ => None,
        }
    }
}

macro_rules! get_nested_string {
    ($root:expr, $path:expr $(, $paths:tt)*) => {
        {
            let result = $root.get($path).map(|v| v.as_value_ref())
            $(
                .and_then(|v| Getter{value: $paths}.get(v))
            )*;
            match result {
                Some(JValueRef::String(str)) => &str[..],
                _ => JavaStr::from_str(""),
            }
        }
    }
}

pub(crate) fn register() {
    types::structure_feature_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(JValue::List(JList::Compound(children))) = data.get_mut("Children") {
            for child in children {
                if !matches!(child.get("id"), Some(JValue::String(id)) if piece_type().contains(id)) {
                    continue;
                }
                if !matches!(child.get("pool_element"), Some(JValue::String(str)) if str == "minecraft:pool_element") {
                    continue;
                }
                if let Some(JValue::Compound(feature)) = child.get("feature") {
                    if let Some(replacement) = convert_to_string(feature) {
                        child.insert("feature", replacement);
                    }
                }
            }
        }
    }));
}

fn convert_to_string(feature: &JCompound) -> Option<JavaString> {
    get_replacement(
        get_nested_string!(feature, "type"),
        get_nested_string!(feature, "name"),
        get_nested_string!(feature, "config", "state_provider", "type"),
        get_nested_string!(feature, "config", "state_provider", "state", "Name"),
        get_nested_string!(
            feature,
            "config",
            "state_provider",
            "entries",
            0,
            "data",
            "Name"
        ),
        get_nested_string!(feature, "config", "foliage_placer", "type"),
        get_nested_string!(feature, "config", "leaves_provider", "state", "Name"),
    )
}

fn get_replacement(
    typ: &JavaStr,
    name: &JavaStr,
    state_type: &JavaStr,
    state_name: &JavaStr,
    first_entry_name: &JavaStr,
    foliage_name: &JavaStr,
    leaves_name: &JavaStr,
) -> Option<JavaString> {
    let actual_type = if !typ.is_empty() {
        typ
    } else {
        match name.as_bytes() {
            b"" => return None,
            b"minecraft:normal_tree" => JavaStr::from_str("minecraft:tree"),
            _ => name,
        }
    };

    if !features().contains(actual_type) {
        return None;
    }

    struct S<'a> {
        actual_type: &'a [u8],
        state_type: &'a [u8],
        state_name: &'a [u8],
        first_entry_name: &'a [u8],
        foliage_name: &'a [u8],
        leaves_name: &'a [u8],
    }

    let result = match (S {
        actual_type: actual_type.as_bytes(),
        state_type: state_type.as_bytes(),
        state_name: state_name.as_bytes(),
        first_entry_name: first_entry_name.as_bytes(),
        foliage_name: foliage_name.as_bytes(),
        leaves_name: leaves_name.as_bytes(),
    }) {
        S {
            actual_type: b"minecraft:random_patch",
            state_type: b"minecraft:simple_state_provider",
            state_name: b"minecraft:sweet_berry_bush",
            ..
        } => "minecraft:patch_berry_bush",
        S {
            actual_type: b"minecraft:random_patch",
            state_type: b"minecraft:simple_state_provider",
            state_name: b"minecraft:cactus",
            ..
        } => "minecraft:patch_cactus",
        S {
            actual_type: b"minecraft:random_patch",
            state_type: b"minecraft:weighted_state_provider",
            first_entry_name: b"minecraft:grass",
            ..
        }
        | S {
            actual_type: b"minecraft:random_patch",
            state_type: b"minecraft:weighted_state_provider",
            first_entry_name: b"minecraft:fern",
            ..
        } => "minecraft:patch_taiga_grass",
        S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:weighted_state_provider",
            first_entry_name: b"minecraft:packed_ice",
            ..
        }
        | S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:weighted_state_provider",
            first_entry_name: b"minecraft:blue_ice",
            ..
        } => "minecraft:pile_ice",
        S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:weighted_state_provider",
            first_entry_name: b"minecraft:jack_o_lantern",
            ..
        }
        | S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:weighted_state_provider",
            first_entry_name: b"minecraft:pumpkin",
            ..
        } => "minecraft:pile_pumpkin",
        S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:simple_state_provider",
            state_name: b"minecraft:hay_block",
            ..
        }
        | S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:rotated_block_provider",
            state_name: b"minecraft:hay_block",
            ..
        } => "minecraft:pile_hay",
        S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:simple_state_provider",
            state_name: b"minecraft:melon",
            ..
        }
        | S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:rotated_block_provider",
            state_name: b"minecraft:melon",
            ..
        } => "minecraft:pile_melon",
        S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:simple_state_provider",
            state_name: b"minecraft:snow",
            ..
        }
        | S {
            actual_type: b"minecraft:block_pile",
            state_type: b"minecraft:rotated_block_provider",
            state_name: b"minecraft:snow",
            ..
        } => "minecraft:pile_snow",
        S {
            actual_type: b"minecraft:flower",
            ..
        } => "minecraft:flower_plain",
        S {
            actual_type: b"minecraft:tree",
            foliage_name: b"minecraft:acacia_foliage_placer",
            ..
        } => "minecraft:acacia",
        S {
            actual_type: b"minecraft:tree",
            foliage_name: b"minecraft:blob_foliage_placer",
            leaves_name: b"minecraft:oak_leaves",
            ..
        } => "minecraft:oak",
        S {
            actual_type: b"minecraft:tree",
            foliage_name: b"minecraft:pine_foliage_placer",
            ..
        } => "minecraft:pine",
        S {
            actual_type: b"minecraft:tree",
            foliage_name: b"minecraft:spruce_foliage_placer",
            ..
        } => "minecraft:spruce",
        _ => return None,
    };

    Some(JavaString::from(result))
}
