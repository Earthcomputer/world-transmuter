use crate::helpers::mc_namespace_map::McNamespaceSet;
use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use std::sync::OnceLock;
use valence_nbt::value::ValueRef;
use valence_nbt::{Compound, List, Value};

const VERSION: u32 = 2701;

static PIECE_TYPE: OnceLock<McNamespaceSet> = OnceLock::new();

fn piece_type() -> &'static McNamespaceSet<'static> {
    PIECE_TYPE.get_or_init(|| {
        let mut set = McNamespaceSet::new();
        set.insert_mc("jigsaw");
        set.insert_mc("nvi");
        set.insert_mc("pcp");
        set.insert_mc("bastionremnant");
        set.insert_mc("runtime");
        set
    })
}

static FEATURES: OnceLock<McNamespaceSet> = OnceLock::new();

fn features() -> &'static McNamespaceSet<'static> {
    FEATURES.get_or_init(|| {
        let mut set = McNamespaceSet::new();
        set.insert_mc("tree");
        set.insert_mc("flower");
        set.insert_mc("block_pile");
        set.insert_mc("random_patch");
        set
    })
}

struct Getter<T> {
    value: T,
}

impl Getter<&str> {
    fn get<'a>(&self, obj: ValueRef<'a>) -> Option<ValueRef<'a>> {
        match obj {
            ValueRef::Compound(compound) => compound.get(self.value).map(|v| v.as_value_ref()),
            _ => None,
        }
    }
}

impl Getter<i32> {
    fn get<'a>(&self, obj: ValueRef<'a>) -> Option<ValueRef<'a>> {
        match obj {
            ValueRef::List(list) => list.get(self.value as usize),
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
                Some(ValueRef::String(str)) => &str[..],
                _ => "",
            }
        }
    }
}

pub(crate) fn register(types: MinecraftTypesMut) {
    types.structure_feature().borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(Value::List(List::Compound(children))) = data.get_mut("Children") {
            for child in children {
                if !matches!(child.get("id"), Some(Value::String(id)) if piece_type().contains(id)) {
                    continue;
                }
                if !matches!(child.get("pool_element"), Some(Value::String(str)) if str == "minecraft:pool_element") {
                    continue;
                }
                if let Some(Value::Compound(feature)) = child.get("feature") {
                    if let Some(replacement) = convert_to_string(feature) {
                        child.insert("feature", replacement);
                    }
                }
            }
        }
    }));
}

fn convert_to_string(feature: &Compound) -> Option<String> {
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
    typ: &str,
    name: &str,
    state_type: &str,
    state_name: &str,
    first_entry_name: &str,
    foliage_name: &str,
    leaves_name: &str,
) -> Option<String> {
    let actual_type = if !typ.is_empty() {
        typ
    } else {
        match name {
            "" => return None,
            "minecraft:normal_tree" => "minecraft:tree",
            _ => name,
        }
    };

    if !features().contains(actual_type) {
        return None;
    }

    struct S<'a> {
        actual_type: &'a str,
        state_type: &'a str,
        state_name: &'a str,
        first_entry_name: &'a str,
        foliage_name: &'a str,
        leaves_name: &'a str,
    }

    let result = match (S {
        actual_type,
        state_type,
        state_name,
        first_entry_name,
        foliage_name,
        leaves_name,
    }) {
        S {
            actual_type: "minecraft:random_patch",
            state_type: "minecraft:simple_state_provider",
            state_name: "minecraft:sweet_berry_bush",
            ..
        } => "minecraft:patch_berry_bush",
        S {
            actual_type: "minecraft:random_patch",
            state_type: "minecraft:simple_state_provider",
            state_name: "minecraft:cactus",
            ..
        } => "minecraft:patch_cactus",
        S {
            actual_type: "minecraft:random_patch",
            state_type: "minecraft:weighted_state_provider",
            first_entry_name: "minecraft:grass",
            ..
        }
        | S {
            actual_type: "minecraft:random_patch",
            state_type: "minecraft:weighted_state_provider",
            first_entry_name: "minecraft:fern",
            ..
        } => "minecraft:patch_taiga_grass",
        S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:weighted_state_provider",
            first_entry_name: "minecraft:packed_ice",
            ..
        }
        | S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:weighted_state_provider",
            first_entry_name: "minecraft:blue_ice",
            ..
        } => "minecraft:pile_ice",
        S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:weighted_state_provider",
            first_entry_name: "minecraft:jack_o_lantern",
            ..
        }
        | S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:weighted_state_provider",
            first_entry_name: "minecraft:pumpkin",
            ..
        } => "minecraft:pile_pumpkin",
        S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:simple_state_provider",
            state_name: "minecraft:hay_block",
            ..
        }
        | S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:rotated_block_provider",
            state_name: "minecraft:hay_block",
            ..
        } => "minecraft:pile_hay",
        S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:simple_state_provider",
            state_name: "minecraft:melon",
            ..
        }
        | S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:rotated_block_provider",
            state_name: "minecraft:melon",
            ..
        } => "minecraft:pile_melon",
        S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:simple_state_provider",
            state_name: "minecraft:snow",
            ..
        }
        | S {
            actual_type: "minecraft:block_pile",
            state_type: "minecraft:rotated_block_provider",
            state_name: "minecraft:snow",
            ..
        } => "minecraft:pile_snow",
        S {
            actual_type: "minecraft:flower",
            ..
        } => "minecraft:flower_plain",
        S {
            actual_type: "minecraft:tree",
            foliage_name: "minecraft:acacia_foliage_placer",
            ..
        } => "minecraft:acacia",
        S {
            actual_type: "minecraft:tree",
            foliage_name: "minecraft:blob_foliage_placer",
            leaves_name: "minecraft:oak_leaves",
            ..
        } => "minecraft:oak",
        S {
            actual_type: "minecraft:tree",
            foliage_name: "minecraft:pine_foliage_placer",
            ..
        } => "minecraft:pine",
        S {
            actual_type: "minecraft:tree",
            foliage_name: "minecraft:spruce_foliage_placer",
            ..
        } => "minecraft:spruce",
        _ => return None,
    };

    Some(result.to_owned())
}
