use crate::helpers::mc_namespace_map::McNamespaceSet;
use crate::types::MinecraftTypes;
use rust_dataconverter_engine::{map_data_converter_func, DataVersion, MapDataConverterFunc};
use std::sync::OnceLock;
use valence_nbt::{Compound, List, Value};

const VERSION: u32 = 3322;

static EFFECT_ITEM_TYPES: OnceLock<McNamespaceSet> = OnceLock::new();

fn effect_item_types() -> &'static McNamespaceSet<'static> {
    EFFECT_ITEM_TYPES.get_or_init(|| {
        let mut set = McNamespaceSet::new();
        set.insert_mc("potion");
        set.insert_mc("splash_potion");
        set.insert_mc("lingering_potion");
        set.insert_mc("tipped_arrow");
        set
    })
}

fn update_effect_list(root: &mut Compound, path: &str) {
    let Some(Value::List(List::Compound(effects))) = root.get_mut(path) else {
        return;
    };
    for data in effects {
        let duration = data.get("Duration").and_then(|v| v.as_i32()).unwrap_or(-1);

        let Some(Value::Compound(factor_data)) = data.get_mut("FactorCalculationData") else {
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

pub(crate) fn register<'a>(types: &'a MinecraftTypes<'a>) {
    struct EntityEffectFix;
    impl MapDataConverterFunc for EntityEffectFix {
        fn convert(
            &self,
            data: &mut Compound,
            _from_version: DataVersion,
            _to_version: DataVersion,
        ) {
            update_effect_list(data, "Effects");
            update_effect_list(data, "ActiveEffects");
            update_effect_list(data, "CustomPotionEffects");
        }
    }

    types
        .player
        .borrow_mut()
        .add_structure_converter(VERSION, EntityEffectFix);
    types
        .entity
        .borrow_mut()
        .add_structure_converter(VERSION, EntityEffectFix);

    types.item_stack.borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::String(id)) = data.get("id") else {
                return;
            };
            if !effect_item_types().contains(&id[..]) {
                return;
            }

            if let Some(Value::Compound(tag)) = data.get_mut("tag") {
                update_effect_list(tag, "CustomPotionEffects");
            }
        }),
    );
}
