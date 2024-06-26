use crate::types;
use crate::versions::v100;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 1904;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:ocelot",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let cat_type = data.get("CatType").and_then(|v| v.as_i32()).unwrap_or(0);
            if cat_type == 0 {
                if matches!(data.get("Owner"), Some(JValue::String(str)) if !str.is_empty())
                    || matches!(data.get("OwnerUUID"), Some(JValue::String(str)) if !str.is_empty())
                {
                    data.insert("Trusting", true);
                }
            } else if cat_type > 0 && cat_type < 4 {
                data.insert("id", "minecraft:cat");
                if !matches!(data.get("OwnerUUID"), Some(JValue::String(_))) {
                    data.insert("OwnerUUID", String::new());
                }
            }
        }),
    );

    v100::register_equipment(VERSION, "minecraft:cat");
}
