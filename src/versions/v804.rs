use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 804;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.item_stack.borrow_mut().add_converter_for_id("minecraft:banner", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        let _: Option<_> = try {
            let tag = data.get_map("tag")?;
            let block_entity = tag.get_map("BlockEntityTag")?;
            let base = (block_entity.get_i64("Base").unwrap_or(0) & 15) as i16;

            data.set("Damage", T::Object::create_short(base));

            let tag = data.get_map("tag").unwrap();

            if let Some(display) = tag.get_map("display") {
                if let Some(lore) = display.get_list("Lore") {
                    if lore.size() == 1 && lore.get(0).as_string() == Some("(+NBT)") {
                        return;
                    }
                }
            }

            let tag = data.get_map_mut("tag").unwrap();
            let block_entity = tag.get_map_mut("BlockEntityTag").unwrap();
            block_entity.remove("Base");
            let remove_block_entity = block_entity.is_empty();
            if remove_block_entity {
                tag.remove("BlockEntityTag");
            }

            let remove_tag = tag.is_empty();
            if remove_tag {
                data.remove("tag");
            }
        };
    }));
}
