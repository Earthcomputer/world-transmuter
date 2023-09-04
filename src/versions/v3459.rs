use crate::types;
use valence_nbt::{Compound, Value};
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 3459;

pub(crate) fn register() {
    types::level_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if data.contains_key("DragonFight") {
                return;
            }

            let Some(Value::Compound(dimension_data)) = data.get("DimensionData") else {
                return;
            };

            let Some(Value::Compound(end_data)) = dimension_data.get("1") else {
                return;
            };

            data.insert(
                "DragonFight",
                match end_data.get("DragonFight") {
                    Some(Value::Compound(dragon_fight)) => dragon_fight.clone(),
                    _ => Compound::new(),
                },
            );
        }),
    );
}
