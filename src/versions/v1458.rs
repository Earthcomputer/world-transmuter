use crate::helpers::components::make_literal_component;
use crate::types;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 1458;

pub(crate) fn register() {
    // From CB
    types::player_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            update_custom_name(data);
        }),
    );

    types::entity_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if !matches!(data.get("id"), Some(JValue::String(str)) if str == "minecraft:commandblock_minecart") {
            update_custom_name(data);
        }
    }));

    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(tag)) = data.get_mut("tag") {
                if let Some(JValue::Compound(display)) = tag.get_mut("display") {
                    if let Some(JValue::String(name)) = display.get_mut("Name") {
                        *name = make_literal_component(name);
                    }
                    /* In 1.20.5, Mojang removed this branch (ItemCustomNameToComponentFix) */
                    /* else if let Some(JValue::String(loc_name)) = display.get("LocName") {
                        let new_name = format!(
                            "{{\"translate\":\"{}\"}}",
                            loc_name.replace('\\', "\\\\").replace('"', "\\\"")
                        );
                        display.remove("LocName");
                        display.insert("Name", new_name);
                    } */
                }
            }
        }),
    );

    types::tile_entity_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if !matches!(data.get("id"), Some(JValue::String(str)) if str == "minecraft:command_block") {
            update_custom_name(data);
        }
    }));
}

pub(super) fn update_custom_name(data: &mut JCompound) {
    if let Some(JValue::String(custom_name)) = data.get_mut("CustomName") {
        if custom_name.is_empty() {
            data.remove("CustomName");
        } else {
            *custom_name = make_literal_component(custom_name);
        }
    }
}
