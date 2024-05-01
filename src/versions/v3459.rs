use crate::types;
use crate::versions::v3807;
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

            let mut dragon_fight = match end_data.get("DragonFight") {
                Some(JValue::Compound(dragon_fight)) => dragon_fight.clone(),
                _ => JCompound::new(),
            };
            v3807::flatten_block_pos(&mut dragon_fight, "ExitPortalLocation");

            data.insert("DragonFight", dragon_fight);
        }),
    );
}
