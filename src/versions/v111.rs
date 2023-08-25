use crate::MinecraftTypesMut;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Compound;

const VERSION: u32 = 111;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

pub(crate) fn register(types: &MinecraftTypesMut) {
    types.entity.borrow_mut().add_converter_for_id(
        "Painting",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            convert_entity_rotation(data, false);
        }),
    );
    types.entity.borrow_mut().add_converter_for_id(
        "ItemFrame",
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            convert_entity_rotation(data, true);
        }),
    );
}

fn convert_entity_rotation(data: &mut Compound, is_item_frame: bool) {
    if data.get("Facing").map(|v| v.is_number()) == Some(true) {
        return;
    }

    let facing = if let Some(direction) = data.remove("Direction").and_then(|o| o.as_i8()) {
        let facing = direction.rem_euclid(DIRECTIONS.len() as i8);
        let (x_off, z_off) = DIRECTIONS[facing as usize];
        data.insert(
            "TileX",
            data.get("TileX").and_then(|v| v.as_i32()).unwrap_or(0) + x_off,
        );
        data.insert(
            "TileZ",
            data.get("TileZ").and_then(|v| v.as_i32()).unwrap_or(0) + z_off,
        );
        if is_item_frame {
            if let Some(rotation) = data.get("ItemRotation").and_then(|v| v.as_i8()) {
                data.insert("ItemRotation", rotation * 2);
            }
        }
        facing
    } else {
        data.remove("Dir")
            .and_then(|o| o.as_i64())
            .unwrap_or(0)
            .rem_euclid(DIRECTIONS.len() as i64) as i8
    };

    data.insert("Facing", facing);
}
