use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 3828;

pub(crate) fn register() {
    types::villager_trade_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(buy_b)) = data.get("buyB") else {
                return;
            };

            let id = buy_b.get("id");
            let is_air = id.is_none() || matches!(id, Some(JValue::String(id)) if ResourceLocation::make_correct(id) == "minecraft:air");
            if is_air || buy_b.get("count").and_then(|o| o.as_i32()).unwrap_or(0) <= 0 {
                data.remove("buyB");
            }
        }),
    );
}
