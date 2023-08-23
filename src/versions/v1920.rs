use rust_dataconverter_engine::{DataWalkerMapListPaths, map_data_converter_func};
use valence_nbt::Value;
use crate::helpers::resource_location::ResourceLocation;
use crate::MinecraftTypesMut;

const VERSION: u32 = 1920;

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        if let Some(Value::Compound(level)) = data.get_mut("Level") {
            if let Some(Value::Compound(structures)) = level.get_mut("Structures") {
                if let Some(Value::Compound(starts)) = structures.get_mut("Starts") {
                    if let Some(Value::Compound(village)) = starts.remove("New_Village") {
                        starts.insert("Village", village);
                    } else {
                        starts.remove("Village");
                    }
                }

                if let Some(Value::Compound(references)) = structures.get_mut("References") {
                    if let Some(Value::Compound(village)) = references.remove("New_Village") {
                        references.insert("Village", village);
                    } else {
                        references.remove("Village");
                    }
                }
            }
        }
    }));

    types.structure_feature.borrow_mut().add_structure_converter(VERSION, map_data_converter_func(|data, _from_version, _to_version| {
        let Some(Value::String(id)) = data.get_mut("id") else { return };
        if id.parse::<ResourceLocation>()
            .map(|rl| rl.to_string())
            .as_deref() == Ok("minecraft:new_village")
        {
            *id = "minecraft:village".to_owned();
        }
    }));

    types.tile_entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:campfire", DataWalkerMapListPaths::new(types.item_stack, "Items"));
}
