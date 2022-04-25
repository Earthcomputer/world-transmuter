use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1458;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    // From CB
    types.player.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        update_custom_name::<T>(data);
    }));

    types.entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("id").map(|str| str == "minecraft:commandblock_minecart") != Some(true) {
            update_custom_name::<T>(data);
        }
    }));

    types.item_stack.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(tag) = data.get_map_mut("tag") {
            if let Some(display) = tag.get_map_mut("display") {
                if let Some(name) = display.get_string("Name") {
                    let new_name = format!("{{\"text\":\"{}\"}}", name.replace('\\', "\\\\").replace('"', "\\\""));
                    display.set("Name", T::Object::create_string(new_name));
                } else if let Some(loc_name) = display.get_string("LocName") {
                    let new_name = format!("{{\"translate\":\"{}\"}}", loc_name.replace('\\', "\\\\").replace('"', "\\\""));
                    display.remove("LocName");
                    display.set("Name", T::Object::create_string(new_name));
                }
            }
        }
    }));

    types.tile_entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get_string("id").map(|str| str == "minecraft:command_block") != Some(true) {
            update_custom_name::<T>(data);
        }
    }));
}

fn update_custom_name<T: Types + ?Sized>(data: &mut T::Map) {
    if let Some(custom_name) = data.get_string("CustomName") {
        if custom_name.is_empty() {
            data.remove("CustomName");
        } else {
            let new_name = format!("{{\"text\":\"{}\"}}", custom_name.replace('\\', "\\\\").replace('"', "\\\""));
            data.set("CustomName", T::Object::create_string(new_name));
        }
    }
}
