use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use crate::types;
use crate::versions::v1458;
use java_string::JavaString;
use world_transmuter_engine::{map_data_converter_func, JValue, JValueRef};

const VERSION: u32 = 1488;

pub(crate) fn register() {
    rename_block(VERSION, |name| match name.as_bytes() {
        b"minecraft:kelp_top" => Some(JavaString::from("minecraft:kelp")),
        b"minecraft:kelp" => Some(JavaString::from("minecraft:kelp_plant")),
        _ => None,
    });
    rename_item(
        VERSION,
        simple_rename("minecraft:kelp_top", "minecraft:kelp"),
    );

    // Don't ask me why in V1458 they wrote the converter to NOT do command blocks and THEN in THIS version
    // to ONLY do command blocks. I don't know.

    types::tile_entity_mut().add_converter_for_id(
        "minecraft:command_block",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            v1458::update_custom_name(data);
        }),
    );

    types::entity_mut().add_converter_for_id(
        "minecraft:commandblock_minecart",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            v1458::update_custom_name(data);
        }),
    );

    types::structure_feature_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::List(children)) = data.get_mut("Children") else {
                return;
            };

            let is_child_igloo = |child: JValueRef| {
                let JValueRef::Compound(child) = child else {
                    return false;
                };
                let Some(JValue::String(id)) = child.get("id") else {
                    return false;
                };
                id == "Iglu"
            };

            let is_igloo = children.iter().all(is_child_igloo);

            if is_igloo {
                data.remove("Children");
                data.insert("id", "Igloo");
            } else {
                for i in (0..children.len()).rev() {
                    if is_child_igloo(children.get(i).unwrap()) {
                        children.remove(i);
                    }
                }
            }
        }),
    );
}
