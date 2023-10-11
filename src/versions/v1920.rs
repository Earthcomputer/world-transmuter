use crate::helpers::resource_location::ResourceLocation;
use crate::types;
use java_string::{JavaStr, JavaString};
use world_transmuter_engine::{map_data_converter_func, DataWalkerMapListPaths, JValue};

const VERSION: u32 = 1920;

pub(crate) fn register() {
    types::chunk_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::Compound(level)) = data.get_mut("Level") {
                if let Some(JValue::Compound(structures)) = level.get_mut("Structures") {
                    if let Some(JValue::Compound(starts)) = structures.get_mut("Starts") {
                        if let Some(JValue::Compound(village)) = starts.remove("New_Village") {
                            starts.insert("Village", village);
                        } else {
                            starts.remove("Village");
                        }
                    }

                    if let Some(JValue::Compound(references)) = structures.get_mut("References") {
                        if let Some(JValue::Compound(village)) = references.remove("New_Village") {
                            references.insert("Village", village);
                        } else {
                            references.remove("Village");
                        }
                    }
                }
            }
        }),
    );

    types::structure_feature_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(JValue::String(id)) = data.get_mut("id") else {
                return;
            };
            if ResourceLocation::parse(id)
                .map(|rl| rl.to_java_string())
                .as_deref()
                == Ok(JavaStr::from_str("minecraft:new_village"))
            {
                *id = JavaString::from("minecraft:village");
            }
        }),
    );

    types::tile_entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:campfire",
        DataWalkerMapListPaths::new(types::item_stack_ref(), "Items"),
    );
}
