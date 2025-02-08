use crate::{static_string_mc_set, types};
use world_transmuter_engine::{
    map_data_converter_func, DataVersion, JCompound, JList, JValue, MapDataConverterFunc,
};

const VERSION: u32 = 3322;

static_string_mc_set! {
    effect_item_types = {
        "potion",
        "splash_potion",
        "lingering_potion",
        "tipped_arrow",
    }
}

fn update_effect_list(root: &mut JCompound, path: &str) {
    let Some(JValue::List(JList::Compound(effects))) = root.get_mut(path) else {
        return;
    };
    for data in effects {
        let duration = data.get("Duration").and_then(|v| v.as_i32()).unwrap_or(-1);

        let Some(JValue::Compound(factor_data)) = data.get_mut("FactorCalculationData") else {
            continue;
        };
        let timestamp = factor_data
            .remove("effect_changed_timestamp")
            .and_then(|v| v.as_i32())
            .unwrap_or(-1);

        let ticks_active = timestamp - duration;
        factor_data.insert("ticks_active", ticks_active);
    }
}

pub(crate) fn register() {
    struct EntityEffectFix;
    impl MapDataConverterFunc for EntityEffectFix {
        fn convert(
            &self,
            data: &mut JCompound,
            _from_version: DataVersion,
            _to_version: DataVersion,
        ) {
            update_effect_list(data, "Effects");
            update_effect_list(data, "ActiveEffects");
            update_effect_list(data, "CustomPotionEffects");
        }
    }

    types::player_mut().add_structure_converter(VERSION, EntityEffectFix);
    types::entity_mut().add_structure_converter(VERSION, EntityEffectFix);

    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::String(id)) = data.get("id") else {
                return;
            };
            if !effect_item_types().contains(&id[..]) {
                return;
            }

            if let Some(JValue::Compound(tag)) = data.get_mut("tag") {
                update_effect_list(tag, "CustomPotionEffects");
            }
        }),
    );
}
