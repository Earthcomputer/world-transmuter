use crate::types;
use world_transmuter_engine::{map_data_converter_func, JCompound, JValue};

const VERSION: u32 = 3459;

pub(crate) fn register() {
    types::level_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.contains_key("DragonFight") {
                return;
            }

            let Some(JValue::Compound(dimension_data)) = data.get("DimensionData") else {
                return;
            };

            let Some(JValue::Compound(end_data)) = dimension_data.get("1") else {
                return;
            };

            data.insert(
                "DragonFight",
                match end_data.get("DragonFight") {
                    Some(JValue::Compound(dragon_fight)) => dragon_fight.clone(),
                    _ => JCompound::new(),
                },
            );
        }),
    );
}
