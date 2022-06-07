use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::helpers::rename::rename_advancement;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2503;

static WALL_BLOCKS: SyncOnceCell<rust_dataconverter_engine::Map<&'static str, ()>> = SyncOnceCell::new();

fn wall_blocks() -> &'static rust_dataconverter_engine::Map<&'static str, ()> {
    WALL_BLOCKS.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("minecraft:andesite_wall", ());
        map.insert("minecraft:brick_wall", ());
        map.insert("minecraft:cobblestone_wall", ());
        map.insert("minecraft:diorite_wall", ());
        map.insert("minecraft:end_stone_brick_wall", ());
        map.insert("minecraft:granite_wall", ());
        map.insert("minecraft:mossy_cobblestone_wall", ());
        map.insert("minecraft:mossy_stone_brick_wall", ());
        map.insert("minecraft:nether_brick_wall", ());
        map.insert("minecraft:prismarine_wall", ());
        map.insert("minecraft:red_nether_brick_wall", ());
        map.insert("minecraft:red_sandstone_wall", ());
        map.insert("minecraft:sandstone_wall", ());
        map.insert("minecraft:stone_brick_wall", ());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.block_state.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("Name").map(|name| wall_blocks().contains_key(name)) == Some(true) {
            if let Some(properties) = data.get_map_mut("Properties") {
                for side in ["east", "west", "north", "south"] {
                    if let Some(value) = properties.get_string(side) {
                        let new_value = if value == "true" { "low" } else { "none" };
                        properties.set(side, T::Object::create_string(new_value.to_owned()));
                    }
                }
            }
        }
    }));

    rename_advancement(types, VERSION, |name| {
        if name == "minecraft:recipes/misc/composter" {
            Some("minecraft:recipes/decorations/composter".to_owned())
        } else {
            None
        }
    });
}
