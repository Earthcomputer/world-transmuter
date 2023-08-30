use crate::helpers::spawn_egg_name_v105;
use crate::MinecraftTypes;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{Compound, Value};

const VERSION: u32 = 105;

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    types.item_stack.borrow_mut().add_converter_for_id(
        "minecraft:spawn_egg",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let damage = data.get("Damage").and_then(|v| v.as_i16()).unwrap_or(0);
            if damage != 0 {
                data.insert("Damage", 0i16);
            }
            let Value::Compound(tag) = data.entry("tag").or_insert_with(Compound::new) else {
                return;
            };
            let Value::Compound(entity_tag) = tag.entry("EntityTag").or_insert_with(Compound::new)
            else {
                return;
            };
            if !matches!(entity_tag.get("id"), Some(Value::String(_))) {
                if let Some(converted) =
                    spawn_egg_name_v105::get_spawn_name_from_id((damage & 255) as u8)
                {
                    entity_tag.insert("id", converted);
                }
            }
        }),
    );
}
