use crate::helpers::item_stack_to_data_components_fix;
use crate::types;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 3820;

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:skull",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let skull_owner = data.remove("SkullOwner");
            let extra_type = data.remove("ExtraType");

            if skull_owner.is_none() && extra_type.is_none() {
                return;
            }

            let profile = match skull_owner.or(extra_type) {
                Some(owner) => item_stack_to_data_components_fix::convert_profile(owner),
                None => jcompound! {
                    "name" => "",
                },
            };

            data.insert("profile", profile);
        }),
    );
    types::item_stack_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::Compound(components)) = data.get_mut("components") else {
                return;
            };

            let Some(mut lodestone_tracker) = components.remove("minecraft:lodestone_target")
            else {
                return;
            };
            if let JValue::Compound(lodestone_tracker) = &mut lodestone_tracker {
                if lodestone_tracker.contains_key("pos")
                    && lodestone_tracker.contains_key("dimension")
                {
                    let pos = lodestone_tracker.remove("pos").unwrap();
                    let dim = lodestone_tracker.remove("dimension").unwrap();
                    lodestone_tracker.insert(
                        "target",
                        jcompound! {
                            "pos" => pos,
                            "dimension" => dim,
                        },
                    );
                }
            }
            components.insert("minecraft:lodestone_tracker", lodestone_tracker);
        }),
    );
}
