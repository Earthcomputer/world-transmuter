use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 3833;

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id("minecraft:brushable_block", VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        let Some(JValue::Compound(item)) = data.get("item") else {
            return;
        };

        let id = item.get("id");
        let is_air = id.is_none() || matches!(id, Some(JValue::String(id)) if ResourceLocation::make_correct(id) == "minecraft:air");
        if is_air || item.get("count").and_then(|o| o.as_i32()).unwrap_or(0) <= 0 {
            data.remove("item");
        }
    }));
}
