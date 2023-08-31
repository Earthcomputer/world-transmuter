use crate::helpers::item_name_v102;
use crate::MinecraftTypesMut;
use log::warn;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{Compound, Value};

const VERSION: u32 = 102;

pub(crate) fn register(types: MinecraftTypesMut) {
    // V102 schema only modifies ITEM_STACK to have only a string ID, but our ITEM_NAME is generic (int or String) so we don't
    // actually need to update the walker

    // We skip the item_name data converter here because our data converters do not support changing types. Instead(), we add
    // converters for flower pots and item stacks, which are the only two places that have item_name in their walkers at this
    // version.

    // Note: Vanilla does not properly handle this case, it will not convert int ids!
    types.tile_entity().borrow_mut().add_converter_for_id(
        "FlowerPot",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(item) = data.get("Item").and_then(|v| v.as_i32()) {
                let name = item_name_v102::get_name_from_id(item).unwrap_or_else(|| {
                    warn!("Unknown legacy integer id (V102): {}", item);
                    item_name_v102::get_name_from_id(0).unwrap()
                });
                data.insert("Item", name);
            }
        }),
    );

    types.item_stack().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(id) = data.get("id").and_then(|v| v.as_i32()) {
                let name = item_name_v102::get_name_from_id(id).unwrap_or_else(|| {
                    warn!("Unknown legacy integer id (V102): {}", id);
                    item_name_v102::get_name_from_id(0).unwrap()
                });
                data.insert("id", name);
            }
        }),
    );

    types.item_stack().borrow_mut().add_converter_for_id(
        "minecraft:potion",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(damage) = data.get("Damage").and_then(|v| v.as_i16()) {
                if damage != 0 {
                    data.insert("Damage", 0i16);
                }
                let tag = data.entry("tag").or_insert_with(Compound::new);
                if let Value::Compound(tag) = tag {
                    if !matches!(tag.get("Potion"), Some(Value::String(_))) {
                        let converted = item_name_v102::get_potion_name_from_id(damage as i32)
                            .unwrap_or("minecraft:water");
                        tag.insert("Potion", converted);
                        if (damage & 16384) == 16384 {
                            data.insert("id", "minecraft:splash_potion");
                        }
                    }
                }
            }
        }),
    );
}
