use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2702;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    for id in ["minecraft:arrow", "minecraft:spectral_arrow", "minecraft:trident"] {
        types.entity.borrow_mut().add_converter_for_id(id, VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
            if data.has_key("pickup") {
                return;
            }
            let player = data.remove("player").and_then(|o| o.as_bool()).unwrap_or(true);
            data.set("pickup", T::Object::create_byte(if player { 1 } else { 0 }));
        }));
    }
}
