use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{Compound, Value};

const VERSION: u32 = 2531;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.block_state().borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if !matches!(data.get("Name"), Some(Value::String(str)) if str == "minecraft:redstone_wire") {
            return;
        }

        if let Some(Value::Compound(properties)) = data.get_mut("Properties") {
            let east = get_side(properties, "east");
            let west = get_side(properties, "west");
            let north = get_side(properties, "north");
            let south = get_side(properties, "south");

            let connected_x = is_connected(east) || is_connected(west);
            let connected_z = is_connected(north) || is_connected(south);

            let new_east = if !is_connected(east) && !connected_z { "side".to_owned() } else { east.to_owned() };
            let new_west = if !is_connected(west) && !connected_z { "side".to_owned() } else { west.to_owned() };
            let new_north = if !is_connected(north) && !connected_x { "side".to_owned() } else { north.to_owned() };
            let new_south = if !is_connected(south) && !connected_x { "side".to_owned() } else { south.to_owned() };

            if properties.contains_key("east") {
                properties.insert("east", new_east);
            }
            if properties.contains_key("west") {
                properties.insert("west", new_west);
            }
            if properties.contains_key("north") {
                properties.insert("north", new_north);
            }
            if properties.contains_key("south") {
                properties.insert("south", new_south);
            }
        }
    }));
}

fn get_side<'a>(properties: &'a Compound, side: &str) -> &'a str {
    match properties.get(side) {
        Some(Value::String(str)) => &str[..],
        _ => "none",
    }
}

fn is_connected(facing: &str) -> bool {
    facing != "none"
}
