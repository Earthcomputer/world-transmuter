use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 111;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("Painting", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        convert_entity_rotation::<T>(data, false);
    }));
    types.entity.borrow_mut().add_converter_for_id("ItemFrame", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        convert_entity_rotation::<T>(data, true);
    }));
}

fn convert_entity_rotation<T: Types + ?Sized>(data: &mut T::Map, is_item_frame: bool) {
    if data.get_i64("Facing").is_some() {
        return;
    }

    let facing = if let Some(direction) = data.remove("Direction").and_then(|o| o.as_i64()) {
        let facing = direction.rem_euclid(DIRECTIONS.len() as i64) as i8;
        let (x_off, z_off) = DIRECTIONS[facing as usize];
        data.set("TileX", T::Object::create_int(data.get_i64("TileX").unwrap_or(0) as i32 + x_off));
        data.set("TileZ", T::Object::create_int(data.get_i64("TileZ").unwrap_or(0) as i32 + z_off));
        if is_item_frame {
            if let Some(rotation) = data.get_i64("ItemRotation") {
                data.set("ItemRotation", T::Object::create_byte(rotation as i8 * 2));
            }
        }
        facing
    } else {
        data.remove("Dir").and_then(|o| o.as_i64()).unwrap_or(0).rem_euclid(DIRECTIONS.len() as i64) as i8
    };

    data.set("Facing", T::Object::create_byte(facing));
}
