use crate::types;
use java_string::JavaStr;
use valence_nbt::{compound, jcompound};
use world_transmuter_engine::{
    map_data_converter_func, rename_key, DataWalkerMapTypePaths, JCompound, JValue,
};

const VERSION: u32 = 3685;

fn get_arrow_type(arrow: &JCompound) -> &'static JavaStr {
    match arrow.get("Potion") {
        Some(JValue::String(potion)) if potion != "minecraft:empty" => {
            JavaStr::from_str("minecraft:tipped_arrow")
        }
        _ => JavaStr::from_str("minecraft:arrow"),
    }
}

fn register_arrow(id: &str) {
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapTypePaths::new(types::block_state_ref(), "inBlockState"),
    );
    // new: item
    types::entity_mut().add_walker_for_id(
        VERSION,
        id,
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "item"),
    );
}

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:trident",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            rename_key(data, "Trident", "item");
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:arrow",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let arrow_type = get_arrow_type(data);
            data.insert(
                "item",
                jcompound! {
                    "id" => arrow_type,
                    "Count" => 1,
                },
            );
        }),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:spectral_arrow",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            data.insert(
                "item",
                jcompound! {
                    "id" => "minecraft:spectral_arrow",
                    "Count" => 1,
                },
            );
        }),
    );

    register_arrow("minecraft:trident");
    register_arrow("minecraft:spectral_arrow");
    register_arrow("minecraft:arrow");
}
