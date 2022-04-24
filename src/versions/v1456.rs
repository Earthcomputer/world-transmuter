use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1456;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:item_frame", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        // convert direction from 2d to 3d
        let new_dir = match data.get_i64("Facing").unwrap_or(0) as i8 {
            0 => 3,
            1 => 4,
            3 => 5,
            _ => 2,
        };
        data.set("Facing", T::Object::create_byte(new_dir));
    }));
}
