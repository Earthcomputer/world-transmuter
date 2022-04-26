use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1492;

static RENAMES: SyncOnceCell<rust_dataconverter_engine::Map<String, (String, rust_dataconverter_engine::Map<String, String>)>> = SyncOnceCell::new();

fn renames() -> &'static rust_dataconverter_engine::Map<String, (String, rust_dataconverter_engine::Map<String, String>)> {
    RENAMES.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert("EndCity".to_owned(), ("ECP".to_owned(), {
            let mut map = rust_dataconverter_engine::Map::new();
            map.insert("second_floor".to_owned(), "second_floor_1".to_owned());
            map.insert("third_floor".to_owned(), "third_floor_1".to_owned());
            map.insert("third_floor_c".to_owned(), "third_floor_2".to_owned());
            map
        }));
        map.insert("Mansion".to_owned(), ("WMP".to_owned(), {
            let mut map = rust_dataconverter_engine::Map::new();
            map.insert("carpet_south".to_owned(), "carpet_south_1".to_owned());
            map.insert("carpet_west".to_owned(), "carpet_west_1".to_owned());
            map.insert("indoors_door".to_owned(), "indoors_door_1".to_owned());
            map.insert("indoors_wall".to_owned(), "indoors_wall_1".to_owned());
            map
        }));
        map.insert("Igloo".to_owned(), ("Iglu".to_owned(), {
            let mut map = rust_dataconverter_engine::Map::new();
            map.insert("minecraft:igloo/igloo_bottom".to_owned(), "minecraft:igloo/bottom".to_owned());
            map.insert("minecraft:igloo/igloo_middle".to_owned(), "minecraft:igloo/middle".to_owned());
            map.insert("minecraft:igloo/igloo_top".to_owned(), "minecraft:igloo/top".to_owned());
            map
        }));
        map.insert("Ocean_Ruin".to_owned(), ("ORP".to_owned(), {
            let mut map = rust_dataconverter_engine::Map::new();
            map.insert("minecraft:ruin/big_ruin1_brick".to_owned(), "minecraft:underwater_ruin/big_brick_1".to_owned());
            map.insert("minecraft:ruin/big_ruin2_brick".to_owned(), "minecraft:underwater_ruin/big_brick_2".to_owned());
            map.insert("minecraft:ruin/big_ruin3_brick".to_owned(), "minecraft:underwater_ruin/big_brick_3".to_owned());
            map.insert("minecraft:ruin/big_ruin8_brick".to_owned(), "minecraft:underwater_ruin/big_brick_8".to_owned());
            map.insert("minecraft:ruin/big_ruin1_cracked".to_owned(), "minecraft:underwater_ruin/big_cracked_1".to_owned());
            map.insert("minecraft:ruin/big_ruin2_cracked".to_owned(), "minecraft:underwater_ruin/big_cracked_2".to_owned());
            map.insert("minecraft:ruin/big_ruin3_cracked".to_owned(), "minecraft:underwater_ruin/big_cracked_3".to_owned());
            map.insert("minecraft:ruin/big_ruin8_cracked".to_owned(), "minecraft:underwater_ruin/big_cracked_8".to_owned());
            map.insert("minecraft:ruin/big_ruin1_mossy".to_owned(), "minecraft:underwater_ruin/big_mossy_1".to_owned());
            map.insert("minecraft:ruin/big_ruin2_mossy".to_owned(), "minecraft:underwater_ruin/big_mossy_2".to_owned());
            map.insert("minecraft:ruin/big_ruin3_mossy".to_owned(), "minecraft:underwater_ruin/big_mossy_3".to_owned());
            map.insert("minecraft:ruin/big_ruin8_mossy".to_owned(), "minecraft:underwater_ruin/big_mossy_8".to_owned());
            map.insert("minecraft:ruin/big_ruin_warm4".to_owned(), "minecraft:underwater_ruin/big_warm_4".to_owned());
            map.insert("minecraft:ruin/big_ruin_warm5".to_owned(), "minecraft:underwater_ruin/big_warm_5".to_owned());
            map.insert("minecraft:ruin/big_ruin_warm6".to_owned(), "minecraft:underwater_ruin/big_warm_6".to_owned());
            map.insert("minecraft:ruin/big_ruin_warm7".to_owned(), "minecraft:underwater_ruin/big_warm_7".to_owned());
            map.insert("minecraft:ruin/ruin1_brick".to_owned(), "minecraft:underwater_ruin/brick_1".to_owned());
            map.insert("minecraft:ruin/ruin2_brick".to_owned(), "minecraft:underwater_ruin/brick_2".to_owned());
            map.insert("minecraft:ruin/ruin3_brick".to_owned(), "minecraft:underwater_ruin/brick_3".to_owned());
            map.insert("minecraft:ruin/ruin4_brick".to_owned(), "minecraft:underwater_ruin/brick_4".to_owned());
            map.insert("minecraft:ruin/ruin5_brick".to_owned(), "minecraft:underwater_ruin/brick_5".to_owned());
            map.insert("minecraft:ruin/ruin6_brick".to_owned(), "minecraft:underwater_ruin/brick_6".to_owned());
            map.insert("minecraft:ruin/ruin7_brick".to_owned(), "minecraft:underwater_ruin/brick_7".to_owned());
            map.insert("minecraft:ruin/ruin8_brick".to_owned(), "minecraft:underwater_ruin/brick_8".to_owned());
            map.insert("minecraft:ruin/ruin1_cracked".to_owned(), "minecraft:underwater_ruin/cracked_1".to_owned());
            map.insert("minecraft:ruin/ruin2_cracked".to_owned(), "minecraft:underwater_ruin/cracked_2".to_owned());
            map.insert("minecraft:ruin/ruin3_cracked".to_owned(), "minecraft:underwater_ruin/cracked_3".to_owned());
            map.insert("minecraft:ruin/ruin4_cracked".to_owned(), "minecraft:underwater_ruin/cracked_4".to_owned());
            map.insert("minecraft:ruin/ruin5_cracked".to_owned(), "minecraft:underwater_ruin/cracked_5".to_owned());
            map.insert("minecraft:ruin/ruin6_cracked".to_owned(), "minecraft:underwater_ruin/cracked_6".to_owned());
            map.insert("minecraft:ruin/ruin7_cracked".to_owned(), "minecraft:underwater_ruin/cracked_7".to_owned());
            map.insert("minecraft:ruin/ruin8_cracked".to_owned(), "minecraft:underwater_ruin/cracked_8".to_owned());
            map.insert("minecraft:ruin/ruin1_mossy".to_owned(), "minecraft:underwater_ruin/mossy_1".to_owned());
            map.insert("minecraft:ruin/ruin2_mossy".to_owned(), "minecraft:underwater_ruin/mossy_2".to_owned());
            map.insert("minecraft:ruin/ruin3_mossy".to_owned(), "minecraft:underwater_ruin/mossy_3".to_owned());
            map.insert("minecraft:ruin/ruin4_mossy".to_owned(), "minecraft:underwater_ruin/mossy_4".to_owned());
            map.insert("minecraft:ruin/ruin5_mossy".to_owned(), "minecraft:underwater_ruin/mossy_5".to_owned());
            map.insert("minecraft:ruin/ruin6_mossy".to_owned(), "minecraft:underwater_ruin/mossy_6".to_owned());
            map.insert("minecraft:ruin/ruin7_mossy".to_owned(), "minecraft:underwater_ruin/mossy_7".to_owned());
            map.insert("minecraft:ruin/ruin8_mossy".to_owned(), "minecraft:underwater_ruin/mossy_8".to_owned());
            map.insert("minecraft:ruin/ruin_warm1".to_owned(), "minecraft:underwater_ruin/warm_1".to_owned());
            map.insert("minecraft:ruin/ruin_warm2".to_owned(), "minecraft:underwater_ruin/warm_2".to_owned());
            map.insert("minecraft:ruin/ruin_warm3".to_owned(), "minecraft:underwater_ruin/warm_3".to_owned());
            map.insert("minecraft:ruin/ruin_warm4".to_owned(), "minecraft:underwater_ruin/warm_4".to_owned());
            map.insert("minecraft:ruin/ruin_warm5".to_owned(), "minecraft:underwater_ruin/warm_5".to_owned());
            map.insert("minecraft:ruin/ruin_warm6".to_owned(), "minecraft:underwater_ruin/warm_6".to_owned());
            map.insert("minecraft:ruin/ruin_warm7".to_owned(), "minecraft:underwater_ruin/warm_7".to_owned());
            map.insert("minecraft:ruin/ruin_warm8".to_owned(), "minecraft:underwater_ruin/warm_8".to_owned());
            map.insert("minecraft:ruin/big_brick_1".to_owned(), "minecraft:underwater_ruin/big_brick_1".to_owned());
            map.insert("minecraft:ruin/big_brick_2".to_owned(), "minecraft:underwater_ruin/big_brick_2".to_owned());
            map.insert("minecraft:ruin/big_brick_3".to_owned(), "minecraft:underwater_ruin/big_brick_3".to_owned());
            map.insert("minecraft:ruin/big_brick_8".to_owned(), "minecraft:underwater_ruin/big_brick_8".to_owned());
            map.insert("minecraft:ruin/big_mossy_1".to_owned(), "minecraft:underwater_ruin/big_mossy_1".to_owned());
            map.insert("minecraft:ruin/big_mossy_2".to_owned(), "minecraft:underwater_ruin/big_mossy_2".to_owned());
            map.insert("minecraft:ruin/big_mossy_3".to_owned(), "minecraft:underwater_ruin/big_mossy_3".to_owned());
            map.insert("minecraft:ruin/big_mossy_8".to_owned(), "minecraft:underwater_ruin/big_mossy_8".to_owned());
            map.insert("minecraft:ruin/big_cracked_1".to_owned(), "minecraft:underwater_ruin/big_cracked_1".to_owned());
            map.insert("minecraft:ruin/big_cracked_2".to_owned(), "minecraft:underwater_ruin/big_cracked_2".to_owned());
            map.insert("minecraft:ruin/big_cracked_3".to_owned(), "minecraft:underwater_ruin/big_cracked_3".to_owned());
            map.insert("minecraft:ruin/big_cracked_8".to_owned(), "minecraft:underwater_ruin/big_cracked_8".to_owned());
            map.insert("minecraft:ruin/big_warm_4".to_owned(), "minecraft:underwater_ruin/big_warm_4".to_owned());
            map.insert("minecraft:ruin/big_warm_5".to_owned(), "minecraft:underwater_ruin/big_warm_5".to_owned());
            map.insert("minecraft:ruin/big_warm_6".to_owned(), "minecraft:underwater_ruin/big_warm_6".to_owned());
            map.insert("minecraft:ruin/big_warm_7".to_owned(), "minecraft:underwater_ruin/big_warm_7".to_owned());
            map
        }));
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.structure_feature.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let id = match data.get_string("id") {
            Some(id) => id,
            None => return
        };

        let (child_id, template_renames) = match renames().get(id) {
            Some(pair) => pair,
            None => return
        };

        if let Some(children) = data.get_list_mut("Children") {
            for child in children.iter_mut() {
                if let Some(child) = child.as_map_mut() {
                    if child.get_string("id").map(|str| str == child_id) == Some(true) {
                        if let Some(new_template) = child.get_string("Template").and_then(|template| template_renames.get(template)) {
                            child.set("Template", T::Object::create_string(new_template.clone()));
                        }
                    }
                }
            }
        }
    }));
}
