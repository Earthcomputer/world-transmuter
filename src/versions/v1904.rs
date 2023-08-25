use crate::MinecraftTypesMut;
use rust_dataconverter_engine::{map_data_converter_func, DataWalkerMapListPaths};
use valence_nbt::Value;

const VERSION: u32 = 1904;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_converter_for_id(
        "minecraft:ocelot",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let cat_type = data.get("CatType").and_then(|v| v.as_i32()).unwrap_or(0);
            if cat_type == 0 {
                if matches!(data.get("Owner"), Some(Value::String(str)) if !str.is_empty())
                    || matches!(data.get("OwnerUUID"), Some(Value::String(str)) if !str.is_empty())
                {
                    data.insert("Trusting", true);
                }
            } else if cat_type > 0 && cat_type < 4 {
                data.insert("id", "minecraft:cat");
                if !matches!(data.get("OwnerUUID"), Some(Value::String(_))) {
                    data.insert("OwnerUUID", String::new());
                }
            }
        }),
    );

    types.entity.borrow_mut().add_walker_for_id(
        VERSION,
        "minecraft:cat",
        DataWalkerMapListPaths::new_multi(
            types.item_stack,
            vec!["ArmorItems".to_owned(), "HandItems".to_owned()],
        ),
    );
}
