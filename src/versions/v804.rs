use crate::types;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 804;

pub(crate) fn register() {
    types::item_stack_mut().add_converter_for_id(
        "minecraft:banner",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(tag)) = data.get("tag") else {
                return;
            };
            let Some(JValue::Compound(block_entity)) = tag.get("BlockEntityTag") else {
                return;
            };
            let base = block_entity
                .get("Base")
                .and_then(|v| v.as_i16())
                .unwrap_or(0)
                & 15;

            data.insert("Damage", base);

            let Some(JValue::Compound(tag)) = data.get_mut("tag") else {
                unreachable!()
            };

            if let Some(JValue::Compound(display)) = tag.get("display") {
                if let Some(JValue::List(JList::String(lore))) = display.get("Lore") {
                    if lore.len() == 1 && lore[0] == "(+NBT)" {
                        return;
                    }
                }
            }

            let Some(JValue::Compound(block_entity)) = tag.get_mut("BlockEntityTag") else {
                unreachable!()
            };
            block_entity.remove("Base");
            let remove_block_entity = block_entity.is_empty();
            if remove_block_entity {
                tag.remove("BlockEntityTag");
            }

            let remove_tag = tag.is_empty();
            if remove_tag {
                data.remove("tag");
            }
        }),
    );
}
