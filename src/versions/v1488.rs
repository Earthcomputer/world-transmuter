use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use crate::MinecraftTypesMut;
use crate::versions::v1458;

const VERSION: u32 = 1488;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_block::<T>(types, VERSION, |name| {
        match name {
            "minecraft:kelp_top" => Some("minecraft:kelp".to_owned()),
            "minecraft:kelp" => Some("minecraft:kelp_plant".to_owned()),
            _ => None
        }
    });
    rename_item::<T>(types, VERSION, simple_rename("minecraft:kelp_top", "minecraft:kelp"));

    // Don't ask me why in V1458 they wrote the converter to NOT do command blocks and THEN in THIS version
    // to ONLY do command blocks. I don't know.

    types.tile_entity.borrow_mut().add_converter_for_id("minecraft:command_block", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        v1458::update_custom_name::<T>(data);
    }));

    types.entity.borrow_mut().add_converter_for_id("minecraft:commandblock_minecart", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        v1458::update_custom_name::<T>(data);
    }));

    types.structure_feature.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let children = match data.get_list_mut("Children") {
            Some(list) => list,
            None => return
        };

        let is_child_igloo = |child: &T::Object| {
            child.as_map().and_then(|child| child.get_string("id")).map(|str| str == "Iglu") == Some(true)
        };

        let is_igloo = children.iter().all(is_child_igloo);

        if is_igloo {
            data.remove("Children");
            data.set("id", T::Object::create_string("Igloo".to_owned()));
        } else {
            for i in (0..children.size()).rev() {
                if is_child_igloo(children.get(i)) {
                    children.remove(i);
                }
            }
        }
    }));
}
