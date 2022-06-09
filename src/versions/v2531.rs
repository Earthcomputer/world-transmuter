use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2531;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.block_state.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("Name") != Some("minecraft:redstone_wire") {
            return;
        }

        if let Some(properties) = data.get_map_mut("Properties") {
            let east = properties.get_string("east").unwrap_or("none");
            let west = properties.get_string("west").unwrap_or("none");
            let north = properties.get_string("north").unwrap_or("none");
            let south = properties.get_string("south").unwrap_or("none");

            let connected_x = is_connected(east) || is_connected(west);
            let connected_z = is_connected(north) || is_connected(south);

            let new_east = if !is_connected(east) && !connected_z { "side".to_owned() } else { east.to_owned() };
            let new_west = if !is_connected(west) && !connected_z { "side".to_owned() } else { west.to_owned() };
            let new_north = if !is_connected(north) && !connected_x { "side".to_owned() } else { north.to_owned() };
            let new_south = if !is_connected(south) && !connected_x { "side".to_owned() } else { south.to_owned() };

            if properties.has_key("east") {
                properties.set("east", T::Object::create_string(new_east));
            }
            if properties.has_key("west") {
                properties.set("west", T::Object::create_string(new_west));
            }
            if properties.has_key("north") {
                properties.set("north", T::Object::create_string(new_north));
            }
            if properties.has_key("south") {
                properties.set("south", T::Object::create_string(new_south));
            }
        }
    }));
}

fn is_connected(facing: &str) -> bool {
    facing != "none"
}
