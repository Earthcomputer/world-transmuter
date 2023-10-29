use crate::types;
use tracing::warn;
use world_transmuter_engine::{map_data_converter_func, JValue};

const VERSION: u32 = 108;

pub(crate) fn register() {
    // Convert String UUID into UUIDMost and UUIDLeast
    types::entity_mut().add_structure_converter(
        VERSION,
        map_data_converter_func(|data, _from_version, _to_version| {
            if let Some(JValue::String(uuid)) = data.remove("UUID") {
                let uuid = match uuid.as_str().map(uuid::Uuid::parse_str) {
                    Ok(Ok(uuid)) => uuid,
                    Ok(Err(err)) => {
                        warn!(
                            "Failed to parse UUID for legacy entity (V108): {}: {}",
                            uuid, err
                        );
                        return;
                    }
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
