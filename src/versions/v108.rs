use log::warn;
use rust_dataconverter_engine::{data_converter_func, MapType, ObjectType, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 108;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    // Convert String UUID into UUIDMost and UUIDLeast
    types.entity.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(uuid) = data.remove("UUID").and_then(|o| o.into_string()) {
            let uuid = match uuid::Uuid::parse_str(&uuid) {
                Ok(uuid) => uuid,
                Err(err) => {
                    warn!("Failed to parse UUID for legacy entity (V108): {}: {}", uuid, err);
                    return;
                }
            };

            let (most, least) = uuid.as_u64_pair();
            data.set("UUIDMost", T::Object::create_long(most as i64));
            data.set("UUIDLeast", T::Object::create_long(least as i64));
        }
    }));
}
