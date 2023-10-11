use crate::helpers::spawn_egg_name_v105;
use crate::types;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 105;

pub(crate) fn register() {
    types::item_stack_mut().add_converter_for_id(
        "minecraft:spawn_egg",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let damage = data.get("Damage").and_then(|v| v.as_i16()).unwrap_or(0);
            if damage != 0 {
                data.insert("Damage", 0i16);
            }
            let JValue::Compound(tag) = data.entry("tag").or_insert_with(JCompound::new) else {
                return;
            };
            let JValue::Compound(entity_tag) =
                tag.entry("EntityTag").or_insert_with(JCompound::new)
            else {
                return;
            };
            if !matches!(entity_tag.get("id"), Some(JValue::String(_))) {
                if let Some(converted) =
                    spawn_egg_name_v105::get_spawn_name_from_id((damage & 255) as u8)
                {
                    entity_tag.insert("id", converted);
                }
            }
        }),
    );
}
