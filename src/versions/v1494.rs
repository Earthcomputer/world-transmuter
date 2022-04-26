use std::lazy::SyncOnceCell;
use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1494;

static ENCH_ID_TO_NAME: SyncOnceCell<rust_dataconverter_engine::Map<u8, String>> = SyncOnceCell::new();

fn ench_id_to_name() -> &'static rust_dataconverter_engine::Map<u8, String> {
    ENCH_ID_TO_NAME.get_or_init(|| {
        let mut map = rust_dataconverter_engine::Map::new();
        map.insert(0, "minecraft:protection".to_owned());
        map.insert(1, "minecraft:fire_protection".to_owned());
        map.insert(2, "minecraft:feather_falling".to_owned());
        map.insert(3, "minecraft:blast_protection".to_owned());
        map.insert(4, "minecraft:projectile_protection".to_owned());
        map.insert(5, "minecraft:respiration".to_owned());
        map.insert(6, "minecraft:aqua_affinity".to_owned());
        map.insert(7, "minecraft:thorns".to_owned());
        map.insert(8, "minecraft:depth_strider".to_owned());
        map.insert(9, "minecraft:frost_walker".to_owned());
        map.insert(10, "minecraft:binding_curse".to_owned());
        map.insert(16, "minecraft:sharpness".to_owned());
        map.insert(17, "minecraft:smite".to_owned());
        map.insert(18, "minecraft:bane_of_arthropods".to_owned());
        map.insert(19, "minecraft:knockback".to_owned());
        map.insert(20, "minecraft:fire_aspect".to_owned());
        map.insert(21, "minecraft:looting".to_owned());
        map.insert(22, "minecraft:sweeping".to_owned());
        map.insert(32, "minecraft:efficiency".to_owned());
        map.insert(33, "minecraft:silk_touch".to_owned());
        map.insert(34, "minecraft:unbreaking".to_owned());
        map.insert(35, "minecraft:fortune".to_owned());
        map.insert(48, "minecraft:power".to_owned());
        map.insert(49, "minecraft:punch".to_owned());
        map.insert(50, "minecraft:flame".to_owned());
        map.insert(51, "minecraft:infinity".to_owned());
        map.insert(61, "minecraft:luck_of_the_sea".to_owned());
        map.insert(62, "minecraft:lure".to_owned());
        map.insert(65, "minecraft:loyalty".to_owned());
        map.insert(66, "minecraft:impaling".to_owned());
        map.insert(67, "minecraft:riptide".to_owned());
        map.insert(68, "minecraft:channeling".to_owned());
        map.insert(70, "minecraft:mending".to_owned());
        map.insert(71, "minecraft:vanishing_curse".to_owned());
        map
    })
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.item_stack.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(tag) = data.get_map_mut("tag") {
            if let Some(mut ench) = tag.remove("ench").and_then(|o| o.into_list()) {
                for ench in ench.iter_mut() {
                    if let Some(ench) = ench.as_map_mut() {
                        let new_id = ench.get_i64("id").and_then(|id| ench_id_to_name().get(&(id as u8))).cloned().unwrap_or_else(|| "null".to_owned());
                        ench.set("id", T::Object::create_string(new_id));
                    }
                }

                tag.set("Enchantments", T::Object::create_list(ench));
            }

            if let Some(stored_enchants) = tag.get_list_mut("StoredEnchantments") {
                for ench in stored_enchants.iter_mut() {
                    if let Some(ench) = ench.as_map_mut() {
                        let new_id = ench.get_i64("id").and_then(|id| ench_id_to_name().get(&(id as u8))).cloned().unwrap_or_else(|| "null".to_owned());
                        ench.set("id", T::Object::create_string(new_id));
                    }
                }
            }
        }
    }));
}
