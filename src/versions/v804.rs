use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{List, Value};
use crate::MinecraftTypesMut;

const VERSION: u32 = 804;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:banner", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        let Some(Value::Compound(tag)) = data.get("tag") else { return };
        let Some(Value::Compound(block_entity)) = tag.get("BlockEntityTag") else { return };
        let base = block_entity.get("Base").and_then(|v| v.as_i16()).unwrap_or(0) & 15;

        data.insert("Damage", base);

        let Some(Value::Compound(tag)) = data.get_mut("tag") else { unreachable!() };

        if let Some(Value::Compound(display)) = tag.get("display") {
            if let Some(Value::List(List::String(lore))) = display.get("Lore") {
                if lore.len() == 1 && lore[0] == "(+NBT)" {
                    return;
                }
            }
        }

        let Some(Value::Compound(block_entity)) = tag.get_mut("BlockEntityTag") else { unreachable!() };
        block_entity.remove("Base");
        let remove_block_entity = block_entity.is_empty();
        if remove_block_entity {
            tag.remove("BlockEntityTag");
        }

        let remove_tag = tag.is_empty();
        if remove_tag {
            data.remove("tag");
        }
    }));
}
