use crate::types::MinecraftTypesMut;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 2967;

pub(crate) fn register(types: MinecraftTypesMut) {
    types
        .world_gen_settings()
        .borrow_mut()
        .add_structure_converter(
            VERSION,
            map_data_converter_func(|data, _from_version, _to_version| {
                let Some(Value::Compound(dimensions)) = data.get_mut("dimensions") else {
                    return;
                };

                for dimension in dimensions.values_mut() {
                    let Value::Compound(dimension) = dimension else {
                        continue;
                    };

                    let Some(Value::Compound(generator)) = dimension.get_mut("generator") else {
                        continue;
                    };

                    let Some(Value::Compound(settings)) = generator.get_mut("settings") else {
                        continue;
                    };

                    let Some(Value::Compound(structures)) = settings.get_mut("structures") else {
                        continue;
                    };

                    for structure in structures.values_mut() {
                        if let Value::Compound(structure) = structure {
                            structure.insert("type", "minecraft:random_spread");
                        }
                    }

                    if let Some(Value::Compound(stronghold)) = structures.get_mut("stronghold") {
                        stronghold.insert("type", "minecraft:concentric_rings");
                        let stronghold = stronghold.clone();
                        structures.insert("minecraft:stronghold", stronghold);
                    }
                }
            }),
        );
}
