use crate::types;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 2967;

pub(crate) fn register() {
    types::world_gen_settings_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(dimensions)) = data.get_mut("dimensions") else {
                return;
            };

            for dimension in dimensions.values_mut() {
                let JValue::Compound(dimension) = dimension else {
                    continue;
                };

                let Some(JValue::Compound(generator)) = dimension.get_mut("generator") else {
                    continue;
                };

                let Some(JValue::Compound(settings)) = generator.get_mut("settings") else {
                    continue;
                };

                let Some(JValue::Compound(structures)) = settings.get_mut("structures") else {
                    continue;
                };

                for structure in structures.values_mut() {
                    if let JValue::Compound(structure) = structure {
                        structure.insert("type", "minecraft:random_spread");
                    }
                }

                if let Some(JValue::Compound(stronghold)) = structures.get_mut("stronghold") {
                    stronghold.insert("type", "minecraft:concentric_rings");
                    let stronghold = stronghold.clone();
                    structures.insert("minecraft:stronghold", stronghold);
                }
            }
        }),
    );
}
