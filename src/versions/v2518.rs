use crate::types;
use valence_nbt::Value;
use world_transmuter_engine::map_data_converter_func;

const VERSION: u32 = 2518;

pub(crate) fn register() {
    types::tile_entity_mut().add_converter_for_id(
        "minecraft:jigsaw",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            let Some(Value::String(typ)) = data.remove("attachment_type") else {
                return;
            };
            let Some(Value::String(pool)) = data.remove("target_pool") else {
                return;
            };
            data.insert("name", typ.clone());
            data.insert("target", typ);
            data.insert("pool", pool);
        }),
    );

    types::block_state_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if matches!(data.get("Name"), Some(Value::String(str)) if str == "minecraft:jigsaw") {
                if let Some(Value::Compound(properties)) = data.get_mut("Properties") {
                    let facing = match properties.remove("facing") {
                        Some(Value::String(facing)) => Some(facing),
                        _ => None,
                    };
                    let facing = match facing.as_deref() {
                        Some("down") => "down_south",
                        Some("up") => "up_north",
                        Some("south") => "south_up",
                        Some("west") => "west_up",
                        Some("east") => "east_up",
                        _ => "north_up",
                    };
                    properties.insert("orientation", facing);
                }
            }
        }),
    );
}
