use crate::types;
use java_string::{JavaStr, JavaString};
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 2531;

pub(crate) fn register() {
    types::block_state_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if !matches!(data.get("Name"), Some(JValue::String(str)) if str == "minecraft:redstone_wire") {
            return;
        }

        if let Some(JValue::Compound(properties)) = data.get_mut("Properties") {
            let east = get_side(properties, "east");
            let west = get_side(properties, "west");
            let north = get_side(properties, "north");
            let south = get_side(properties, "south");

            let connected_x = is_connected(east) || is_connected(west);
            let connected_z = is_connected(north) || is_connected(south);

            let new_east = if !is_connected(east) && !connected_z { JavaString::from("side") } else { east.to_owned() };
            let new_west = if !is_connected(west) && !connected_z { JavaString::from("side") } else { west.to_owned() };
            let new_north = if !is_connected(north) && !connected_x { JavaString::from("side") } else { north.to_owned() };
            let new_south = if !is_connected(south) && !connected_x { JavaString::from("side") } else { south.to_owned() };

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

fn get_side<'a>(properties: &'a JCompound, side: &str) -> &'a JavaStr {
    match properties.get(side) {
        Some(JValue::String(str)) => &str[..],
        _ => JavaStr::from_str("none"),
    }
}

fn is_connected(facing: &JavaStr) -> bool {
    facing != "none"
}
