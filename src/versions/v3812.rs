use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use world_transmuter_engine::{map_data_converter_func, JList, JValue};

const VERSION: u32 = 3812;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:wolf",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let mut double_health = false;

            if let Some(JValue::List(JList::Compound(attributes))) = data.get_mut("Attributes") {
                for attribute in attributes {
                    let Some(JValue::String(name)) = attribute.get("Name") else {
                        continue;
                    };
                    if ResourceLocation::parse(name)
                        != Ok(ResourceLocation::minecraft("generic.max_health"))
                    {
                        continue;
                    }

                    let Some(base) = attribute.get_mut("Base") else {
                        continue;
                    };
                    let base_value = base.as_f64().unwrap_or(0.0);
                    if base_value == 20.0 {
                        *base = JValue::Double(40.0);
                        double_health = true;
                    }
                }
            }

            if double_health {
                let Some(health) = data.get_mut("Health") else {
                    return;
                };
                let health_value = health.as_f32().unwrap_or(0.0);
                *health = JValue::Float(health_value * 2.0);
            }
        }),
    );
}
