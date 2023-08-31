use crate::MinecraftTypesMut;
use log::warn;
use rust_dataconverter_engine::map_data_converter_func;
use valence_nbt::Value;

const VERSION: u32 = 108;

pub(crate) fn register(types: MinecraftTypesMut) {
    // Convert String UUID into UUIDMost and UUIDLeast
    types.entity().borrow_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(Value::String(uuid)) = data.remove("UUID") {
                let uuid = match uuid::Uuid::parse_str(&uuid) {
                    Ok(uuid) => uuid,
                    Err(err) => {
                        warn!(
                            "Failed to parse UUID for legacy entity (V108): {}: {}",
                            uuid, err
                        );
                        return;
                    }
                };

                let (most, least) = uuid.as_u64_pair();
                data.insert("UUIDMost", most as i64);
                data.insert("UUIDLeast", least as i64);
            }
        }),
    );
}
