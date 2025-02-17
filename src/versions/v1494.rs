use crate::types;
use ahash::AHashMap;
use std::sync::OnceLock;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 1494;

fn ench_id_to_name() -> &'static AHashMap<u8, &'static str> {
    static ENCH_ID_TO_NAME: OnceLock<AHashMap<u8, &'static str>> = OnceLock::new();
    ENCH_ID_TO_NAME.get_or_init(|| {
        let mut map = AHashMap::new();
        map.insert(0, "minecraft:protection");
        map.insert(1, "minecraft:fire_protection");
        map.insert(2, "minecraft:feather_falling");
        map.insert(3, "minecraft:blast_protection");
        map.insert(4, "minecraft:projectile_protection");
        map.insert(5, "minecraft:respiration");
        map.insert(6, "minecraft:aqua_affinity");
        map.insert(7, "minecraft:thorns");
        map.insert(8, "minecraft:depth_strider");
        map.insert(9, "minecraft:frost_walker");
        map.insert(10, "minecraft:binding_curse");
        map.insert(16, "minecraft:sharpness");
        map.insert(17, "minecraft:smite");
        map.insert(18, "minecraft:bane_of_arthropods");
        map.insert(19, "minecraft:knockback");
        map.insert(20, "minecraft:fire_aspect");
        map.insert(21, "minecraft:looting");
        map.insert(22, "minecraft:sweeping");
        map.insert(32, "minecraft:efficiency");
        map.insert(33, "minecraft:silk_touch");
        map.insert(34, "minecraft:unbreaking");
        map.insert(35, "minecraft:fortune");
        map.insert(48, "minecraft:power");
        map.insert(49, "minecraft:punch");
        map.insert(50, "minecraft:flame");
        map.insert(51, "minecraft:infinity");
        map.insert(61, "minecraft:luck_of_the_sea");
        map.insert(62, "minecraft:lure");
        map.insert(65, "minecraft:loyalty");
        map.insert(66, "minecraft:impaling");
        map.insert(67, "minecraft:riptide");
        map.insert(68, "minecraft:channeling");
        map.insert(70, "minecraft:mending");
        map.insert(71, "minecraft:vanishing_curse");
        map
    })
}

pub(crate) fn register() {
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                return;
            };

            if let Some(JValue::List(JList::Compound(mut ench))) = tag.remove("ench") {
                for ench in &mut ench {
                    let new_id = ench
                        .get("id")
                        .and_then(|v| v.as_i8())
                        .and_then(|id| ench_id_to_name().get(&(id as u8)))
                        .copied()
                        .unwrap_or("null");
                    ench.insert("id", new_id);
                }

                tag.insert("Enchantments", JList::Compound(ench));
            }

            if let Some(JValue::List(JList::Compound(stored_enchants))) =
                tag.get_mut("StoredEnchantments")
            {
                for ench in stored_enchants {
                    let new_id = ench
                        .get("id")
                        .and_then(|v| v.as_i8())
                        .and_then(|id| ench_id_to_name().get(&(id as u8)))
                        .copied()
                        .unwrap_or("null");
                    ench.insert("id", new_id);
                }
            }
        }),
    );
}
