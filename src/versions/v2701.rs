use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2701;

static PIECE_TYPE: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, ()>> = SyncOnceCell::new();

fn piece_type() -> &'static rust_dataconverter_engine::Map<&'static str, ()> {
    PIECE_TYPE.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:jigsaw", ());
        map.insert("minecraft:nvi", ());
        map.insert("minecraft:pcp", ());
        map.insert("minecraft:bastionremnant", ());
        map.insert("minecraft:runtime", ());
        map
    })
}

static FEATURES: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, ()>> = SyncOnceCell::new();

fn features() -> &'static rust_dataconverter_engine::Map<&'static str, ()> {
    FEATURES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:tree", ());
        map.insert("minecraft:flower", ());
        map.insert("minecraft:block_pile", ());
        map.insert("minecraft:random_patch", ());
        map
    })
}

struct Getter<T> {
    value: T,
}

impl Getter<&str> {
    fn get<'a, T: Types + ?Sized>(&self, obj: &'a T::Object) -> Option<&'a T::Object> {
        obj.as_map().and_then(|o| o.get(self.value))
    }
}

impl Getter<i32> {
    fn get<'a, T: Types + ?Sized>(&self, obj: &'a T::Object) -> Option<&'a T::Object> {
        obj.as_list().and_then(|o| if (self.value as usize) < o.size() { Some(o.get(self.value as usize)) } else { None })
    }
}

macro_rules! get_nested_string {
    ($t: ty, $root:expr, $path:expr $(, $paths:tt)*) => {
        $root.get($path)
        $(
            .and_then(|o| Getter{value: $paths}.get::<$t>(o))
        )*
        .and_then(|o| o.as_string())
        .unwrap_or("")
    }
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.structure_feature.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(children) = data.get_list_mut("Children") {
            for child in children.iter_mut() {
                if let Some(child) = child.as_map_mut() {
                    let child: &mut T::Map = child;
                    if child.get_string("id").map(|id| piece_type().contains_key(id)) != Some(true) {
                        continue;
                    }
                    if child.get_string("pool_element") != Some("minecraft:feature_pool_element") {
                        continue;
                    }
                    if let Some(feature) = child.get_map("feature") {
                        if let Some(replacement) = convert_to_string::<T>(feature) {
                            child.set("feature", T::Object::create_string(replacement));
                        }
                    }
                }
            }
        }
    }));
}

fn convert_to_string<T: Types + ?Sized>(feature: &T::Map) -> Option<String> {
    get_replacement(
        get_nested_string!(T, feature, "type"),
        get_nested_string!(T, feature, "name"),
        get_nested_string!(T, feature, "config", "state_provider", "type"),
        get_nested_string!(T, feature, "config", "state_provider", "state", "Name"),
        get_nested_string!(T, feature, "config", "state_provider", "entries", 0, "data", "Name"),
        get_nested_string!(T, feature, "config", "foliage_placer", "type"),
        get_nested_string!(T, feature, "config", "leaves_provider", "state", "Name"),
    )
}

fn get_replacement(typ: &str, name: &str, state_type: &str, state_name: &str, first_entry_name: &str, foliage_name: &str, leaves_name: &str) -> Option<String> {
    let actual_type = if !typ.is_empty() {
        typ
    } else {
        match name {
            "" => return None,
            "minecraft:normal_tree" => "minecraft:tree",
            _ => name,
        }
    };

    if !features().contains_key(actual_type) {
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

    let result = match (S { actual_type, state_type, state_name, first_entry_name, foliage_name, leaves_name }) {
        S {actual_type: "minecraft:random_patch", state_type: "minecraft:simple_state_provider", state_name: "minecraft:sweet_berry_bush", ..} => "minecraft:patch_berry_bush",
        S {actual_type: "minecraft:random_patch", state_type: "minecraft:simple_state_provider", state_name: "minecraft:cactus", ..} => "minecraft:patch_cactus",
        S {actual_type: "minecraft:random_patch", state_type: "minecraft:weighted_state_provider", first_entry_name: "minecraft:grass", ..} |
        S {actual_type: "minecraft:random_patch", state_type: "minecraft:weighted_state_provider", first_entry_name: "minecraft:fern", ..} => "minecraft:patch_taiga_grass",
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:weighted_state_provider", first_entry_name: "minecraft:packed_ice", ..} |
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:weighted_state_provider", first_entry_name: "minecraft:blue_ice", ..} => "minecraft:pile_ice",
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:weighted_state_provider", first_entry_name: "minecraft:jack_o_lantern", ..} |
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:weighted_state_provider", first_entry_name: "minecraft:pumpkin", ..} => "minecraft:pile_pumpkin",
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:simple_state_provider", state_name: "minecraft:hay_block", ..} |
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:rotated_block_provider", state_name: "minecraft:hay_block", ..} => "minecraft:pile_hay",
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:simple_state_provider", state_name: "minecraft:melon", ..} |
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:rotated_block_provider", state_name: "minecraft:melon", ..} => "minecraft:pile_melon",
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:simple_state_provider", state_name: "minecraft:snow", ..} |
        S {actual_type: "minecraft:block_pile", state_type: "minecraft:rotated_block_provider", state_name: "minecraft:snow", ..} => "minecraft:pile_snow",
        S {actual_type: "minecraft:flower", ..} => "minecraft:flower_plain",
        S {actual_type: "minecraft:tree", foliage_name: "minecraft:acacia_foliage_placer", ..} => "minecraft:acacia",
        S {actual_type: "minecraft:tree", foliage_name: "minecraft:blob_foliage_placer", leaves_name: "minecraft:oak_leaves", ..} => "minecraft:oak",
        S {actual_type: "minecraft:tree", foliage_name: "minecraft:pine_foliage_placer", ..} => "minecraft:pine",
        S {actual_type: "minecraft:tree", foliage_name: "minecraft:spruce_foliage_placer", ..} => "minecraft:spruce",
        _ => return None
    };

    Some(result.to_owned())
}
