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

            let is_air =
                matches!(buy_b.get("id"), Some(JValue::String(id)) if id == "minecraft:air");
            if is_air || buy_b.get("count").and_then(|o| o.as_i32()).unwrap_or(0) <= 0 {
                data.remove("buyB");
            }
        }),
    );
}
