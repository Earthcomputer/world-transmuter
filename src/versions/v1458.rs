use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{Compound, Value};

const VERSION: u32 = 1458;

pub(crate) fn register(types: MinecraftTypesMut) {
    // From CB
    types.player().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_custom_name(data);
        }),
    );

    types.entity().borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if !matches!(data.get("id"), Some(Value::String(str)) if str == "minecraft:commandblock_minecart") {
            update_custom_name(data);
        }
    }));

    types.item_stack().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::Compound(tag)) = data.get_mut("tag") {
                if let Some(Value::Compound(display)) = tag.get_mut("display") {
                    if let Some(Value::String(name)) = display.get_mut("Name") {
                        let new_name = format!(
                            "{{\"text\":\"{}\"}}",
                            name.replace('\\', "\\\\").replace('"', "\\\"")
                        );
                        *name = new_name;
                    } else if let Some(Value::String(loc_name)) = display.get("LocName") {
                        let new_name = format!(
                            "{{\"translate\":\"{}\"}}",
                            loc_name.replace('\\', "\\\\").replace('"', "\\\"")
                        );
                        display.remove("LocName");
                        display.insert("Name", new_name);
                    }
                }
            }
        }),
    );

    types.tile_entity().borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if !matches!(data.get("id"), Some(Value::String(str)) if str == "minecraft:command_block") {
            update_custom_name(data);
        }
    }));
}

pub(super) fn update_custom_name(data: &mut Compound) {
    if let Some(Value::String(custom_name)) = data.get_mut("CustomName") {
        if custom_name.is_empty() {
            data.remove("CustomName");
        } else {
            let new_name = format!(
                "{{\"text\":\"{}\"}}",
                custom_name.replace('\\', "\\\\").replace('"', "\\\"")
            );
            *custom_name = new_name;
        }
    }
}
