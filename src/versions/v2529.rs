use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2529;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_converter_for_id("minecraft:strider", VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if data.get("NoGravity").and_then(|o| o.as_ref().as_bool()) == Some(true) {
            data.set("NoGravity", T::Object::create_bool(false));
        }
    }));
}
