use crate::types::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::{Compound, Value};

const VERSION: u32 = 3459;

pub(crate) fn register(types: MinecraftTypesMut) {
    types.level().borrow_mut().add_structure_converter(
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
