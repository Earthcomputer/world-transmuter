use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use crate::versions::v1458;
use crate::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::value::ValueRef;
use valence_nbt::Value;

const VERSION: u32 = 1488;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    rename_block(types, VERSION, |name| match name {
        "minecraft:kelp_top" => Some("minecraft:kelp".to_owned()),
        "minecraft:kelp" => Some("minecraft:kelp_plant".to_owned()),
        _ => None,
    });
    rename_item(
        types,
        VERSION,
        simple_rename("minecraft:kelp_top", "minecraft:kelp"),
    );

    // Don't ask me why in V1458 they wrote the converter to NOT do command blocks and THEN in THIS version
    // to ONLY do command blocks. I don't know.

    types.tile_entity.borrow_mut().add_converter_for_id(
        "minecraft:command_block",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            v1458::update_custom_name(data);
        }),
    );

    types.entity.borrow_mut().add_converter_for_id(
        "minecraft:commandblock_minecart",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            v1458::update_custom_name(data);
        }),
    );

    types
        .structure_feature
        .borrow_mut()
        .add_structure_converter(
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                let Some(Value::List(children)) = data.get_mut("Children") else {
                    return;
                };

                let is_child_igloo = |child: ValueRef| {
                    let ValueRef::Compound(child) = child else {
                        return false;
                    };
                    let Some(Value::String(id)) = child.get("id") else {
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
