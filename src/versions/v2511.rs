use crate::types;
use world_transmuter_engine::{
    map_data_converter_func, DataVersion, DataWalkerMapTypePaths, JCompound, JValue,
};

const VERSION: u32 = 2511;

pub(crate) fn register() {
    types::entity_mut().add_converter_for_id(
        "minecraft:egg",
        VERSION,
        map_data_converter_func(throwable_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:ender_pearl",
        VERSION,
        map_data_converter_func(throwable_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:experience_bottle",
        VERSION,
        map_data_converter_func(throwable_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:snowball",
        VERSION,
        map_data_converter_func(throwable_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:potion",
        VERSION,
        map_data_converter_func(throwable_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:potion",
        VERSION,
        map_data_converter_func(potion_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:llama_spit",
        VERSION,
        map_data_converter_func(llama_spit_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:arrow",
        VERSION,
        map_data_converter_func(arrow_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:spectral_arrow",
        VERSION,
        map_data_converter_func(arrow_converter),
    );
    types::entity_mut().add_converter_for_id(
        "minecraft:trident",
        VERSION,
        map_data_converter_func(arrow_converter),
    );

    // Vanilla migrates the potion item but does not change the schema.
    types::entity_mut().add_walker_for_id(
        VERSION,
        "minecraft:potion",
        DataWalkerMapTypePaths::new(types::item_stack_ref(), "Item"),
    );
}

fn set_uuid(data: &mut JCompound, most: i64, least: i64) {
    // if either most or least is 0, that's an invalid uuid so don't convert
    // this checks for cases where there wasn't a uuid to start with
    if most != 0 && least != 0 {
        data.insert(
            "OwnerUUID",
            vec![
                (most as u64 >> 32) as i32,
                most as i32,
                (least as u64 >> 32) as i32,
                least as i32,
            ],
        );
    }
}

fn throwable_converter(data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
    if let Some(JValue::Compound(owner)) = data.remove("owner") {
        set_uuid(
            data,
            owner.get("M").and_then(|v| v.as_i64()).unwrap_or(0),
            owner.get("L").and_then(|v| v.as_i64()).unwrap_or(0),
        );
    }
}

fn potion_converter(data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
    if let Some(JValue::Compound(potion)) = data.remove("Potion") {
        data.insert("Item", potion);
    } else {
        data.insert("Item", JCompound::new());
    }
}

fn llama_spit_converter(
    data: &mut JCompound,
    _from_version: DataVersion,
    _to_version: DataVersion,
) {
    if let Some(JValue::Compound(owner)) = data.remove("Owner") {
        set_uuid(
            data,
            owner
                .get("OwnerUUIDMost")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            owner
                .get("OwnerUUIDLeast")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
        );
    }
}

fn arrow_converter(data: &mut JCompound, _from_version: DataVersion, _to_version: DataVersion) {
    let most = data
        .remove("OwnerUUIDMost")
        .and_then(|o| o.as_i64())
        .unwrap_or(0);
    let least = data
        .remove("OwnerUUIDLeast")
        .and_then(|o| o.as_i64())
        .unwrap_or(0);
    set_uuid(data, most, least);
}
