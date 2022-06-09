use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2535;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:shulker", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        // Mojang uses doubles for whatever reason... rotation is in FLOAT. by using double here
        // the entity load will just ignore rotation and set it to 0...
        if let Some(rotation) = data.get_list_mut("Rotation") {
            if !rotation.is_empty() {
                if let Some(yaw) = rotation.get(0).as_f64() {
                    rotation.set(0, T::Object::create_float((yaw - 180.0) as f32));
                }
            }
        }
    }));
}
